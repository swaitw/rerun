use std::{
    collections::{BTreeMap, BTreeSet},
    ops::RangeInclusive,
};

use egui::*;

use re_data_store::{InstanceId, ObjectTree};
use re_log_types::*;

use crate::{
    misc::time_control::Looping, time_axis::TimelineAxis, Selection, TimeControl, TimeView,
    ViewerContext,
};

use super::Blueprint;

/// A panel that shows objects to the left, time on the top.
///
/// This includes the timeline controls and streams view.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub(crate) struct TimePanel {
    /// Width of the object name columns previous frame.
    prev_col_width: f32,

    /// The right side of the object name column; updated during its painting.
    #[serde(skip)]
    next_col_right: f32,

    /// The time axis view, regenerated each frame.
    #[serde(skip)]
    time_ranges_ui: TimeRangesUi,
}

impl Default for TimePanel {
    fn default() -> Self {
        Self {
            prev_col_width: 400.0,
            next_col_right: 0.0,
            time_ranges_ui: Default::default(),
        }
    }
}

impl TimePanel {
    pub fn show_panel(
        &mut self,
        ctx: &mut ViewerContext<'_>,
        blueprint: &mut Blueprint,
        egui_ctx: &egui::Context,
    ) {
        let panel_frame = ctx.re_ui.panel_frame();

        let collapsed = egui::TopBottomPanel::bottom("time_panel_collapsed")
            .resizable(false)
            .frame(panel_frame)
            .default_height(16.0);
        let expanded = egui::TopBottomPanel::bottom("time_panel_expanded")
            .resizable(true)
            .frame(panel_frame)
            .default_height(250.0);

        egui::TopBottomPanel::show_animated_between(
            egui_ctx,
            blueprint.time_panel_expanded,
            collapsed,
            expanded,
            |ui: &mut egui::Ui, expansion: f32| {
                if expansion < 1.0 {
                    // Collapsed or animating
                    ui.horizontal(|ui| {
                        self.collapsed_ui(ctx, ui);
                    });
                } else {
                    // Expanded:
                    self.expanded_ui(ctx, ui);
                }
            },
        );
    }

