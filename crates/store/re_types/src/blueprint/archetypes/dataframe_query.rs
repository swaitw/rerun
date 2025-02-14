// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/dataframe_query.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: The query for the dataframe view.
#[derive(Clone, Debug, Default)]
pub struct DataframeQuery {
    /// The timeline for this query.
    ///
    /// If unset, the timeline currently active on the time panel is used.
    pub timeline: Option<SerializedComponentBatch>,

    /// If provided, only rows whose timestamp is within this range will be shown.
    ///
    /// Note: will be unset as soon as `timeline` is changed.
    pub filter_by_range: Option<SerializedComponentBatch>,

    /// If provided, only show rows which contains a logged event for the specified component.
    pub filter_is_not_null: Option<SerializedComponentBatch>,

    /// Should empty cells be filled with latest-at queries?
    pub apply_latest_at: Option<SerializedComponentBatch>,

    /// Selected columns. If unset, all columns are selected.
    pub select: Option<SerializedComponentBatch>,
}

impl DataframeQuery {
    /// Returns the [`ComponentDescriptor`] for [`Self::timeline`].
    #[inline]
    pub fn descriptor_timeline() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.DataframeQuery".into()),
            component_name: "rerun.blueprint.components.TimelineName".into(),
            archetype_field_name: Some("timeline".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::filter_by_range`].
    #[inline]
    pub fn descriptor_filter_by_range() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.DataframeQuery".into()),
            component_name: "rerun.blueprint.components.FilterByRange".into(),
            archetype_field_name: Some("filter_by_range".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::filter_is_not_null`].
    #[inline]
    pub fn descriptor_filter_is_not_null() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.DataframeQuery".into()),
            component_name: "rerun.blueprint.components.FilterIsNotNull".into(),
            archetype_field_name: Some("filter_is_not_null".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::apply_latest_at`].
    #[inline]
    pub fn descriptor_apply_latest_at() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.DataframeQuery".into()),
            component_name: "rerun.blueprint.components.ApplyLatestAt".into(),
            archetype_field_name: Some("apply_latest_at".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::select`].
    #[inline]
    pub fn descriptor_select() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.DataframeQuery".into()),
            component_name: "rerun.blueprint.components.SelectedColumns".into(),
            archetype_field_name: Some("select".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.DataframeQuery".into()),
            component_name: "rerun.blueprint.components.DataframeQueryIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [DataframeQuery::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            DataframeQuery::descriptor_timeline(),
            DataframeQuery::descriptor_filter_by_range(),
            DataframeQuery::descriptor_filter_is_not_null(),
            DataframeQuery::descriptor_apply_latest_at(),
            DataframeQuery::descriptor_select(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            DataframeQuery::descriptor_indicator(),
            DataframeQuery::descriptor_timeline(),
            DataframeQuery::descriptor_filter_by_range(),
            DataframeQuery::descriptor_filter_is_not_null(),
            DataframeQuery::descriptor_apply_latest_at(),
            DataframeQuery::descriptor_select(),
        ]
    });

impl DataframeQuery {
    /// The total number of components in the archetype: 0 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 6usize;
}

/// Indicator component for the [`DataframeQuery`] [`::re_types_core::Archetype`]
pub type DataframeQueryIndicator = ::re_types_core::GenericIndicatorComponent<DataframeQuery>;

impl ::re_types_core::Archetype for DataframeQuery {
    type Indicator = DataframeQueryIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.DataframeQuery".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Dataframe query"
    }

    #[inline]
    fn indicator() -> SerializedComponentBatch {
        #[allow(clippy::unwrap_used)]
        DataframeQueryIndicator::DEFAULT.serialized().unwrap()
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let timeline = arrays_by_descr
            .get(&Self::descriptor_timeline())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_timeline()));
        let filter_by_range = arrays_by_descr
            .get(&Self::descriptor_filter_by_range())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_filter_by_range())
            });
        let filter_is_not_null = arrays_by_descr
            .get(&Self::descriptor_filter_is_not_null())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_filter_is_not_null())
            });
        let apply_latest_at = arrays_by_descr
            .get(&Self::descriptor_apply_latest_at())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_apply_latest_at())
            });
        let select = arrays_by_descr
            .get(&Self::descriptor_select())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_select()));
        Ok(Self {
            timeline,
            filter_by_range,
            filter_is_not_null,
            apply_latest_at,
            select,
        })
    }
}

