use std::sync::Arc;

use nohash_hasher::IntMap;
use parking_lot::Mutex;

use re_chunk::{Chunk, ChunkResult, RowId};
use re_chunk_store::{
    ChunkStore, ChunkStoreConfig, ChunkStoreEvent, ChunkStoreSubscriber, GarbageCollectionOptions,
    GarbageCollectionTarget,
};
use re_log_types::{
    ApplicationId, ComponentPath, EntityPath, EntityPathHash, LogMsg, ResolvedTimeRange,
    ResolvedTimeRangeF, SetStoreInfo, StoreId, StoreInfo, StoreKind, Timeline,
};

use crate::{Error, TimesPerTimeline};

// ----------------------------------------------------------------------------

/// See [`GarbageCollectionOptions::time_budget`].
const DEFAULT_GC_TIME_BUDGET: std::time::Duration = std::time::Duration::from_micros(3500); // empirical

// ----------------------------------------------------------------------------

/// An in-memory database built from a stream of [`LogMsg`]es.
///
/// NOTE: all mutation is to be done via public functions!
pub struct EntityDb {
    /// Set by whomever created this [`EntityDb`].
    ///
    /// Clones of an [`EntityDb`] gets a `None` source.
    pub data_source: Option<re_smart_channel::SmartChannelSource>,

    /// Comes in a special message, [`LogMsg::SetStoreInfo`].
    set_store_info: Option<SetStoreInfo>,

    /// Keeps track of the last time data was inserted into this store (viewer wall-clock).
    last_modified_at: web_time::Instant,

    /// The highest `RowId` in the store,
    /// which corresponds to the last edit time.
    /// Ignores deletions.
    latest_row_id: Option<RowId>,

    /// In many places we just store the hashes, so we need a way to translate back.
    entity_path_from_hash: IntMap<EntityPathHash, EntityPath>,

    /// The global-scope time tracker.
    ///
    /// For each timeline, keeps track of what times exist, recursively across all
    /// entities/components.
    ///
    /// Used for time control.
    times_per_timeline: TimesPerTimeline,

    /// A tree-view (split on path components) of the entities.
    tree: crate::EntityTree,

    /// Stores all components for all entities for all timelines.
    data_store: ChunkStore,

    /// The active promise resolver for this DB.
    resolver: re_query::PromiseResolver,

    /// Query caches for the data in [`Self::data_store`].
    query_caches: re_query::Caches,

    stats: IngestionStatistics,
}

impl EntityDb {
    pub fn new(store_id: StoreId) -> Self {
        Self::with_store_config(store_id, ChunkStoreConfig::from_env().unwrap_or_default())
    }

    pub fn with_store_config(store_id: StoreId, store_config: ChunkStoreConfig) -> Self {
        let data_store = ChunkStore::new(store_id.clone(), store_config);
        let query_caches = re_query::Caches::new(&data_store);

        Self {
            data_source: None,
            set_store_info: None,
            last_modified_at: web_time::Instant::now(),
            latest_row_id: None,
            entity_path_from_hash: Default::default(),
            times_per_timeline: Default::default(),
            tree: crate::EntityTree::root(),
            data_store,
            resolver: re_query::PromiseResolver::default(),
            query_caches,
            stats: IngestionStatistics::new(store_id),
        }
    }

    #[inline]
    pub fn tree(&self) -> &crate::EntityTree {
        &self.tree
    }

    #[inline]
    pub fn data_store(&self) -> &ChunkStore {
        &self.data_store
    }

    pub fn store_info_msg(&self) -> Option<&SetStoreInfo> {
        self.set_store_info.as_ref()
    }

    pub fn store_info(&self) -> Option<&StoreInfo> {
        self.store_info_msg().map(|msg| &msg.info)
    }

    pub fn app_id(&self) -> Option<&ApplicationId> {
        self.store_info().map(|ri| &ri.application_id)
    }

    #[inline]
    pub fn query_caches(&self) -> &re_query::Caches {
        &self.query_caches
    }

    #[inline]
    pub fn resolver(&self) -> &re_query::PromiseResolver {
        &self.resolver
    }