    #[allow(clippy::unused_self)]
    fn collapsed_ui(&mut self, ctx: &mut ViewerContext<'_>, ui: &mut egui::Ui) {
        ctx.rec_cfg
            .time_ctrl
            .timeline_selector_ui(ctx.log_db.times_per_timeline(), ui);

        ui.separator();

        ctx.rec_cfg
            .time_ctrl
            .play_pause_ui(ctx.log_db.times_per_timeline(), ui);

        ui.separator();

        {
            let time_range_width = 400.0;
            let mut time_range_rect = ui.available_rect_before_wrap();
            time_range_rect.max.x = f32::min(
                time_range_rect.max.x - 220.0, // save space for current time and help button,
                time_range_rect.min.x + time_range_width,
            );

            if time_range_rect.width() > 50.0 {
                let time_ranges_ui =
                    initialize_time_ranges_ui(ctx, time_range_rect.x_range(), None, 0.0);
                time_ranges_ui.snap_time_control(ctx);

                let painter = ui.painter_at(time_range_rect.expand(4.0));
                painter.hline(
                    time_range_rect.x_range(),
                    time_range_rect.center().y,
                    ui.visuals().widgets.inactive.fg_stroke,
                );
                time_marker_ui(
                    &time_ranges_ui,
                    &mut ctx.rec_cfg.time_ctrl,
                    ui,
                    &painter,
                    &time_range_rect,
                    time_range_rect.bottom(),
                );

                ui.allocate_rect(time_range_rect, egui::Sense::hover());
            }
        }

        current_time_ui(ctx, ui);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            help_button(ui);
        });
    }

    fn expanded_ui(&mut self, ctx: &mut ViewerContext<'_>, ui: &mut egui::Ui) {
        crate::profile_function!();

        // play control and current time
        top_row_ui(ctx, ui);

        ui.add_space(2.0);

        self.next_col_right = ui.min_rect().left(); // next_col_right will expand during the call

        let time_x_left = ui.min_rect().left() + self.prev_col_width + ui.spacing().item_spacing.x;

        // Where the time will be shown.
        let time_x_range = {
            let right =
                ui.max_rect().right() - ui.spacing().scroll_bar_width - ui.spacing().item_spacing.x;
            time_x_left..=right
        };

        self.time_ranges_ui = initialize_time_ranges_ui(
            ctx,
            time_x_range.clone(),
            ctx.rec_cfg.time_ctrl.time_view(),
            SIDE_MARGIN,
        );

        // includes the loop selection and time ticks rows.
        let time_area_rect = Rect::from_x_y_ranges(
            time_x_range.clone(),
            ui.min_rect().bottom()..=ui.max_rect().bottom(),
        );

        let loop_selection_rect = {
            let response = ui.label(" "); // Add some vertical space large enough to fit text for the loop selection row.
            let y_range = response.rect.y_range();
            Rect::from_x_y_ranges(time_x_range.clone(), y_range)
        };

        let timeline_rect = {
            let response = ui.weak("Streams");
            let y_range = response.rect.y_range();
            Rect::from_x_y_ranges(time_x_range.clone(), y_range)
        };

        let time_area_painter = ui.painter().with_clip_rect(time_area_rect);

        ui.painter()
            .rect_filled(time_area_rect, 1.0, ui.visuals().extreme_bg_color);

        ui.separator();

        paint_time_ranges_and_ticks(
            &self.time_ranges_ui,
            ui,
            &time_area_painter,
            loop_selection_rect.top()..=timeline_rect.bottom(),
            timeline_rect.top()..=time_area_rect.bottom(),
            ctx.rec_cfg.time_ctrl.time_type(),
        );
        loop_selection_ui(
            &self.time_ranges_ui,
            &mut ctx.rec_cfg.time_ctrl,
            ui,
            &time_area_painter,
            &loop_selection_rect,
        );
        time_marker_ui(
            &self.time_ranges_ui,
            &mut ctx.rec_cfg.time_ctrl,
            ui,
            &time_area_painter,
            &timeline_rect,
            time_area_rect.bottom(),
        );
        let time_area_response = interact_with_time_area(
            &self.time_ranges_ui,
            &mut ctx.rec_cfg.time_ctrl,
            ui,
            &time_area_rect,
        );

        // Don't draw on top of the time ticks
        let lower_time_area_painter = ui.painter().with_clip_rect(Rect::from_x_y_ranges(
            time_x_range,
            ui.min_rect().bottom()..=ui.max_rect().bottom(),
        ));

        // all the object rows:
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                crate::profile_scope!("tree_ui");
                if time_area_response.dragged_by(PointerButton::Primary) {
                    ui.scroll_with_delta(Vec2::Y * time_area_response.drag_delta().y);
                }
                self.tree_ui(ctx, &time_area_response, &lower_time_area_painter, ui);
            });

        // TODO(emilk): fix problem of the fade covering the hlines. Need Shape Z values! https://github.com/emilk/egui/issues/1516
        if true {
            fade_sides(ui, time_area_rect);
        }

        self.time_ranges_ui.snap_time_control(ctx);

        // remember where to show the time for next frame:
        self.prev_col_width = self.next_col_right - ui.min_rect().left();
    }

    fn tree_ui(
        &mut self,
        ctx: &mut ViewerContext<'_>,
        time_area_response: &egui::Response,
        time_area_painter: &egui::Painter,
        ui: &mut egui::Ui,
    ) {
        self.show_children(
            ctx,
            time_area_response,
            time_area_painter,
            &ctx.log_db.obj_db.tree,
            ui,
        );
    }

    fn show_tree(
        &mut self,
        ctx: &mut ViewerContext<'_>,
        time_area_response: &egui::Response,
        time_area_painter: &egui::Painter,
        // the parent path of the name component
        last_component: &ObjPathComp,
        tree: &ObjectTree,
        ui: &mut egui::Ui,
    ) {
        use egui::*;

        if !tree
            .prefix_times
            .has_timeline(ctx.rec_cfg.time_ctrl.timeline())
        {
            return; // ignore objects that have no data for the current timeline
        }

        // The last part of the the path component
        let text = if tree.is_leaf() {
            last_component.to_string()
        } else {
            format!("{}/", last_component) // show we have children with a /
        };

        let collapsing_header_id = ui.make_persistent_id(&tree.path);
        let default_open = tree.path.len() <= 1 && !tree.is_leaf();
        let (_collapsing_button_response, custom_header_response, body_returned) =
            egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                collapsing_header_id,
                default_open,
            )
            .show_header(ui, |ui| ctx.obj_path_button_to(ui, text, &tree.path))
            .body(|ui| {
                self.show_children(ctx, time_area_response, time_area_painter, tree, ui);
            });

        let is_closed = body_returned.is_none();
        let response = custom_header_response.response;
        let response_rect = response.rect;
        self.next_col_right = self.next_col_right.max(response_rect.right());

        let full_width_rect = Rect::from_x_y_ranges(
            response_rect.left()..=ui.max_rect().right(),
            response_rect.y_range(),
        );

        let is_visible = ui.is_rect_visible(full_width_rect);

        if is_visible {
            response.on_hover_ui(|ui| {
                ui.label(tree.path.to_string());
            });
        }

        if is_visible {
            // paint hline guide:
            let mut stroke = ui.visuals().widgets.noninteractive.bg_stroke;
            stroke.color = stroke.color.linear_multiply(0.5);
            let left = response_rect.left() + ui.spacing().indent;
            let y = response_rect.bottom() + ui.spacing().item_spacing.y * 0.5;
            ui.painter().hline(left..=ui.max_rect().right(), y, stroke);
        }

        // ----------------------------------------------

        // show the data in the time area:

        if is_visible && is_closed {
            if let Some(messages_over_time) =
                tree.prefix_times.get(ctx.rec_cfg.time_ctrl.timeline())
            {
                show_data_over_time(
                    ctx,
                    time_area_response,
                    time_area_painter,
                    ui,
                    messages_over_time,
                    full_width_rect,
                    &self.time_ranges_ui,
                    Some(Selection::Instance(InstanceId {
                        obj_path: tree.path.clone(),
                        instance_index: None,
                    })),
                );
            }
        }
    }

    fn show_children(
        &mut self,
        ctx: &mut ViewerContext<'_>,
        time_area_response: &egui::Response,
        time_area_painter: &egui::Painter,
        tree: &ObjectTree,
        ui: &mut egui::Ui,
    ) {
        for (last_component, child) in &tree.children {
            self.show_tree(
                ctx,
                time_area_response,
                time_area_painter,
                last_component,
                child,
                ui,
            );
        }

        // If this is an object:
        if !tree.fields.is_empty() {
            let indent = ui.spacing().indent;

            for (field_name, data) in &tree.fields {
                if !data.times.contains_key(ctx.rec_cfg.time_ctrl.timeline()) {
                    continue; // ignore fields that have no data for the current timeline
                }

                let data_path = DataPath::new(tree.path.clone(), *field_name);

                let response = ui
                    .horizontal(|ui| {
                        // Add some spacing to match CollapsingHeader:
                        ui.spacing_mut().item_spacing.x = 0.0;
                        let response =
                            ui.allocate_response(egui::vec2(indent, 0.0), egui::Sense::hover());
                        ui.painter().circle_filled(
                            response.rect.center(),
                            2.0,
                            ui.visuals().text_color(),
                        );
                        ctx.data_path_button_to(ui, field_name.as_str(), &data_path);
                    })
                    .response;

                self.next_col_right = self.next_col_right.max(response.rect.right());

                let full_width_rect = Rect::from_x_y_ranges(
                    response.rect.left()..=ui.max_rect().right(),
                    response.rect.y_range(),
                );
                let is_visible = ui.is_rect_visible(full_width_rect);

                if is_visible {
                    // paint hline guide:
                    let mut stroke = ui.visuals().widgets.noninteractive.bg_stroke;
                    stroke.color = stroke.color.linear_multiply(0.5);
                    let left = response.rect.left() + ui.spacing().indent;
                    let y = response.rect.bottom() + ui.spacing().item_spacing.y * 0.5;
                    ui.painter().hline(left..=ui.max_rect().right(), y, stroke);
                }

                if is_visible {
                    response.on_hover_ui(|ui| {
                        ui.label(data_path.to_string());
                        let summary = data.summary();
                        if !summary.is_empty() {
                            ui.label(summary);
                        }
                    });
                }

                // show the data in the time area:

                if is_visible {
                    if let Some(messages_over_time) =
                        data.times.get(ctx.rec_cfg.time_ctrl.timeline())
                    {
                        show_data_over_time(
                            ctx,
                            time_area_response,
                            time_area_painter,
                            ui,
                            messages_over_time,
                            full_width_rect,
                            &self.time_ranges_ui,
                            Some(Selection::DataPath(data_path)),
                        );
                    }
                }
            }
        }
    }
}

fn top_row_ui(ctx: &mut ViewerContext<'_>, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ctx.rec_cfg
            .time_ctrl
            .timeline_selector_ui(ctx.log_db.times_per_timeline(), ui);

        ui.separator();

        ctx.rec_cfg
            .time_ctrl
            .play_pause_ui(ctx.log_db.times_per_timeline(), ui);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            help_button(ui);
            ui.vertical_centered(|ui| {
                current_time_ui(ctx, ui);
            });
        });
    });
}

fn help_button(ui: &mut egui::Ui) {
    crate::misc::help_hover_button(ui).on_hover_text(
        "Drag main area to pan.\n\
         Zoom: Ctrl/cmd + scroll, or drag up/down with secondary mouse button.\n\
         Double-click to reset view.\n\
         Press spacebar to pause/resume.",
    );
}

fn current_time_ui(ctx: &mut ViewerContext<'_>, ui: &mut egui::Ui) {
    if let Some(time_int) = ctx.rec_cfg.time_ctrl.time_int() {
        let time_type = ctx.rec_cfg.time_ctrl.time_type();
        ui.monospace(time_type.format(time_int));
    }
}

