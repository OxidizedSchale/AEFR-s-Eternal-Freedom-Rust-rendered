/*
 * GNA:AEFR (GNA's Not AA:AEFR's Eternal Freedom & Rust-rendered)
 * Copyright (C) 2026 OxidizedSchale & The Executive Committee of GNA: AEFR
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License.
 *
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * 
 * GitHub: https://github.com/OxidizedSchale/GNA-AEFR
 *
 * 版权所有 (C) 2026 OxidizedSchale & The Executive Committee of GNA: AEFR
 *
 * 本程序是自由软件：您可以自由分发和/或修改它。
 * 它遵循由自由软件基金会（Free Software Foundation）发布的
 * GNU Affero 通用公共许可证（GNU Affero General Public License）第 3 版。
 * 本程序的 git 仓库应带有 AGPL3 许可证，请自行查看
 */

#![allow(warnings)]

// 注册子模块
mod constants;
mod types;
mod file_picker;
mod scheduler;
mod audio;
mod render;
mod ui;

use ui::app::AefrApp;

#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])      
            .with_title("GNA's Not AA : AEFR's Eternal Freedom & Rust_rendered"),
        vsync: true,                               
        ..Default::default()
    };
    eframe::run_native("AEFR_App", options, Box::new(|cc| Box::new(AefrApp::new(cc))))
}

#[cfg(target_os = "android")]
fn main() -> eframe::Result<()> {
    eframe::run_native("AEFR_App", eframe::NativeOptions::default(), Box::new(|cc| Box::new(AefrApp::new(cc))))
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let _ = eframe::run_native("AEFR_App", eframe::NativeOptions::default(), Box::new(|cc| Box::new(AefrApp::new(cc))));
}