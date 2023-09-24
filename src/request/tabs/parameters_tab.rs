// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{rc::Rc, cell::RefCell};

use egui::{Ui, Button};
use egui_extras::{TableBuilder, Column};

use serde::{Serialize, Deserialize};

use url::Url;
use percent_encoding;

use crate::{request::{RequestData, self}, tabs::Tab};


// TODO: Paremeters broke
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ParametersTab {
    parameters: Vec<(String, String)>,
    new_param: (String, String),
}

impl ParametersTab {
    pub fn new() -> Self {
        Self {
            parameters: vec![],
            new_param: (String::new(), String::new()),
        }
    }
    
    fn update_new_param(&mut self) -> bool{
        if self.new_param.0.is_empty() && self.new_param.1.is_empty() {
            return false;
        }
        let param = std::mem::replace(&mut self.new_param, (String::new(), String::new()));
        self.parameters.push(param);
        true
    }
    
    fn update_url_from_params(&mut self, request_data: &mut RequestData) {
        let mut url_parts: Vec<String> = Vec::new();
        
        for (param_name, param_value) in &self.parameters {
            let encoded_name = percent_encoding::percent_encode(param_name.as_bytes(), percent_encoding::NON_ALPHANUMERIC);
            let encoded_value = percent_encoding::percent_encode(param_value.as_bytes(), percent_encoding::NON_ALPHANUMERIC).collect::<String>();
            if encoded_value.is_empty() {
                url_parts.push(format!("{}", encoded_name));
            } else {
                url_parts.push(format!("{}={}", encoded_name, encoded_value));
            }
        }
    
        let url = &mut request_data.url_string;
        let base_url = get_base_url(&url);
        println!("Url Parts: {}", url_parts.len());
        let new_url = if url_parts.is_empty() {
            base_url.to_string()
        } else {
            format!("{}?{}", base_url, url_parts.join("&"))
        };
        println!("New url: {}", &new_url);
        *url = new_url;
    }
    
    pub fn url_to_params(&mut self, request_data: &mut RequestData) {
        let url = &request_data.url_string;
        self.parameters = url::Url::options()
                    .parse(&url)
                    .map(|u| {
                        u.query_pairs()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect()
                    })
                    .unwrap_or_else(|_| vec![]);
    }
}

impl Tab for ParametersTab {
    type T = RequestData;
        
    fn render(&mut self, ui: &mut Ui, request_data: &mut Self::T) {
        let mut params_changed = false;
        let mut remove_param = None;
        // TODO: Figure out how to have the rows distributed so that the x is small. Maybe replace table with Grid
        TableBuilder::new(ui)
            .column(Column::initial(128.).resizable(true))
            .column(Column::initial(128.).resizable(true))
            .column(Column::remainder())
            .body(|mut body| {
                for (i, (key,value)) in self.parameters.iter_mut().enumerate() {
                    body.row(24., |mut row| {
                        row.col(|ui| {
                            let resp = ui.text_edit_singleline(key);
                            params_changed |= resp.changed();
                        });
                        row.col(|ui| {
                            let resp = ui.text_edit_singleline(value);
                            params_changed |= resp.changed();
                        });
                        row.col(|ui| {
                            let b = Button::new("x");
                            if ui.add_sized(ui.available_size(), b).clicked() {
                                remove_param = Some(i);
                            }
                        });
                    });
                }
                body.row(24., |mut row| {
                    row.col(|ui| {
                        let resp = ui.text_edit_singleline(&mut self.new_param.0);
                        if resp.changed() {
                            params_changed |= self.update_new_param();
                        }
                    });
                    row.col(|ui| {
                        let resp = ui.text_edit_singleline(&mut self.new_param.1);
                        if resp.changed() {
                            params_changed |= self.update_new_param()
                        }
                    });
                });
            });
        if let Some(i) = remove_param {
            self.parameters.remove(i);
            params_changed = true;
        }
        if params_changed {
            // remove empty pairs
            self.parameters.retain(|e| !(e.0.is_empty() && e.1.is_empty()));
            self.update_url_from_params(request_data);
        }
    }
    
}

fn get_base_url(url: &str) -> &str {
    if let Some(index) = url.find('?') {
        &url[0..index]
    } else {
        url
    }
}