#[allow(clippy::too_many_arguments)]
fn show_data_over_time(
    ctx: &mut ViewerContext<'_>,
    time_area_response: &egui::Response,
    time_area_painter: &egui::Painter,
    ui: &mut egui::Ui,
    messages_over_time: &BTreeMap<TimeInt, BTreeSet<MsgId>>,
    full_width_rect: Rect,
    time_ranges_ui: &TimeRangesUi,
    select_on_click: Option<Selection>,
) {
    crate::profile_function!();

    let cur_selection = ctx.selection();
    let is_selected = cur_selection.is_some() && select_on_click.as_ref() == Some(&cur_selection);

    // painting each data point as a separate circle is slow (too many circles!)
    // so we join time points that are close together.
    let points_per_time = time_ranges_ui.points_per_time().unwrap_or(f32::INFINITY);
    let max_stretch_length_in_time = 1.0 / points_per_time as f64; // TODO(emilk)

    let pointer_pos = ui.input().pointer.hover_pos();

    let hovered_color = ui.visuals().widgets.hovered.text_color();
    let inactive_color = if is_selected {
        ui.visuals().selection.stroke.color
    } else {
        ui.visuals()
            .widgets
            .inactive
            .text_color()
            .linear_multiply(0.75)
    };

    struct Stretch<'a> {
        start_x: f32,
        start_time: TimeInt,
        stop_time: TimeInt,
        selected: bool,
        msg_ids: Vec<&'a BTreeSet<MsgId>>,
    }

    let mut shapes = vec![];
    let mut scatter = BallScatterer::default();
    let mut hovered_messages = vec![];
    let mut hovered_time = None;

    let mut paint_stretch = |stretch: &Stretch<'_>| {
        let stop_x = time_ranges_ui
            .x_from_time(stretch.stop_time.into())
            .unwrap_or(stretch.start_x);

        let num_messages: usize = stretch.msg_ids.iter().map(|l| l.len()).sum();
        let radius = 2.5 * (1.0 + 0.5 * (num_messages as f32).log10());
        let radius = radius.at_most(full_width_rect.height() / 3.0);

        let x = (stretch.start_x + stop_x) * 0.5;
        let pos = scatter.add(x, radius, (full_width_rect.top(), full_width_rect.bottom()));

        let is_hovered = pointer_pos.map_or(false, |pointer_pos| {
            pos.distance(pointer_pos) < radius + 1.0
        });

        let mut color = if is_hovered {
            hovered_color
        } else {
            inactive_color
        };
        if ui.visuals().dark_mode {
            color = color.additive();
        }

        let radius = if is_hovered {
            1.75 * radius
        } else if stretch.selected {
            1.25 * radius
        } else {
            radius
        };

        shapes.push(Shape::circle_filled(pos, radius, color));

        if is_hovered {
            hovered_messages.extend(stretch.msg_ids.iter().copied().flatten().copied());
            hovered_time.get_or_insert(stretch.start_time);
        }
    };

    let mut stretch: Option<Stretch<'_>> = None;

    let selected_time_range = ctx.rec_cfg.time_ctrl.active_loop_selection();

    for (&time, msg_ids) in messages_over_time {
        let time_real = TimeReal::from(time);

        let selected = selected_time_range.map_or(true, |range| range.contains(time_real));

        if let Some(current_stretch) = &mut stretch {
            if current_stretch.selected == selected
                && (time - current_stretch.start_time).as_f64() < max_stretch_length_in_time
            {
                // extend:
                current_stretch.stop_time = time;
                current_stretch.msg_ids.push(msg_ids);
            } else {
                // stop the previous…
                paint_stretch(current_stretch);

                stretch = None;
            }
        }

        if stretch.is_none() {
            if let Some(x) = time_ranges_ui.x_from_time(time_real) {
                stretch = Some(Stretch {
                    start_x: x,
                    start_time: time,
                    stop_time: time,
                    selected,
                    msg_ids: vec![msg_ids],
                });
            }
        }
    }

    if let Some(stretch) = stretch {
        paint_stretch(&stretch);
    }

    time_area_painter.extend(shapes);

    if !hovered_messages.is_empty() {
        if time_area_response.clicked_by(egui::PointerButton::Primary) {
            if let Some(select_on_click) = select_on_click {
                ctx.set_selection(select_on_click);
            } else {
                ctx.clear_selection();
            }

            if let Some(hovered_time) = hovered_time {
                ctx.rec_cfg.time_ctrl.set_time(hovered_time);
                ctx.rec_cfg.time_ctrl.pause();
            }
        } else if !ui.ctx().memory().is_anything_being_dragged() {
            show_msg_ids_tooltip(ctx, ui.ctx(), &hovered_messages);
        }
    }
}

fn show_msg_ids_tooltip(ctx: &mut ViewerContext<'_>, egui_ctx: &egui::Context, msg_ids: &[MsgId]) {
    show_tooltip_at_pointer(egui_ctx, Id::new("data_tooltip"), |ui| {
        // TODO(emilk): show as a table?
        if msg_ids.len() == 1 {
            let msg_id = msg_ids[0];
            if let Some(msg) = ctx.log_db.get_log_msg(&msg_id) {
                ui.push_id(msg_id, |ui| {
                    ui.group(|ui| {
                        crate::data_ui::show_log_msg(ctx, ui, msg, crate::Preview::Small);
                    });
                });
            }
        } else {
            ui.label(format!(
                "{} messages",
                re_format::format_number(msg_ids.len())
            ));
        }
    });
}

// ----------------------------------------------------------------------------

fn initialize_time_ranges_ui(
    ctx: &mut ViewerContext<'_>,
    time_x_range: RangeInclusive<f32>,
    time_view: Option<TimeView>,
    side_margin: f32,
) -> TimeRangesUi {
    crate::profile_function!();
    if let Some(times) = ctx
        .log_db
        .times_per_timeline()
        .get(ctx.rec_cfg.time_ctrl.timeline())
    {
        let timeline_axis = TimelineAxis::new(ctx.rec_cfg.time_ctrl.time_type(), times);
        let time_view = time_view
            .unwrap_or_else(|| view_everything(&time_x_range, &timeline_axis, side_margin));

        TimeRangesUi::new(time_x_range, time_view, &timeline_axis.ranges)
    } else {
        Default::default()
    }
}

fn paint_time_ranges_and_ticks(
    time_ranges_ui: &TimeRangesUi,
    ui: &mut egui::Ui,
    time_area_painter: &egui::Painter,
    line_y_range: RangeInclusive<f32>,
    segment_y_range: RangeInclusive<f32>,
    time_type: TimeType,
) {
    for segment in &time_ranges_ui.segments {
        let bg_stroke = ui.visuals().widgets.noninteractive.bg_stroke;
        let rect = Rect::from_x_y_ranges(segment.x.clone(), segment_y_range.clone());
        time_area_painter.rect_filled(rect, 1.0, bg_stroke.color.linear_multiply(0.5));

        let rect = Rect::from_x_y_ranges(segment.x.clone(), line_y_range.clone());
        paint_time_range_ticks(ui, time_area_painter, &rect, time_type, &segment.time);
    }

    if false {
        // visually separate the different ranges:
        use itertools::Itertools as _;
        for (a, b) in time_ranges_ui.segments.iter().tuple_windows() {
            let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
            let x = lerp(*a.x.end()..=*b.x.start(), 0.5);
            let y_top = *segment_y_range.start();
            let y_bottom = *segment_y_range.end();
            time_area_painter.vline(x, y_top..=y_bottom, stroke);
        }
    }
}

/// Returns a scroll delta
#[must_use]
fn interact_with_time_area(
    time_ranges_ui: &TimeRangesUi,
    time_ctrl: &mut TimeControl,
    ui: &mut egui::Ui,
    full_rect: &Rect,
) -> egui::Response {
    let pointer_pos = ui.input().pointer.hover_pos();

    let time_area_response = ui.interact(
        *full_rect,
        ui.id().with("time_area_interact"),
        egui::Sense::click_and_drag(),
    );

    let mut delta_x = 0.0;
    let mut zoom_factor = 1.0;

    if time_area_response.hovered() {
        delta_x += ui.input().scroll_delta.x;
        zoom_factor *= ui.input().zoom_delta_2d().x;
    }

    if time_area_response.dragged_by(PointerButton::Primary) {
        delta_x += time_area_response.drag_delta().x;
        ui.output().cursor_icon = CursorIcon::AllScroll;
    }
    if time_area_response.dragged_by(PointerButton::Secondary) {
        zoom_factor *= (time_area_response.drag_delta().y * 0.01).exp();
    }

    if delta_x != 0.0 {
        if let Some(new_view_range) = time_ranges_ui.pan(-delta_x) {
            time_ctrl.set_time_view(new_view_range);
        }
    }

    if zoom_factor != 1.0 {
        if let Some(pointer_pos) = pointer_pos {
            if let Some(new_view_range) = time_ranges_ui.zoom_at(pointer_pos.x, zoom_factor) {
                time_ctrl.set_time_view(new_view_range);
            }
        }
    }

    if time_area_response.double_clicked() {
        time_ctrl.reset_time_view();
    }

    time_area_response
}

