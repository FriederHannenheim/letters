// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod tabs;

use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hasher, Hash};
use std::{cell::RefCell, rc::Rc};

use egui::{Ui, Layout, Align, TextEdit, ScrollArea, TopBottomPanel};

use serde::{Serialize, Deserialize};

use uuid::Uuid;

use poll_promise::Promise;
use ehttp;

use crate::tabs::auth::AuthData;
use crate::tabs::Tab;
use crate::{tabs::auth::AuthType, collection::CollectionData};
use crate::request::tabs::auth_tab::AuthorizationTab;

use self::tabs::body_tab::{BodyType, BodyData, BodyTab};
use self::tabs::headers_tab::HeadersTab;
use self::tabs::parameters_tab::ParametersTab;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Hash)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct RequestData {
    pub name: String,
    
    pub method: RequestMethod,
    pub url_string: String,
    pub headers: Vec<(String, String)>,
    
    pub auth: BTreeMap<AuthType, AuthData>,
    pub selected_auth: AuthType,
    
    pub body: BTreeMap<BodyType, BodyData>,
    pub selected_body: BodyType,
}

impl Default for RequestData {
    fn default() -> Self {
        Self {
            name: String::from("New Request"),
            method: Default::default(),
            url_string: String::new(),
            headers: vec![],
            auth: Default::default(),
            selected_auth: Default::default(),
            body: Default::default(),
            selected_body: Default::default(),
        }
    }
}

// TODO: Move name to RequestData and have RequestData.changed = true if the data has been modified since the last save
// TODO: Remove the Rc<RefCell<RequestData>> and pass the RequestData into the Tabs on render
#[derive(Serialize, Deserialize)]
pub struct Request {
    pub uuid: Uuid,
    
    request_data: RequestData,
    // TODO: This is probably also not serialized correctly. Find an answer! Maybe pass in the data when rendering the request
    collection_data: Rc<RefCell<CollectionData>>,
    
    #[serde(skip)]
    promise: Option<Promise<ehttp::Result<String>>>,
    
    tab: RequestTab,
    
    auth_tab: AuthorizationTab,
    params_tab: ParametersTab,
    headers_tab: HeadersTab,
    body_tab: BodyTab,
    
    pub wants_save: bool,
    pub saved_data_hash: Option<u64>,
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request UUID: {}", self.uuid)
    }
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
            // We don't want to clone the reference, but the data
            request_data: self.request_data.clone(),
            collection_data: Rc::clone(&self.collection_data),
            promise: None,
            tab: self.tab.clone(),
            auth_tab: self.auth_tab.clone(),
            params_tab: self.params_tab.clone(),
            headers_tab: self.headers_tab.clone(),
            body_tab: self.body_tab.clone(),
            wants_save: false,
            saved_data_hash: None,
        }
    }
}

impl Request {
    
    pub fn new(collection_data: Rc<RefCell<CollectionData>>)-> Self {
        Self {
            uuid: Uuid::new_v4(),
            request_data: Default::default(),
            collection_data,
            promise: Default::default(),
            tab: RequestTab::Parameters,
            auth_tab: AuthorizationTab::new(),
            params_tab: ParametersTab::new(),
            headers_tab: HeadersTab::new(),
            body_tab: BodyTab::new(),
            wants_save: false,
            saved_data_hash: None,
        }
    }
    
    pub fn duplicate(&self) -> Self {
        let mut cloned = self.clone();
        cloned.uuid = Uuid::new_v4();
        cloned
    }
    
    pub fn render(&mut self, ui: &mut Ui) {
        let mut uri_changed = false;
        TopBottomPanel::top(format!("request_top_panel_{}", &self.uuid)).resizable(true).show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                let name = &mut self.request_data.name;
                ui.text_edit_singleline(name);
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    if ui.button("Save").clicked() {
                        self.wants_save = true;
                    }
                });
            });
            ui.add_space(10.);

            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source("request_method")
                    .selected_text(format!("{:?}", self.request_data.method))
                    .show_ui(ui, |ui| {
                        let method = &mut self.request_data.method;
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
                    let host = &mut self.request_data.url_string;
                    let host_bar = egui::TextEdit::singleline(host)
                                                            .hint_text("https://...")
                                                            .desired_width(ui.available_width());
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
            ui.add_space(5.);
            
            match self.tab {
                RequestTab::Parameters => {
                    self.params_tab.render(ui, &mut self.request_data);
                },
                RequestTab::Authorization => {
                    self.auth_tab.render(ui, &mut self.request_data);
                },
                RequestTab::Headers => {
                    self.headers_tab.render(ui, &mut self.request_data);
                },
                RequestTab::Body => {
                    self.body_tab.render(ui, &mut self.request_data);
                }
            }
            
            ui.add_space(10.);
        });
        if let Some(promise) = &mut self.promise {
            if let Some(result) = promise.ready() {
                let mut response_text = match result {
                    Ok(s) => s,
                    Err(s) => s,
                }.as_str();
                let textedit = TextEdit::multiline(&mut response_text)
                    .frame(true)
                    .code_editor();
                ScrollArea::horizontal().show(ui, |ui| {
                    ui.add_sized(ui.available_size(), textedit);
                });
            } else {
                // TODO: Loading screen
                ui.horizontal_centered(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Waiting for a Response...");
                        ui.add_space(5.);
                        ui.spinner();
                    })
                });
            }
        }
        if uri_changed {
            self.params_tab.url_to_params(&mut self.request_data);
        }
    }
    
    pub fn name(&self) -> String {
        self.request_data.name.clone()
    }
    
    /// Checks if we want to save and marks the saved data as unchanged if we do
    pub fn do_save(&mut self) -> bool {
        // Return early if we don't actually want to save
        if !std::mem::replace(&mut self.wants_save, false) {
            return false;
        }
        
        let mut hasher = DefaultHasher::new();
        self.request_data.hash(&mut hasher);
        self.saved_data_hash = Some(hasher.finish());
        
        true
    }
    
    pub fn changed_since_save(&self) -> bool {
        let Some(saved_hash) = self.saved_data_hash else {
            return true;
        };
        
        let mut hasher = DefaultHasher::new();
        self.request_data.hash(&mut hasher);
        
        saved_hash != hasher.finish()
    }
    
    fn send_request(&mut self, ctx: &egui::Context) {
        let mut headers = BTreeMap::new();
        for (key, value) in self.request_data.headers.clone() {
            headers.insert(key, value);
        }
        
        let ctx = ctx.clone();
        let (sender, promise) = Promise::new();
        let body = self.request_data.body.get(&self.request_data.selected_body).map(|b| b.clone().to_body()).unwrap_or_default();
        let request = ehttp::Request{
            method: self.request_data.method.to_string(),
            url: self.request_data.url_string.clone(),
            body,
            headers,
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