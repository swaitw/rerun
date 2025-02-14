// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/force_many_body.fbs".

#include "force_many_body.hpp"

#include "../../collection_adapter_builtins.hpp"

namespace rerun::blueprint::archetypes {
    ForceManyBody ForceManyBody::clear_fields() {
        auto archetype = ForceManyBody();
        archetype.enabled =
            ComponentBatch::empty<rerun::blueprint::components::Enabled>(Descriptor_enabled)
                .value_or_throw();
        archetype.strength =
            ComponentBatch::empty<rerun::blueprint::components::ForceStrength>(Descriptor_strength)
                .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> ForceManyBody::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(3);
        if (enabled.has_value()) {
            columns.push_back(enabled.value().partitioned(lengths_).value_or_throw());
        }
        if (strength.has_value()) {
            columns.push_back(strength.value().partitioned(lengths_).value_or_throw());
        }
        columns.push_back(
            ComponentColumn::from_indicators<ForceManyBody>(static_cast<uint32_t>(lengths_.size()))
                .value_or_throw()
        );
        return columns;
    }

    Collection<ComponentColumn> ForceManyBody::columns() {
        if (enabled.has_value()) {
            return columns(std::vector<uint32_t>(enabled.value().length(), 1));
        }
        if (strength.has_value()) {
            return columns(std::vector<uint32_t>(strength.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::blueprint::archetypes

namespace rerun {

    Result<Collection<ComponentBatch>>
        AsComponents<blueprint::archetypes::ForceManyBody>::as_batches(
            const blueprint::archetypes::ForceManyBody& archetype
        ) {
        using namespace blueprint::archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(3);

        if (archetype.enabled.has_value()) {
            cells.push_back(archetype.enabled.value());
        }
        if (archetype.strength.has_value()) {
            cells.push_back(archetype.strength.value());
        }
        {
            auto result = ComponentBatch::from_indicator<ForceManyBody>();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return rerun::take_ownership(std::move(cells));
    }
} // namespace rerun