fn initial_time_selection(
    time_ranges_ui: &TimeRangesUi,
    time_type: TimeType,
) -> Option<TimeRangeF> {
    let ranges = &time_ranges_ui.segments;

    // Try to find a long duration first, then fall back to shorter
    for min_duration in [2.0, 0.5, 0.0] {
        for segment in ranges {
            let range = &segment.tight_time;
            if range.min < range.max {
                match time_type {
                    TimeType::Time => {
                        let seconds = Duration::from(range.max - range.min).as_secs_f64();
                        if seconds > min_duration {
                            let one_sec = TimeInt::from(Duration::from_secs(1.0));
                            return Some(TimeRangeF::new(range.min, range.min + one_sec));
                        }
                    }
                    TimeType::Sequence => {
                        return Some(TimeRangeF::new(
                            range.min,
                            TimeReal::from(range.min)
                                + TimeReal::from((range.max - range.min).as_f64() / 2.0),
                        ));
                    }
                }
            }
        }
    }

    // all ranges have just a single data point in it. sight

    if ranges.len() < 2 {
        None // not enough to show anything meaningful
    } else {
        let end = (ranges.len() / 2).at_least(1);
        Some(TimeRangeF::new(
            ranges[0].tight_time.min,
            ranges[end].tight_time.max,
        ))
    }
}

fn loop_selection_ui(
    time_ranges_ui: &TimeRangesUi,
    time_ctrl: &mut TimeControl,
    ui: &mut egui::Ui,
    time_area_painter: &egui::Painter,
    rect: &Rect,
) {
    if time_ctrl.loop_selection().is_none() {
        // Helpfully select a time slice so that there always is a selection.
        // This helps new users ("what is that?").
        if let Some(selection) = initial_time_selection(time_ranges_ui, time_ctrl.time_type()) {
            time_ctrl.set_loop_selection(selection);
        }
    }

    if time_ctrl.loop_selection().is_none() && time_ctrl.looping == Looping::Selection {
        time_ctrl.looping = Looping::Off;
    }

    // TODO(emilk): click to toggle on/off
    // when off, you cannot modify, just drag out a new one.

    let selection_color = re_ui::ReUi::loop_selection_color();

    let mut did_interact = false;

    let is_active = time_ctrl.looping == Looping::Selection;

    let pointer_pos = ui.input().pointer.hover_pos();
    let is_pointer_in_rect = pointer_pos.map_or(false, |pointer_pos| rect.contains(pointer_pos));

    let left_edge_id = ui.id().with("selection_left_edge");
    let right_edge_id = ui.id().with("selection_right_edge");
    let move_id = ui.id().with("selection_move");

    let interact_radius = ui.style().interaction.resize_grab_radius_side;

    let mut is_hovering_existing = false;

    let transparent = if ui.visuals().dark_mode { 0.06 } else { 0.3 };

    // Paint existing selection and detect drag starting and hovering:
    if let Some(selected_range) = time_ctrl.loop_selection() {
        let min_x = time_ranges_ui.x_from_time(selected_range.min);
        let max_x = time_ranges_ui.x_from_time(selected_range.max);

        if let (Some(min_x), Some(max_x)) = (min_x, max_x) {
            let mut rect = Rect::from_x_y_ranges(min_x..=max_x, rect.y_range());

            // Make sure it is visible:
            if rect.width() < 2.0 {
                rect = Rect::from_x_y_ranges(
                    (rect.center().x - 1.0)..=(rect.center().x - 1.0),
                    rect.y_range(),
                );
            }

            let full_y_range = rect.top()..=time_area_painter.clip_rect().bottom();

            if is_active {
                let bg_color = selection_color.linear_multiply(transparent);
                time_area_painter.rect_filled(
                    Rect::from_x_y_ranges(rect.x_range(), full_y_range),
                    1.0,
                    bg_color,
                );
            }

            let main_color = if is_active {
                selection_color
            } else {
                selection_color.linear_multiply(transparent)
            };
            time_area_painter.rect_filled(rect, 1.0, main_color);

            if is_active && !selected_range.is_empty() {
                let range_text =
                    format_duration(time_ctrl.time_type(), selected_range.length().abs());
                if !range_text.is_empty() {
                    let font_id = egui::TextStyle::Body.resolve(ui.style());
                    let text_color = ui.visuals().strong_text_color();
                    time_area_painter.text(
                        rect.left_center(),
                        Align2::LEFT_CENTER,
                        range_text,
                        font_id,
                        text_color,
                    );
                }
            }

            // Check for interaction:
            if let Some(pointer_pos) = pointer_pos {
                if rect.expand(interact_radius).contains(pointer_pos) {
                    let center_dist = (pointer_pos.x - rect.center().x).abs(); // make sure we can always move even small rects
                    let left_dist = (pointer_pos.x - min_x).abs();
                    let right_dist = (pointer_pos.x - max_x).abs();

                    let hovering_left =
                        left_dist < center_dist.min(right_dist).min(interact_radius);
                    let hovering_right =
                        !hovering_left && right_dist <= interact_radius.min(center_dist);
                    let hovering_move = !hovering_left
                        && !hovering_right
                        && (min_x <= pointer_pos.x && pointer_pos.x <= max_x);

                    let drag_started =
                        ui.input().pointer.any_pressed() && ui.input().pointer.primary_down();

                    if hovering_left {
                        ui.output().cursor_icon = CursorIcon::ResizeWest;
                        if drag_started {
                            ui.memory().set_dragged_id(left_edge_id);
                        }
                    } else if hovering_right {
                        ui.output().cursor_icon = CursorIcon::ResizeEast;
                        if drag_started {
                            ui.memory().set_dragged_id(right_edge_id);
                        }
                    } else if hovering_move {
                        ui.output().cursor_icon = CursorIcon::Move;
                        if drag_started {
                            ui.memory().set_dragged_id(move_id);
                        }
                    }

                    is_hovering_existing = hovering_left | hovering_right | hovering_move;
                }
            }
        }
    }

    // Start new selection?
    if let Some(pointer_pos) = pointer_pos {
        let is_anything_being_dragged = ui.memory().is_anything_being_dragged();
        if !is_hovering_existing
            && is_pointer_in_rect
            && !is_anything_being_dragged
            && ui.input().pointer.primary_down()
        {
            if let Some(time) = time_ranges_ui.time_from_x(pointer_pos.x) {
                time_ctrl.set_loop_selection(TimeRangeF::point(time));
                did_interact = true;
                ui.memory().set_dragged_id(right_edge_id);
            }
        }
    }

    // Resize/move (interact)
    if let Some(pointer_pos) = pointer_pos {
        if let Some(mut selected_range) = time_ctrl.loop_selection() {
            // Use "smart_aim" to find a natural length of the time interval
            let aim_radius = ui.input().aim_radius();
            use egui::emath::smart_aim::best_in_range_f64;

            if ui.memory().is_being_dragged(left_edge_id) {
                if let (Some(time_low), Some(time_high)) = (
                    time_ranges_ui.time_from_x(pointer_pos.x - aim_radius),
                    time_ranges_ui.time_from_x(pointer_pos.x + aim_radius),
                ) {
                    let low_length = selected_range.max - time_low;
                    let high_length = selected_range.max - time_high;
                    let best_length = TimeReal::from(best_in_range_f64(
                        low_length.as_f64(),
                        high_length.as_f64(),
                    ));

                    selected_range.min = selected_range.max - best_length;

                    if selected_range.min > selected_range.max {
                        std::mem::swap(&mut selected_range.min, &mut selected_range.max);
                        ui.memory().set_dragged_id(right_edge_id);
                    }

                    time_ctrl.set_loop_selection(selected_range);
                    did_interact = true;
                }
            }

            if ui.memory().is_being_dragged(right_edge_id) {
                if let (Some(time_low), Some(time_high)) = (
                    time_ranges_ui.time_from_x(pointer_pos.x - aim_radius),
                    time_ranges_ui.time_from_x(pointer_pos.x + aim_radius),
                ) {
                    let low_length = time_low - selected_range.min;
                    let high_length = time_high - selected_range.min;
                    let best_length = TimeReal::from(best_in_range_f64(
                        low_length.as_f64(),
                        high_length.as_f64(),
                    ));

                    selected_range.max = selected_range.min + best_length;

                    if selected_range.min > selected_range.max {
                        std::mem::swap(&mut selected_range.min, &mut selected_range.max);
                        ui.memory().set_dragged_id(left_edge_id);
                    }

                    time_ctrl.set_loop_selection(selected_range);
                    did_interact = true;
                }
            }

            if ui.memory().is_being_dragged(move_id) {
                (|| {
                    let min_x = time_ranges_ui.x_from_time(selected_range.min)?;
                    let max_x = time_ranges_ui.x_from_time(selected_range.max)?;

                    let min_x = min_x + ui.input().pointer.delta().x;
                    let max_x = max_x + ui.input().pointer.delta().x;

                    let min_time = time_ranges_ui.time_from_x(min_x)?;
                    let max_time = time_ranges_ui.time_from_x(max_x)?;

                    let mut new_range = TimeRangeF::new(min_time, max_time);

                    if egui::emath::almost_equal(
                        selected_range.length().as_f32(),
                        new_range.length().as_f32(),
                        1e-5,
                    ) {
                        // Avoid numerical inaccuracies: maintain length if very close
                        new_range.max = new_range.min + selected_range.length();
                    }

                    time_ctrl.set_loop_selection(new_range);
                    did_interact = true;
                    Some(())
                })();
            }
        }
    }

    if ui.memory().is_being_dragged(left_edge_id) {
        ui.output().cursor_icon = CursorIcon::ResizeWest;
    }
    if ui.memory().is_being_dragged(right_edge_id) {
        ui.output().cursor_icon = CursorIcon::ResizeEast;
    }
    if ui.memory().is_being_dragged(move_id) {
        ui.output().cursor_icon = CursorIcon::Move;
    }

    if did_interact {
        time_ctrl.looping = Looping::Selection;
    }
}

