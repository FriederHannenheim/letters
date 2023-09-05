// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod tabs;

use std::{collections::HashMap, cell::RefCell, rc::Rc};

use egui::collapsing_header::HeaderResponse;
use egui::{Ui, Layout, Align};

use serde::{Serialize, Deserialize};
use egui::{TopBottomPanel};

use uuid::Uuid;

use poll_promise::Promise;
use ehttp;

use crate::{tabs::auth::Auth, collection::CollectionData};
use crate::request::tabs::auth_tab::AuthorizationTab;

use self::tabs::parameters_tab::ParametersTab;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum RequestMethod {
    Options,
    Head,
    Get,
    Post,
    Put,
    Patch
}

impl ToString for RequestMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Options => "OPTIONS",
            Self::Head => "HEAD",
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
        }.to_string()
    }
}

impl Default for RequestMethod {
    fn default() -> Self {
        Self::Get
    }
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

// TODO: Url gets own field url::Url and is updated with updates to url
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RequestData {
    pub method: RequestMethod,
    pub url_string: String,
    
    pub auth: HashMap<String, String>,
    pub selected_auth: Auth,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub uuid: Uuid,
    pub name: String,
    
    data: Rc<RefCell<RequestData>>,
    collection_data: Rc<RefCell<CollectionData>>,
    
    #[serde(skip)]
    promise: Option<Promise<ehttp::Result<String>>>,
    
    tab: RequestTab,
    
    auth_tab: AuthorizationTab,
    params_tab: ParametersTab,
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Request {}

impl Clone for Request {
    fn clone(&self) -> Self {
        Self {
            uuid: self.uuid,
            name: self.name.clone(),
            data: Rc::clone(&self.data),
            collection_data: Rc::clone(&self.collection_data),
            promise: None,
            tab: self.tab.clone(),
            auth_tab: self.auth_tab.clone(),
            params_tab: self.params_tab.clone(),
        }
    }
}

impl Request {
    
    pub fn new(name: &str, collection_data: Rc<RefCell<CollectionData>>)-> Self {
        let data = Rc::new(RefCell::new(Default::default()));
        Self {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            data: Rc::clone(&data),
            collection_data,
            promise: Default::default(),
            tab: RequestTab::Parameters,
            auth_tab: AuthorizationTab::new(Rc::clone(&data)),
            params_tab: ParametersTab::new(Rc::clone(&data)),
        }
    }
    
    pub fn render(&mut self, ui: &mut Ui) {
        let mut uri_changed = false;
        TopBottomPanel::top("request_top_panel").resizable(true).show_inside(ui, |ui| {
            ui.text_edit_singleline(&mut self.name);
            ui.add_space(10.);

            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source("request_method")
                    .selected_text(format!("{:?}", self.data.borrow_mut().method))
                    .show_ui(ui, |ui| {
                        let method = &mut self.data.borrow_mut().method;
                        ui.selectable_value(method, RequestMethod::Options, "OPTIONS");
                        ui.selectable_value(method, RequestMethod::Head, "HEAD");
                        ui.selectable_value(method, RequestMethod::Get, "GET");
                        ui.selectable_value(method, RequestMethod::Patch, "PATCH");
                        ui.selectable_value(method, RequestMethod::Post, "POST");
                        ui.selectable_value(method, RequestMethod::Put, "PUT");
                    });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let resp = ui.button("Send");
                    if resp.clicked() {
                        self.send_request(&resp.ctx);
                    }
                    let host = &mut self.data.borrow_mut().url_string;
                    let mut host_bar = egui::TextEdit::singleline(host);
                    host_bar = host_bar.desired_width(ui.available_width());
                    uri_changed |= ui.add(host_bar).changed();
                });
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, RequestTab::Parameters, "Parameters");
                ui.selectable_value(&mut self.tab, RequestTab::Authorization, "Authorization");
                ui.selectable_value(&mut self.tab, RequestTab::Headers, "Headers");
                ui.selectable_value(&mut self.tab, RequestTab::Body, "Body");
            });
            
            match self.tab {
                RequestTab::Parameters => {
                    self.params_tab.render(ui);
                },
                RequestTab::Authorization => {
                    self.auth_tab.render(ui);
                }
                _ => {}
            }
            
            ui.add_space(10.);
        });
        if let Some(promise) = &mut self.promise {
            if let Some(result) = promise.ready_mut() {
                let response_text = match result {
                    Ok(s) => s,
                    Err(s) => s,
                };
                ui.text_edit_multiline(response_text);
            }
        }
        if uri_changed {
            self.params_tab.url_to_params();
        }
    }
    
    fn send_request(&mut self, ctx: &egui::Context) {
        let request_data = self.data.borrow_mut();
        
        let ctx = ctx.clone();
        let (sender, promise) = Promise::new();
        let request = ehttp::Request{
            method: request_data.method.to_string(),
            url: request_data.url_string.clone(),
            body: vec![],
            headers: Default::default(),
        };
        ehttp::fetch(request, move |response| {
            ctx.request_repaint(); // wake up UI thread
            let resource = response.and_then(|response| {
                match String::from_utf8(response.bytes) {
                    Ok(s) => Ok(s),
                    Err(_e) => Err(String::from("Response is invalid UTF-8")),
                }
            });
            sender.send(resource);
        });
        self.promise = Some(promise);
    }
}