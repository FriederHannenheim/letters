// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use egui::Ui;
use serde::{Serialize, Deserialize};
use std::{rc::Rc, cell::RefCell};

use crate::{request::RequestData, tabs::{auth::AuthType, Tab}};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthorizationTab;

impl AuthorizationTab {
    pub fn new() -> Self {
        Self {}
    }
}
    
impl Tab for AuthorizationTab {
    type T = RequestData;
    
    fn render(&mut self, ui: &mut Ui, request_data: &mut Self::T) {
        ui.label("Authorization");
        egui::ComboBox::from_id_source("auth_method")
            .selected_text(format!("{}", request_data.selected_auth.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut request_data.selected_auth, AuthType::None, "None");
                ui.selectable_value(&mut request_data.selected_auth, AuthType::Inherit, "Inherit");
                ui.selectable_value(&mut request_data.selected_auth, AuthType::Basic, "Basic");
                ui.selectable_value(&mut request_data.selected_auth, AuthType::Bearer, "Bearer Token");
            });
        
        request_data.selected_auth.clone().render(&mut request_data.auth, ui);
        
        let auth_header = match request_data.selected_auth {
            AuthType::None => None,
            AuthType::Inherit => todo!(),
            _ => {
                // TODO: This may panic. Fix that
                Some(request_data.auth.get(&request_data.selected_auth).unwrap().to_header())
            }
        };
        
        let auth_header_index = request_data.headers.iter().enumerate().find(|(_, (k,_))| k == "Authorization");
        if let Some((i, _)) = auth_header_index {
            request_data.headers.remove(i);
        }
        if let Some(auth_header) = auth_header {
            request_data.headers.push((String::from("Authorization"), auth_header));
        }
    }
}