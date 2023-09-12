// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use egui::Ui;
use serde::{Serialize, Deserialize};
use std::{rc::Rc, cell::RefCell};

use crate::{request::RequestData, tabs::auth::AuthType};


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
                ui.selectable_value(&mut data.selected_auth, AuthType::None, "None");
                ui.selectable_value(&mut data.selected_auth, AuthType::Inherit, "Inherit");
                ui.selectable_value(&mut data.selected_auth, AuthType::Basic, "Basic");
                ui.selectable_value(&mut data.selected_auth, AuthType::Bearer, "Bearer Token");
            });
        
        data.selected_auth.clone().render(&mut data.auth, ui);
        
        let auth_header = match data.selected_auth {
            AuthType::None => None,
            AuthType::Inherit => todo!(),
            _ => {
                // TODO: This may panic. Fix that
                Some(data.auth.get(&data.selected_auth).unwrap().to_header())
            }
        };
        
        let auth_header_index = data.headers.iter().enumerate().find(|(_, (k,_))| k == "Authorization");
        if let Some((i, _)) = auth_header_index {
            data.headers.remove(i);
        }
        if let Some(auth_header) = auth_header {
            data.headers.push((String::from("Authorization"), auth_header));
        }
    }
}