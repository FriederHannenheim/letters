// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::RefCell;
use std::rc::Rc;
use std::{vec};

use egui::{collapsing_header::CollapsingState, RichText};
use egui::{ScrollArea, Layout, TextEdit, Stroke, Rounding, Modifiers, Key};

use egui_dock::{DockArea, DockState, Style, TabStyle};

use uuid::Uuid;

use crate::collection::Collection;
use crate::tabs::TabViewer;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// TODO: Manually implement Deserialize so the Rc<RefCells<>> Work
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PacketsApp {
    #[serde(skip)]
    selected_collection: Option<usize>,
    #[serde(skip)]
    selected_request: Option<usize>,
    
    #[serde(skip)]
    new_collection_name: String,
 
    collections: Rc<RefCell<Vec<Collection>>>,
    
    #[serde(skip)]
    dock_state: DockState<Uuid>,
    
    #[serde(skip)]
    tab_viewer: TabViewer,
}

impl Default for PacketsApp {
    fn default() -> Self {
        let collections = Rc::new(RefCell::new(vec![]));
        Self {
            selected_collection: None,
            selected_request: None,
            new_collection_name: String::new(),
            collections: Rc::clone(&collections),
            dock_state: DockState::new(vec![]),
            tab_viewer: TabViewer::new(Rc::clone(&collections)),
        }
    }
}

// TODO: Delete Collection
// TODO: Free-Standing Requests
// TODO: Global ctrl+s shortcut
impl PacketsApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut app: PacketsApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            app.tab_viewer = TabViewer::new(Rc::clone(&app.collections));
            return app;
        }

        Default::default()
    }
}

impl eframe::App for PacketsApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut delete_request = None;
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(8.);
            ui.heading(RichText::new("Packets").size(20.).strong());
            ui.add_space(6.);
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Collections");
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("+").clicked() && !self.new_collection_name.trim().is_empty() {
                        self.collections.borrow_mut().push(Collection::new(self.new_collection_name.clone()));
                        self.new_collection_name = String::new();
                    }
                    let collection_entry = TextEdit::singleline(&mut self.new_collection_name).hint_text("Collection Name").desired_width(ui.available_width());
                    let response = ui.add(collection_entry);
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.collections.borrow_mut().push(Collection::new(self.new_collection_name.clone()));
                        self.new_collection_name = String::new();
                    }
                });
            });
            ui.separator();
            ScrollArea::vertical()
                .show(ui, |ui| {
                    for (i, collection) in self.collections.borrow_mut().iter_mut().enumerate() {
                        let id = ui.make_persistent_id(format!("collection_{}", collection.uuid.to_string()));
                        CollapsingState::load_with_default_open(ui.ctx(), id, true)
                            .show_header(ui, |ui| {
                                let selected = if let Some(c) = self.selected_collection {
                                    c == i
                                } else {
                                    false
                                };
                                
                                let label = egui::SelectableLabel::new(selected, &collection.name);
                                
                                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("+").clicked() {
                                        collection.create_request("New Request");
                                    }
                                    if ui.add_sized(ui.available_size(), label).clicked() {
                                        if let Some(tab_location) = self.dock_state.find_tab(&collection.uuid) {
                                            self.dock_state.set_active_tab(tab_location);
                                        } else {
                                            self.dock_state.push_to_focused_leaf(collection.uuid);
                                        }
                                        
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
                                    let label = egui::SelectableLabel::new(selected, request.name());
                                    
                                    let resp = ui.add(label);
                                    if resp.clicked() {
                                        if let Some(tab_location) = self.dock_state.find_tab(&request.uuid) {
                                            self.dock_state.set_active_tab(tab_location);
                                        } else {
                                            self.tab_viewer.requests.insert(request.uuid, request.clone());
                                            self.dock_state.push_to_focused_leaf(request.uuid);
                                        }
                                        
                                        
                                        self.selected_collection = Some(i);
                                        self.selected_request = Some(j);
                                    }
                                    resp.context_menu(|ui| {
                                        if ui.button("Duplicate Request").clicked() {
                                            // TODO: Implement Duplicating Tabs
                                            let new_request = request.duplicate();
                                            let uuid = new_request.uuid.clone();
                                            self.tab_viewer.requests.insert(uuid, new_request);
                                            self.dock_state.push_to_focused_leaf(uuid);
                                            
                                            ui.close_menu();
                                        }
                                        if ui.button("Delete").clicked() {
                                            if let Some(tab_location) = self.dock_state.find_tab(&request.uuid) {
                                                self.dock_state.remove_tab(tab_location);
                                            }
                                            self.tab_viewer.requests.remove(&request.uuid);
                                            delete_request = Some((i, j));
                                            
                                            ui.close_menu();
                                        }
                                    });
                                } 
                            });
                        ui.separator();
                    }
                });
        });
        
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut { modifiers: Modifiers::CTRL, key: Key::S })) {
            let leaf = self.dock_state.find_active_focused();
            if let Some((_, tab)) = leaf {
                if let Some(request) = self.tab_viewer.requests.get_mut(tab) {
                    request.wants_save = true;
                }
            }
        }
        
        let mut dock_style = Style::from_egui(&ctx.style());
        dock_style.main_surface_border_stroke = Stroke::NONE;
        dock_style.main_surface_border_rounding = Rounding::none();
        
        dock_style.tab.tab_body.stroke = Stroke::NONE;
        dock_style.tab_bar.rounding = Rounding::none();
        
        dock_style.tab.active.rounding = Rounding::none();
        dock_style.tab.active.outline_color = dock_style.tab.active.bg_fill;
        
        dock_style.tab.inactive.rounding = Rounding::none();
        dock_style.tab.inactive.outline_color = dock_style.tab.inactive.bg_fill;
        
        dock_style.tab.focused.rounding = Rounding::none();
        dock_style.tab.focused.outline_color = dock_style.tab.focused.bg_fill;
        
        dock_style.tab.hovered.rounding = Rounding::none();
        dock_style.tab.focused.outline_color = dock_style.tab.focused.bg_fill;


        DockArea::new(&mut self.dock_state)
            .style(dock_style)
            .show(ctx, &mut self.tab_viewer);
        
        if let Some((collection_index, request_index)) = delete_request {
            self.tab_viewer.collections.borrow_mut()[collection_index].requests.remove(request_index);
        }
    }
}
