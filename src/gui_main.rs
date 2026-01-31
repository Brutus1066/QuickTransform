//! QuickTransform GUI
//! LAZYFROG-kindware.dev | MIT License
//! Modern 2026 UI Design

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Color32, FontId, Margin, RichText, Rounding, Stroke, Vec2};
use quicktransform::transforms::{encode, hash, generate};
use quicktransform::{APP_NAME, BRAND, VERSION};
use std::path::PathBuf;

// ============================================================================
// THEME SYSTEM
// ============================================================================

#[derive(Default, Clone, Copy, PartialEq)]
enum Theme {
    #[default]
    Dark,
    Light,
}

struct Colors;
impl Colors {
    // Get colors based on theme
    fn bg_dark(theme: Theme) -> Color32 {
        match theme {
            Theme::Dark => Color32::from_rgb(15, 17, 21),
            Theme::Light => Color32::from_rgb(250, 250, 252),
        }
    }
    fn bg_primary(theme: Theme) -> Color32 {
        match theme {
            Theme::Dark => Color32::from_rgb(22, 25, 30),
            Theme::Light => Color32::from_rgb(255, 255, 255),
        }
    }
    fn bg_card(theme: Theme) -> Color32 {
        match theme {
            Theme::Dark => Color32::from_rgb(30, 34, 42),
            Theme::Light => Color32::from_rgb(241, 245, 249),
        }
    }
    fn bg_input(theme: Theme) -> Color32 {
        match theme {
            Theme::Dark => Color32::from_rgb(18, 21, 26),
            Theme::Light => Color32::from_rgb(248, 250, 252),
        }
    }
    fn bg_hover(theme: Theme) -> Color32 {
        match theme {
            Theme::Dark => Color32::from_rgb(40, 45, 55),
            Theme::Light => Color32::from_rgb(226, 232, 240),
        }
    }

    // Accent - consistent across themes
    const ACCENT: Color32 = Color32::from_rgb(20, 184, 166);
    const ACCENT_SOFT: Color32 = Color32::from_rgb(13, 148, 136);

    // Secondary accents
    const PURPLE: Color32 = Color32::from_rgb(139, 92, 246);
    const BLUE: Color32 = Color32::from_rgb(59, 130, 246);
    const GREEN: Color32 = Color32::from_rgb(34, 197, 94);
    const AMBER: Color32 = Color32::from_rgb(251, 191, 36);
    const RED: Color32 = Color32::from_rgb(239, 68, 68);

    // Text - adapts to theme
    fn text_primary(theme: Theme) -> Color32 {
        match theme {
            Theme::Dark => Color32::from_rgb(241, 245, 249),
            Theme::Light => Color32::from_rgb(15, 23, 42),
        }
    }
    fn text_secondary(theme: Theme) -> Color32 {
        match theme {
            Theme::Dark => Color32::from_rgb(148, 163, 184),
            Theme::Light => Color32::from_rgb(71, 85, 105),
        }
    }
    fn text_muted(theme: Theme) -> Color32 {
        match theme {
            Theme::Dark => Color32::from_rgb(100, 116, 139),
            Theme::Light => Color32::from_rgb(148, 163, 184),
        }
    }
    fn text_on_accent(_theme: Theme) -> Color32 {
        Color32::from_rgb(255, 255, 255)
    }

