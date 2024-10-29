// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/length.fbs".

#pragma once

#include "../datatypes/float32.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace rerun::components {
    /// **Component**: Length, or one-dimensional size.
    ///
    /// Measured in its local coordinate system; consult the archetype in use to determine which
    /// axis or part of the entity this is the length of.
    struct Length {
        rerun::datatypes::Float32 length;

      public:
        Length() = default;

        Length(rerun::datatypes::Float32 length_) : length(length_) {}

        Length& operator=(rerun::datatypes::Float32 length_) {
            length = length_;
            return *this;
        }

        Length(float value_) : length(value_) {}

        Length& operator=(float value_) {
            length = value_;
            return *this;
        }

        /// Cast to the underlying Float32 datatype
        operator rerun::datatypes::Float32() const {
            return length;
        }
    };
} // namespace rerun::components

namespace rerun {
    static_assert(sizeof(rerun::datatypes::Float32) == sizeof(components::Length));

    /// \private
    template <>
    struct Loggable<components::Length> {
        static constexpr const char Name[] = "rerun.components.Length";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::Float32>::arrow_datatype();
        }

        /// Serializes an array of `rerun::components::Length` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::Length* instances, size_t num_instances
        ) {
            if (num_instances == 0) {
                return Loggable<rerun::datatypes::Float32>::to_arrow(nullptr, 0);
            } else if (instances == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Passed array instances is null when num_elements> 0."
                );
            } else {
                return Loggable<rerun::datatypes::Float32>::to_arrow(
                    &instances->length,
                    num_instances
                );
            }
        }
    };
} // namespace rerun
