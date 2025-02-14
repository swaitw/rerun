// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/components/force_iterations.fbs".

#pragma once

#include "../../component_descriptor.hpp"
#include "../../datatypes/uint64.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <memory>

namespace rerun::blueprint::components {
    /// **Component**: Specifies how often this force should be applied per iteration.
    ///
    /// Increasing this parameter can lead to better results at the cost of longer computation time.
    struct ForceIterations {
        rerun::datatypes::UInt64 distance;

      public:
        ForceIterations() = default;

        ForceIterations(rerun::datatypes::UInt64 distance_) : distance(distance_) {}

        ForceIterations& operator=(rerun::datatypes::UInt64 distance_) {
            distance = distance_;
            return *this;
        }

        ForceIterations(uint64_t value_) : distance(value_) {}

        ForceIterations& operator=(uint64_t value_) {
            distance = value_;
            return *this;
        }

        /// Cast to the underlying UInt64 datatype
        operator rerun::datatypes::UInt64() const {
            return distance;
        }
    };
} // namespace rerun::blueprint::components

namespace rerun {
    static_assert(
        sizeof(rerun::datatypes::UInt64) == sizeof(blueprint::components::ForceIterations)
    );

    /// \private
    template <>
    struct Loggable<blueprint::components::ForceIterations> {
        static constexpr ComponentDescriptor Descriptor =
            "rerun.blueprint.components.ForceIterations";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::UInt64>::arrow_datatype();
        }

        /// Serializes an array of `rerun::blueprint:: components::ForceIterations` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const blueprint::components::ForceIterations* instances, size_t num_instances
        ) {
            if (num_instances == 0) {
                return Loggable<rerun::datatypes::UInt64>::to_arrow(nullptr, 0);
            } else if (instances == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Passed array instances is null when num_elements> 0."
                );
            } else {
                return Loggable<rerun::datatypes::UInt64>::to_arrow(
                    &instances->distance,
                    num_instances
                );
            }
        }
    };
} // namespace rerun
