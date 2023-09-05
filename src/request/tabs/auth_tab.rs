// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3-or-later

use egui::Ui;
use serde::{Serialize, Deserialize};
use std::{rc::Rc, cell::RefCell};

use crate::{request::RequestData, tabs::auth::Auth};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthorizationTab {
    request_data: Rc<RefCell<RequestData>>,
}

// TODO: Edit headers with authorization
impl AuthorizationTab {
    pub fn new(request_data: Rc<RefCell<RequestData>>) -> Self {
        Self {
            request_data,
        }
    }
    
    pub fn render(&mut self, ui: &mut Ui) {
        let data = &mut self.request_data.borrow_mut();
        
        ui.label("Authorization");
        egui::ComboBox::from_id_source("auth_method")
            .selected_text(format!("{}", data.selected_auth.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut data.selected_auth, Auth::None, "None");
                ui.selectable_value(&mut data.selected_auth, Auth::Inherit, "Inherit");
                ui.selectable_value(&mut data.selected_auth, Auth::Basic, "Basic");
                ui.selectable_value(&mut data.selected_auth, Auth::Bearer, "Bearer Token");
            });
        
        data.selected_auth.clone().render(&mut data.auth, ui);
    }
}