// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::{HashMap, BTreeMap};

use egui::{Ui, TextEdit};
use serde::{Serialize, Deserialize};
use base64_url;


#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuthType {
    None,
    Inherit,
    Basic,
    Bearer,
}

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AuthData {
    None,
    Basic {
        username: String,
        password: String,
    },
    Bearer {
        token: String,
    }
}
impl Default for AuthData {
    fn default() -> Self {
        Self::None
    }
}

impl AuthData {
    pub fn to_header(&self) -> String {
        match self {
            AuthData::None => panic!("None AuthData cannot be converted to header"),
            AuthData::Basic { username, password } => {
                let cred = format!("{}:{}", username, password);
                base64_url::encode(&cred)
            },
            AuthData::Bearer { token } => format!("Bearer {}", token),
        }
    }
    fn get_type(&self) -> AuthType {
        match self {
            Self::None => AuthType::None,
            Self::Basic {..} => AuthType::Basic,
            Self::Bearer {..} => AuthType::Bearer,
        }
    }
    fn default_from_type(auth_type: &AuthType) -> Self {
        match auth_type {
            AuthType::None | AuthType::Inherit => Self::None,
            AuthType::Basic => Self::Basic { username: String::new(), password: String::new() },
            AuthType::Bearer => Self::Bearer { token: String::new() },
        }
    }
}

impl Default for AuthType {
    fn default() -> Self {
        Self::None
    }
}
impl AuthType {    
    pub fn render(&self, credentials: &mut BTreeMap<AuthType, AuthData>, ui: &mut Ui) {
        match credentials.entry(self.clone()).or_insert(AuthData::default_from_type(&self)) {
            AuthData::None => {},
            AuthData::Basic {username, password} => {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Username");
                    ui.text_edit_singleline(username);
                });
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Password");
                    // TODO: Maybe custom widget with hide/show password
                    let password_entry = TextEdit::singleline(password).password(true);
                    ui.add(password_entry);
                });
            },
            AuthData::Bearer {token} => {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Token");
                    let token_entry = TextEdit::singleline(token).password(true);
                    ui.add(token_entry);
                });
            }
        }
    }
}

impl ToString for AuthType {
    fn to_string(&self) -> String {
        match self {
            Self::None => "None",
            Self::Inherit => "Inherit",
            Self::Basic  => "Basic",
            Self::Bearer => "Bearer Token",
        }.to_string()
    }
}