    /// Queries for the given `component_names` using latest-at semantics.
    ///
    /// See [`re_query::LatestAtResults`] for more information about how to handle the results.
    ///
    /// This is a cached API -- data will be lazily cached upon access.
    #[inline]
    pub fn latest_at(
        &self,
        query: &re_chunk_store::LatestAtQuery,
        entity_path: &EntityPath,
        component_names: impl IntoIterator<Item = re_types_core::ComponentName>,
    ) -> re_query::LatestAtResults {
        self.query_caches()
            .latest_at(self.store(), query, entity_path, component_names)
    }

    /// Get the latest index and value for a given dense [`re_types_core::Component`].
    ///
    /// This assumes that the row we get from the store contains at most one instance for this
    /// component; it will log a warning otherwise.
    ///
    /// This should only be used for "mono-components" such as `Transform` and `Tensor`.
    ///
    /// This is a best-effort helper, it will merely log errors on failure.
    #[inline]
    pub fn latest_at_component<C: re_types_core::Component>(
        &self,
        entity_path: &EntityPath,
        query: &re_chunk_store::LatestAtQuery,
    ) -> Option<re_query::LatestAtMonoResult<C>> {
        self.query_caches().latest_at_component::<C>(
            self.store(),
            self.resolver(),
            entity_path,
            query,
        )
    }

    /// Get the latest index and value for a given dense [`re_types_core::Component`].
    ///
    /// This assumes that the row we get from the store contains at most one instance for this
    /// component; it will log a warning otherwise.
    ///
    /// This should only be used for "mono-components" such as `Transform` and `Tensor`.
    ///
    /// This is a best-effort helper, and will quietly swallow any errors.
    #[inline]
    pub fn latest_at_component_quiet<C: re_types_core::Component>(
        &self,
        entity_path: &EntityPath,
        query: &re_chunk_store::LatestAtQuery,
    ) -> Option<re_query::LatestAtMonoResult<C>> {
        self.query_caches().latest_at_component_quiet::<C>(
            self.store(),
            self.resolver(),
            entity_path,
            query,
        )
    }

    #[inline]
    pub fn latest_at_component_at_closest_ancestor<C: re_types_core::Component>(
        &self,
        entity_path: &EntityPath,
        query: &re_chunk_store::LatestAtQuery,
    ) -> Option<(EntityPath, re_query::LatestAtMonoResult<C>)> {
        self.query_caches()
            .latest_at_component_at_closest_ancestor::<C>(
                self.store(),
                self.resolver(),
                entity_path,
                query,
            )
    }

    #[inline]
    pub fn store(&self) -> &ChunkStore {
        &self.data_store
    }

    #[inline]
    pub fn store_kind(&self) -> StoreKind {
        self.store_id().kind
    }

    #[inline]
    pub fn store_id(&self) -> &StoreId {
        self.data_store.id()
    }

    /// If this entity db is the result of a clone, which store was it cloned from?
    ///
    /// A cloned store always gets a new unique ID.
    ///
    /// We currently only use entity db cloning for blueprints:
    /// when we activate a _default_ blueprint that was received on the wire (e.g. from a recording),
    /// we clone it and make the clone the _active_ blueprint.
    /// This means all active blueprints are clones.
    #[inline]
    pub fn cloned_from(&self) -> Option<&StoreId> {
        self.store_info().and_then(|info| info.cloned_from.as_ref())
    }

    pub fn timelines(&self) -> impl ExactSizeIterator<Item = &Timeline> {
        self.times_per_timeline().keys()
    }

    pub fn times_per_timeline(&self) -> &TimesPerTimeline {
        &self.times_per_timeline
    }

    pub fn has_any_data_on_timeline(&self, timeline: &Timeline) -> bool {
        if let Some(times) = self.times_per_timeline.get(timeline) {
            !times.is_empty()
        } else {
            false
        }
    }

    /// Returns the time range of data on the given timeline, ignoring any static times.
    ///
    /// This is O(N) in the number of times on the timeline.
    pub fn time_range_for(&self, timeline: &Timeline) -> Option<ResolvedTimeRange> {
        let (mut start, mut end) = (None, None);
        for time in self
            .times_per_timeline()
            .get(timeline)?
            .keys()
            .filter(|v| !v.is_static())
            .copied()
        {
            if start.is_none() || Some(time) < start {
                start = Some(time);
            }
            if end.is_none() || Some(time) > end {
                end = Some(time);
            }
        }
        Some(ResolvedTimeRange::new(start?, end?))
    }

