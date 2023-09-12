// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod auth;

use std::{collections::HashMap, rc::Rc, cell::RefCell};

use egui::Ui;
use serde::{Serialize, Deserialize};

use uuid::Uuid;

use crate::{collection::Collection, request::Request};




#[derive(Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TabViewer {
    pub collections: Rc<RefCell<Vec<Collection>>>,
    
    pub requests: HashMap<Uuid, Request>,
}

impl TabViewer {
    pub fn new(collections: Rc<RefCell<Vec<Collection>>>) -> Self {
        Self {
            collections,
            ..Default::default()
        }
    }
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = Uuid;
    
    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        let mut collections = self.collections.borrow_mut();
        let collection = collections.iter_mut().find(|c| &c.uuid == tab);
        let request = self.requests.get_mut(tab);
       
        if let Some(collection) = collection {
            collection.render(ui);
        }
       
        if let Some(request) = request {
            if request.do_save() {
                println!("SÖVE");
                for collection in collections.iter_mut() {
                    if let Some(collection_request) = collection.requests.iter_mut().find(|cr| cr.uuid == request.uuid) {
                        *collection_request = request.clone();
                    }
                };
            }
            request.render(ui);
        }
    }
    
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let collections = self.collections.borrow_mut();
        let collection = collections.iter().find(|c| &c.uuid == tab);
        let request = self.requests.get(tab);
        
        let Some(name) = 
             collection.map(|c| c.name.clone())
             .or(request.map(|r| {
                    match r.changed_since_save() {
                        true => format!("*{}", r.name()),
                        false => r.name()
                    }
                })) 
         else {
            panic!("Tab has invalid uuid");
        };
        
        name.into()
    }

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab)
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        self.requests.remove(tab);
        true
    }
}