// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{rc::Rc, cell::RefCell, default};

use egui::Ui;
use serde::{Serialize, Deserialize};

use crate::request::RequestData;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BodyType {
    #[default]
    None,
    Raw,
    Binary
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BodyData {
    #[default]
    None,
    Raw { data: String },
    Binary { data: Vec<u8> }
}

impl BodyData {
    pub fn to_body(self) -> Vec<u8> {
        match self {
            Self::None => vec![],
            Self::Raw { data } => data.as_bytes().into(),
            Self::Binary { data } => data,
        } 
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BodyTab {
    request_data: Rc<RefCell<RequestData>>,
}

impl BodyTab {
    pub fn new(request_data: Rc<RefCell<RequestData>>) -> Self  {
        Self {
            request_data
        }
    }
    
    pub fn render(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let selected_body = &mut self.request_data.borrow_mut().selected_body;
            ui.radio_value(selected_body, BodyType::None, "None");
            ui.radio_value(selected_body, BodyType::Raw, "Raw");
            ui.radio_value(selected_body, BodyType::Binary, "Binary");
        });
    }
}