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

use crate::types::{AppCommand, Scenario};
use std::sync::mpsc::Sender;
use std::thread;

#[cfg(not(target_os = "android"))]
pub fn save_scenario(tx: Sender<AppCommand>, json_data: String) {
    thread::spawn(move || {
        if let Some(p) = rfd::FileDialog::new().set_file_name("scenario.json").save_file() {
            if std::fs::write(&p, json_data).is_ok() {
                let _ = tx.send(AppCommand::Log(format!("[系统] 剧本已保存: {}", p.display())));
            }
        }
    });
}

#[cfg(not(target_os = "android"))]
pub fn load_scenario(tx: Sender<AppCommand>) {
    thread::spawn(move || {
        if let Some(p) = rfd::FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
            if let Ok(data) = std::fs::read_to_string(&p) {
                if let Ok(s) = serde_json::from_str::<Scenario>(&data) {
                    let _ = tx.send(AppCommand::ScenarioLoaded(s));
                }
            }
        }
    });
}

#[cfg(not(target_os = "android"))]
pub fn pick_spine(tx: Sender<AppCommand>, slot: usize) {
    thread::spawn(move || {
        if let Some(p) = rfd::FileDialog::new().add_filter("Atlas", &["atlas"]).pick_file() {
            let _ = tx.send(AppCommand::RequestLoad { slot_idx: slot, path: p.display().to_string() });
        }
    });
}

#[cfg(not(target_os = "android"))]
pub fn pick_bg(tx: Sender<AppCommand>) {
    thread::spawn(move || {
        if let Some(p) = rfd::FileDialog::new().add_filter("Images", &["png", "jpg"]).pick_file() {
            let _ = tx.send(AppCommand::LoadBackground(p.display().to_string()));
        }
    });
}

#[cfg(not(target_os = "android"))]
pub fn pick_bgm(tx: Sender<AppCommand>) {
    thread::spawn(move || {
        if let Some(p) = rfd::FileDialog::new().add_filter("Audio", &["mp3", "wav", "ogg"]).pick_file() {
            let _ = tx.send(AppCommand::PlayBgm(p.display().to_string()));
        }
    });
}

#[cfg(not(target_os = "android"))]
pub fn pick_se(tx: Sender<AppCommand>) {
    thread::spawn(move || {
        if let Some(p) = rfd::FileDialog::new().add_filter("Audio", &["mp3", "wav", "ogg"]).pick_file() {
            let _ = tx.send(AppCommand::PlaySe(p.display().to_string()));
        }
    });
}

#[cfg(target_os = "android")]
pub fn save_scenario(tx: Sender<AppCommand>, _json_data: String) { let _ = tx.send(AppCommand::Log("[系统] 安卓端请使用指令保存".into())); }
#[cfg(target_os = "android")]
pub fn load_scenario(tx: Sender<AppCommand>) { let _ = tx.send(AppCommand::Log("[系统] 正在唤起安卓 SAF...".into())); }
#[cfg(target_os = "android")]
pub fn pick_spine(tx: Sender<AppCommand>, _slot: usize) { let _ = tx.send(AppCommand::Log("[系统] 正在唤起安卓文件选择器...".into())); }
#[cfg(target_os = "android")]
pub fn pick_bg(tx: Sender<AppCommand>) { let _ = tx.send(AppCommand::Log("[系统] 正在唤起安卓文件选择器...".into())); }
#[cfg(target_os = "android")]
pub fn pick_bgm(tx: Sender<AppCommand>) { let _ = tx.send(AppCommand::Log("[系统] 正在唤起安卓文件选择器...".into())); }
#[cfg(target_os = "android")]
pub fn pick_se(tx: Sender<AppCommand>) { let _ = tx.send(AppCommand::Log("[系统] 正在唤起安卓文件选择器...".into())); }