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

use eframe::egui::{self, Color32, Pos2, Rect, TextureHandle, Vec2};
use std::sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex};
use rayon::prelude::*;

use crate::constants::*;
use crate::types::{AppCommand, Scenario, Scene};
use crate::scheduler::schedule::AefrScheduler;
use crate::audio::manager::AudioManager;
use crate::render::spine::SpineObject;

use super::theme::{setup_custom_theme, setup_embedded_font};
use super::components::{draw_top_right_buttons, draw_ba_dialogue, draw_creator_panel};

pub struct AefrApp {
    pub scheduler: AefrScheduler,      
    pub audio_manager: Option<AudioManager>, 
    pub scenario: Scenario,            
    pub current_scene_idx: usize,      
    pub target_chars: Vec<char>,       
    pub visible_count: usize,          
    pub type_timer: f32,               
    
    // 渲染层分离变量
    pub display_speaker_name: String,
    pub display_speaker_aff: String,

    pub is_auto_enabled: bool,         
    pub show_dialogue: bool,           
    pub console_open: bool,            
    pub selected_slot: usize,          
    pub console_input: String,         
    pub console_logs: Vec<String>,     
    pub show_anim_preview: bool,       
    pub preview_anim_idx: usize,       
    pub characters: Vec<Option<Arc<Mutex<SpineObject>>>>, 
    pub background: Option<TextureHandle>, 
    pub tx: Sender<AppCommand>,        
    pub rx: Receiver<AppCommand>,      
}

impl AefrApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        setup_embedded_font(&cc.egui_ctx);
        setup_custom_theme(&cc.egui_ctx); 
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let (tx, rx) = channel();
        let audio_manager = AudioManager::new().ok();
        
        let startup_text = "GNA:AEFR 已启动！\n正在等待指令......";
        let mut first_scene = Scene::default();
        first_scene.speaker_name = "OxidizedSchale".into();
        first_scene.speaker_aff = "The Executive Committee of GNA:AEFR".into();
        first_scene.dialogue_content = startup_text.into();

        Self {
            scheduler: AefrScheduler::new(), is_auto_enabled: true, show_dialogue: true,
            scenario: Scenario { scenes: vec![first_scene.clone()] }, current_scene_idx: 0,
            target_chars: startup_text.chars().collect(), visible_count: 0, type_timer: 0.0,
            
            display_speaker_name: first_scene.speaker_name.clone(),
            display_speaker_aff: first_scene.speaker_aff.clone(),

            console_open: false, selected_slot: 0, console_input: String::new(),
            console_logs: vec!["[系统] 编辑器就绪。".into()],
            show_anim_preview: false, preview_anim_idx: 0,
            characters: (0..5).map(|_| None).collect(), background: None,
            audio_manager, tx, rx,
        }
    }

    pub fn sync_scene_to_ui(&mut self) {
        if let Some(scene) = self.scenario.scenes.get(self.current_scene_idx) {
            self.target_chars = scene.dialogue_content.chars().collect();
            self.display_speaker_name = scene.speaker_name.clone();
            self.display_speaker_aff = scene.speaker_aff.clone();
            self.visible_count = 0; self.type_timer = 0.0;
        }
    }

    pub fn parse_and_send_command(&mut self, input: &str) {
        let input_trimmed = input.trim(); if input_trimmed.is_empty() { return; }
        self.console_logs.push(format!("> {}", input_trimmed));
        let tx = self.tx.clone(); let cmd_lower = input_trimmed.to_lowercase();

        if cmd_lower.starts_with("load ") {
            let parts: Vec<&str> = input_trimmed.splitn(2, ' ').collect();
            if parts.len() == 2 { if let Ok(idx) = parts[0][5..].trim().parse::<usize>() { let _ = tx.send(AppCommand::RequestLoad { slot_idx: idx, path: parts[1].replace("\"", "") }); } }
        } else if cmd_lower.starts_with("anim ") {
            let parts: Vec<&str> = input_trimmed.split_whitespace().collect();
            if parts.len() >= 2 { if let Ok(idx) = parts[1].parse::<usize>() { let anim_name = parts[2].to_string(); let loop_anim = parts.get(3).map_or(true, |s| s.to_lowercase() == "true"); let _ = tx.send(AppCommand::SetAnimation { slot_idx: idx, anim_name, loop_anim }); } }
        } else if cmd_lower.starts_with("bgm ") { let _ = tx.send(AppCommand::PlayBgm(input_trimmed[4..].trim().replace("\"", ""))); }
        else if cmd_lower.starts_with("se ") { let _ = tx.send(AppCommand::PlaySe(input_trimmed[3..].trim().replace("\"", ""))); }
        else if cmd_lower == "stop" { let _ = tx.send(AppCommand::StopBgm); }
        else if cmd_lower.starts_with("talk ") {
            let p: Vec<&str> = input_trimmed[5..].split('|').collect();
            if p.len() == 3 { let _ = tx.send(AppCommand::Dialogue { name: p[0].into(), affiliation: p[1].into(), content: p[2].into() }); }
        } else if cmd_lower.starts_with("bg ") { let _ = tx.send(AppCommand::LoadBackground(input_trimmed[3..].trim().replace("\"", ""))); }
    }

    pub fn handle_async_events(&mut self, ctx: &egui::Context) {
        while let Ok(cmd) = self.rx.try_recv() {
            match cmd {
                AppCommand::Dialogue { name, affiliation, content } => {
                    let scene = &mut self.scenario.scenes[self.current_scene_idx];
                    scene.speaker_name = name; scene.speaker_aff = affiliation; scene.dialogue_content = content;
                    self.sync_scene_to_ui();  
                }
                AppCommand::Log(msg) => self.console_logs.push(msg),
                AppCommand::RequestLoad { slot_idx, path } => {
                    let tx_cb = self.tx.clone(); self.console_logs.push(format!("[解析] {}", path));
                    let path_clone = path.clone();
                    std::thread::spawn(move || {
                        if let Ok((obj, img, page, anims)) = SpineObject::load_async_no_gpu(&path_clone) { 
                            let _ = tx_cb.send(AppCommand::LoadSuccess(slot_idx, Box::new(obj), img, page, anims)); 
                        }
                    });
                }
                AppCommand::LoadSuccess(idx, obj, img, page, anims) => {
                    if let Some(slot) = self.characters.get_mut(idx) {
                        let mut loaded = *obj; let handle = ctx.load_texture(page, img, egui::TextureOptions::LINEAR);
                        loaded.texture_id = Some(handle.id()); loaded._texture = Some(handle);
                        *slot = Some(Arc::new(Mutex::new(loaded)));
                    }
                }
                AppCommand::RemoveCharacter(idx) => { self.characters[idx] = None; }
                AppCommand::LoadBackground(path) => {
                    let tx_cb = self.tx.clone(); let path_clone = path.clone();
                    std::thread::spawn(move || { if let Ok(img) = image::open(&path_clone) {
                        let c_img = egui::ColorImage::from_rgba_unmultiplied([img.width() as _, img.height() as _], img.to_rgba8().as_raw());
                        let _ = tx_cb.send(AppCommand::LoadBackgroundSuccess(c_img));
                    }});
                    self.scenario.scenes[self.current_scene_idx].bg_path = Some(path);
                }
                AppCommand::LoadBackgroundSuccess(c_img) => { self.background = Some(ctx.load_texture("bg", c_img, egui::TextureOptions::LINEAR)); }
                AppCommand::SetAnimation { slot_idx, anim_name, loop_anim } => {
                     if let Some(Some(char_arc)) = self.characters.get(slot_idx) { if let Ok(mut char) = char_arc.lock() { let _ = char.set_animation_by_name(&anim_name, loop_anim); } }
                }
                AppCommand::PlayBgm(path) => {
                    let tx_cb = self.tx.clone(); let path_clone = path.clone();
                    std::thread::spawn(move || { if let Ok(d) = std::fs::read(&path_clone) { let _ = tx_cb.send(AppCommand::AudioReady(d, true)); } });
                    self.scenario.scenes[self.current_scene_idx].bgm_path = Some(path);
                }
                AppCommand::PlaySe(path) => {
                    let tx_cb = self.tx.clone(); let path_clone = path.clone();
                    std::thread::spawn(move || { if let Ok(d) = std::fs::read(&path_clone) { let _ = tx_cb.send(AppCommand::AudioReady(d, false)); } });
                }
                AppCommand::AudioReady(data, is_bgm) => { if let Some(mgr) = &self.audio_manager { if is_bgm { mgr.play_bgm(data); } else { mgr.play_se(data); } } }
                AppCommand::StopBgm => { if let Some(mgr) = &self.audio_manager { mgr.stop_bgm(); } }
                AppCommand::ScenarioLoaded(s) => { self.scenario = s; self.current_scene_idx = 0; self.sync_scene_to_ui(); self.visible_count = self.target_chars.len(); }
            }
        }
    }
}

