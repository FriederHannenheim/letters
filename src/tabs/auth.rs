// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use egui::{Ui, TextEdit};
use serde::{Serialize, Deserialize};


#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Auth {
    None,
    Inherit,
    Basic {
        username: String,
        password: String,
    },
    Bearer {
        token: String,
    }
}

impl ToString for Auth {
    fn to_string(&self) -> String {
        match self {
            Self::None => "None",
            Self::Inherit => "Inherit",
            Self::Basic {..} => "Basic",
            Self::Bearer {..}=> "Bearer Token",
        }.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthorizationTab {
    auth: Auth,
    in_collection: bool,
}

impl AuthorizationTab {
    pub fn new(auth: Auth, in_collection: bool) -> Self {
        Self {
            auth,
            in_collection
        }
    }
    
    pub fn render(&mut self, ui: &mut Ui) {
        ui.label("Authorization");
        egui::ComboBox::from_id_source("auth_method")
            .selected_text(format!("{}", self.auth.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.auth, Auth::None, "None");
                if !self.in_collection {
                    ui.selectable_value(&mut self.auth, Auth::Inherit, "Inherit");
                }
                ui.selectable_value(&mut self.auth, Auth::Basic { username: String::new(), password: String::new() }, "Basic");
                ui.selectable_value(&mut self.auth, Auth::Bearer { token: String::new() }, "Bearer Token");
            });
            
        match &mut self.auth {
            Auth::None | Auth::Inherit => {},
            Auth::Basic { username, password } => {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Username");
                    ui.text_edit_singleline(username);
                });
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Password");
                    let password_entry = TextEdit::singleline(password).password(true);
                    ui.add(password_entry);
                });
            },
            Auth::Bearer { token } => {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Token");
                    let token_entry = TextEdit::singleline(token).password(true);
                    ui.add(token_entry);
                });
            }
        }
    }
}