use egui::{collapsing_header::CollapsingState, RichText};
use egui::ScrollArea;

use crate::collection::Collection;
use crate::request::{Request, RequestMethod, RequestTab};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LettersApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: i32,
    
    request: Request,
    
    collection: Collection,
}

impl Default for LettersApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 0,
            request: Request::new(RequestMethod::Get, String::from(""), RequestTab::Body),
            collection: Collection::new(),
        }
    }
}

impl LettersApp {
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

impl eframe::App for LettersApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(8.);
            ui.heading(RichText::new("Letters").size(20.).strong());
            ui.add_space(6.);
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
                                let label = egui::SelectableLabel::new(self.value == i, i.to_string());
                                
                                if ui.add_sized(ui.available_size(), label).clicked() {
                                    self.value = i;
                                }
                            })
                            .body(|ui| ui.label("Body"));
                        ui.separator();
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // self.request.render(ui);
            self.collection.render(ui);
        });
    }
}
