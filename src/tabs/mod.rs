// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use egui::Ui;

pub mod auth;




pub trait Tab {
    type T;
    
    fn render(&mut self, ui: &mut Ui, request_data: &mut Self::T);
}