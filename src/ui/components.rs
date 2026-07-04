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

use eframe::egui::{self, Color32, Mesh, Pos2, Rect, RichText, Shape, Stroke, Vec2, epaint::Vertex};
use crate::constants::*;
use crate::types::AppCommand;
use crate::ui::app::AefrApp;
use crate::file_picker;

pub fn draw_top_right_buttons(ui: &mut egui::Ui, screen: Rect, is_auto: &mut bool) {
    let (btn_w, btn_h, margin) = (90.0, 32.0, 20.0);
    let auto_rect = Rect::from_min_size(Pos2::new(screen.right() - btn_w * 2.0 - margin - 10.0, margin), Vec2::new(btn_w, btn_h));
    if ui.allocate_rect(auto_rect, egui::Sense::click()).clicked() { *is_auto = !*is_auto; }
    ui.painter().rect_filled(auto_rect, 4.0, if *is_auto { Color32::from_rgb(255, 215, 0) } else { Color32::WHITE });
    ui.painter().text(auto_rect.center(), egui::Align2::CENTER_CENTER, "AUTO", egui::FontId::proportional(18.0), Color32::from_rgb(20, 30, 50));
    ui.painter().rect_filled(Rect::from_min_size(Pos2::new(screen.right() - btn_w - margin, margin), Vec2::new(btn_w, btn_h)), 4.0, Color32::WHITE);
    ui.painter().text(Pos2::new(screen.right() - btn_w / 2.0 - margin, margin + btn_h / 2.0), egui::Align2::CENTER_CENTER, "MENU", egui::FontId::proportional(18.0), Color32::from_rgb(20, 30, 50));
}

pub fn draw_ba_dialogue(ui: &mut egui::Ui, screen: Rect, name: &str, affiliation: &str, content: &str, is_finished: bool) -> bool {
    let box_h = screen.height() * DIALOGUE_BOX_RATIO;
    let box_rect = Rect::from_min_max(Pos2::new(screen.left(), screen.bottom() - box_h), screen.max);
    let line_y = box_rect.top() + (box_h * 0.30);  
    ui.painter().rect_filled(Rect::from_min_max(Pos2::new(screen.left(), line_y), screen.max), 0.0, Color32::from_rgba_unmultiplied(12, 18, 28, 252));
    let gradient_rect = Rect::from_min_max(box_rect.left_top(), Pos2::new(screen.right(), line_y));
    let mut mesh = Mesh::default();
    let (c_bot, c_top) = (Color32::from_rgba_unmultiplied(12, 18, 28, 245), Color32::from_rgba_unmultiplied(12, 18, 28, 0));
    mesh.vertices.push(Vertex { pos: gradient_rect.left_top(), uv: Pos2::ZERO, color: c_top });
    mesh.vertices.push(Vertex { pos: gradient_rect.right_top(), uv: Pos2::ZERO, color: c_top });
    mesh.vertices.push(Vertex { pos: gradient_rect.right_bottom(), uv: Pos2::ZERO, color: c_bot });
    mesh.vertices.push(Vertex { pos: gradient_rect.left_bottom(), uv: Pos2::ZERO, color: c_bot });
    mesh.add_triangle(0, 1, 2); mesh.add_triangle(0, 2, 3);
    ui.painter().add(Shape::mesh(mesh));
    let resp = ui.allocate_rect(box_rect, egui::Sense::click());
    let pad_x = (screen.width() * 0.08).max(100.0);
    ui.painter().line_segment([Pos2::new(pad_x, line_y), Pos2::new(screen.right() - pad_x, line_y)], Stroke::new(1.5, Color32::from_rgb(100, 120, 150)));
    
    if !name.is_empty() {
        let n_size = (box_h * 0.16).clamp(22.0, 30.0);
        let n_gal = ui.painter().layout_no_wrap(name.into(), egui::FontId::proportional(n_size), Color32::WHITE);
        let n_pos = Pos2::new(box_rect.left() + pad_x, line_y - n_gal.rect.height() - 4.0);
        if !affiliation.is_empty() {
            let aff_gal = ui.painter().layout_no_wrap(affiliation.into(), egui::FontId::proportional(n_size * 0.75), Color32::from_rgb(100, 200, 255));
            ui.painter().galley(n_pos, n_gal.clone(), Color32::WHITE);
            ui.painter().galley(n_pos + Vec2::new(n_gal.rect.width() + 15.0, n_gal.rect.height() - aff_gal.rect.height()), aff_gal, Color32::from_rgb(100, 200, 255));
        } else {
            ui.painter().galley(n_pos, n_gal, Color32::WHITE);
        }
    }
    ui.painter().text(Pos2::new(box_rect.left() + pad_x, line_y + box_h * 0.05), egui::Align2::LEFT_TOP, content, egui::FontId::proportional((box_h * 0.13).clamp(18.0, 25.0)), Color32::WHITE);
    if is_finished {
        let tri_center = Pos2::new(screen.right() - pad_x, screen.bottom() - (box_h * 0.15) + (ui.input(|i| i.time) * 3.0).sin() as f32 * 3.0);
        let ts = box_h * 0.04;  
        ui.painter().add(Shape::convex_polygon(vec![tri_center + Vec2::new(-ts, -ts), tri_center + Vec2::new(ts, -ts), tri_center + Vec2::new(0.0, ts)], Color32::from_rgb(0, 180, 255), Stroke::NONE));
    }
    resp.clicked()
}