/// Human-readable description of a duration
pub fn format_duration(time_typ: TimeType, duration: TimeReal) -> String {
    match time_typ {
        TimeType::Time => Duration::from(duration).to_string(),
        TimeType::Sequence => duration.round().as_i64().to_string(), // TODO(emilk): show real part?
    }
}

fn time_marker_ui(
    time_ranges_ui: &TimeRangesUi,
    time_ctrl: &mut TimeControl,
    ui: &mut egui::Ui,
    time_area_painter: &egui::Painter,
    timeline_rect: &Rect,
    bottom_y: f32,
) {
    // full_rect: full area.
    // timeline_rect: top part with the second ticks and time marker

    let pointer_pos = ui.input().pointer.hover_pos();
    let is_pointer_in_timeline_rect =
        pointer_pos.map_or(false, |pointer_pos| timeline_rect.contains(pointer_pos));

    // ------------------------------------------------

    let time_drag_id = ui.id().with("time_drag_id");

    let mut is_hovering = false;
    let mut is_dragging = ui.memory().is_being_dragged(time_drag_id);

    if is_pointer_in_timeline_rect {
        ui.output().cursor_icon = CursorIcon::ResizeHorizontal;
    }

    let mut is_anything_being_dragged = ui.memory().is_anything_being_dragged();

    // show current time as a line:
    if let Some(time) = time_ctrl.time() {
        if let Some(x) = time_ranges_ui.x_from_time(time) {
            if let Some(pointer_pos) = pointer_pos {
                let line_rect = Rect::from_x_y_ranges(x..=x, timeline_rect.top()..=bottom_y);

                is_hovering = line_rect.distance_to_pos(pointer_pos)
                    <= ui.style().interaction.resize_grab_radius_side;

                if ui.input().pointer.any_pressed()
                    && ui.input().pointer.primary_down()
                    && is_hovering
                {
                    ui.memory().set_dragged_id(time_drag_id);
                    is_dragging = true; // avoid frame delay
                    is_anything_being_dragged = true;
                }
            }

            if is_dragging || (is_hovering && !is_anything_being_dragged) {
                ui.output().cursor_icon = CursorIcon::ResizeHorizontal;
            }

            let stroke = if is_dragging {
                ui.style().visuals.widgets.active.bg_stroke
            } else if is_hovering {
                ui.style().visuals.widgets.hovered.bg_stroke
            } else {
                ui.visuals().widgets.inactive.fg_stroke
            };
            paint_time_cursor(time_area_painter, x, timeline_rect.top()..=bottom_y, stroke);
        }
    }

    // Show preview: "click here to view time here"
    if let Some(pointer_pos) = pointer_pos {
        if !is_hovering && !is_anything_being_dragged && is_pointer_in_timeline_rect {
            time_area_painter.vline(
                pointer_pos.x,
                timeline_rect.top()..=ui.max_rect().bottom(),
                ui.visuals().widgets.noninteractive.bg_stroke,
            );
        }

        if is_dragging
            || (ui.input().pointer.primary_down()
                && is_pointer_in_timeline_rect
                && !is_anything_being_dragged)
        {
            if let Some(time) = time_ranges_ui.time_from_x(pointer_pos.x) {
                let time = time_ranges_ui.clamp_time(time);
                time_ctrl.set_time(time);
                time_ctrl.pause();
                ui.memory().set_dragged_id(time_drag_id);
            }
        }
    }
}

pub fn paint_time_cursor(
    painter: &egui::Painter,
    x: f32,
    y: RangeInclusive<f32>,
    stroke: egui::Stroke,
) {
    let y_min = *y.start();
    let y_max = *y.end();

    let stroke = egui::Stroke {
        width: 1.5 * stroke.width,
        color: stroke.color,
    };

    let w = 10.0;
    let triangle = vec![
        pos2(x - 0.5 * w, y_min), // left top
        pos2(x + 0.5 * w, y_min), // right top
        pos2(x, y_min + w),       // bottom
    ];
    painter.add(egui::Shape::convex_polygon(
        triangle,
        stroke.color,
        egui::Stroke::NONE,
    ));
    painter.vline(x, (y_min + w)..=y_max, stroke);
}

// ----------------------------------------------------------------------------

const MAX_GAP: f32 = 32.0;

/// How much space on side of the data in the default view.
const SIDE_MARGIN: f32 = MAX_GAP;

