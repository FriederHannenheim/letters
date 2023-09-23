#![warn(clippy::all, rust_2018_idioms)]

// SPDX-FileCopyrightText: 2023 Frieder Hannenheim <frieder.hannenheim@pm.me>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod app;
mod request;
mod collection;
mod tabs;

pub use app::PacketsApp;