    // Borders
    fn border(theme: Theme) -> Color32 {
        match theme {
            Theme::Dark => Color32::from_rgb(51, 65, 85),
            Theme::Light => Color32::from_rgb(203, 213, 225),
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() -> Result<(), eframe::Error> {
    let icon = load_icon();

    eframe::run_native(
        APP_NAME,
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([640.0, 680.0])
                .with_min_inner_size([520.0, 560.0])
                .with_icon(icon)
                .with_title(APP_NAME),
            ..Default::default()
        },
        Box::new(|cc| {
            configure_style(&cc.egui_ctx, Theme::Dark);
            Ok(Box::new(App::default()))
        }),
    )
}

fn load_icon() -> egui::IconData {
    #[cfg(target_os = "windows")]
    let bytes = include_bytes!("../assets/lazyfrog-kindware.ico");
    #[cfg(not(target_os = "windows"))]
    let bytes = include_bytes!("../assets/lazyfrog-kindware-background.png");

    image::load_from_memory(bytes)
        .map(|img| {
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();
            egui::IconData { rgba: rgba.into_raw(), width: w, height: h }
        })
        .unwrap_or(egui::IconData { rgba: vec![0; 4], width: 1, height: 1 })
}

fn configure_style(ctx: &egui::Context, theme: Theme) {
    let mut style = (*ctx.style()).clone();

    style.spacing.item_spacing = Vec2::new(10.0, 8.0);
    style.spacing.button_padding = Vec2::new(16.0, 8.0);
    style.spacing.window_margin = Margin::same(20.0);

    let v = &mut style.visuals;
    v.dark_mode = matches!(theme, Theme::Dark);
    v.panel_fill = Colors::bg_dark(theme);
    v.window_fill = Colors::bg_primary(theme);
    v.extreme_bg_color = Colors::bg_input(theme);
    v.faint_bg_color = Colors::bg_card(theme);

    v.widgets.noninteractive.bg_fill = Colors::bg_card(theme);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Colors::text_secondary(theme));
    v.widgets.noninteractive.rounding = Rounding::same(8.0);

    v.widgets.inactive.bg_fill = Colors::bg_card(theme);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, Colors::text_primary(theme));
    v.widgets.inactive.rounding = Rounding::same(8.0);

    v.widgets.hovered.bg_fill = Colors::bg_hover(theme);
    v.widgets.hovered.fg_stroke = Stroke::new(1.0, Colors::text_primary(theme));
    v.widgets.hovered.rounding = Rounding::same(8.0);

    v.widgets.active.bg_fill = Colors::ACCENT_SOFT;
    v.widgets.active.fg_stroke = Stroke::new(1.0, Colors::text_on_accent(theme));
    v.widgets.active.rounding = Rounding::same(8.0);

    v.selection.bg_fill = Colors::ACCENT.gamma_multiply(0.3);
    v.selection.stroke = Stroke::new(1.0, Colors::ACCENT);

    v.window_rounding = Rounding::same(12.0);
    v.window_stroke = Stroke::new(1.0, Colors::border(theme));

    ctx.set_style(style);
}

// ============================================================================
// APP STATE
// ============================================================================

#[derive(Default)]
struct App {
    input: String,
    output: String,
    tab: Tab,
    encode_op: EncodeOp,
    hash_algo: HashAlgo,
    gen_len: String,
    file: Option<PathBuf>,
    status: Option<(String, bool)>, // (message, is_error)
    show_help: bool,
    help_section: usize,
    theme: Theme,
}

#[derive(Default, Clone, Copy, PartialEq)]
enum Tab { #[default] Encode, Hash, Generate }

#[derive(Default, Clone, Copy, PartialEq)]
enum EncodeOp {
    #[default] B64Enc, B64Dec, HexEnc, HexDec, UrlEnc, UrlDec, HtmlEnc, HtmlDec
}

#[derive(Default, Clone, Copy, PartialEq)]
enum HashAlgo { MD5, SHA1, #[default] SHA256, SHA512, All }

// ============================================================================
// UI RENDERING
// ============================================================================

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let t = self.theme;
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Colors::bg_dark(t)).inner_margin(24.0))
            .show(ctx, |ui| {
                self.header(ui, ctx);
                ui.add_space(20.0);

                if self.show_help {
                    self.help_panel(ui);
                } else {
                    self.main_content(ui);
                }

                self.footer(ui);
            });
    }
}

impl App {
    // ========================================================================
    // HEADER
    // ========================================================================