    /// Histogram of all events on the timeeline, of all entities.
    pub fn time_histogram(&self, timeline: &Timeline) -> Option<&crate::TimeHistogram> {
        self.tree().subtree.time_histogram.get(timeline)
    }

    /// Total number of static messages for any entity.
    pub fn num_static_messages(&self) -> u64 {
        self.tree.num_static_messages_recursive()
    }

    /// Returns whether a component is static.
    pub fn is_component_static(&self, component_path: &ComponentPath) -> Option<bool> {
        if let Some(entity_tree) = self.tree().subtree(component_path.entity_path()) {
            entity_tree
                .entity
                .components
                .get(&component_path.component_name)
                .map(|component_histogram| component_histogram.is_static())
        } else {
            None
        }
    }

    #[inline]
    pub fn num_rows(&self) -> u64 {
        self.data_store.stats().total().total_num_rows
    }

    /// Return the current `ChunkStoreGeneration`. This can be used to determine whether the
    /// database has been modified since the last time it was queried.
    #[inline]
    pub fn generation(&self) -> re_chunk_store::ChunkStoreGeneration {
        self.data_store.generation()
    }

    #[inline]
    pub fn last_modified_at(&self) -> web_time::Instant {
        self.last_modified_at
    }

    /// The highest `RowId` in the store,
    /// which corresponds to the last edit time.
    /// Ignores deletions.
    #[inline]
    pub fn latest_row_id(&self) -> Option<RowId> {
        self.latest_row_id
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.set_store_info.is_none() && self.num_rows() == 0
    }

    /// A sorted list of all the entity paths in this database.
    pub fn entity_paths(&self) -> Vec<&EntityPath> {
        use itertools::Itertools as _;
        self.entity_path_from_hash.values().sorted().collect()
    }

    #[inline]
    pub fn ingestion_stats(&self) -> &IngestionStatistics {
        &self.stats
    }

    #[inline]
    pub fn entity_path_from_hash(&self, entity_path_hash: &EntityPathHash) -> Option<&EntityPath> {
        self.entity_path_from_hash.get(entity_path_hash)
    }

    /// Returns `true` also for entities higher up in the hierarchy.
    #[inline]
    pub fn is_known_entity(&self, entity_path: &EntityPath) -> bool {
        self.tree.subtree(entity_path).is_some()
    }

    /// If you log `world/points`, then that is a logged entity, but `world` is not,
    /// unless you log something to `world` too.
    #[inline]
    pub fn is_logged_entity(&self, entity_path: &EntityPath) -> bool {
        self.entity_path_from_hash.contains_key(&entity_path.hash())
    }

    pub fn add(&mut self, msg: &LogMsg) -> Result<(), Error> {
        re_tracing::profile_function!();

        debug_assert_eq!(msg.store_id(), self.store_id());

        match &msg {
            LogMsg::SetStoreInfo(msg) => self.set_store_info(msg.clone()),

            LogMsg::ArrowMsg(_, arrow_msg) => {
                self.last_modified_at = web_time::Instant::now();

                let mut chunk = re_chunk::Chunk::from_arrow_msg(arrow_msg)?;
                chunk.sort_if_unsorted();
                self.add_chunk(&Arc::new(chunk))?;
            }

            LogMsg::BlueprintActivationCommand(_) => {
                // Not for us to handle
            }
        }

        Ok(())
    }

    pub fn add_chunk(&mut self, chunk: &Arc<Chunk>) -> Result<(), Error> {
        let store_events = self.data_store.insert_chunk(chunk)?;

        self.register_entity_path(chunk.entity_path());

        if self.latest_row_id < chunk.row_id_range().map(|(_, row_id_max)| row_id_max) {
            self.latest_row_id = chunk.row_id_range().map(|(_, row_id_max)| row_id_max);
        }

        {
            // Update our internal views by notifying them of resulting [`ChunkStoreEvent`]s.
            self.times_per_timeline.on_events(&store_events);
            self.query_caches.on_events(&store_events);
            self.tree.on_store_additions(&store_events);
            self.tree.on_store_deletions(&store_events);

            // We inform the stats last, since it measures e2e latency.
            self.stats.on_events(&store_events);
        }

        Ok(())
    }

    fn register_entity_path(&mut self, entity_path: &EntityPath) {
        self.entity_path_from_hash
            .entry(entity_path.hash())
            .or_insert_with(|| entity_path.clone());
    }

