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

use serde::{Serialize, Deserialize};
use eframe::egui;
use crate::render::spine::SpineObject;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Scene {
    pub bg_path: Option<String>,              
    pub bgm_path: Option<String>,             
    pub char_paths: [Option<String>; 5],       
    pub char_anims: [Option<String>; 5],       
    pub speaker_name: String,                 
    pub speaker_aff: String,                  
    pub dialogue_content: String,             
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Scenario {
    pub scenes: Vec<Scene>,                   
}

pub enum AppCommand {
    Dialogue { name: String, affiliation: String, content: String },
    RequestLoad { slot_idx: usize, path: String },
    LoadSuccess(usize, Box<SpineObject>, egui::ColorImage, String, Vec<String>),
    RemoveCharacter(usize),
    LoadBackground(String),
    LoadBackgroundSuccess(egui::ColorImage),
    PlayBgm(String),
    PlaySe(String),
    AudioReady(Vec<u8>, bool),  
    StopBgm,
    SetAnimation { slot_idx: usize, anim_name: String, loop_anim: bool },
    Log(String),
    ScenarioLoaded(Scenario),
}