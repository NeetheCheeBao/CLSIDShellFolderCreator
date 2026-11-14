use std::path::{Path, PathBuf};
use std::time::Duration;

use eframe::egui::containers::scroll_area::ScrollBarVisibility;
use eframe::egui::{self, Color32, FontId, RichText, Stroke};

use crate::data::{self, Entry};

const TOOLBAR: Color32 = Color32::from_rgb(0xF3, 0xF3, 0xF3);
const HOVER: Color32 = Color32::from_rgb(0xE5, 0xE5, 0xE5);
const PRESS: Color32 = Color32::from_rgb(0xDC, 0xDC, 0xDC);
const TEXT: Color32 = Color32::from_rgb(0x33, 0x33, 0x33);
const DIM: Color32 = Color32::from_rgb(0x99, 0x99, 0x99);
const LOG_BG: Color32 = Color32::from_rgb(0xF9, 0xF9, 0xF9);
const BORDER: Color32 = Color32::from_rgb(0xE0, 0xE0, 0xE0);
const GREEN: Color32 = Color32::from_rgb(0x28, 0xA7, 0x45);
const RED: Color32 = Color32::from_rgb(0xDC, 0x35, 0x45);
const YELLOW: Color32 = Color32::from_rgb(0xD3, 0x9E, 0x00);
const BTN_BORDER: Color32 = Color32::from_rgb(0xC8, 0xC8, 0xC8);
const BTN_HOVER: Color32 = Color32::from_rgb(0xE5, 0xF1, 0xFB);
const BTN_PRESS: Color32 = Color32::from_rgb(0xCC, 0xE4, 0xF7);
const BTN_BLUE: Color32 = Color32::from_rgb(0x00, 0x78, 0xD7);

const URL: &str = "https://github.com/NeetheCheeBao/CLSIDShellFolderCreator";
const BATCH_CHUNK: usize = 8;
const MAX_LOG_LINES: usize = 200;
fn path_edit_id() -> egui::Id {
    egui::Id::new("path_edit")
}

struct LogLine {
    parts: Vec<(String, Color32)>,
}

