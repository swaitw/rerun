// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/rotation_axis_angle.fbs".

#pragma once

#include "../component_descriptor.hpp"
#include "../result.hpp"
#include "angle.hpp"
#include "vec3d.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    class Array;
    class DataType;
    class StructBuilder;
} // namespace arrow

namespace rerun::datatypes {
    /// **Datatype**: 3D rotation represented by a rotation around a given axis.
    struct RotationAxisAngle {
        /// Axis to rotate around.
        ///
        /// This is not required to be normalized.
        /// However, if normalization of the rotation axis fails (typically due to a zero vector)
        /// the rotation is treated as an invalid transform, unless the angle is zero in which case
        /// it is treated as an identity.
        rerun::datatypes::Vec3D axis;

        /// How much to rotate around the axis.
        rerun::datatypes::Angle angle;

      public: // START of extensions from rotation_axis_angle_ext.cpp:
        RotationAxisAngle(const Vec3D& _axis, const Angle& _angle) : axis(_axis), angle(_angle) {}

        // END of extensions from rotation_axis_angle_ext.cpp, start of generated code:

      public:
        RotationAxisAngle() = default;
    };
} // namespace rerun::datatypes

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<datatypes::RotationAxisAngle> {
        static constexpr ComponentDescriptor Descriptor = "rerun.datatypes.RotationAxisAngle";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::datatypes::RotationAxisAngle` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const datatypes::RotationAxisAngle* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::StructBuilder* builder, const datatypes::RotationAxisAngle* elements,
            size_t num_elements
        );
    };
} // namespace rerun