pub fn draw_creator_panel(ctx: &egui::Context, app: &mut AefrApp) {
    let mut cmd_to_send = None;  
    let danger_bg = Color32::from_rgb(220, 38, 38);
    
    egui::Window::new("创作者面板 - GNA:AEFR")
        .default_size([500.0, 600.0])
        .show(ctx, |ui| {
            ui.heading("🎬 剧本幕数管理");
            ui.horizontal(|ui| {
                if ui.button("⬅ 上一幕").clicked() && app.current_scene_idx > 0 {
                    app.current_scene_idx -= 1; app.sync_scene_to_ui(); app.visible_count = app.target_chars.len();  
                }
                ui.label(format!(" 第 {} / {} 幕 ", app.current_scene_idx + 1, app.scenario.scenes.len()));
                if ui.button("下一幕 ➡").clicked() && app.current_scene_idx < app.scenario.scenes.len() - 1 {
                    app.current_scene_idx += 1; app.sync_scene_to_ui(); app.visible_count = app.target_chars.len();
                }
                ui.separator();
                if ui.button("➕ 增加一幕").clicked() {
                    let mut new_scene = app.scenario.scenes[app.current_scene_idx].clone();
                    new_scene.dialogue_content.clear();  
                    app.scenario.scenes.insert(app.current_scene_idx + 1, new_scene);
                    app.current_scene_idx += 1; app.sync_scene_to_ui();
                }
                let btn_text = RichText::new("❌ 删除").color(Color32::WHITE);
                if ui.add(egui::Button::new(btn_text).fill(danger_bg)).clicked() && app.scenario.scenes.len() > 1 {
                    app.scenario.scenes.remove(app.current_scene_idx);
                    app.current_scene_idx = app.current_scene_idx.min(app.scenario.scenes.len() - 1);
                    app.sync_scene_to_ui();
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("跳转:");
                let mut jump = app.current_scene_idx + 1;
                let len = app.scenario.scenes.len();
                if ui.add(egui::DragValue::new(&mut jump).clamp_range(1..=len)).changed() {
                    app.current_scene_idx = jump - 1; app.sync_scene_to_ui(); app.visible_count = app.target_chars.len();
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("💾 保存剧本").clicked() { 
                    if let Ok(json_data) = serde_json::to_string_pretty(&app.scenario) { 
                        file_picker::save_scenario(app.tx.clone(), json_data); 
                    } 
                }
                if ui.button("📂 重载剧本").clicked() { file_picker::load_scenario(app.tx.clone()); }
            });

            ui.separator();
            ui.heading("📂 资源管理");
            ui.horizontal(|ui| {
                ui.label("槽位:");
                for i in 0..5 { if ui.radio_value(&mut app.selected_slot, i, format!("[{}]", i)).clicked() { app.preview_anim_idx = 0; } }
            });
            
            ui.horizontal(|ui| {
                if ui.button("📥 导入 Spine 立绘").clicked() { file_picker::pick_spine(app.tx.clone(), app.selected_slot); }
                if ui.button("🖼 背景").clicked() { file_picker::pick_bg(app.tx.clone()); }
                let btn_text = RichText::new("🗑 立绘移除").color(Color32::WHITE);
                if ui.add(egui::Button::new(btn_text).fill(danger_bg)).clicked() { cmd_to_send = Some(AppCommand::RemoveCharacter(app.selected_slot)); }
                if ui.button("🏃 动作预览").clicked() { app.show_anim_preview = true; }
            });

            ui.separator();
            ui.heading("🎵 音频管理");
            ui.horizontal(|ui| {
                if ui.button("🔁 导入音乐(循环)").clicked() { file_picker::pick_bgm(app.tx.clone()); }
                if ui.button("🔊 音效").clicked() { file_picker::pick_se(app.tx.clone()); }
                let btn_text = RichText::new("⏹ 停止音乐").color(Color32::WHITE);
                if ui.add(egui::Button::new(btn_text).fill(danger_bg)).clicked() { cmd_to_send = Some(AppCommand::StopBgm); }
            });

            ui.separator();
            ui.heading("💬 对话 (当前幕)");
            
            let scene = &mut app.scenario.scenes[app.current_scene_idx];
            
            ui.horizontal(|ui| {
                ui.label("名称:"); 
                // 🌟 修复：输入框绑定到 scene 数据层（作为未提交的草稿）
                ui.add(egui::TextEdit::singleline(&mut scene.speaker_name).desired_width(80.0));
                
                ui.label("所属:"); 
                ui.add(egui::TextEdit::singleline(&mut scene.speaker_aff).desired_width(80.0));
            });
            
            ui.add(egui::TextEdit::multiline(&mut scene.dialogue_content).desired_width(f32::INFINITY));
            
            if ui.button("▶ 发送对话 (TALK)").clicked() { 
                // 🌟 修复：直接同步。sync 会把刚才输入框修改的 scene 数据同步给渲染层，并触发打字机
                app.sync_scene_to_ui(); 
            }

            ui.separator();
            ui.horizontal(|ui| {
                let res = ui.add(egui::TextEdit::singleline(&mut app.console_input).hint_text("SHELL..."));
                if ui.button("发送").clicked() || (res.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter))) {
                    let input = app.console_input.clone(); app.parse_and_send_command(&input); app.console_input.clear(); res.request_focus();  
                }
            });
            
            egui::ScrollArea::vertical().stick_to_bottom(true).max_height(60.0).show(ui, |ui| { 
                for log in &app.console_logs { ui.label(log); } 
            });
        });

    if app.show_anim_preview {
        egui::Window::new("动作").open(&mut app.show_anim_preview).show(ctx, |ui| {
            if let Some(Some(char_arc)) = app.characters.get(app.selected_slot) {
                if let Ok(char) = char_arc.lock() {
                    let anims = char.get_anim_names();
                    if !anims.is_empty() {
                        if app.preview_anim_idx >= anims.len() { app.preview_anim_idx = 0; }
                        ui.heading(&anims[app.preview_anim_idx]);
                        ui.horizontal(|ui| {
                            if ui.button("⬅").clicked() { 
                                app.preview_anim_idx = (app.preview_anim_idx + anims.len() - 1) % anims.len(); 
                                cmd_to_send = Some(AppCommand::SetAnimation { slot_idx: app.selected_slot, anim_name: anims[app.preview_anim_idx].clone(), loop_anim: true }); 
                            }
                            if ui.button("➡").clicked() { 
                                app.preview_anim_idx = (app.preview_anim_idx + 1) % anims.len(); 
                                cmd_to_send = Some(AppCommand::SetAnimation { slot_idx: app.selected_slot, anim_name: anims[app.preview_anim_idx].clone(), loop_anim: true }); 
                            }
                        });
                    }
                }
            }
        });
    }
    
    if let Some(cmd) = cmd_to_send { let _ = app.tx.send(cmd); }
}