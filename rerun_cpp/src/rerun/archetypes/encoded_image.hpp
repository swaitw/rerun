// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/encoded_image.fbs".

#pragma once

#include "../collection.hpp"
#include "../component_batch.hpp"
#include "../component_column.hpp"
#include "../components/blob.hpp"
#include "../components/draw_order.hpp"
#include "../components/media_type.hpp"
#include "../components/opacity.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <filesystem>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: An image encoded as e.g. a JPEG or PNG.
    ///
    /// Rerun also supports uncompressed images with the `archetypes::Image`.
    /// For images that refer to video frames see `archetypes::VideoFrameReference`.
    ///
    /// ## Example
    ///
    /// ### encoded_image:
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <filesystem>
    /// #include <fstream>
    /// #include <iostream>
    /// #include <vector>
    ///
    /// namespace fs = std::filesystem;
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_encoded_image");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     fs::path image_filepath = fs::path(__FILE__).parent_path() / "ferris.png";
    ///
    ///     rec.log("image", rerun::EncodedImage::from_file(image_filepath).value_or_throw());
    /// }
    /// ```
    struct EncodedImage {
        /// The encoded content of some image file, e.g. a PNG or JPEG.
        std::optional<ComponentBatch> blob;

        /// The Media Type of the asset.
        ///
        /// Supported values:
        /// * `image/jpeg`
        /// * `image/png`
        ///
        /// If omitted, the viewer will try to guess from the data blob.
        /// If it cannot guess, it won't be able to render the asset.
        std::optional<ComponentBatch> media_type;

        /// Opacity of the image, useful for layering several images.
        ///
        /// Defaults to 1.0 (fully opaque).
        std::optional<ComponentBatch> opacity;

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        std::optional<ComponentBatch> draw_order;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.EncodedImageIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.archetypes.EncodedImage";

        /// `ComponentDescriptor` for the `blob` field.
        static constexpr auto Descriptor_blob = ComponentDescriptor(
            ArchetypeName, "blob", Loggable<rerun::components::Blob>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `media_type` field.
        static constexpr auto Descriptor_media_type = ComponentDescriptor(
            ArchetypeName, "media_type",
            Loggable<rerun::components::MediaType>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `opacity` field.
        static constexpr auto Descriptor_opacity = ComponentDescriptor(
            ArchetypeName, "opacity",
            Loggable<rerun::components::Opacity>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `draw_order` field.
        static constexpr auto Descriptor_draw_order = ComponentDescriptor(
            ArchetypeName, "draw_order",
            Loggable<rerun::components::DrawOrder>::Descriptor.component_name
        );

      public: // START of extensions from encoded_image_ext.cpp:
        /// Create a new `EncodedImage` from the contents of a file on disk, e.g. a PNG or JPEG.
        static Result<EncodedImage> from_file(const std::filesystem::path& filepath);

        /// Create a new `EncodedImage` from the contents of an image file, like a PNG or JPEG.
        ///
        /// If no `MediaType` is specified, the Rerun Viewer will try to guess one from the data
        /// at render-time. If it can't, rendering will fail with an error.
        static EncodedImage from_bytes(
            rerun::Collection<uint8_t> image_contents,
            std::optional<rerun::components::MediaType> media_type = {}
        ) {
            auto encoded_image = EncodedImage().with_blob(image_contents);
            if (media_type.has_value()) {
                return std::move(encoded_image).with_media_type(media_type.value());
            }
            return encoded_image;
        }

        // END of extensions from encoded_image_ext.cpp, start of generated code:

      public:
        EncodedImage() = default;
        EncodedImage(EncodedImage&& other) = default;
        EncodedImage(const EncodedImage& other) = default;
        EncodedImage& operator=(const EncodedImage& other) = default;
        EncodedImage& operator=(EncodedImage&& other) = default;

        /// Update only some specific fields of a `EncodedImage`.
        static EncodedImage update_fields() {
            return EncodedImage();
        }

        /// Clear all the fields of a `EncodedImage`.
        static EncodedImage clear_fields();

        /// The encoded content of some image file, e.g. a PNG or JPEG.
        EncodedImage with_blob(const rerun::components::Blob& _blob) && {
            blob = ComponentBatch::from_loggable(_blob, Descriptor_blob).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `blob` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_blob` should
        /// be used when logging a single row's worth of data.
        EncodedImage with_many_blob(const Collection<rerun::components::Blob>& _blob) && {
            blob = ComponentBatch::from_loggable(_blob, Descriptor_blob).value_or_throw();
            return std::move(*this);
        }

        /// The Media Type of the asset.
        ///
        /// Supported values:
        /// * `image/jpeg`
        /// * `image/png`
        ///
        /// If omitted, the viewer will try to guess from the data blob.
        /// If it cannot guess, it won't be able to render the asset.
        EncodedImage with_media_type(const rerun::components::MediaType& _media_type) && {
            media_type =
                ComponentBatch::from_loggable(_media_type, Descriptor_media_type).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `media_type` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_media_type` should
        /// be used when logging a single row's worth of data.
        EncodedImage with_many_media_type(
            const Collection<rerun::components::MediaType>& _media_type
        ) && {
            media_type =
                ComponentBatch::from_loggable(_media_type, Descriptor_media_type).value_or_throw();
            return std::move(*this);
        }

        /// Opacity of the image, useful for layering several images.
        ///
        /// Defaults to 1.0 (fully opaque).
        EncodedImage with_opacity(const rerun::components::Opacity& _opacity) && {
            opacity = ComponentBatch::from_loggable(_opacity, Descriptor_opacity).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `opacity` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_opacity` should
        /// be used when logging a single row's worth of data.
        EncodedImage with_many_opacity(const Collection<rerun::components::Opacity>& _opacity) && {
            opacity = ComponentBatch::from_loggable(_opacity, Descriptor_opacity).value_or_throw();
            return std::move(*this);
        }

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        EncodedImage with_draw_order(const rerun::components::DrawOrder& _draw_order) && {
            draw_order =
                ComponentBatch::from_loggable(_draw_order, Descriptor_draw_order).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `draw_order` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_draw_order` should
        /// be used when logging a single row's worth of data.
        EncodedImage with_many_draw_order(
            const Collection<rerun::components::DrawOrder>& _draw_order
        ) && {
            draw_order =
                ComponentBatch::from_loggable(_draw_order, Descriptor_draw_order).value_or_throw();
            return std::move(*this);
        }

        /// Partitions the component data into multiple sub-batches.
        ///
        /// Specifically, this transforms the existing `ComponentBatch` data into `ComponentColumn`s
        /// instead, via `ComponentBatch::partitioned`.
        ///
        /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
        ///
        /// The specified `lengths` must sum to the total length of the component batch.
        Collection<ComponentColumn> columns(const Collection<uint32_t>& lengths_);

        /// Partitions the component data into unit-length sub-batches.
        ///
        /// This is semantically similar to calling `columns` with `std::vector<uint32_t>(n, 1)`,
        /// where `n` is automatically guessed.
        Collection<ComponentColumn> columns();
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::EncodedImage> {
        /// Serialize all set component batches.
        static Result<Collection<ComponentBatch>> as_batches(
            const archetypes::EncodedImage& archetype
        );
    };
} // namespace rerun
