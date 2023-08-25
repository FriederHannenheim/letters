use std::{rc::Rc, cell::RefCell};

use egui::Ui;
use egui_extras::{TableBuilder, Column};

use http::Uri;
use serde::{Serialize, Deserialize};

use crate::request::RequestData;



#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ParametersTab {
    request_data: Rc<RefCell<RequestData>>
}

impl ParametersTab {
    pub fn new(request_data: Rc<RefCell<RequestData>>) -> Self {
        Self {
            request_data
        }
    }
    
    pub fn render(&mut self, ui: &mut Ui) {
        println!("rendering");
        let request_data = self.request_data.borrow_mut();
        TableBuilder::new(ui)
            .column(Column::initial(128.).resizable(true))
            .column(Column::remainder())
            .body(|mut body| {
                if let Some(parameters) = get_parameters(&request_data.uri) {
                    for (mut key, mut value) in parameters {
                        body.row(24., |mut row| {
                            row.col(|ui| {
                                ui.text_edit_singleline(&mut key);
                            });
                            row.col(|ui| {
                                ui.text_edit_singleline(&mut value);
                            });
                        });
                    }
                }
            });
    }
    
}

fn get_parameters(uri: &str) -> Option<Vec<(String, String)>> {
   let uri = uri.parse::<Uri>().ok()?;
   
   let mut params = vec![];
   for pair in uri.query()?.split('&') {
       let mut split = pair.split('=');
       params.push((split.next()?.to_string(), split.next()?.to_string()));
   }
   Some(params)
}
