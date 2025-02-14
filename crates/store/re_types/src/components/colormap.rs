// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/colormap.fbs".

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
#![allow(non_camel_case_types)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: Colormap for mapping scalar values within a given range to a color.
///
/// This provides a number of popular pre-defined colormaps.
/// In the future, the Rerun Viewer will allow users to define their own colormaps,
/// but currently the Viewer is limited to the types defined here.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Colormap {
    /// A simple black to white gradient.
    ///
    /// This is a sRGB gray gradient which is perceptually uniform.
    Grayscale = 1,

    /// The Inferno colormap from Matplotlib.
    ///
    /// This is a perceptually uniform colormap.
    /// It interpolates from black to red to bright yellow.
    Inferno = 2,

    /// The Magma colormap from Matplotlib.
    ///
    /// This is a perceptually uniform colormap.
    /// It interpolates from black to purple to white.
    Magma = 3,

    /// The Plasma colormap from Matplotlib.
    ///
    /// This is a perceptually uniform colormap.
    /// It interpolates from dark blue to purple to yellow.
    Plasma = 4,

    /// Google's Turbo colormap map.
    ///
    /// This is a perceptually non-uniform rainbow colormap addressing many issues of
    /// more traditional rainbow colormaps like Jet.
    /// It is more perceptually uniform without sharp transitions and is more colorblind-friendly.
    /// Details: <https://research.google/blog/turbo-an-improved-rainbow-colormap-for-visualization/>
    #[default]
    Turbo = 5,

    /// The Viridis colormap from Matplotlib
    ///
    /// This is a perceptually uniform colormap which is robust to color blindness.
    /// It interpolates from dark purple to green to yellow.
    Viridis = 6,

    /// Rasmusgo's Cyan to Yellow colormap
    ///
    /// This is a perceptually uniform colormap which is robust to color blindness.
    /// It is especially suited for visualizing signed values.
    /// It interpolates from cyan to blue to dark gray to brass to yellow.
    CyanToYellow = 7,
}

impl ::re_types_core::Component for Colormap {
    #[inline]
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("rerun.components.Colormap")
    }
}

::re_types_core::macros::impl_into_cow!(Colormap);

impl ::re_types_core::Loggable for Colormap {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::UInt8
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
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| *datum as u8);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            as_array_ref(PrimitiveArray::<UInt8Type>::new(
                ScalarBuffer::from(
                    data0
                        .into_iter()
                        .map(|v| v.unwrap_or_default())
                        .collect::<Vec<_>>(),
                ),
                data0_validity,
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
            .downcast_ref::<UInt8Array>()
            .ok_or_else(|| {
                let expected = Self::arrow_datatype();
                let actual = arrow_data.data_type().clone();
                DeserializationError::datatype_mismatch(expected, actual)
            })
            .with_context("rerun.components.Colormap#enum")?
            .into_iter()
            .map(|typ| match typ {
                Some(1) => Ok(Some(Self::Grayscale)),
                Some(2) => Ok(Some(Self::Inferno)),
                Some(3) => Ok(Some(Self::Magma)),
                Some(4) => Ok(Some(Self::Plasma)),
                Some(5) => Ok(Some(Self::Turbo)),
                Some(6) => Ok(Some(Self::Viridis)),
                Some(7) => Ok(Some(Self::CyanToYellow)),
                None => Ok(None),
                Some(invalid) => Err(DeserializationError::missing_union_arm(
                    Self::arrow_datatype(),
                    "<invalid>",
                    invalid as _,
                )),
            })
            .collect::<DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.components.Colormap")?)
    }
}

impl std::fmt::Display for Colormap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Grayscale => write!(f, "Grayscale"),
            Self::Inferno => write!(f, "Inferno"),
            Self::Magma => write!(f, "Magma"),
            Self::Plasma => write!(f, "Plasma"),
            Self::Turbo => write!(f, "Turbo"),
            Self::Viridis => write!(f, "Viridis"),
            Self::CyanToYellow => write!(f, "CyanToYellow"),
        }
    }
}

impl ::re_types_core::reflection::Enum for Colormap {
    #[inline]
    fn variants() -> &'static [Self] {
        &[
            Self::Grayscale,
            Self::Inferno,
            Self::Magma,
            Self::Plasma,
            Self::Turbo,
            Self::Viridis,
            Self::CyanToYellow,
        ]
    }

    #[inline]
    fn docstring_md(self) -> &'static str {
        match self {
            Self::Grayscale => {
                "A simple black to white gradient.\n\nThis is a sRGB gray gradient which is perceptually uniform."
            }
            Self::Inferno => {
                "The Inferno colormap from Matplotlib.\n\nThis is a perceptually uniform colormap.\nIt interpolates from black to red to bright yellow."
            }
            Self::Magma => {
                "The Magma colormap from Matplotlib.\n\nThis is a perceptually uniform colormap.\nIt interpolates from black to purple to white."
            }
            Self::Plasma => {
                "The Plasma colormap from Matplotlib.\n\nThis is a perceptually uniform colormap.\nIt interpolates from dark blue to purple to yellow."
            }
            Self::Turbo => {
                "Google's Turbo colormap map.\n\nThis is a perceptually non-uniform rainbow colormap addressing many issues of\nmore traditional rainbow colormaps like Jet.\nIt is more perceptually uniform without sharp transitions and is more colorblind-friendly.\nDetails: <https://research.google/blog/turbo-an-improved-rainbow-colormap-for-visualization/>"
            }
            Self::Viridis => {
                "The Viridis colormap from Matplotlib\n\nThis is a perceptually uniform colormap which is robust to color blindness.\nIt interpolates from dark purple to green to yellow."
            }
            Self::CyanToYellow => {
                "Rasmusgo's Cyan to Yellow colormap\n\nThis is a perceptually uniform colormap which is robust to color blindness.\nIt is especially suited for visualizing signed values.\nIt interpolates from cyan to blue to dark gray to brass to yellow."
            }
        }
    }
}

impl ::re_byte_size::SizeBytes for Colormap {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }

    #[inline]
    fn is_pod() -> bool {
        true
    }
}