/// Sze of the gap between time segments.
fn gap_width(x_range: &RangeInclusive<f32>, segments: &[TimeRange]) -> f32 {
    let num_gaps = segments.len().saturating_sub(1);
    if num_gaps == 0 {
        // gap width doesn't matter when there are no gaps
        MAX_GAP
    } else {
        // shrink gaps if there are a lot of them
        let width = *x_range.end() - *x_range.start();
        (width / (4.0 * num_gaps as f32)).at_most(MAX_GAP)
    }
}

/// Find a nice view of everything.
fn view_everything(
    x_range: &RangeInclusive<f32>,
    timeline_axis: &TimelineAxis,
    side_margin: f32,
) -> TimeView {
    let gap_width = gap_width(x_range, &timeline_axis.ranges);
    let num_gaps = timeline_axis.ranges.len().saturating_sub(1);
    let width = *x_range.end() - *x_range.start();
    let width_sans_gaps = width - num_gaps as f32 * gap_width;

    let factor = if width_sans_gaps > 0.0 {
        width / width_sans_gaps
    } else {
        1.0 // too narrow to fit everything anyways
    };

    let min = timeline_axis.min();
    let time_spanned = timeline_axis.sum_time_lengths().as_f64() * factor as f64;

    // Leave some room on the margins:
    let time_margin = time_spanned * (side_margin / width.at_least(64.0)) as f64;
    let min = min - TimeReal::from(time_margin);
    let time_spanned = time_spanned + 2.0 * time_margin;

    TimeView { min, time_spanned }
}

struct Segment {
    /// Matches [`Self::time`] (linear transform).
    x: RangeInclusive<f32>,

    /// Matches [`Self::x`] (linear transform).
    time: TimeRangeF,

    /// does NOT match any of the above. Instead this is a tight bound.
    tight_time: TimeRange,
}

/// Represents a compressed view of time.
/// It does so by breaking up the timeline in linear segments.
///
/// Recreated each frame.
struct TimeRangesUi {
    /// The total x-range we are viewing
    x_range: RangeInclusive<f32>,

    time_view: TimeView,

    /// x ranges matched to time ranges
    segments: Vec<Segment>,

    /// x distance per time unit
    points_per_time: f32,
}

impl Default for TimeRangesUi {
    /// Safe, meaningless default
    fn default() -> Self {
        Self {
            x_range: 0.0..=1.0,
            time_view: TimeView {
                min: TimeReal::from(0),
                time_spanned: 1.0,
            },
            segments: vec![],
            points_per_time: 1.0,
        }
    }
}

impl TimeRangesUi {
    fn new(x_range: RangeInclusive<f32>, time_view: TimeView, segments: &[TimeRange]) -> Self {
        crate::profile_function!();

        //        <------- time_view ------>
        //        <-------- x_range ------->
        //        |                        |
        //    [segment] [long segment]
        //             ^ gap

        let gap_width = gap_width(&x_range, segments);
        let width = *x_range.end() - *x_range.start();
        let points_per_time = width / time_view.time_spanned as f32;
        let points_per_time = if points_per_time > 0.0 && points_per_time.is_finite() {
            points_per_time
        } else {
            1.0
        };

        let mut left = 0.0; // we will translate things left/right later
        let ranges = segments
            .iter()
            .map(|range| {
                let range_width = range.length().as_f32() * points_per_time;
                let right = left + range_width;
                let x_range = left..=right;
                left = right + gap_width;

                let tight_time = *range;

                // expand each span outwards a bit to make selection of outer data points easier.
                // Also gives zero-width segments some width!
                let expansion = gap_width / 3.0;
                let x_range = (*x_range.start() - expansion)..=(*x_range.end() + expansion);

                let range = if range.min == range.max {
                    TimeRangeF::from(*range) // don't expand zero-width segments (e.g. `TimeInt::BEGINNING`).
                } else {
                    let time_expansion = TimeReal::from(expansion / points_per_time);
                    TimeRangeF::new(range.min - time_expansion, range.max + time_expansion)
                };

                Segment {
                    x: x_range,
                    time: range,
                    tight_time,
                }
            })
            .collect();

        let mut slf = Self {
            x_range: x_range.clone(),
            time_view,
            segments: ranges,
            points_per_time,
        };

        if let Some(time_start_x) = slf.x_from_time(time_view.min) {
            // Now move things left/right to align `x_range` and `view_range`:
            let x_translate = *x_range.start() - time_start_x;
            for segment in &mut slf.segments {
                segment.x = (*segment.x.start() + x_translate)..=(*segment.x.end() + x_translate);
            }
        }

        slf
    }

    /// Clamp the time to the valid ranges.
    ///
    /// Used when user is dragging the time handle.
    fn clamp_time(&self, mut time: TimeReal) -> TimeReal {
        if let (Some(first), Some(last)) = (self.segments.first(), self.segments.last()) {
            time = time.clamp(
                TimeReal::from(first.tight_time.min),
                TimeReal::from(last.tight_time.max),
            );

            // Special: don't allow users dragging time between
            // BEGINNING (-∞ = timeless data) and some real time.
            // Otherwise we get weird times (e.g. dates in 1923).
            // Selecting times between other segments is not as problematic, as all other segments are
            // real times, so interpolating between them always produces valid times.
            // By disallowing times between BEGINNING and the first real segment,
            // we also disallow users dragging the time to be between -∞ and the
            // real beginning of their data. That further highlights the specialness of -∞.
            // Furthermore, we want users to have a smooth experience dragging the time handle anywhere else.
            if first.tight_time == TimeRange::point(TimeInt::BEGINNING) {
                if let Some(second) = self.segments.get(1) {
                    if TimeInt::BEGINNING < time && time < second.tight_time.min {
                        time = TimeReal::from(second.tight_time.min);
                    }
                }
            }
        }
        time
    }

    /// Make sure the time is not between segments.
    ///
    /// This is so that the playback doesn't get stuck between segments.
    fn snap_time_to_segments(&self, value: TimeReal) -> TimeReal {
        for segment in &self.segments {
            if value < segment.time.min {
                return segment.time.min;
            } else if value <= segment.time.max {
                return value;
            }
        }
        value
    }

    // Make sure playback time doesn't get stuck between non-continuos regions:
    fn snap_time_control(&self, ctx: &mut ViewerContext<'_>) {
        if !ctx.rec_cfg.time_ctrl.is_playing() {
            return;
        }

        // Make sure time doesn't get stuck between non-continuos regions:
        if let Some(time) = ctx.rec_cfg.time_ctrl.time() {
            let time = self.snap_time_to_segments(time);
            ctx.rec_cfg.time_ctrl.set_time(time);
        } else if let Some(selection) = ctx.rec_cfg.time_ctrl.loop_selection() {
            let snapped_min = self.snap_time_to_segments(selection.min);
            let snapped_max = self.snap_time_to_segments(selection.max);

            let min_was_good = selection.min == snapped_min;
            let max_was_good = selection.max == snapped_max;

            if min_was_good || max_was_good {
                return;
            }

            // Keeping max works better when looping
            ctx.rec_cfg.time_ctrl.set_loop_selection(TimeRangeF::new(
                snapped_max - selection.length(),
                snapped_max,
            ));
        }
    }