pub struct App {
    entries: Vec<Entry>,
    path: String,
    log: Vec<LogLine>,
    log_counter: u32,
    log_offset: egui::Vec2,
    log_stick: bool,
    editing_path: bool,
    tip: Option<crate::tip::Tip>,
    is_creating: bool,
    batch_pos: Option<usize>,
    unlock_at: Option<f64>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        install_style(&cc.egui_ctx);
        install_fonts(&cc.egui_ctx);
        App {
            entries: data::load_entries(),
            path: std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("CreatorOutput")
                .display()
                .to_string(),
            log: Vec::new(),
            log_counter: 1,
            log_offset: egui::Vec2::ZERO,
            log_stick: true,
            editing_path: false,
            tip: crate::tip::Tip::new(),
            is_creating: false,
            batch_pos: None,
            unlock_at: None,
        }
    }

    fn push_line(&mut self, parts: Vec<(String, Color32)>) {
        self.log.push(LogLine { parts });
        self.log_stick = true;
        if self.log.len() > MAX_LOG_LINES {
            let cut = self.log.len() - MAX_LOG_LINES;
            self.log.drain(..cut);
        }
    }

    fn log(&mut self, msg: &str, color: Color32) {
        let n = self.log_counter;
        self.log_counter += 1;
        self.push_line(vec![(format!("[{n}] {msg}"), color)]);
    }

    fn clear_log(&mut self) {
        if self.is_creating {
            return;
        }
        self.log.clear();
    }

    fn create_folder(&mut self, idx: usize) {
        let clsid = self.entries[idx].clsid.clone();
        let name = self.entries[idx].name.clone();
        let dir = PathBuf::from(&self.path);
        if !dir.exists() {
            let _ = std::fs::create_dir_all(&dir);
        }
        let target = dir.join(&clsid);
        let shown = target.display().to_string();
        if target.exists() {
            self.log(&format!("{name}创建失败，CLSID为{clsid}，已存在于路径{shown}"), RED);
            return;
        }
        // 目录名就是 ".{GUID}"，Shell 会把它当成命名空间入口
        match std::fs::create_dir(&target) {
            Ok(_) => self.log(&format!("{name}创建成功，CLSID为{clsid}，路径为{shown}"), GREEN),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                self.log(&format!("{name}创建失败，CLSID为{clsid}，已存在于路径{shown}"), RED)
            }
            Err(e) => self.log(&format!("{name}创建失败，CLSID为{clsid}，原因：{e}"), RED),
        }
    }

    fn create_all(&mut self, ctx: &egui::Context) {
        if self.is_creating || self.entries.is_empty() {
            return;
        }
        self.is_creating = true;
        self.batch_pos = Some(0);
        ctx.request_repaint();
    }

    // 分批推进，每帧处理若干条，界面保持可重绘
    fn batch_step(&mut self, ctx: &egui::Context) {
        if let Some(pos) = self.batch_pos {
            let end = (pos + BATCH_CHUNK).min(self.entries.len());
            for i in pos..end {
                self.create_folder(i);
            }
            if end >= self.entries.len() {
                self.batch_pos = None;
                self.unlock_at = Some(ctx.input(|i| i.time) + 1.0);
            } else {
                self.batch_pos = Some(end);
                ctx.request_repaint();
            }
        }
        if let Some(t) = self.unlock_at {
            if ctx.input(|i| i.time) >= t {
                self.unlock_at = None;
                self.is_creating = false;
            } else {
                ctx.request_repaint_after(Duration::from_millis(120));
            }
        }
    }

    fn copy_clsid(&mut self, ctx: &egui::Context, idx: usize) {
        let clsid = self.entries[idx].clsid.clone();
        ctx.copy_text(clsid.clone());
        let n = self.log_counter;
        self.log_counter += 1;
        self.push_line(vec![
            (format!("[{n}][拷贝成功]"), GREEN),
            (format!("CLSID：“{clsid}”"), YELLOW),
            ("已拷贝到剪贴板".to_string(), GREEN),
        ]);
    }

    fn browse_dir(&mut self) {
        if self.is_creating {
            return;
        }
        if let Some(dir) = rfd::FileDialog::new().set_directory(&self.path).pick_folder() {
            self.path = dir.display().to_string();
        }
    }

    fn keys(&mut self, ctx: &egui::Context) {
        let (f1, f2, f3) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::F1),
                i.key_pressed(egui::Key::F2),
                i.key_pressed(egui::Key::F3),
            )
        });
        if f1 {
            self.create_all(ctx);
        }
        if f2 {
            self.clear_log();
        }
        if f3 {
            ctx.open_url(egui::OpenUrl::new_tab(URL));
        }
    }

    fn toolbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar")
            .exact_height(45.0)
            .frame(egui::Frame::NONE.fill(TOOLBAR))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(4.0);
                    if styled_button(ui, "创建全部 (F1)", !self.is_creating, 30.0).clicked() {
                        self.create_all(ctx);
                    }
                    if styled_button(ui, "清空终端 (F2)", !self.is_creating, 30.0).clicked() {
                        self.clear_log();
                    }
                    if styled_button(ui, "项目地址 (F3)", true, 30.0).clicked() {
                        ctx.open_url(egui::OpenUrl::new_tab(URL));
                    }
                });
            });
    }

    fn path_row(&mut self, ui: &mut egui::Ui) {
        let row_h = 26.0;
        let full_w = ui.available_width();
        ui.allocate_ui_with_layout(
            egui::vec2(full_w, row_h),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.label(RichText::new("输出路径:").color(TEXT).size(14.0));
                // 右对齐布局：先放按钮，输入框吃掉剩余宽度
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if styled_button(ui, "浏览...", !self.is_creating, row_h).clicked() {
                        self.browse_dir();
                    }
                    ui.add_space(6.0);
                    let edit_w = ui.available_width();
                    let inner_w = edit_w - 10.0;
                    let inner_h = row_h - 8.0;
                    egui::Frame::NONE
                        .fill(LOG_BG)
                        .stroke(Stroke::new(1.0, BORDER))
                        .inner_margin(egui::Margin::symmetric(4, 3))
                        .show(ui, |ui| {
                            if self.editing_path {
                                let resp = ui.add(
                                    egui::TextEdit::singleline(&mut self.path)
                                        .id(path_edit_id())
                                        .frame(false)
                                        .margin(egui::Margin::ZERO)
                                        .desired_width(inner_w)
                                        .min_size(egui::vec2(inner_w, inner_h))
                                        .text_color(TEXT)
                                        .vertical_align(egui::Align::Center)
                                        .cursor_at_end(true),
                                );
                                if resp.lost_focus() {
                                    self.editing_path = false;
                                } else if !resp.has_focus() {
                                    resp.request_focus();
                                }
                            } else {
                                // 未编辑时显示路径尾部，超出部分从左侧裁掉
                                let (rect, resp) = ui.allocate_exact_size(
                                    egui::vec2(inner_w, inner_h),
                                    egui::Sense::click(),
                                );
                                let galley = ui.painter().layout_no_wrap(
                                    self.path.clone(),
                                    FontId::proportional(14.0),
                                    TEXT,
                                );
                                let over = (galley.size().x - rect.width()).max(0.0);
                                ui.painter().with_clip_rect(rect).galley(
                                    rect.left_top() - egui::vec2(over, 0.0),
                                    galley,
                                    TEXT,
                                );
                                if resp.clicked() {
                                    self.editing_path = true;
                                    ui.memory_mut(|m| m.request_focus(path_edit_id()));
                                }
                            }
                        });
                });
            },
        );
    }

    fn log_box(&mut self, ui: &mut egui::Ui) {
        let bar = 14.0;
        let gap = 2.0;
        let box_h = 122.0;
        let avail = ui.available_rect_before_wrap();
        let box_w = (avail.width() - bar - gap).max(120.0);

        if ui.rect_contains_pointer(avail) && ui.input(|i| i.raw_scroll_delta.y.abs() > 0.0) {
            self.log_stick = false;
        }

        let out = egui::Frame::NONE
            .fill(LOG_BG)
            .stroke(Stroke::new(1.0, BORDER))
            .inner_margin(egui::Margin::same(4))
            .show(ui, |ui| {
                ui.set_width(box_w - 10.0);
                ui.set_height(box_h - 10.0);
                egui::ScrollArea::both()
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .vertical_scroll_offset(self.log_offset.y)
                    .horizontal_scroll_offset(self.log_offset.x)
                    .auto_shrink([false, false])
                    .stick_to_bottom(self.log_stick)
                    .scroll_source(egui::containers::scroll_area::ScrollSource {
                        drag: false,
                        ..Default::default()
                    })
                    .show(ui, |ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        ui.spacing_mut().item_spacing.y = 2.0;
                        for line in &self.log {
                            // 整行合成一个 galley，框选范围才和文字一致
                            let mut job = egui::text::LayoutJob::default();
                            for (text, color) in &line.parts {
                                job.append(
                                    text,
                                    0.0,
                                    egui::TextFormat {
                                        font_id: FontId::monospace(13.0),
                                        color: *color,
                                        ..Default::default()
                                    },
                                );
                            }
                            ui.add(egui::Label::new(job).selectable(true));
                        }
                    })
            });

        // 以实际渲染出的边框为准摆放滚动条
        let frame = out.response.rect;
        let scroll = out.inner;
        self.log_offset = scroll.state.offset;
        let view = scroll.inner_rect.size();

        // 先把横条要占的高度留出来，后面内容不会压到它
        ui.add_space(bar + gap);

        let v_rect = egui::Rect::from_min_size(
            egui::pos2(frame.right() + gap, frame.top()),
            egui::vec2(bar, frame.height()),
        );
        let h_rect = egui::Rect::from_min_size(
            egui::pos2(frame.left(), frame.bottom() + gap),
            egui::vec2(frame.width(), bar),
        );
        self.scrollbar(ui, v_rect, true, scroll.content_size.y, view.y, 18.0);
        self.scrollbar(ui, h_rect, false, scroll.content_size.x, view.x, 40.0);
    }

    fn scrollbar(&mut self, ui: &egui::Ui, rect: egui::Rect, vertical: bool, content: f32, view: f32, step: f32) {
        let arrow = 14.0_f32.min(if vertical { rect.height() } else { rect.width() } / 3.0);
        let (start_rect, end_rect, track_rect) = if vertical {
            (
                egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), arrow)),
                egui::Rect::from_min_size(
                    egui::pos2(rect.left(), rect.bottom() - arrow),
                    egui::vec2(rect.width(), arrow),
                ),
                egui::Rect::from_min_max(
                    egui::pos2(rect.left(), rect.top() + arrow),
                    egui::pos2(rect.right(), rect.bottom() - arrow),
                ),
            )
        } else {
            (
                egui::Rect::from_min_size(rect.min, egui::vec2(arrow, rect.height())),
                egui::Rect::from_min_size(
                    egui::pos2(rect.right() - arrow, rect.top()),
                    egui::vec2(arrow, rect.height()),
                ),
                egui::Rect::from_min_max(
                    egui::pos2(rect.left() + arrow, rect.top()),
                    egui::pos2(rect.right() - arrow, rect.bottom()),
                ),
            )
        };

        let max_scroll = (content - view).max(0.0);
        let mut offset = if vertical { self.log_offset.y } else { self.log_offset.x };
        ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(0xF0, 0xF0, 0xF0));

        // 两端箭头按钮
        for (r, forward) in [(start_rect, false), (end_rect, true)] {
            let resp = ui.interact(
                r,
                ui.id().with(("sb_arrow", vertical, forward)),
                egui::Sense::click(),
            );
            if resp.hovered() {
                ui.painter().rect_filled(r, 0.0, Color32::from_rgb(0xE2, 0xE2, 0xE2));
            }
            let c = r.center();
            let s = 3.5;
            let (a, b, d) = match (vertical, forward) {
                (true, false) => (
                    egui::pos2(c.x, c.y - s),
                    egui::pos2(c.x - s, c.y + s),
                    egui::pos2(c.x + s, c.y + s),
                ),
                (true, true) => (
                    egui::pos2(c.x, c.y + s),
                    egui::pos2(c.x - s, c.y - s),
                    egui::pos2(c.x + s, c.y - s),
                ),
                (false, false) => (
                    egui::pos2(c.x - s, c.y),
                    egui::pos2(c.x + s, c.y - s),
                    egui::pos2(c.x + s, c.y + s),
                ),
                (false, true) => (
                    egui::pos2(c.x + s, c.y),
                    egui::pos2(c.x - s, c.y - s),
                    egui::pos2(c.x - s, c.y + s),
                ),
            };
            let tint = if resp.hovered() {
                Color32::from_rgb(0x33, 0x33, 0x33)
            } else {
                Color32::from_rgb(0x60, 0x60, 0x60)
            };
            ui.painter().add(egui::Shape::convex_polygon(
                vec![a, b, d],
                tint,
                Stroke::NONE,
            ));
            if resp.clicked() && max_scroll > 0.0 {
                self.log_stick = false;
                let delta = if forward { step } else { -step };
                offset = (offset + delta).clamp(0.0, max_scroll);
            }
        }

        let track = if vertical { track_rect.height() } else { track_rect.width() };
        if max_scroll <= 0.0 || track <= 8.0 {
            return;
        }
        let thumb_len = (track * view / content).clamp(16.0, track);
        let travel = track - thumb_len;
        let pos = offset / max_scroll * travel;

        let thumb = if vertical {
            egui::Rect::from_min_size(
                egui::pos2(track_rect.left(), track_rect.top() + pos),
                egui::vec2(track_rect.width(), thumb_len),
            )
        } else {
            egui::Rect::from_min_size(
                egui::pos2(track_rect.left() + pos, track_rect.top()),
                egui::vec2(thumb_len, track_rect.height()),
            )
        };

        let resp = ui.interact(
            track_rect,
            ui.id().with(("scrollbar", vertical)),
            egui::Sense::click_and_drag(),
        );
        let color = if resp.hovered() || resp.dragged() {
            Color32::from_rgb(0xA6, 0xA6, 0xA6)
        } else {
            Color32::from_rgb(0xC1, 0xC1, 0xC1)
        };
        ui.painter().rect_filled(thumb, 0.0, color);

        let mut next = offset;
        if resp.dragged() || resp.clicked() {
            self.log_stick = false;
        }
        if resp.dragged() {
            let d = if vertical { resp.drag_delta().y } else { resp.drag_delta().x };
            if travel > 0.0 {
                next = ((pos + d) / travel * max_scroll).clamp(0.0, max_scroll);
            }
        } else if resp.clicked() {
            if let Some(p) = resp.interact_pointer_pos() {
                let at = if vertical { p.y - track_rect.top() } else { p.x - track_rect.left() };
                if at < pos {
                    next = (offset - view).max(0.0);
                } else if at > pos + thumb_len {
                    next = (offset + view).min(max_scroll);
                }
            }
        }
        if vertical {
            self.log_offset.y = next;
        } else {
            self.log_offset.x = next;
        }
    }

    fn list(&mut self, ui: &mut egui::Ui) {
        let row_h = 46.0;
        let btn_h = 34.0;
        let index_w = 32.0;
        let gap = 5.0;
        let btn_w = 228.0;
        let mut clicked = None;
        let mut copied = None;
        let mut tip: Option<String> = None;
        let entries = &self.entries;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, row_h, entries.len(), |ui, range| {
                ui.spacing_mut().item_spacing.y = 12.0;
                for i in range {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        let total = index_w + gap + btn_w;
                        let pad = ((ui.available_width() - total) / 2.0).max(0.0);
                        ui.add_space(pad);
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(index_w, btn_h), egui::Sense::hover());
                        ui.painter().text(
                            rect.right_center(),
                            egui::Align2::RIGHT_CENTER,
                            format!("({})", i + 1),
                            FontId::proportional(14.0),
                            DIM,
                        );
                        ui.add_space(gap);
                        let resp = ui.add_sized(
                            [btn_w, btn_h],
                            egui::Button::new(RichText::new(&entries[i].name).color(TEXT)),
                        );
                        if resp.clicked() {
                            clicked = Some(i);
                        }
                        if resp.secondary_clicked() {
                            copied = Some(i);
                        }
                        if resp.hovered() {
                            tip = Some(entries[i].clsid.clone());
                        }
                    });
                }
            });

        match (&self.tip, tip) {
            (Some(w), Some(text)) => w.show(&text),
            (Some(w), None) => w.hide(),
            _ => {}
        }

        if let Some(i) = copied {
            self.copy_clsid(ui.ctx(), i);
        } else if let Some(i) = clicked {
            if !self.is_creating {
                self.create_folder(i);
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.keys(ctx);
        self.batch_step(ctx);
        self.toolbar(ctx);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(Color32::WHITE)
                    .inner_margin(egui::Margin { left: 15, right: 15, top: 0, bottom: 0 }),
            )
            .show(ctx, |ui| {
                ui.add_space(10.0);
                self.path_row(ui);
                ui.add_space(5.0);
                self.log_box(ui);
                ui.add_space(8.0);
                self.list(ui);
            });
    }
}