    pub fn set_store_info(&mut self, store_info: SetStoreInfo) {
        self.set_store_info = Some(store_info);
    }

    pub fn gc_everything_but_the_latest_row_on_non_default_timelines(&mut self) {
        re_tracing::profile_function!();

        self.gc(&GarbageCollectionOptions {
            target: GarbageCollectionTarget::Everything,
            protect_latest: 1, // TODO(jleibs): Bump this after we have an undo buffer
            time_budget: DEFAULT_GC_TIME_BUDGET,
        });
    }

    /// Free up some RAM by forgetting the older parts of all timelines.
    pub fn purge_fraction_of_ram(&mut self, fraction_to_purge: f32) {
        re_tracing::profile_function!();

        assert!((0.0..=1.0).contains(&fraction_to_purge));
        self.gc(&GarbageCollectionOptions {
            target: GarbageCollectionTarget::DropAtLeastFraction(fraction_to_purge as _),
            protect_latest: 1,
            time_budget: DEFAULT_GC_TIME_BUDGET,
        });
    }

    pub fn gc(&mut self, gc_options: &GarbageCollectionOptions) {
        re_tracing::profile_function!();

        let (store_events, stats_diff) = self.data_store.gc(gc_options);

        re_log::trace!(
            num_row_ids_dropped = store_events.len(),
            size_bytes_dropped = re_format::format_bytes(stats_diff.total().total_size_bytes as _),
            "purged datastore"
        );

        self.on_store_deletions(&store_events);
    }

    fn on_store_deletions(&mut self, store_events: &[ChunkStoreEvent]) {
        re_tracing::profile_function!();

        let Self {
            data_source: _,
            set_store_info: _,
            last_modified_at: _,
            latest_row_id: _,
            entity_path_from_hash: _,
            times_per_timeline,
            tree,
            data_store: _,
            resolver: _,
            query_caches,
            stats: _,
        } = self;

        times_per_timeline.on_events(store_events);
        query_caches.on_events(store_events);

        tree.on_store_deletions(store_events);
    }

    /// Key used for sorting recordings in the UI.
    pub fn sort_key(&self) -> impl Ord + '_ {
        self.store_info()
            .map(|info| (info.application_id.0.as_str(), info.started))
    }

    /// Export the contents of the current database to a sequence of messages.
    ///
    /// If `time_selection` is specified, then only data for that specific timeline over that
    /// specific time range will be accounted for.
    pub fn to_messages(
        &self,
        time_selection: Option<(Timeline, ResolvedTimeRangeF)>,
    ) -> ChunkResult<Vec<LogMsg>> {
        re_tracing::profile_function!();

        let set_store_info_msg = self
            .store_info_msg()
            .map(|msg| Ok(LogMsg::SetStoreInfo(msg.clone())));

        let time_filter = time_selection.map(|(timeline, range)| {
            (
                timeline,
                ResolvedTimeRange::new(range.min.floor(), range.max.ceil()),
            )
        });

        let data_messages = self
            .store()
            .iter_chunks()
            .filter(|chunk| {
                let Some((timeline, time_range)) = time_filter else {
                    return true;
                };

                // TODO(cmc): chunk.slice_time_selection(time_selection)
                chunk
                    .timelines()
                    .get(&timeline)
                    .map_or(false, |time_chunk| {
                        time_range.contains(time_chunk.time_range().min())
                            || time_range.contains(time_chunk.time_range().max())
                    })
            })
            .map(|chunk| {
                chunk
                    .to_arrow_msg()
                    .map(|msg| LogMsg::ArrowMsg(self.store_id().clone(), msg))
            });

        // If this is a blueprint, make sure to include the `BlueprintActivationCommand` message.
        // We generally use `to_messages` to export a blueprint via "save". In that
        // case, we want to make the blueprint active and default when it's reloaded.
        // TODO(jleibs): Coupling this with the stored file instead of injecting seems
        // architecturally weird. Would be great if we didn't need this in `.rbl` files
        // at all.
        let blueprint_ready = if self.store_kind() == StoreKind::Blueprint {
            let activate_cmd =
                re_log_types::BlueprintActivationCommand::make_active(self.store_id().clone());

            itertools::Either::Left(std::iter::once(Ok(activate_cmd.into())))
        } else {
            itertools::Either::Right(std::iter::empty())
        };

        let messages: Result<Vec<_>, _> = set_store_info_msg
            .into_iter()
            .chain(data_messages)
            .chain(blueprint_ready)
            .collect();

        messages
    }

    /// Make a clone of this [`EntityDb`], assigning it a new [`StoreId`].
    pub fn clone_with_new_id(&self, new_id: StoreId) -> Result<Self, Error> {
        re_tracing::profile_function!();

        let mut new_db = Self::new(new_id.clone());

        new_db.last_modified_at = self.last_modified_at;
        new_db.latest_row_id = self.latest_row_id;

        // We do NOT clone the `data_source`, because the reason we clone an entity db
        // is so that we can modify it, and then it would be wrong to say its from the same source.
        // Specifically: if we load a blueprint from an `.rdd`, then modify it heavily and save it,
        // it would be wrong to claim that this was the blueprint from that `.rrd`,
        // and it would confuse the user.
        // TODO(emilk): maybe we should use a special `Cloned` data source,
        // wrapping either the original source, the original StoreId, or both.

        if let Some(store_info) = self.store_info() {
            let mut new_info = store_info.clone();
            new_info.store_id = new_id;
            new_info.cloned_from = Some(self.store_id().clone());

            new_db.set_store_info(SetStoreInfo {
                row_id: *RowId::new(),
                info: new_info,
            });
        }

        for chunk in self.store().iter_chunks() {
            new_db.add_chunk(&Arc::clone(chunk))?;
        }

        Ok(new_db)
    }
}

