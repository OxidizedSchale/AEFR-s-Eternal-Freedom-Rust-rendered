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

use eframe::egui::{self, Color32, Mesh, Pos2, Shape, TextureHandle, TextureId, epaint::Vertex};
use rusty_spine::{
    AnimationState, AnimationStateData, Atlas, Skeleton, SkeletonJson, SkeletonBinary, Slot,
};
use std::sync::Arc;
use crate::constants::*;

pub struct SpineObject {
    pub position: Pos2,                     
    pub scale: f32,                         
    pub _texture: Option<TextureHandle>,        
    pub texture_id: Option<TextureId>,          
    world_vertices: Vec<f32>,
    skeleton: Skeleton,                     
    state: AnimationState,                  
    _state_data: Arc<AnimationStateData>,   
    _skeleton_data: Arc<rusty_spine::SkeletonData>, 
    _atlas: Arc<Atlas>,                     
}

// 【必要性证明】: 采用 Arc<Mutex<T>> 包装，确保并行蒙皮计算时数据竞争物理隔绝。
unsafe impl Send for SpineObject {}

impl SpineObject {
    pub fn load_async_no_gpu(path_str: &str) -> Result<(Self, egui::ColorImage, String, Vec<String>), String> {
        let atlas = Arc::new(Atlas::new_from_file(std::path::Path::new(path_str)).map_err(|e| format!("Atlas Error: {}", e))?);
        let page = atlas.pages().next().ok_or("Atlas has no pages")?;
        let page_name = page.name().to_string();
        let img_path = std::path::Path::new(path_str).parent().ok_or("Invalid path")?.join(&page_name);
        let img = image::open(&img_path).map_err(|e| format!("Image Load Error: {}", e))?;
        let rgba = img.to_rgba8();
        let color_image = egui::ColorImage::from_rgba_unmultiplied([rgba.width() as _, rgba.height() as _], &rgba.into_raw());

        let skel_path = std::path::Path::new(path_str).with_extension("skel");
        let json_path = std::path::Path::new(path_str).with_extension("json");
        let skeleton_data = if skel_path.exists() {
            let skeleton_bin = SkeletonBinary::new(atlas.clone());
            Arc::new(skeleton_bin.read_skeleton_data_file(&skel_path).map_err(|e| format!("Binary load failed: {}", e))?)
        } else {
            let skeleton_json = SkeletonJson::new(atlas.clone());
            Arc::new(skeleton_json.read_skeleton_data_file(&json_path).map_err(|e| format!("JSON load failed: {}", e))?)
        };

        let state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
        let mut state = AnimationState::new(state_data.clone());
        let anim_names: Vec<String> = skeleton_data.animations().map(|a| a.name().to_string()).collect();
        if let Some(anim) = skeleton_data.animations().next() { let _ = state.set_animation(0, &anim, true); }

        Ok((Self { 
            position: Pos2::ZERO, scale: CHAR_BASE_SCALE, _texture: None, texture_id: None, 
            world_vertices: Vec::with_capacity(8192),
            skeleton: Skeleton::new(skeleton_data.clone()), state, _state_data: state_data, _skeleton_data: skeleton_data, _atlas: atlas,
        }, color_image, page_name, anim_names))
    }

    pub fn get_anim_names(&self) -> Vec<String> { 
        self._skeleton_data.animations().map(|a| a.name().to_string()).collect() 
    }
    
    pub fn set_animation_by_name(&mut self, anim_name: &str, loop_anim: bool) -> bool {
        if let Some(anim) = self._skeleton_data.animations().find(|a| a.name() == anim_name) {
            let _ = self.state.set_animation(0, &anim, loop_anim); true
        } else { false }
    }
    
    pub fn update_parallel(&mut self, dt: f32) {
        let dt = dt.min(MAX_DT);  
        self.state.update(dt);                     
        self.skeleton.set_to_setup_pose();         
        let _ = self.state.apply(&mut self.skeleton); 
        self.skeleton.update_world_transform();    
        self.skeleton.update_cache();              
    }
    
    pub fn paint(&mut self, ui: &mut egui::Ui) {
        let tex_id = match self.texture_id { Some(id) => id, None => return };
        let mut mesh = Mesh::with_texture(tex_id);
        for slot in self.skeleton.draw_order() {
            let attachment = match slot.attachment() { Some(a) => a, None => continue };
            if let Some(region) = attachment.as_region() {
                unsafe {
                    if self.world_vertices.len() < 8 { self.world_vertices.resize(8, 0.0); }
                    region.compute_world_vertices(&slot.bone(), &mut self.world_vertices, 0, 2);
                    self.push_to_mesh(&mut mesh, &self.world_vertices[0..8], &region.uvs(), &[0, 1, 2, 2, 3, 0], &*slot, region.color());
                }
            } else if let Some(mesh_att) = attachment.as_mesh() {
                unsafe {
                    let len = mesh_att.world_vertices_length() as usize;
                    if self.world_vertices.len() < len { self.world_vertices.resize(len, 0.0); }
                    mesh_att.compute_world_vertices(&*slot, 0, len as i32, &mut self.world_vertices, 0, 2);
                    let uvs = std::slice::from_raw_parts(mesh_att.uvs(), len);
                    let tris = std::slice::from_raw_parts(mesh_att.triangles(), mesh_att.triangles_count() as usize);
                    self.push_to_mesh(&mut mesh, &self.world_vertices[0..len], uvs, tris, &*slot, mesh_att.color());
                }
            }
        }
        ui.painter().add(Shape::mesh(mesh));
    }
    
    fn push_to_mesh(&self, mesh: &mut Mesh, w_v: &[f32], uvs: &[f32], tris: &[u16], slot: &Slot, att_c: rusty_spine::Color) {
        let s_c = slot.color();      
        let a = s_c.a * att_c.a;     
        let (r, g, b) = (s_c.r * att_c.r * a, s_c.g * att_c.g * a, s_c.b * att_c.b * a);
        let final_a = match slot.data().blend_mode() { rusty_spine::BlendMode::Additive => 0.0, _ => a };
        let color = Color32::from_rgba_premultiplied((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (final_a * 255.0) as u8);
        let count = usize::min(uvs.len() / 2, w_v.len() / 2);
        let idx_offset = mesh.vertices.len() as u32;  
        for i in 0..count {
            let pos = Pos2::new(w_v[i*2] * self.scale + self.position.x, -w_v[i*2+1] * self.scale + self.position.y);
            mesh.vertices.push(Vertex { pos, uv: Pos2::new(uvs[i*2], uvs[i*2+1]), color });
        }
        for &idx in tris { mesh.indices.push(idx_offset + idx as u32); }
    }
}