// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/ellipsoids3d.fbs".

#pragma once

#include "../collection.hpp"
#include "../component_batch.hpp"
#include "../components/class_id.hpp"
#include "../components/color.hpp"
#include "../components/fill_mode.hpp"
#include "../components/half_size3d.hpp"
#include "../components/pose_rotation_axis_angle.hpp"
#include "../components/pose_rotation_quat.hpp"
#include "../components/pose_translation3d.hpp"
#include "../components/radius.hpp"
#include "../components/show_labels.hpp"
#include "../components/text.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: 3D ellipsoids or spheres.
    ///
    /// This archetype is for ellipsoids or spheres whose size is a key part of the data
    /// (e.g. a bounding sphere).
    /// For points whose radii are for the sake of visualization, use `archetypes::Points3D` instead.
    ///
    /// Note that orienting and placing the ellipsoids/spheres is handled via `[archetypes.InstancePoses3D]`.
    /// Some of its component are repeated here for convenience.
    /// If there's more instance poses than half sizes, the last half size will be repeated for the remaining poses.
    ///
    /// ## Example
    ///
    /// ### Covariance ellipsoid
    /// ![image](https://static.rerun.io/elliopsoid3d_simple/bd5d46e61b80ae44792b52ee07d750a7137002ea/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <algorithm>
    /// #include <random>
    /// #include <vector>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_ellipsoid_simple");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     const float sigmas[3] = {5.0f, 3.0f, 1.0f};
    ///
    ///     std::default_random_engine gen;
    ///     std::normal_distribution<float> dist(0.0, 1.0f);
    ///
    ///     std::vector<rerun::Position3D> points3d(50000);
    ///     std::generate(points3d.begin(), points3d.end(), [&] {
    ///         return rerun::Position3D(
    ///             sigmas[0] * dist(gen),
    ///             sigmas[1] * dist(gen),
    ///             sigmas[2] * dist(gen)
    ///         );
    ///     });
    ///
    ///     rec.log(
    ///         "points",
    ///         rerun::Points3D(points3d).with_radii(0.02f).with_colors(rerun::Rgba32(188, 77, 185))
    ///     );
    ///
    ///     rec.log(
    ///         "ellipsoid",
    ///         rerun::Ellipsoids3D::from_centers_and_half_sizes(
    ///             {
    ///                 {0.0f, 0.0f, 0.0f},
    ///                 {0.0f, 0.0f, 0.0f},
    ///             },
    ///             {
    ///                 {sigmas[0], sigmas[1], sigmas[2]},
    ///                 {3.0f * sigmas[0], 3.0f * sigmas[1], 3.0f * sigmas[2]},
    ///             }
    ///         )
    ///             .with_colors({
    ///                 rerun::Rgba32(255, 255, 0),
    ///                 rerun::Rgba32(64, 64, 0),
    ///             })
    ///     );
    /// }
    /// ```
    struct Ellipsoids3D {
        /// For each ellipsoid, half of its size on its three axes.
        ///
        /// If all components are equal, then it is a sphere with that radius.
        std::optional<ComponentBatch> half_sizes;

        /// Optional center positions of the ellipsoids.
        ///
        /// If not specified, the centers will be at (0, 0, 0).
        /// Note that this uses a `components::PoseTranslation3D` which is also used by `archetypes::InstancePoses3D`.
        std::optional<ComponentBatch> centers;

        /// Rotations via axis + angle.
        ///
        /// If no rotation is specified, the axes of the ellipsoid align with the axes of the local coordinate system.
        /// Note that this uses a `components::PoseRotationAxisAngle` which is also used by `archetypes::InstancePoses3D`.
        std::optional<ComponentBatch> rotation_axis_angles;

        /// Rotations via quaternion.
        ///
        /// If no rotation is specified, the axes of the ellipsoid align with the axes of the local coordinate system.
        /// Note that this uses a `components::PoseRotationQuat` which is also used by `archetypes::InstancePoses3D`.
        std::optional<ComponentBatch> quaternions;

        /// Optional colors for the ellipsoids.
        std::optional<ComponentBatch> colors;

        /// Optional radii for the lines used when the ellipsoid is rendered as a wireframe.
        std::optional<ComponentBatch> line_radii;

        /// Optionally choose whether the ellipsoids are drawn with lines or solid.
        std::optional<ComponentBatch> fill_mode;

        /// Optional text labels for the ellipsoids.
        std::optional<ComponentBatch> labels;

        /// Optional choice of whether the text labels should be shown by default.
        std::optional<ComponentBatch> show_labels;

        /// Optional class ID for the ellipsoids.
        ///
        /// The class ID provides colors and labels if not specified explicitly.
        std::optional<ComponentBatch> class_ids;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.Ellipsoids3DIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.archetypes.Ellipsoids3D";

        /// `ComponentDescriptor` for the `half_sizes` field.
        static constexpr auto Descriptor_half_sizes = ComponentDescriptor(
            ArchetypeName, "half_sizes",
            Loggable<rerun::components::HalfSize3D>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `centers` field.
        static constexpr auto Descriptor_centers = ComponentDescriptor(
            ArchetypeName, "centers",
            Loggable<rerun::components::PoseTranslation3D>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `rotation_axis_angles` field.
        static constexpr auto Descriptor_rotation_axis_angles = ComponentDescriptor(
            ArchetypeName, "rotation_axis_angles",
            Loggable<rerun::components::PoseRotationAxisAngle>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `quaternions` field.
        static constexpr auto Descriptor_quaternions = ComponentDescriptor(
            ArchetypeName, "quaternions",
            Loggable<rerun::components::PoseRotationQuat>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `colors` field.
        static constexpr auto Descriptor_colors = ComponentDescriptor(
            ArchetypeName, "colors", Loggable<rerun::components::Color>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `line_radii` field.
        static constexpr auto Descriptor_line_radii = ComponentDescriptor(
            ArchetypeName, "line_radii",
            Loggable<rerun::components::Radius>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `fill_mode` field.
        static constexpr auto Descriptor_fill_mode = ComponentDescriptor(
            ArchetypeName, "fill_mode",
            Loggable<rerun::components::FillMode>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `labels` field.
        static constexpr auto Descriptor_labels = ComponentDescriptor(
            ArchetypeName, "labels", Loggable<rerun::components::Text>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `show_labels` field.
        static constexpr auto Descriptor_show_labels = ComponentDescriptor(
            ArchetypeName, "show_labels",
            Loggable<rerun::components::ShowLabels>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `class_ids` field.
        static constexpr auto Descriptor_class_ids = ComponentDescriptor(
            ArchetypeName, "class_ids",
            Loggable<rerun::components::ClassId>::Descriptor.component_name
        );

      public: // START of extensions from ellipsoids3d_ext.cpp:
        /// Creates new `Ellipsoids3D` that are spheres, with `half_sizes` created from radii.
        //
        // TODO(andreas): This should not take an std::vector.
        static Ellipsoids3D from_radii(const std::vector<float>& sizes);

        /// Creates new `Ellipsoids3D` that are spheres, with `half_sizes` and `centers` created
        /// from centers and radii.
        //
        // TODO(andreas): This should not take an std::vector.
        static Ellipsoids3D from_centers_and_radii(
            const std::vector<datatypes::Vec3D>& centers, const std::vector<float>& radii
        );

        /// Creates new `Ellipsoids3D` with `half_sizes` centered around the local origin.
        static Ellipsoids3D from_half_sizes(Collection<components::HalfSize3D> half_sizes) {
            return Ellipsoids3D().with_half_sizes(std::move(half_sizes));
        }

        /// Creates new `Ellipsoids3D` with `centers` and `half_sizes`.
        static Ellipsoids3D from_centers_and_half_sizes(
            Collection<components::PoseTranslation3D> centers,
            Collection<components::HalfSize3D> half_sizes
        ) {
            return Ellipsoids3D()
                .with_half_sizes(std::move(half_sizes))
                .with_centers(std::move(centers));
        }

        // END of extensions from ellipsoids3d_ext.cpp, start of generated code:

      public:
        Ellipsoids3D() = default;
        Ellipsoids3D(Ellipsoids3D&& other) = default;
        Ellipsoids3D(const Ellipsoids3D& other) = default;
        Ellipsoids3D& operator=(const Ellipsoids3D& other) = default;
        Ellipsoids3D& operator=(Ellipsoids3D&& other) = default;

        /// Update only some specific fields of a `Ellipsoids3D`.
        static Ellipsoids3D update_fields() {
            return Ellipsoids3D();
        }

        /// Clear all the fields of a `Ellipsoids3D`.
        static Ellipsoids3D clear_fields();

        /// For each ellipsoid, half of its size on its three axes.
        ///
        /// If all components are equal, then it is a sphere with that radius.
        Ellipsoids3D with_half_sizes(const Collection<rerun::components::HalfSize3D>& _half_sizes
        ) && {
            half_sizes =
                ComponentBatch::from_loggable(_half_sizes, Descriptor_half_sizes).value_or_throw();
            return std::move(*this);
        }

        /// Optional center positions of the ellipsoids.
        ///
        /// If not specified, the centers will be at (0, 0, 0).
        /// Note that this uses a `components::PoseTranslation3D` which is also used by `archetypes::InstancePoses3D`.
        Ellipsoids3D with_centers(const Collection<rerun::components::PoseTranslation3D>& _centers
        ) && {
            centers = ComponentBatch::from_loggable(_centers, Descriptor_centers).value_or_throw();
            return std::move(*this);
        }

        /// Rotations via axis + angle.
        ///
        /// If no rotation is specified, the axes of the ellipsoid align with the axes of the local coordinate system.
        /// Note that this uses a `components::PoseRotationAxisAngle` which is also used by `archetypes::InstancePoses3D`.
        Ellipsoids3D with_rotation_axis_angles(
            const Collection<rerun::components::PoseRotationAxisAngle>& _rotation_axis_angles
        ) && {
            rotation_axis_angles = ComponentBatch::from_loggable(
                                       _rotation_axis_angles,
                                       Descriptor_rotation_axis_angles
            )
                                       .value_or_throw();
            return std::move(*this);
        }

        /// Rotations via quaternion.
        ///
        /// If no rotation is specified, the axes of the ellipsoid align with the axes of the local coordinate system.
        /// Note that this uses a `components::PoseRotationQuat` which is also used by `archetypes::InstancePoses3D`.
        Ellipsoids3D with_quaternions(
            const Collection<rerun::components::PoseRotationQuat>& _quaternions
        ) && {
            quaternions = ComponentBatch::from_loggable(_quaternions, Descriptor_quaternions)
                              .value_or_throw();
            return std::move(*this);
        }

        /// Optional colors for the ellipsoids.
        Ellipsoids3D with_colors(const Collection<rerun::components::Color>& _colors) && {
            colors = ComponentBatch::from_loggable(_colors, Descriptor_colors).value_or_throw();
            return std::move(*this);
        }

        /// Optional radii for the lines used when the ellipsoid is rendered as a wireframe.
        Ellipsoids3D with_line_radii(const Collection<rerun::components::Radius>& _line_radii) && {
            line_radii =
                ComponentBatch::from_loggable(_line_radii, Descriptor_line_radii).value_or_throw();
            return std::move(*this);
        }

        /// Optionally choose whether the ellipsoids are drawn with lines or solid.
        Ellipsoids3D with_fill_mode(const rerun::components::FillMode& _fill_mode) && {
            fill_mode =
                ComponentBatch::from_loggable(_fill_mode, Descriptor_fill_mode).value_or_throw();
            return std::move(*this);
        }

        /// Optional text labels for the ellipsoids.
        Ellipsoids3D with_labels(const Collection<rerun::components::Text>& _labels) && {
            labels = ComponentBatch::from_loggable(_labels, Descriptor_labels).value_or_throw();
            return std::move(*this);
        }

        /// Optional choice of whether the text labels should be shown by default.
        Ellipsoids3D with_show_labels(const rerun::components::ShowLabels& _show_labels) && {
            show_labels = ComponentBatch::from_loggable(_show_labels, Descriptor_show_labels)
                              .value_or_throw();
            return std::move(*this);
        }

        /// Optional class ID for the ellipsoids.
        ///
        /// The class ID provides colors and labels if not specified explicitly.
        Ellipsoids3D with_class_ids(const Collection<rerun::components::ClassId>& _class_ids) && {
            class_ids =
                ComponentBatch::from_loggable(_class_ids, Descriptor_class_ids).value_or_throw();
            return std::move(*this);
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::Ellipsoids3D> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(
            const archetypes::Ellipsoids3D& archetype
        );
    };
} // namespace rerun