    fn x_from_time(&self, needle_time: TimeReal) -> Option<f32> {
        let first_segment = self.segments.first()?;
        let mut last_x = *first_segment.x.start();
        let mut last_time = first_segment.time.min;

        if needle_time < last_time {
            // extrapolate:
            return Some(last_x - self.points_per_time * (last_time - needle_time).as_f32());
        }

        for segment in &self.segments {
            if needle_time < segment.time.min {
                let t = TimeRangeF::new(last_time, segment.time.min).inverse_lerp(needle_time);
                return Some(lerp(last_x..=*segment.x.start(), t));
            } else if needle_time <= segment.time.max {
                let t = segment.time.inverse_lerp(needle_time);
                return Some(lerp(segment.x.clone(), t));
            } else {
                last_x = *segment.x.end();
                last_time = segment.time.max;
            }
        }

        // extrapolate:
        Some(last_x + self.points_per_time * (needle_time - last_time).as_f32())
    }

    fn time_from_x(&self, needle_x: f32) -> Option<TimeReal> {
        let first_segment = self.segments.first()?;
        let mut last_x = *first_segment.x.start();
        let mut last_time = first_segment.time.min;

        if needle_x < last_x {
            // extrapolate:
            return Some(last_time + TimeReal::from((needle_x - last_x) / self.points_per_time));
        }

        for segment in &self.segments {
            if needle_x < *segment.x.start() {
                let t = remap(needle_x, last_x..=*segment.x.start(), 0.0..=1.0);
                return Some(TimeRangeF::new(last_time, segment.time.min).lerp(t));
            } else if needle_x <= *segment.x.end() {
                let t = remap(needle_x, segment.x.clone(), 0.0..=1.0);
                return Some(segment.time.lerp(t));
            } else {
                last_x = *segment.x.end();
                last_time = segment.time.max;
            }
        }

        // extrapolate:
        Some(last_time + TimeReal::from((needle_x - last_x) / self.points_per_time))
    }

    /// Pan the view, returning the new view.
    fn pan(&self, delta_x: f32) -> Option<TimeView> {
        Some(TimeView {
            min: self.time_from_x(*self.x_range.start() + delta_x)?,
            time_spanned: self.time_view.time_spanned,
        })
    }

    /// Zoom the view around the given x, returning the new view.
    fn zoom_at(&self, x: f32, zoom_factor: f32) -> Option<TimeView> {
        let mut min_x = *self.x_range.start();
        let max_x = *self.x_range.end();
        let t = remap(x, min_x..=max_x, 0.0..=1.0);

        let width = max_x - min_x;

        let new_width = width / zoom_factor;
        let width_delta = new_width - width;

        min_x -= t * width_delta;

        Some(TimeView {
            min: self.time_from_x(min_x)?,
            time_spanned: self.time_view.time_spanned / zoom_factor as f64,
        })
    }

    /// How many egui points for each time unit?
    fn points_per_time(&self) -> Option<f32> {
        for segment in &self.segments {
            let dx = *segment.x.end() - *segment.x.start();
            let dt = segment.time.length().as_f32();
            if dx > 0.0 && dt > 0.0 {
                return Some(dx / dt);
            }
        }
        None
    }
}

fn paint_time_range_ticks(
    ui: &mut egui::Ui,
    time_area_painter: &egui::Painter,
    rect: &Rect,
    time_type: TimeType,
    time_range: &TimeRangeF,
) {
    let font_id = egui::TextStyle::Body.resolve(ui.style());

    let shapes = match time_type {
        TimeType::Time => {
            fn next_grid_tick_magnitude_ns(spacing_ns: i64) -> i64 {
                if spacing_ns <= 1_000_000_000 {
                    spacing_ns * 10 // up to 10 second ticks
                } else if spacing_ns == 10_000_000_000 {
                    spacing_ns * 6 // to the whole minute
                } else if spacing_ns == 60_000_000_000 {
                    spacing_ns * 10 // to ten minutes
                } else if spacing_ns == 600_000_000_000 {
                    spacing_ns * 6 // to an hour
                } else if spacing_ns < 24 * 60 * 60 * 1_000_000_000 {
                    spacing_ns * 24 // to a day
                } else {
                    spacing_ns * 10 // multiple of ten days
                }
            }

            fn grid_text_from_ns(ns: i64) -> String {
                let relative_ns = ns % 1_000_000_000;
                if relative_ns == 0 {
                    let time = Time::from_ns_since_epoch(ns);
                    if time.is_abolute_date() {
                        time.format_time("%H:%M:%S")
                    } else {
                        re_log_types::Duration::from_nanos(ns).to_string()
                    }
                } else {
                    // show relative to whole second:
                    let ms = relative_ns as f64 * 1e-6;
                    if relative_ns % 1_000_000 == 0 {
                        format!("{:+.0} ms", ms)
                    } else if relative_ns % 100_000 == 0 {
                        format!("{:+.1} ms", ms)
                    } else if relative_ns % 10_000 == 0 {
                        format!("{:+.2} ms", ms)
                    } else {
                        format!("{:+.3} ms", ms)
                    }
                }
            }

            paint_ticks(
                &ui.fonts(),
                ui.visuals().dark_mode,
                &font_id,
                rect,
                &ui.clip_rect(),
                time_range, // ns
                1_000,
                next_grid_tick_magnitude_ns,
                grid_text_from_ns,
            )
        }
        TimeType::Sequence => {
            fn next_power_of_10(i: i64) -> i64 {
                i * 10
            }
            paint_ticks(
                &ui.fonts(),
                ui.visuals().dark_mode,
                &font_id,
                rect,
                &ui.clip_rect(),
                time_range,
                1,
                next_power_of_10,
                |seq| format!("#{seq}"),
            )
        }
    };

    time_area_painter.extend(shapes);
}

