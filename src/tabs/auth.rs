// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;

use egui::{Ui, TextEdit};
use serde::{Serialize, Deserialize};


#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Auth {
    None,
    Inherit,
    Basic,
    Bearer,
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}
impl Auth {    
    pub fn render(&self, credentials: &mut HashMap<String,String>, ui: &mut Ui) {
        match self {
            Auth::None | Auth::Inherit => {},
            Auth::Basic => {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Username");
                    ui.text_edit_singleline(credentials.entry("basic_username".to_string()).or_default());
                });
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Password");
                    let password_entry = TextEdit::singleline(credentials.entry("basic_password".to_string()).or_default()).password(true);
                    ui.add(password_entry);
                });
            },
            Auth::Bearer => {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Token");
                    let token_entry = TextEdit::singleline(credentials.entry("bearer_token".to_string()).or_default()).password(true);
                    ui.add(token_entry);
                });
            }
        }
    }
}

impl ToString for Auth {
    fn to_string(&self) -> String {
        match self {
            Self::None => "None",
            Self::Inherit => "Inherit",
            Self::Basic  => "Basic",
            Self::Bearer => "Bearer Token",
        }.to_string()
    }
}

