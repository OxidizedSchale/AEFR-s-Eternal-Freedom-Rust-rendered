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

use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, Rounding, Stroke, Visuals};

// ⚠️ 极其关键：现在 theme.rs 位于 src/ui/ 目录下，
// 相对路径必须通过 "../" 回退到 src/ 目录才能找到字体文件！
const FONT_DATA: &[u8] = include_bytes!("../SarasaTermSCNerd-Regular.ttf");

/// 🌟 极客精神：强制全局使用纯白底色、黑框描边的包豪斯（Bauhaus）风格主题
pub fn setup_custom_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::light();
    
    visuals.window_fill = Color32::WHITE; 
    visuals.panel_fill = Color32::WHITE;  
    visuals.extreme_bg_color = Color32::WHITE; 
    visuals.text_cursor = Stroke::new(2.0, Color32::BLACK); 

    let rounding = Rounding::same(4.0); 

    visuals.widgets.inactive.bg_fill = Color32::WHITE;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::BLACK);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::BLACK);
    visuals.widgets.inactive.rounding = rounding;

    visuals.widgets.hovered.bg_fill = Color32::WHITE;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, Color32::BLACK);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::BLACK);
    visuals.widgets.hovered.rounding = rounding;

    visuals.widgets.active.bg_fill = Color32::WHITE;
    visuals.widgets.active.bg_stroke = Stroke::new(2.0, Color32::BLACK);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::BLACK);
    visuals.widgets.active.rounding = rounding;

    visuals.widgets.open.bg_fill = Color32::WHITE;
    visuals.widgets.open.bg_stroke = Stroke::new(1.5, Color32::BLACK);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, Color32::BLACK);
    visuals.widgets.open.rounding = rounding;

    visuals.selection.bg_fill = Color32::BLACK; 
    visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE); 
    visuals.faint_bg_color = Color32::WHITE; 
    
    ctx.set_visuals(visuals);
}

pub fn setup_embedded_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("sarasa_font".to_owned(), FontData::from_static(FONT_DATA));
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "sarasa_font".to_owned());
    fonts.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "sarasa_font".to_owned());
    ctx.set_fonts(fonts);
}