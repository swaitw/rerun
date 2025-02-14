// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/near_clip_plane.fbs".

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

/// **Archetype**: Controls the distance to the near clip plane in 3D scene units.
#[derive(Clone, Debug, Default)]
pub struct NearClipPlane {
    /// Controls the distance to the near clip plane in 3D scene units.
    ///
    /// Content closer than this distance will not be visible.
    pub near_clip_plane: Option<SerializedComponentBatch>,
}

impl NearClipPlane {
    /// Returns the [`ComponentDescriptor`] for [`Self::near_clip_plane`].
    #[inline]
    pub fn descriptor_near_clip_plane() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.NearClipPlane".into()),
            component_name: "rerun.blueprint.components.NearClipPlane".into(),
            archetype_field_name: Some("near_clip_plane".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.NearClipPlane".into()),
            component_name: "rerun.blueprint.components.NearClipPlaneIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [NearClipPlane::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [NearClipPlane::descriptor_near_clip_plane()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            NearClipPlane::descriptor_indicator(),
            NearClipPlane::descriptor_near_clip_plane(),
        ]
    });

impl NearClipPlane {
    /// The total number of components in the archetype: 0 required, 1 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`NearClipPlane`] [`::re_types_core::Archetype`]
pub type NearClipPlaneIndicator = ::re_types_core::GenericIndicatorComponent<NearClipPlane>;

impl ::re_types_core::Archetype for NearClipPlane {
    type Indicator = NearClipPlaneIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.NearClipPlane".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Near clip plane"
    }

    #[inline]
    fn indicator() -> SerializedComponentBatch {
        #[allow(clippy::unwrap_used)]
        NearClipPlaneIndicator::DEFAULT.serialized().unwrap()
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
        let near_clip_plane = arrays_by_descr
            .get(&Self::descriptor_near_clip_plane())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_near_clip_plane())
            });
        Ok(Self { near_clip_plane })
    }
}

impl ::re_types_core::AsComponents for NearClipPlane {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [Some(Self::indicator()), self.near_clip_plane.clone()]
            .into_iter()
            .flatten()
            .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for NearClipPlane {}

impl NearClipPlane {
    /// Create a new `NearClipPlane`.
    #[inline]
    pub fn new(near_clip_plane: impl Into<crate::blueprint::components::NearClipPlane>) -> Self {
        Self {
            near_clip_plane: try_serialize_field(
                Self::descriptor_near_clip_plane(),
                [near_clip_plane],
            ),
        }
    }

    /// Update only some specific fields of a `NearClipPlane`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `NearClipPlane`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            near_clip_plane: Some(SerializedComponentBatch::new(
                crate::blueprint::components::NearClipPlane::arrow_empty(),
                Self::descriptor_near_clip_plane(),
            )),
        }
    }

    /// Controls the distance to the near clip plane in 3D scene units.
    ///
    /// Content closer than this distance will not be visible.
    #[inline]
    pub fn with_near_clip_plane(
        mut self,
        near_clip_plane: impl Into<crate::blueprint::components::NearClipPlane>,
    ) -> Self {
        self.near_clip_plane =
            try_serialize_field(Self::descriptor_near_clip_plane(), [near_clip_plane]);
        self
    }
}

impl ::re_byte_size::SizeBytes for NearClipPlane {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.near_clip_plane.heap_size_bytes()
    }
}