    fn header(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let t = self.theme;

        // Top row: Logo and controls
        ui.horizontal(|ui| {
            // Logo
            ui.label(RichText::new(APP_NAME).size(24.0).color(Colors::ACCENT).strong());

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Help toggle
                let help_text = if self.show_help { "Close" } else { "Help" };
                let help_color = if self.show_help { Colors::ACCENT } else { Colors::text_secondary(t) };

                if ui.add(
                    egui::Button::new(RichText::new(format!("? {}", help_text)).size(13.0).color(help_color))
                        .fill(if self.show_help { Colors::bg_card(t) } else { Color32::TRANSPARENT })
                        .stroke(Stroke::new(1.0, Colors::border(t)))
                        .rounding(Rounding::same(6.0))
                ).clicked() {
                    self.show_help = !self.show_help;
                }

                ui.add_space(8.0);

                // Theme toggle
                let theme_icon = match self.theme {
                    Theme::Dark => "Light",
                    Theme::Light => "Dark",
                };
                if ui.add(
                    egui::Button::new(RichText::new(theme_icon).size(12.0).color(Colors::text_secondary(t)))
                        .fill(Color32::TRANSPARENT)
                        .stroke(Stroke::new(1.0, Colors::border(t)))
                        .rounding(Rounding::same(6.0))
                ).clicked() {
                    self.theme = match self.theme {
                        Theme::Dark => Theme::Light,
                        Theme::Light => Theme::Dark,
                    };
                    configure_style(ctx, self.theme);
                }

                ui.add_space(12.0);
                ui.label(RichText::new(format!("v{}", VERSION)).size(12.0).color(Colors::text_muted(t)));
            });
        });

        // Second row: Tagline and branding
        ui.horizontal(|ui| {
            ui.label(RichText::new("Encode • Hash • Generate").size(12.0).color(Colors::text_secondary(t)));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new(BRAND).size(11.0).color(Colors::ACCENT_SOFT));
            });
        });
    }

    // ========================================================================
    // MAIN CONTENT
    // ========================================================================

    fn main_content(&mut self, ui: &mut egui::Ui) {
        let t = self.theme;
        // Tab bar
        self.tab_bar(ui);
        ui.add_space(16.0);

        // Content card
        egui::Frame::none()
            .fill(Colors::bg_primary(t))
            .rounding(Rounding::same(12.0))
            .stroke(Stroke::new(1.0, Colors::border(t)))
            .inner_margin(20.0)
            .show(ui, |ui| {
                match self.tab {
                    Tab::Encode => self.encode_panel(ui),
                    Tab::Hash => self.hash_panel(ui),
                    Tab::Generate => self.generate_panel(ui),
                }
            });
    }

    fn tab_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            self.tab_button(ui, "Encode", Tab::Encode, Colors::ACCENT);
            ui.add_space(4.0);
            self.tab_button(ui, "Hash", Tab::Hash, Colors::PURPLE);
            ui.add_space(4.0);
            self.tab_button(ui, "Generate", Tab::Generate, Colors::GREEN);
        });
    }

    fn tab_button(&mut self, ui: &mut egui::Ui, label: &str, tab: Tab, color: Color32) {
        let t = self.theme;
        let active = self.tab == tab;
        let bg = if active { color.gamma_multiply(0.2) } else { Color32::TRANSPARENT };
        let text_color = if active { color } else { Colors::text_secondary(t) };
        let stroke = if active { Stroke::new(2.0, color) } else { Stroke::NONE };

        if ui.add(
            egui::Button::new(RichText::new(label).size(14.0).color(text_color).strong())
                .fill(bg)
                .stroke(stroke)
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(100.0, 36.0))
        ).clicked() {
            self.tab = tab;
            self.status = None;
        }
    }

    // ========================================================================
    // ENCODE PANEL
    // ========================================================================

    fn encode_panel(&mut self, ui: &mut egui::Ui) {
        let t = self.theme;
        ui.label(RichText::new("Operation").size(12.0).color(Colors::text_muted(t)));
        ui.add_space(8.0);

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(6.0, 6.0);
            self.encode_chip(ui, "Base64 Enc", EncodeOp::B64Enc);
            self.encode_chip(ui, "Base64 Dec", EncodeOp::B64Dec);
            self.encode_chip(ui, "Hex Enc", EncodeOp::HexEnc);
            self.encode_chip(ui, "Hex Dec", EncodeOp::HexDec);
            self.encode_chip(ui, "URL Enc", EncodeOp::UrlEnc);
            self.encode_chip(ui, "URL Dec", EncodeOp::UrlDec);
            self.encode_chip(ui, "HTML Enc", EncodeOp::HtmlEnc);
            self.encode_chip(ui, "HTML Dec", EncodeOp::HtmlDec);
        });

        ui.add_space(20.0);
        self.io_section(ui, true);
    }

    fn encode_chip(&mut self, ui: &mut egui::Ui, label: &str, op: EncodeOp) {
        let t = self.theme;
        let active = self.encode_op == op;
        let bg = if active { Colors::ACCENT.gamma_multiply(0.2) } else { Colors::bg_card(t) };
        let text = if active { Colors::ACCENT } else { Colors::text_secondary(t) };
        let stroke = if active { Stroke::new(1.0, Colors::ACCENT.gamma_multiply(0.5)) } else { Stroke::NONE };

        if ui.add(
            egui::Button::new(RichText::new(label).size(12.0).color(text))
                .fill(bg)
                .stroke(stroke)
                .rounding(Rounding::same(16.0))
                .min_size(Vec2::new(0.0, 30.0))
        ).clicked() {
            self.encode_op = op;
        }
    }

    // ========================================================================
    // HASH PANEL
    // ========================================================================

    fn hash_panel(&mut self, ui: &mut egui::Ui) {
        let t = self.theme;
        ui.horizontal(|ui| {
            ui.label(RichText::new("Algorithm").size(12.0).color(Colors::text_muted(t)));
            ui.add_space(12.0);

            self.hash_chip(ui, "MD5", HashAlgo::MD5, Colors::RED);
            self.hash_chip(ui, "SHA-1", HashAlgo::SHA1, Colors::AMBER);
            self.hash_chip(ui, "SHA-256", HashAlgo::SHA256, Colors::GREEN);
            self.hash_chip(ui, "SHA-512", HashAlgo::SHA512, Colors::BLUE);
            self.hash_chip(ui, "All", HashAlgo::All, Colors::PURPLE);
        });

        ui.add_space(16.0);

        // File selector
        ui.horizontal(|ui| {
            if ui.add(
                egui::Button::new(RichText::new("Select File").size(12.0).color(Colors::text_primary(t)))
                    .fill(Colors::bg_card(t))
                    .stroke(Stroke::new(1.0, Colors::border(t)))
                    .rounding(Rounding::same(6.0))
            ).clicked() {
                if let Some(p) = rfd::FileDialog::new().pick_file() {
                    self.file = Some(p.clone());
                    self.input = p.display().to_string();
                }
            }

            ui.add_space(8.0);

            if let Some(ref p) = self.file {
                let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                ui.label(RichText::new(name).size(12.0).color(Colors::ACCENT));
                if ui.add(
                    egui::Button::new(RichText::new("×").size(14.0).color(Colors::text_muted(t)))
                        .fill(Color32::TRANSPARENT)
                        .frame(false)
                ).clicked() {
                    self.file = None;
                    self.input.clear();
                }
            } else {
                ui.label(RichText::new("or enter text below").size(12.0).color(Colors::text_muted(t)));
            }
        });

        ui.add_space(16.0);
        self.io_section(ui, self.file.is_none());
    }

    fn hash_chip(&mut self, ui: &mut egui::Ui, label: &str, algo: HashAlgo, color: Color32) {
        let t = self.theme;
        let active = self.hash_algo == algo;
        let bg = if active { color.gamma_multiply(0.2) } else { Colors::bg_card(t) };
        let text = if active { color } else { Colors::text_secondary(t) };

        if ui.add(
            egui::Button::new(RichText::new(label).size(11.0).color(text))
                .fill(bg)
                .rounding(Rounding::same(14.0))
                .min_size(Vec2::new(0.0, 28.0))
        ).clicked() {
            self.hash_algo = algo;
        }
    }

    // ========================================================================
    // GENERATE PANEL
    // ========================================================================

    fn generate_panel(&mut self, ui: &mut egui::Ui) {
        let t = self.theme;
        ui.horizontal(|ui| {
            ui.label(RichText::new("Length").size(12.0).color(Colors::text_muted(t)));
            ui.add_space(8.0);

            ui.add(
                egui::TextEdit::singleline(&mut self.gen_len)
                    .desired_width(70.0)
                    .font(FontId::monospace(14.0))
                    .margin(Margin::symmetric(12.0, 8.0))
            );

            if self.gen_len.is_empty() {
                self.gen_len = "16".to_string();
            }
        });

        ui.add_space(20.0);

        // Generator buttons - modern grid
        ui.horizontal(|ui| {
            if self.gen_button(ui, "UUID v4", Colors::ACCENT) {
                self.output = generate::uuid_v4();
                self.status = Some(("UUID generated".into(), false));
            }
            if self.gen_button(ui, "Password", Colors::GREEN) {
                let len = self.gen_len.parse().unwrap_or(16);
                self.output = generate::strong_password(len);
                self.status = Some((format!("{}-char password", len), false));
            }
            if self.gen_button(ui, "Alphanumeric", Colors::AMBER) {
                let len = self.gen_len.parse().unwrap_or(16);
                self.output = generate::alphanum_password(len);
                self.status = Some((format!("{}-char alphanum", len), false));
            }
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if self.gen_button(ui, "Random Hex", Colors::PURPLE) {
                let len = self.gen_len.parse().unwrap_or(16);
                self.output = generate::random_hex(len);
                self.status = Some((format!("{} bytes hex", len), false));
            }
            if self.gen_button(ui, "Random Base64", Colors::BLUE) {
                let len = self.gen_len.parse().unwrap_or(16);
                self.output = generate::random_base64(len);
                self.status = Some((format!("{} bytes base64", len), false));
            }
        });

        ui.add_space(24.0);

        // Output
        ui.label(RichText::new("Output").size(12.0).color(Colors::text_muted(t)));
        ui.add_space(6.0);

        egui::Frame::none()
            .fill(Colors::bg_input(t))
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(1.0, Colors::border(t)))
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.output)
                        .desired_width(f32::INFINITY)
                        .desired_rows(3)
                        .font(FontId::monospace(13.0))
                        .frame(false)
                );
            });

        ui.add_space(12.0);
        self.action_bar(ui, false);
    }

    fn gen_button(&mut self, ui: &mut egui::Ui, label: &str, color: Color32) -> bool {
        ui.add(
            egui::Button::new(RichText::new(label).size(12.0).color(Colors::text_on_accent(self.theme)))
                .fill(color.gamma_multiply(0.8))
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(110.0, 36.0))
        ).clicked()
    }

    // ========================================================================
    // I/O SECTION
    // ========================================================================

    fn io_section(&mut self, ui: &mut egui::Ui, show_input: bool) {
        let t = self.theme;
        if show_input {
            ui.label(RichText::new("Input").size(12.0).color(Colors::text_muted(t)));
            ui.add_space(6.0);

            egui::Frame::none()
                .fill(Colors::bg_input(t))
                .rounding(Rounding::same(8.0))
                .stroke(Stroke::new(1.0, Colors::border(t)))
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.input)
                            .desired_width(f32::INFINITY)
                            .desired_rows(3)
                            .font(FontId::monospace(13.0))
                            .hint_text("Enter text...")
                            .frame(false)
                    );
                });

            ui.add_space(12.0);
        }

        self.action_bar(ui, show_input);

        ui.add_space(16.0);

        ui.label(RichText::new("Output").size(12.0).color(Colors::text_muted(t)));
        ui.add_space(6.0);

        egui::Frame::none()
            .fill(Colors::bg_input(t))
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(1.0, Colors::border(t)))
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.output)
                        .desired_width(f32::INFINITY)
                        .desired_rows(3)
                        .font(FontId::monospace(13.0))
                        .frame(false)
                );
            });

        ui.add_space(12.0);

        // Copy + status
        ui.horizontal(|ui| {
            if ui.add(
                egui::Button::new(RichText::new("Copy").size(12.0).color(Colors::text_primary(t)))
                    .fill(Colors::bg_card(t))
                    .stroke(Stroke::new(1.0, Colors::border(t)))
                    .rounding(Rounding::same(6.0))
            ).clicked() {
                ui.ctx().copy_text(self.output.clone());
                self.status = Some(("Copied!".into(), false));
            }

            ui.add_space(12.0);

            if let Some((ref msg, is_err)) = self.status {
                let color = if is_err { Colors::RED } else { Colors::GREEN };
                ui.label(RichText::new(msg).size(12.0).color(color));
            }
        });
    }

    fn action_bar(&mut self, ui: &mut egui::Ui, show_swap: bool) {
        let t = self.theme;
        ui.horizontal(|ui| {
            // Transform button - prominent
            if ui.add(
                egui::Button::new(RichText::new("Transform").size(14.0).color(Colors::text_on_accent(t)).strong())
                    .fill(Colors::ACCENT)
                    .rounding(Rounding::same(8.0))
                    .min_size(Vec2::new(120.0, 40.0))
            ).clicked() {
                self.transform();
            }

            ui.add_space(8.0);

            if show_swap {
                if ui.add(
                    egui::Button::new(RichText::new("Swap").size(12.0).color(Colors::text_secondary(t)))
                        .fill(Colors::bg_card(t))
                        .stroke(Stroke::new(1.0, Colors::border(t)))
                        .rounding(Rounding::same(6.0))
                        .min_size(Vec2::new(70.0, 36.0))
                ).clicked() {
                    std::mem::swap(&mut self.input, &mut self.output);
                }
            }

            if ui.add(
                egui::Button::new(RichText::new("Clear").size(12.0).color(Colors::text_secondary(t)))
                    .fill(Colors::bg_card(t))
                    .stroke(Stroke::new(1.0, Colors::border(t)))
                    .rounding(Rounding::same(6.0))
                    .min_size(Vec2::new(70.0, 36.0))
            ).clicked() {
                self.input.clear();
                self.output.clear();
                self.file = None;
                self.status = None;
            }
        });
    }

    // ========================================================================
    // TRANSFORM LOGIC
    // ========================================================================

    fn transform(&mut self) {
        let input = self.input.trim();

        let result: Result<String, String> = match self.tab {
            Tab::Encode => match self.encode_op {
                EncodeOp::B64Enc => Ok(encode::base64_encode(input)),
                EncodeOp::B64Dec => encode::base64_decode(input),
                EncodeOp::HexEnc => Ok(encode::hex_encode(input)),
                EncodeOp::HexDec => encode::hex_decode(input),
                EncodeOp::UrlEnc => Ok(encode::url_encode(input)),
                EncodeOp::UrlDec => encode::url_decode(input),
                EncodeOp::HtmlEnc => Ok(encode::html_encode(input)),
                EncodeOp::HtmlDec => Ok(encode::html_decode(input)),
            },
            Tab::Hash => {
                if let Some(ref p) = self.file {
                    match self.hash_algo {
                        HashAlgo::All => hash::hash_file_all(p).map(|r|
                            format!("MD5:    {}\nSHA1:   {}\nSHA256: {}\nSHA512: {}", r.md5, r.sha1, r.sha256, r.sha512)),
                        a => hash::hash_file(p, Self::algo_str(a)),
                    }
                } else {
                    match self.hash_algo {
                        HashAlgo::All => {
                            let r = hash::hash_all(input.as_bytes());
                            Ok(format!("MD5:    {}\nSHA1:   {}\nSHA256: {}\nSHA512: {}", r.md5, r.sha1, r.sha256, r.sha512))
                        }
                        a => hash::hash_string(input, Self::algo_str(a)),
                    }
                }
            }
            Tab::Generate => return,
        };

        match result {
            Ok(s) => { self.output = s; self.status = Some(("Done".into(), false)); }
            Err(e) => { self.output.clear(); self.status = Some((e, true)); }
        }
    }

    fn algo_str(a: HashAlgo) -> &'static str {
        match a { HashAlgo::MD5 => "md5", HashAlgo::SHA1 => "sha1", HashAlgo::SHA256 => "sha256", HashAlgo::SHA512 => "sha512", _ => "sha256" }
    }

    // ========================================================================
    // HELP PANEL
    // ========================================================================

    fn help_panel(&mut self, ui: &mut egui::Ui) {
        let t = self.theme;
        let sections = ["Quick Start", "Encoding", "Hashing", "Generate", "About"];

        // Section tabs
        ui.horizontal(|ui| {
            for (i, name) in sections.iter().enumerate() {
                let active = self.help_section == i;
                let color = if active { Colors::ACCENT } else { Colors::text_secondary(t) };

                if ui.add(
                    egui::Button::new(RichText::new(*name).size(12.0).color(color))
                        .fill(if active { Colors::bg_card(t) } else { Color32::TRANSPARENT })
                        .rounding(Rounding::same(6.0))
                ).clicked() {
                    self.help_section = i;
                }
            }
        });

        ui.add_space(16.0);

        // Content
        egui::Frame::none()
            .fill(Colors::bg_primary(t))
            .rounding(Rounding::same(12.0))
            .stroke(Stroke::new(1.0, Colors::border(t)))
            .inner_margin(20.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical().max_height(380.0).show(ui, |ui| {
                    match self.help_section {
                        0 => self.help_quickstart(ui),
                        1 => self.help_encoding(ui),
                        2 => self.help_hashing(ui),
                        3 => self.help_generate(ui),
                        _ => self.help_about(ui),
                    }
                });
            });
    }

    fn h1(&self, ui: &mut egui::Ui, text: &str) {
        ui.label(RichText::new(text).size(18.0).color(Colors::text_primary(self.theme)).strong());
        ui.add_space(12.0);
    }

    fn h2(&self, ui: &mut egui::Ui, text: &str) {
        ui.add_space(16.0);
        ui.label(RichText::new(text).size(14.0).color(Colors::ACCENT).strong());
        ui.add_space(6.0);
    }

    fn p(&self, ui: &mut egui::Ui, text: &str) {
        ui.label(RichText::new(text).size(13.0).color(Colors::text_secondary(self.theme)));
    }

    fn code(&self, ui: &mut egui::Ui, l: &str, r: &str) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(l).size(12.0).color(Colors::GREEN).monospace());
            ui.label(RichText::new("→").size(12.0).color(Colors::text_muted(self.theme)));
            ui.label(RichText::new(r).size(12.0).color(Colors::text_secondary(self.theme)).monospace());
        });
    }

    fn help_quickstart(&self, ui: &mut egui::Ui) {
        self.h1(ui, "Quick Start");
        self.p(ui, "QX transforms text instantly. 100% offline - your data stays private.");

        self.h2(ui, "How to Use");
        self.p(ui, "1. Select a tab: Encode, Hash, or Generate");
        self.p(ui, "2. Choose an operation");
        self.p(ui, "3. Enter text and click Transform");
        self.p(ui, "4. Copy the result");

        self.h2(ui, "Tips");
        self.p(ui, "• Swap exchanges input ↔ output");
        self.p(ui, "• Hash tab supports file selection");
        self.p(ui, "• Use SHA-256 for security (green)");
        self.p(ui, "• MD5/SHA-1 are legacy only (red/orange)");
    }

    fn help_encoding(&self, ui: &mut egui::Ui) {
        self.h1(ui, "Encoding");

        self.h2(ui, "Base64");
        self.p(ui, "Binary-to-text encoding for emails, data URIs, configs.");
        self.code(ui, "Hello", "SGVsbG8=");

        self.h2(ui, "Hexadecimal");
        self.p(ui, "Byte representation for colors, debugging, crypto.");
        self.code(ui, "Hi", "4869");

        self.h2(ui, "URL Encoding");
        self.p(ui, "Makes text safe for URLs.");
        self.code(ui, "a b", "a%20b");

        self.h2(ui, "HTML Entities");
        self.p(ui, "Escapes special characters for safe HTML display.");
        self.code(ui, "<div>", "&lt;div&gt;");
    }

    fn help_hashing(&self, ui: &mut egui::Ui) {
        let t = self.theme;
        self.h1(ui, "Hashing");
        self.p(ui, "Creates a unique fingerprint. Same input = same hash.");

        self.h2(ui, "Algorithms");
        ui.horizontal(|ui| {
            ui.label(RichText::new("SHA-512").color(Colors::BLUE).strong());
            ui.label(RichText::new("Maximum security").color(Colors::text_muted(t)).size(12.0));
        });
        ui.horizontal(|ui| {
            ui.label(RichText::new("SHA-256").color(Colors::GREEN).strong());
            ui.label(RichText::new("Recommended").color(Colors::text_muted(t)).size(12.0));
        });
        ui.horizontal(|ui| {
            ui.label(RichText::new("SHA-1").color(Colors::AMBER).strong());
            ui.label(RichText::new("Legacy, weak").color(Colors::text_muted(t)).size(12.0));
        });
        ui.horizontal(|ui| {
            ui.label(RichText::new("MD5").color(Colors::RED).strong());
            ui.label(RichText::new("Legacy, broken").color(Colors::text_muted(t)).size(12.0));
        });

        self.h2(ui, "Use Cases");
        self.p(ui, "• Verify downloaded files");
        self.p(ui, "• Compare files quickly");
        self.p(ui, "• Generate checksums");
    }

    fn help_generate(&self, ui: &mut egui::Ui) {
        self.h1(ui, "Generation");

        self.h2(ui, "UUID v4");
        self.p(ui, "128-bit random identifier. Great for IDs and tokens.");

        self.h2(ui, "Password");
        self.p(ui, "Secure random password with letters, numbers, symbols.");
        self.p(ui, "Alphanumeric option excludes symbols.");

        self.h2(ui, "Random Hex/Base64");
        self.p(ui, "Raw random bytes encoded as hex or base64.");
        self.p(ui, "Perfect for API keys and encryption keys.");

        self.h2(ui, "Length");
        self.p(ui, "Password: character count. Hex/B64: byte count.");
    }

    fn help_about(&self, ui: &mut egui::Ui) {
        self.h1(ui, "About QX");

        ui.label(RichText::new(format!("Version {}", VERSION)).size(14.0).color(Colors::text_secondary(self.theme)));
        ui.add_space(8.0);
        ui.label(RichText::new(BRAND).size(16.0).color(Colors::ACCENT).strong());

        self.h2(ui, "Features");
        self.p(ui, "• Encode: Base64, Hex, URL, HTML");
        self.p(ui, "• Hash: MD5, SHA-1, SHA-256, SHA-512");
        self.p(ui, "• Generate: UUID, passwords, random data");
        self.p(ui, "• 100% offline, cross-platform");

        self.h2(ui, "License");
        self.p(ui, "MIT License - Free & Open Source");

        self.h2(ui, "CLI");
        self.p(ui, "Also available from terminal: qx --help");
    }

    // ========================================================================
    // FOOTER
    // ========================================================================

    fn footer(&self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            ui.label(
                RichText::new(format!("MIT License  •  {}  •  Free & Open Source", BRAND))
                    .size(11.0)
                    .color(Colors::text_muted(self.theme))
            );
        });
    }
}
