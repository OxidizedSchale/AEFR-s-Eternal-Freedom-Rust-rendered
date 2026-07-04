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

use std::io::Cursor;
use rodio::Source;

pub struct AudioManager {
    _stream: rodio::OutputStream,           
    _stream_handle: rodio::OutputStreamHandle, 
    bgm_sink: rodio::Sink,                  
    se_sink: rodio::Sink,                   
}

impl AudioManager {
    pub fn new() -> Result<Self, String> {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().map_err(|e| e.to_string())?;
        let bgm_sink = rodio::Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;
        let se_sink = rodio::Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;
        Ok(Self { _stream, _stream_handle: stream_handle, bgm_sink, se_sink })
    }
    
    pub fn play_bgm(&self, data: Vec<u8>) {
        if let Ok(source) = rodio::Decoder::new(Cursor::new(data)) {
            self.bgm_sink.stop();  
            self.bgm_sink.append(source.repeat_infinite()); 
            self.bgm_sink.play();
        }
    }
    
    pub fn play_se(&self, data: Vec<u8>) {
        if let Ok(source) = rodio::Decoder::new(Cursor::new(data)) { 
            self.se_sink.append(source); 
            self.se_sink.play(); 
        }
    }
    
    pub fn stop_bgm(&self) { 
        self.bgm_sink.stop(); 
    }
}