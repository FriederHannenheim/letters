// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{rc::Rc, cell::RefCell, default};

use egui::{Ui, TextEdit};
use serde::{Serialize, Deserialize};

use crate::{request::RequestData, tabs::Tab};

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
pub struct BodyTab;

impl BodyTab {
    pub fn new() -> Self  {
        Self {}
    }
}
impl Tab for BodyTab {
    type T = RequestData;
    
    fn render(&mut self, ui: &mut Ui, request_data: &mut Self::T) {
        ui.horizontal(|ui| {
            let selected_body = &mut request_data.selected_body;
            ui.radio_value(selected_body, BodyType::None, "None");
            ui.radio_value(selected_body, BodyType::Raw, "Raw");
            ui.radio_value(selected_body, BodyType::Binary, "Binary");
        });
        ui.add_space(5.);
            
        match request_data.selected_body {
            BodyType::None => {},
            BodyType::Raw => {
                let body_data = request_data.body.entry(BodyType::Raw)
                    .or_insert(BodyData::Raw { data: String::new() });
                let BodyData::Raw { data } = body_data else {
                    panic!("Someone inserted a wrong body type into the request body value");
                };
                let text_edit = TextEdit::multiline(data).code_editor();
                ui.horizontal(|ui| {
                    ui.add_sized(ui.available_size(), text_edit);
                });
            },
            BodyType::Binary => todo!(),
        }
    }
}