use egui::{Ui, Layout, Align};
use serde::{Serialize, Deserialize};
use egui::{TopBottomPanel, Resize};


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum RequestMethod {
    Options,
    Head,
    Get,
    Post,
    Put,
    Patch
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum RequestTab {
    Parameters,
    Authorization,
    Headers,
    Body,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum RequestResult {
    None,
    Pending,
    Some {
        body: String,
        headers: String
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Request {
    pub name: String,
    
    method: RequestMethod,
    
    host: String,

    tab: RequestTab,
}

impl Request {
    
    pub fn new(name: String)-> Self {
        Self {
            name,
            method: RequestMethod::Get,
            host: String::new(),
            tab: RequestTab::Parameters,
        }
    }
    
    pub fn render(&mut self, ui: &mut Ui) {
        TopBottomPanel::top("request_top_panel").resizable(true).show_inside(ui, |ui| {
            ui.heading("Request");
            ui.add_space(10.);

            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source("request_method")
                    .selected_text(format!("{:?}", self.method))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.method, RequestMethod::Options, "OPTIONS");
                        ui.selectable_value(&mut self.method, RequestMethod::Head, "HEAD");
                        ui.selectable_value(&mut self.method, RequestMethod::Get, "GET");
                        ui.selectable_value(&mut self.method, RequestMethod::Patch, "PATCH");
                        ui.selectable_value(&mut self.method, RequestMethod::Post, "POST");
                        ui.selectable_value(&mut self.method, RequestMethod::Put, "PUT");
                    });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let _ = ui.button("Send");
                    let mut host_bar = egui::TextEdit::singleline(&mut self.host);
                    host_bar = host_bar.desired_width(ui.available_width());
                    ui.add(host_bar);
                });
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, RequestTab::Parameters, "Parameters");
                ui.selectable_value(&mut self.tab, RequestTab::Authorization, "Authorization");
                ui.selectable_value(&mut self.tab, RequestTab::Headers, "Headers");
                ui.selectable_value(&mut self.tab, RequestTab::Body, "Body");
            });
            // requesttab logic
            ui.add_space(10.);
        });
        ui.label("adadad");
    }
}