impl eframe::App for AefrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_async_events(ctx);
        let dt = ctx.input(|i| i.stable_dt);
        
        if self.show_dialogue && self.visible_count < self.target_chars.len() {
            self.type_timer += dt;
            while self.type_timer >= TYPEWRITER_INTERVAL { self.visible_count += 1; self.type_timer -= TYPEWRITER_INTERVAL; }
        }

        let screen = ctx.screen_rect();
        let scale_factor = screen.height() / BASE_HEIGHT;
        
        for (i, slot) in self.characters.iter().enumerate() {
            if let Some(char_arc) = slot { if let Ok(mut char) = char_arc.lock() {
                char.scale = CHAR_BASE_SCALE * scale_factor;
                char.position = Pos2::new(screen.width() * (CHAR_X_START_PERCENT + (i as f32 * CHAR_X_STEP_PERCENT)), screen.bottom() + (30.0 * scale_factor));
            }}
        }

        self.scheduler.run_parallel(|| { self.characters.par_iter().for_each(|slot| { if let Some(char_arc) = slot { if let Ok(mut char) = char_arc.lock() { char.update_parallel(dt); } } }); });

        egui::CentralPanel::default().frame(egui::Frame::none().fill(Color32::BLACK)).show(ctx, |ui| {
            let rect = ui.max_rect();
            if let Some(bg) = &self.background { let img_size = bg.size_vec2(); let scale = (rect.width() / img_size.x).max(rect.height() / img_size.y);
                ui.painter().image(bg.id(), Rect::from_center_size(rect.center(), img_size * scale), Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
            }
            for char_arc in self.characters.iter().flatten() { if let Ok(mut char) = char_arc.lock() { char.paint(ui); } }
            draw_top_right_buttons(ui, rect, &mut self.is_auto_enabled);
            if self.show_dialogue {
                let committed_text: String = self.target_chars.iter().collect();
                if !committed_text.trim().is_empty() {
                    let text: String = self.target_chars.iter().take(self.visible_count).collect();
                    if draw_ba_dialogue(ui, rect, &self.display_speaker_name, &self.display_speaker_aff, &text, self.visible_count >= self.target_chars.len()) { self.visible_count = self.target_chars.len(); }
                }
            }
            
            if ui.put(Rect::from_min_size(Pos2::new(10.0, 10.0), Vec2::new(60.0, 30.0)), egui::Button::new("SHELL")).clicked() { self.console_open = !self.console_open; }
            if self.console_open { draw_creator_panel(ctx, self); }
        });
        ctx.request_repaint();
    }
}