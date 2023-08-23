// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{vec, collections};

use egui::{collapsing_header::CollapsingState, RichText};
use egui::{ScrollArea, Layout, TextEdit, Vec2};

use egui_dock::{Tree, DockArea};

use uuid::Uuid;

use crate::collection::Collection;
use crate::request::{Request, RequestMethod, RequestTab};
use crate::tab_viewer::TabViewer;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LettersApp {
    #[serde(skip)]
    selected_collection: Option<usize>,
    #[serde(skip)]
    selected_request: Option<usize>,
    
    #[serde(skip)]
    new_collection_name: String,
 
    #[serde(skip)]
    collections: Vec<Collection>,
    
    tree: Tree<Uuid>,
    
    tab_viewer: TabViewer,
}

impl Default for LettersApp {
    fn default() -> Self {
        Self {
            selected_collection: None,
            selected_request: None,
            new_collection_name: String::new(),
            collections: vec![],
            tree: Tree::new(vec![]),
            tab_viewer: Default::default(),
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
            
            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("+").clicked() && !self.new_collection_name.trim().is_empty() {
                        self.collections.push(Collection::new(self.new_collection_name.clone()));
                    }
                    ui.add(TextEdit::singleline(&mut self.new_collection_name).hint_text("Collection Name").desired_width(ui.available_width()));
                });
            });
            ui.separator();
            ScrollArea::vertical()
                .show(ui, |ui| {
                    for (i, collection) in self.collections.iter_mut().enumerate() {
                        let id = ui.make_persistent_id(format!("collection_{}", collection.uuid.to_string()));
                        CollapsingState::load_with_default_open(ui.ctx(), id, false)
                            .show_header(ui, |ui| {
                                let selected = if let Some(c) = self.selected_collection {
                                    c == i
                                } else {
                                    false
                                };
                                
                                let label = egui::SelectableLabel::new(selected, &collection.name);
                                
                                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("+").clicked() {
                                        collection.requests.push(Request::new("New Request".to_string()));
                                    }
                                    if ui.add_sized(ui.available_size(), label).clicked() {
                                        self.tab_viewer.collections.push(collection.clone());
                                        self.tree.push_to_focused_leaf(collection.uuid);
                                        
                                        self.selected_collection = Some(i);
                                        self.selected_request = None;
                                    }
                                });
                            })
                            .body(|ui| {
                               for (j, request) in collection.requests.iter().enumerate() {
                                    let selected = if let Some(r) = self.selected_request {
                                        r == j
                                    } else {
                                        false
                                    };
                                    let label = egui::SelectableLabel::new(selected, &request.name);
                                    
                                    if ui.add(label).clicked() {
                                        self.tab_viewer.requests.push(request.clone());
                                        self.tree.push_to_focused_leaf(request.uuid);
                                        
                                        self.selected_collection = Some(i);
                                        self.selected_request = Some(j);
                                    }
                                } 
                            });
                        ui.separator();
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            DockArea::new(&mut self.tree)
                .show_inside(ui, &mut self.tab_viewer);
        });
    }
}