#[allow(clippy::too_many_arguments)]
fn paint_ticks(
    fonts: &egui::epaint::Fonts,
    dark_mode: bool,
    font_id: &egui::FontId,
    canvas: &Rect,
    clip_rect: &Rect,
    time_range: &TimeRangeF,
    min_grid_spacing_time: i64,
    next_time_step: fn(i64) -> i64,
    format_tick: fn(i64) -> String,
) -> Vec<egui::Shape> {
    crate::profile_function!();

    let color_from_alpha = |alpha: f32| -> Color32 {
        if dark_mode {
            Rgba::from_white_alpha(alpha * alpha).into()
        } else {
            Rgba::from_black_alpha(alpha).into()
        }
    };

    let x_from_time = |time: i64| -> f32 {
        let t = (TimeReal::from(time) - time_range.min).as_f32()
            / (time_range.max - time_range.min).as_f32();
        lerp(canvas.x_range(), t)
    };

    let visible_rect = clip_rect.intersect(*canvas);
    let mut shapes = vec![];

    if !visible_rect.is_positive() {
        return shapes;
    }

    let width_time = (time_range.max - time_range.min).as_f32();
    let points_per_time = canvas.width() / width_time;
    let minimum_small_line_spacing = 4.0;
    let expected_text_width = 60.0;

    let line_color_from_spacing = |spacing_time: i64| -> Color32 {
        let next_tick_magnitude = next_time_step(spacing_time) / spacing_time; // usually 10, but could be 6 or 24 for time
        let alpha = remap_clamp(
            spacing_time as f32 * points_per_time,
            minimum_small_line_spacing..=(next_tick_magnitude as f32 * minimum_small_line_spacing),
            0.0..=0.6,
        );
        color_from_alpha(alpha)
    };

    let text_color_from_spacing = |spacing_time: i64| -> Color32 {
        let alpha = remap_clamp(
            spacing_time as f32 * points_per_time,
            expected_text_width..=(3.0 * expected_text_width),
            0.0..=1.0,
        );
        color_from_alpha(alpha)
    };

    let max_small_lines = canvas.width() / minimum_small_line_spacing;
    let mut small_spacing_time = min_grid_spacing_time;
    while width_time / (small_spacing_time as f32) > max_small_lines {
        small_spacing_time = next_time_step(small_spacing_time);
    }
    let medium_spacing_time = next_time_step(small_spacing_time);
    let big_spacing_time = next_time_step(medium_spacing_time);

    // We fade in lines as we zoom in:
    let big_line_color = line_color_from_spacing(big_spacing_time);
    let medium_line_color = line_color_from_spacing(medium_spacing_time);
    let small_line_color = line_color_from_spacing(small_spacing_time);

    let big_text_color = text_color_from_spacing(big_spacing_time);
    let medium_text_color = text_color_from_spacing(medium_spacing_time);
    let small_text_color = text_color_from_spacing(small_spacing_time);

    let mut current_time =
        time_range.min.floor().as_i64() / small_spacing_time * small_spacing_time; // TODO(emilk): start at visible_rect.left()
    while current_time <= time_range.max.ceil().as_i64() {
        let line_x = x_from_time(current_time);

        if visible_rect.min.x <= line_x && line_x <= visible_rect.max.x {
            let medium_line = current_time % medium_spacing_time == 0;
            let big_line = current_time % big_spacing_time == 0;

            let (line_color, text_color) = if big_line {
                (big_line_color, big_text_color)
            } else if medium_line {
                (medium_line_color, medium_text_color)
            } else {
                (small_line_color, small_text_color)
            };

            let top = if current_time % 1_000_000_000 == 0 {
                // TODO(emilk): for sequences (non-nanoseconds)
                canvas.top() // full second
            } else {
                lerp(canvas.y_range(), 0.75)
            };

            shapes.push(egui::Shape::line_segment(
                [pos2(line_x, top), pos2(line_x, canvas.max.y)],
                Stroke::new(1.0, line_color),
            ));

            if text_color != Color32::TRANSPARENT {
                let text = format_tick(current_time);
                let text_x = line_x + 4.0;

                // Text at top:
                shapes.push(egui::Shape::text(
                    fonts,
                    pos2(text_x, canvas.min.y),
                    Align2::LEFT_TOP,
                    &text,
                    font_id.clone(),
                    text_color,
                ));
            }
        }

        current_time += small_spacing_time;
    }

    shapes
}

// ----------------------------------------------------------------------------

/// Positions circles on a horizontal line with some vertical scattering to avoid overlap.
struct BallScatterer {
    recent: [Pos2; Self::MEMORY_SIZE],
    cursor: usize,
}

impl Default for BallScatterer {
    fn default() -> Self {
        Self {
            recent: [Pos2::new(f32::INFINITY, f32::INFINITY); Self::MEMORY_SIZE],
            cursor: 0,
        }
    }
}

impl BallScatterer {
    const MEMORY_SIZE: usize = 8;

    pub fn add(&mut self, x: f32, r: f32, (min_y, max_y): (f32, f32)) -> Pos2 {
        let min_y = min_y + r; // some padding
        let max_y = max_y - r; // some padding

        let r2 = r * r * 3.0; // allow some overlap

        let center_y = 0.5 * (min_y + max_y);

        let y = if max_y <= min_y {
            center_y
        } else {
            let mut best_free_y = f32::INFINITY;
            let mut best_colliding_y = center_y;
            let mut best_colliding_d2 = 0.0;

            let step_size = 2.0; // unit: points

            for y_offset in 0..=((max_y - min_y) / step_size).round() as i32 {
                let y = min_y + step_size * y_offset as f32;
                let d2 = self.closest_dist_sq(&pos2(x, y));
                let intersects = d2 < r2;
                if intersects {
                    // pick least colliding
                    if d2 > best_colliding_d2 {
                        best_colliding_y = y;
                        best_colliding_d2 = d2;
                    }
                } else {
                    // pick closest to center
                    if (y - center_y).abs() < (best_free_y - center_y).abs() {
                        best_free_y = y;
                    }
                }
            }

            if best_free_y.is_finite() {
                best_free_y
            } else {
                best_colliding_y
            }
        };

        let pos = pos2(x, y);
        self.recent[self.cursor] = pos;
        self.cursor = (self.cursor + 1) % Self::MEMORY_SIZE;
        pos
    }

    fn closest_dist_sq(&self, pos: &Pos2) -> f32 {
        let mut d2 = f32::INFINITY;
        for recent in &self.recent {
            d2 = d2.min(recent.distance_sq(*pos));
        }
        d2
    }
}

// ----------------------------------------------------------------------------

/// fade left/right sides of time-area, because it looks nice:
fn fade_sides(ui: &mut egui::Ui, time_area: Rect) {
    let fade_width = SIDE_MARGIN - 1.0;

    let base_rect = time_area.expand(0.5); // expand slightly to cover feathering.

    let window_fill = ui.visuals().window_fill();
    let mut left_rect = base_rect;

    left_rect.set_right(left_rect.left() + fade_width);
    ui.painter()
        .add(fade_mesh(left_rect, [window_fill, Color32::TRANSPARENT]));

    let mut right_rect = base_rect;
    right_rect.set_left(right_rect.right() - fade_width);
    ui.painter()
        .add(fade_mesh(right_rect, [Color32::TRANSPARENT, window_fill]));
}

fn fade_mesh(rect: Rect, [left_color, right_color]: [Color32; 2]) -> egui::Mesh {
    use egui::epaint::Vertex;
    let mut mesh = egui::Mesh::default();

    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(2, 1, 3);

    mesh.vertices.push(Vertex {
        pos: rect.left_top(),
        uv: egui::epaint::WHITE_UV,
        color: left_color,
    });
    mesh.vertices.push(Vertex {
        pos: rect.right_top(),
        uv: egui::epaint::WHITE_UV,
        color: right_color,
    });
    mesh.vertices.push(Vertex {
        pos: rect.left_bottom(),
        uv: egui::epaint::WHITE_UV,
        color: left_color,
    });
    mesh.vertices.push(Vertex {
        pos: rect.right_bottom(),
        uv: egui::epaint::WHITE_UV,
        color: right_color,
    });

    mesh
}

// ----------------------------------------------------------------------------

#[test]
fn test_time_ranges_ui() {
    let time_range_ui = TimeRangesUi::new(
        100.0..=1000.0,
        TimeView {
            min: TimeReal::from(0.5),
            time_spanned: 14.2,
        },
        &[
            TimeRange::new(TimeInt::from(0), TimeInt::from(0)),
            TimeRange::new(TimeInt::from(1), TimeInt::from(5)),
            TimeRange::new(TimeInt::from(10), TimeInt::from(100)),
        ],
    );

    // Sanity check round-tripping:
    for segment in &time_range_ui.segments {
        let pixel_precision = 0.5;

        assert_eq!(
            time_range_ui.time_from_x(*segment.x.start()).unwrap(),
            segment.time.min
        );
        assert_eq!(
            time_range_ui.time_from_x(*segment.x.end()).unwrap(),
            segment.time.max
        );

        if segment.time.is_empty() {
            let x = time_range_ui.x_from_time(segment.time.min).unwrap();
            let mid_x = lerp(segment.x.clone(), 0.5);
            assert!((mid_x - x).abs() < pixel_precision);
        } else {
            let min_x = time_range_ui.x_from_time(segment.time.min).unwrap();
            assert!((min_x - *segment.x.start()).abs() < pixel_precision);

            let max_x = time_range_ui.x_from_time(segment.time.max).unwrap();
            assert!((max_x - *segment.x.end()).abs() < pixel_precision);
        }
    }
}