// 白底 + 描边，悬停变浅蓝
fn styled_button(ui: &mut egui::Ui, text: &str, enabled: bool, height: f32) -> egui::Response {
    let mut w = ui.style().visuals.widgets.clone();
    for v in [&mut w.inactive, &mut w.hovered, &mut w.active] {
        v.corner_radius = egui::CornerRadius::same(3);
    }
    w.inactive.weak_bg_fill = Color32::WHITE;
    w.inactive.bg_stroke = Stroke::new(1.0, BTN_BORDER);
    w.hovered.weak_bg_fill = BTN_HOVER;
    w.hovered.bg_stroke = Stroke::new(1.0, BTN_BLUE);
    w.active.weak_bg_fill = BTN_PRESS;
    w.active.bg_stroke = Stroke::new(1.0, BTN_BLUE);
    ui.scope(|ui| {
        ui.visuals_mut().widgets = w;
        let pad_y = ((height - 14.0) * 0.5).max(2.0);
        ui.spacing_mut().button_padding = egui::vec2(12.0, pad_y);
        ui.add_enabled(enabled, egui::Button::new(text).min_size(egui::vec2(0.0, height)))
    })
    .inner
}

fn install_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals.panel_fill = Color32::WHITE;
    style.visuals.window_fill = Color32::WHITE;
    style.visuals.extreme_bg_color = LOG_BG;
    style.visuals.override_text_color = Some(TEXT);
    style.visuals.selection.bg_fill = Color32::from_rgb(0xCC, 0xE4, 0xFF);

    let w = &mut style.visuals.widgets;
    for widget in [&mut w.inactive, &mut w.hovered, &mut w.active, &mut w.noninteractive] {
        widget.corner_radius = egui::CornerRadius::ZERO;
        widget.bg_stroke = Stroke::NONE;
    }
    w.inactive.weak_bg_fill = TOOLBAR;
    w.inactive.fg_stroke = Stroke::new(1.0, TEXT);
    w.hovered.weak_bg_fill = HOVER;
    w.hovered.fg_stroke = Stroke::new(1.0, TEXT);
    w.active.weak_bg_fill = PRESS;
    w.active.fg_stroke = Stroke::new(1.0, TEXT);

    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.spacing.item_spacing = egui::vec2(6.0, 6.0);
    style.text_styles = [
        (egui::TextStyle::Body, FontId::proportional(14.0)),
        (egui::TextStyle::Button, FontId::proportional(14.0)),
        (egui::TextStyle::Monospace, FontId::monospace(13.0)),
        (egui::TextStyle::Small, FontId::proportional(12.0)),
    ]
    .into();
    ctx.set_style(style);
}

