use egui::Ui;
use serde::{Serialize, Deserialize};

use uuid::Uuid;

use crate::{collection::Collection, request::{Request, self}};




#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct TabViewer {
    pub collections: Vec<Collection>,
    
    pub requests: Vec<Request>,
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = Uuid;
    
    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
       let collection = self.collections.iter_mut().find(|c| c.uuid == *tab);
       let request = self.requests.iter_mut().find(|r| r.uuid == *tab);
       
       if let Some(collection) = collection {
           collection.render(ui);
       }
       
       if let Some(request) = request {
           request.render(ui);
       }
    }
    
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
       let collection = self.collections.iter().find(|c| c.uuid == *tab);
       let request = self.requests.iter().find(|r| r.uuid == *tab);
       
       let Some(name) = collection.map(|c| c.name.clone()).or(request.map(|r| r.name.clone())) else {
           panic!("Tab has invalid uuid");
       };
       
       name.into()
    }
}