impl re_types_core::SizeBytes for EntityDb {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        // TODO(emilk): size of entire EntityDb, including secondary indices etc
        self.data_store().stats().total().total_size_bytes
    }
}

// ----------------------------------------------------------------------------

pub struct IngestionStatistics {
    store_id: StoreId,
    e2e_latency_sec_history: Mutex<emath::History<f32>>,
}

impl ChunkStoreSubscriber for IngestionStatistics {
    #[inline]
    fn name(&self) -> String {
        "rerun.testing.store_subscribers.IngestionStatistics".into()
    }

    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    #[inline]
    fn on_events(&mut self, events: &[ChunkStoreEvent]) {
        for event in events {
            if event.store_id == self.store_id {
                for row_id in event.diff.chunk.row_ids() {
                    self.on_new_row_id(row_id);
                }
            }
        }
    }
}

impl IngestionStatistics {
    pub fn new(store_id: StoreId) -> Self {
        let min_samples = 0; // 0: we stop displaying e2e latency if input stops
        let max_samples = 1024; // don't waste too much memory on this - we just need enough to get a good average
        let max_age = 1.0; // don't keep too long of a rolling average, or the stats get outdated.
        Self {
            store_id,
            e2e_latency_sec_history: Mutex::new(emath::History::new(
                min_samples..max_samples,
                max_age,
            )),
        }
    }

    fn on_new_row_id(&mut self, row_id: RowId) {
        if let Ok(duration_since_epoch) = web_time::SystemTime::UNIX_EPOCH.elapsed() {
            let nanos_since_epoch = duration_since_epoch.as_nanos() as u64;

            // This only makes sense if the clocks are very good, i.e. if the recording was on the same machine!
            if let Some(nanos_since_log) =
                nanos_since_epoch.checked_sub(row_id.nanoseconds_since_epoch())
            {
                let now = nanos_since_epoch as f64 / 1e9;
                let sec_since_log = nanos_since_log as f32 / 1e9;

                self.e2e_latency_sec_history.lock().add(now, sec_since_log);
            }
        }
    }

    /// What is the mean latency between the time data was logged in the SDK and the time it was ingested?
    ///
    /// This is based on the clocks of the viewer and the SDK being in sync,
    /// so if the recording was done on another machine, this is likely very inaccurate.
    pub fn current_e2e_latency_sec(&self) -> Option<f32> {
        let mut e2e_latency_sec_history = self.e2e_latency_sec_history.lock();

        if let Ok(duration_since_epoch) = web_time::SystemTime::UNIX_EPOCH.elapsed() {
            let nanos_since_epoch = duration_since_epoch.as_nanos() as u64;
            let now = nanos_since_epoch as f64 / 1e9;
            e2e_latency_sec_history.flush(now); // make sure the average is up-to-date.
        }

        e2e_latency_sec_history.average()
    }
}
