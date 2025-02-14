// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/angle.fbs".

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

/// **Datatype**: Angle in radians.
#[derive(Clone, Debug, Copy, Default, PartialEq, PartialOrd, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct Angle {
    /// Angle in radians. One turn is equal to 2π (or τ) radians.
    pub radians: f32,
}

::re_types_core::macros::impl_into_cow!(Angle);

impl ::re_types_core::Loggable for Angle {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::Float32
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{arrow_helpers::as_array_ref, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let (somes, radians): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| datum.into_owned().radians);
                    (datum.is_some(), datum)
                })
                .unzip();
            let radians_validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            as_array_ref(PrimitiveArray::<Float32Type>::new(
                ScalarBuffer::from(
                    radians
                        .into_iter()
                        .map(|v| v.unwrap_or_default())
                        .collect::<Vec<_>>(),
                ),
                radians_validity,
            ))
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{arrow_zip_validity::ZipValidity, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok(arrow_data
            .as_any()
            .downcast_ref::<Float32Array>()
            .ok_or_else(|| {
                let expected = Self::arrow_datatype();
                let actual = arrow_data.data_type().clone();
                DeserializationError::datatype_mismatch(expected, actual)
            })
            .with_context("rerun.datatypes.Angle#radians")?
            .into_iter()
            .map(|v| v.ok_or_else(DeserializationError::missing_data))
            .map(|res| res.map(|radians| Some(Self { radians })))
            .collect::<DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.datatypes.Angle#radians")
            .with_context("rerun.datatypes.Angle")?)
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{arrow_zip_validity::ZipValidity, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        if let Some(nulls) = arrow_data.nulls() {
            if nulls.null_count() != 0 {
                return Err(DeserializationError::missing_data());
            }
        }
        Ok({
            let slice = arrow_data
                .as_any()
                .downcast_ref::<Float32Array>()
                .ok_or_else(|| {
                    let expected = DataType::Float32;
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.Angle#radians")?
                .values()
                .as_ref();
            {
                slice
                    .iter()
                    .copied()
                    .map(|radians| Self { radians })
                    .collect::<Vec<_>>()
            }
        })
    }
}

impl From<f32> for Angle {
    #[inline]
    fn from(radians: f32) -> Self {
        Self { radians }
    }
}

impl From<Angle> for f32 {
    #[inline]
    fn from(value: Angle) -> Self {
        value.radians
    }
}

impl ::re_byte_size::SizeBytes for Angle {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.radians.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <f32>::is_pod()
    }
}