impl ::re_types_core::AsComponents for DataframeQuery {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.timeline.clone(),
            self.filter_by_range.clone(),
            self.filter_is_not_null.clone(),
            self.apply_latest_at.clone(),
            self.select.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for DataframeQuery {}

impl DataframeQuery {
    /// Create a new `DataframeQuery`.
    #[inline]
    pub fn new() -> Self {
        Self {
            timeline: None,
            filter_by_range: None,
            filter_is_not_null: None,
            apply_latest_at: None,
            select: None,
        }
    }

    /// Update only some specific fields of a `DataframeQuery`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `DataframeQuery`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            timeline: Some(SerializedComponentBatch::new(
                crate::blueprint::components::TimelineName::arrow_empty(),
                Self::descriptor_timeline(),
            )),
            filter_by_range: Some(SerializedComponentBatch::new(
                crate::blueprint::components::FilterByRange::arrow_empty(),
                Self::descriptor_filter_by_range(),
            )),
            filter_is_not_null: Some(SerializedComponentBatch::new(
                crate::blueprint::components::FilterIsNotNull::arrow_empty(),
                Self::descriptor_filter_is_not_null(),
            )),
            apply_latest_at: Some(SerializedComponentBatch::new(
                crate::blueprint::components::ApplyLatestAt::arrow_empty(),
                Self::descriptor_apply_latest_at(),
            )),
            select: Some(SerializedComponentBatch::new(
                crate::blueprint::components::SelectedColumns::arrow_empty(),
                Self::descriptor_select(),
            )),
        }
    }

    /// The timeline for this query.
    ///
    /// If unset, the timeline currently active on the time panel is used.
    #[inline]
    pub fn with_timeline(
        mut self,
        timeline: impl Into<crate::blueprint::components::TimelineName>,
    ) -> Self {
        self.timeline = try_serialize_field(Self::descriptor_timeline(), [timeline]);
        self
    }

    /// If provided, only rows whose timestamp is within this range will be shown.
    ///
    /// Note: will be unset as soon as `timeline` is changed.
    #[inline]
    pub fn with_filter_by_range(
        mut self,
        filter_by_range: impl Into<crate::blueprint::components::FilterByRange>,
    ) -> Self {
        self.filter_by_range =
            try_serialize_field(Self::descriptor_filter_by_range(), [filter_by_range]);
        self
    }

    /// If provided, only show rows which contains a logged event for the specified component.
    #[inline]
    pub fn with_filter_is_not_null(
        mut self,
        filter_is_not_null: impl Into<crate::blueprint::components::FilterIsNotNull>,
    ) -> Self {
        self.filter_is_not_null =
            try_serialize_field(Self::descriptor_filter_is_not_null(), [filter_is_not_null]);
        self
    }

    /// Should empty cells be filled with latest-at queries?
    #[inline]
    pub fn with_apply_latest_at(
        mut self,
        apply_latest_at: impl Into<crate::blueprint::components::ApplyLatestAt>,
    ) -> Self {
        self.apply_latest_at =
            try_serialize_field(Self::descriptor_apply_latest_at(), [apply_latest_at]);
        self
    }

    /// Selected columns. If unset, all columns are selected.
    #[inline]
    pub fn with_select(
        mut self,
        select: impl Into<crate::blueprint::components::SelectedColumns>,
    ) -> Self {
        self.select = try_serialize_field(Self::descriptor_select(), [select]);
        self
    }
}

impl ::re_byte_size::SizeBytes for DataframeQuery {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.timeline.heap_size_bytes()
            + self.filter_by_range.heap_size_bytes()
            + self.filter_is_not_null.heap_size_bytes()
            + self.apply_latest_at.heap_size_bytes()
            + self.select.heap_size_bytes()
    }
}