// 不内置字体，全部用 Windows 自带字体
fn install_fonts(ctx: &egui::Context) {
    let root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    let dir = PathBuf::from(root).join("Fonts");
    let mut fonts = egui::FontDefinitions::default();

    let mut prop = Vec::new();
    let mut mono = Vec::new();
    // 微软雅黑：中英文同源，混排时基线一致（日志也用这一支）
    if add_font(&mut fonts, &dir, "yahei", "msyh.ttc") {
        prop.push("yahei".to_string());
        mono.push("yahei".to_string());
    }
    // 兜底：Segoe UI
    if add_font(&mut fonts, &dir, "segoe", "segoeui.ttf") {
        prop.push("segoe".to_string());
    }
    if prop.is_empty() {
        return;
    }
    if mono.is_empty() {
        mono = prop.clone();
    }
    fonts.families.insert(egui::FontFamily::Proportional, prop);
    fonts.families.insert(egui::FontFamily::Monospace, mono);
    ctx.set_fonts(fonts);
}

fn add_font(fonts: &mut egui::FontDefinitions, dir: &Path, key: &str, file: &str) -> bool {
    let Ok(bytes) = std::fs::read(dir.join(file)) else {
        return false;
    };
    let mut data = egui::FontData::from_owned(bytes);
    data.index = 0;
    fonts.font_data.insert(key.to_string(), std::sync::Arc::new(data));
    true
}
