use egui::collapsing_header::CollapsingState;
use egui::{Resize, ScrollArea, TopBottomPanel, vec2};
use egui::WidgetType::{SelectableLabel, TextEdit};
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: i32,

    method: RequestMethod,

    request_host: String,

    request_tab: String,

    request_text: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
enum RequestMethod {
    Options,
    Head,
    Get,
    Post,
    Put,
    Patch
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 0,
            method: RequestMethod::Options,
            request_host: String::new(),
            request_tab: String::from("params"),
            request_text: String::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("Letters");
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Collections");
            ui.separator();
            ScrollArea::vertical()
                .show(ui, |ui| {
                    for i in 0..100 {
                        let id = ui.make_persistent_id(format!("my_collapsing_header_{i}"));
                        CollapsingState::load_with_default_open(ui.ctx(), id, false)
                            .show_header(ui, |ui| {
                                if ui.add(egui::SelectableLabel::new(self.value == i, i.to_string())).clicked() {
                                    self.value = i;
                                }
                            })
                            .body(|ui| ui.label("Body"));
                        ui.separator();
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            TopBottomPanel::top("request_top_panel").resizable(true).show_inside(ui, |ui| {
                ui.heading("Request");

                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_source("ayy")
                        .selected_text(format!("{:?}", self.method))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.method, RequestMethod::Options, "OPTIONS");
                            ui.selectable_value(&mut self.method, RequestMethod::Head, "HEAD");
                            ui.selectable_value(&mut self.method, RequestMethod::Get, "GET");
                            ui.selectable_value(&mut self.method, RequestMethod::Patch, "PATCH");
                            ui.selectable_value(&mut self.method, RequestMethod::Post, "POST");
                            ui.selectable_value(&mut self.method, RequestMethod::Put, "PUT");
                        });
                    ui.add(egui::TextEdit::singleline(&mut self.request_host));
                    let _ = ui.button("Send");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.request_tab, String::from("params"), "Parameters");
                    ui.selectable_value(&mut self.request_tab, String::from("auth"), "Authorization");
                    ui.selectable_value(&mut self.request_tab, String::from("headers"), "Headers");
                    ui.selectable_value(&mut self.request_tab, String::from("body"), "Body");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    Resize::default().resizable(true).show(ui, |ui| {
                        ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.request_text));
                    });
                });
                ui.add_space(10.);
            });
            ui.label("adadad");
        });
    }
}
