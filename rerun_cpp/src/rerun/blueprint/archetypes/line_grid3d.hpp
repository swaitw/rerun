// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/line_grid3d.fbs".

#pragma once

#include "../../blueprint/components/grid_spacing.hpp"
#include "../../blueprint/components/visible.hpp"
#include "../../collection.hpp"
#include "../../component_batch.hpp"
#include "../../components/color.hpp"
#include "../../components/plane3d.hpp"
#include "../../components/stroke_width.hpp"
#include "../../indicator_component.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::blueprint::archetypes {
    /// **Archetype**: Configuration for the 3D line grid.
    struct LineGrid3D {
        /// Whether the grid is visible.
        ///
        /// Defaults to true.
        std::optional<ComponentBatch> visible;

        /// Space between grid lines spacing of one line to the next in scene units.
        ///
        /// As you zoom out, successively only every tenth line is shown.
        /// This controls the closest zoom level.
        std::optional<ComponentBatch> spacing;

        /// In what plane the grid is drawn.
        ///
        /// Defaults to whatever plane is determined as the plane at zero units up/down as defined by `components::ViewCoordinates` if present.
        std::optional<ComponentBatch> plane;

        /// How thick the lines should be in ui units.
        ///
        /// Default is 1.0 ui unit.
        std::optional<ComponentBatch> stroke_width;

        /// Color used for the grid.
        ///
        /// Transparency via alpha channel is supported.
        /// Defaults to a slightly transparent light gray.
        std::optional<ComponentBatch> color;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.blueprint.components.LineGrid3DIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.blueprint.archetypes.LineGrid3D";

        /// `ComponentDescriptor` for the `visible` field.
        static constexpr auto Descriptor_visible = ComponentDescriptor(
            ArchetypeName, "visible",
            Loggable<rerun::blueprint::components::Visible>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `spacing` field.
        static constexpr auto Descriptor_spacing = ComponentDescriptor(
            ArchetypeName, "spacing",
            Loggable<rerun::blueprint::components::GridSpacing>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `plane` field.
        static constexpr auto Descriptor_plane = ComponentDescriptor(
            ArchetypeName, "plane", Loggable<rerun::components::Plane3D>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `stroke_width` field.
        static constexpr auto Descriptor_stroke_width = ComponentDescriptor(
            ArchetypeName, "stroke_width",
            Loggable<rerun::components::StrokeWidth>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `color` field.
        static constexpr auto Descriptor_color = ComponentDescriptor(
            ArchetypeName, "color", Loggable<rerun::components::Color>::Descriptor.component_name
        );

      public:
        LineGrid3D() = default;
        LineGrid3D(LineGrid3D&& other) = default;
        LineGrid3D(const LineGrid3D& other) = default;
        LineGrid3D& operator=(const LineGrid3D& other) = default;
        LineGrid3D& operator=(LineGrid3D&& other) = default;

        /// Update only some specific fields of a `LineGrid3D`.
        static LineGrid3D update_fields() {
            return LineGrid3D();
        }

        /// Clear all the fields of a `LineGrid3D`.
        static LineGrid3D clear_fields();

        /// Whether the grid is visible.
        ///
        /// Defaults to true.
        LineGrid3D with_visible(const rerun::blueprint::components::Visible& _visible) && {
            visible = ComponentBatch::from_loggable(_visible, Descriptor_visible).value_or_throw();
            return std::move(*this);
        }

        /// Space between grid lines spacing of one line to the next in scene units.
        ///
        /// As you zoom out, successively only every tenth line is shown.
        /// This controls the closest zoom level.
        LineGrid3D with_spacing(const rerun::blueprint::components::GridSpacing& _spacing) && {
            spacing = ComponentBatch::from_loggable(_spacing, Descriptor_spacing).value_or_throw();
            return std::move(*this);
        }

        /// In what plane the grid is drawn.
        ///
        /// Defaults to whatever plane is determined as the plane at zero units up/down as defined by `components::ViewCoordinates` if present.
        LineGrid3D with_plane(const rerun::components::Plane3D& _plane) && {
            plane = ComponentBatch::from_loggable(_plane, Descriptor_plane).value_or_throw();
            return std::move(*this);
        }

        /// How thick the lines should be in ui units.
        ///
        /// Default is 1.0 ui unit.
        LineGrid3D with_stroke_width(const rerun::components::StrokeWidth& _stroke_width) && {
            stroke_width = ComponentBatch::from_loggable(_stroke_width, Descriptor_stroke_width)
                               .value_or_throw();
            return std::move(*this);
        }

        /// Color used for the grid.
        ///
        /// Transparency via alpha channel is supported.
        /// Defaults to a slightly transparent light gray.
        LineGrid3D with_color(const rerun::components::Color& _color) && {
            color = ComponentBatch::from_loggable(_color, Descriptor_color).value_or_throw();
            return std::move(*this);
        }
    };

} // namespace rerun::blueprint::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<blueprint::archetypes::LineGrid3D> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(
            const blueprint::archetypes::LineGrid3D& archetype
        );
    };
} // namespace rerun
