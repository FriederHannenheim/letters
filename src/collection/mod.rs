use egui::Ui;
use egui::TopBottomPanel;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::auth::Auth;
use crate::auth::AuthorizationTab;
use crate::request::Request;


#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
enum CollectionTab {
    Auth
}


#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Collection {
    pub uuid: Uuid,
    pub name: String,
    pub requests: Vec<Request>,
    
    auth: AuthorizationTab,
    tab: CollectionTab,
    
}

impl Collection {
    
    pub fn new(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            auth: AuthorizationTab::new(Auth::None, true),
            tab: CollectionTab::Auth,
            requests: vec![],
        }
    }
    
    pub fn render(&mut self, ui: &mut Ui) {
        TopBottomPanel::top("collection_top_panel").resizable(true).show_inside(ui, |ui| {
            ui.heading(&self.name);
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

