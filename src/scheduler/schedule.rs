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

use std::thread;

pub struct AefrScheduler { 
    pub pool: rayon::ThreadPool 
}

impl AefrScheduler {
    pub fn new() -> Self {
        let logic_cores = thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
        Self { 
            pool: rayon::ThreadPoolBuilder::new()
                .num_threads(if logic_cores > 2 { logic_cores - 2 } else { 1 })
                .build()
                .unwrap() 
        }
    }
    
    pub fn run_parallel<OP>(&self, op: OP) where OP: FnOnce() + Send { 
        self.pool.install(op); 
    }
}