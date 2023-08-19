use egui::Ui;
use egui::TopBottomPanel;
use serde::Deserialize;
use serde::Serialize;

use crate::auth::Auth;
use crate::auth::AuthorizationTab;


#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
enum CollectionTab {
    Auth
}


#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Collection {
    auth: AuthorizationTab,
    tab: CollectionTab,
}

impl Collection {
    
    pub fn new() -> Self {
        Self {
            auth: AuthorizationTab::new(Auth::None, true),
            tab: CollectionTab::Auth
        }
    }
    
    pub fn render(&mut self, ui: &mut Ui) {
        TopBottomPanel::top("collection_top_panel").resizable(true).show_inside(ui, |ui| {
            ui.heading("Collection");
            ui.add_space(10.);
            
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, CollectionTab::Auth, "Authorization");
            });
            
            match &self.tab {
                CollectionTab::Auth => {
                    self.auth.render(ui);
                }
            }
            ui.add_space(10.)
        });
    }
}

