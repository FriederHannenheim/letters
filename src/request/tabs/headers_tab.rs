// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{rc::Rc, cell::{RefCell, RefMut}};

use egui::{Ui, Button};
use egui_extras::{TableBuilder, Column};

use serde::{Serialize, Deserialize};

use crate::{request::RequestData, tabs::Tab};


#[derive(Serialize, Deserialize, Clone)]
pub struct HeadersTab {
    new_header: (String, String),
}

impl HeadersTab {
    pub fn new() -> Self {
        Self {
            new_header: (String::new(), String::new())
        }
    }
    
    fn update_new_header(&mut self, request_data: &mut RequestData) -> bool{
        if self.new_header.0.is_empty() && self.new_header.1.is_empty() {
            return false;
        }
        let param = std::mem::replace(&mut self.new_header, (String::new(), String::new()));
        
        let headers = &mut request_data.headers;
        headers.push(param);
        true
    }
}
impl Tab for HeadersTab {
    type T = RequestData;
    
    fn render(&mut self, ui: &mut Ui, request_data: &mut Self::T)  {
        let mut headers_changed = false;
        let mut remove_header = None;
        // TODO: Figure out how to have the rows distributed so that the x is small. Maybe replace table with Grid
        TableBuilder::new(ui)
            .column(Column::initial(128.).resizable(true))
            .column(Column::initial(128.).resizable(true))
            .column(Column::remainder())
            .body(|mut body| {
                {
                    let headers = &mut request_data.headers;
                    for (i, (key,value)) in headers.iter_mut().enumerate() {
                        body.row(24., |mut row| {
                            row.col(|ui| {
                                let resp = ui.text_edit_singleline(key);
                                headers_changed |= resp.changed();
                            });
                            row.col(|ui| {
                                let resp = ui.text_edit_singleline(value);
                                headers_changed |= resp.changed();
                            });
                            row.col(|ui| {
                                let b = Button::new("x");
                                if ui.add_sized(ui.available_size(), b).clicked() {
                                    remove_header = Some(i);
                                }
                            });
                        });
                    }
                }
                body.row(24., |mut row| {
                    row.col(|ui| {
                        let resp = ui.text_edit_singleline(&mut self.new_header.0);
                        if resp.changed() {
                            headers_changed |= self.update_new_header(request_data);
                        }
                    });
                    row.col(|ui| {
                        let resp = ui.text_edit_singleline(&mut self.new_header.1);
                        if resp.changed() {
                            headers_changed |= self.update_new_header(request_data);
                        }
                    });
                });
            });
        let headers = &mut request_data.headers;
        if let Some(i) = remove_header {
            headers.remove(i);
            headers_changed = true;
        }
        if headers_changed {
            // remove empty pairs
            headers.retain(|e| !(e.0.is_empty() && e.1.is_empty()));
        }
    }
    
}