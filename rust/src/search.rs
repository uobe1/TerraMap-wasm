// 搜索功能
// 对应原项目的方块、物品、NPC 查找逻辑

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::world_loader::{World, Chest, NPC};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub tile_positions: Vec<TilePosition>,
    pub chest_results: Vec<ChestResult>,
    pub npc_results: Vec<NPCResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChestResult {
    pub chest: Chest,
    pub matching_items: Vec<ItemInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemInfo {
    pub id: i32,
    pub name: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCResult {
    pub npc: NPC,
    pub distance: f32, // 到玩家的距离
}

#[wasm_bindgen]
pub struct Searcher {
    world: World,
}

#[wasm_bindgen]
impl Searcher {
    #[wasm_bindgen(constructor)]
    pub fn new(world: JsValue) -> Result<Searcher, JsValue> {
        let world: World = serde_wasm_bindgen::from_value(world)?;
        Ok(Self { world })
    }

    #[wasm_bindgen]
    pub fn find_tiles(&self, tile_ids: JsValue) -> Result<JsValue, JsValue> {
        let tile_ids: Vec<i32> = serde_wasm_bindgen::from_value(tile_ids)?;
        let results = self.find_tiles_internal(&tile_ids);
        Ok(serde_wasm_bindgen::to_value(&results)?)
    }

    #[wasm_bindgen]
    pub fn find_chests_with_item(&self, item_id: i32) -> Result<JsValue, JsValue> {
        let results = self.find_chests_with_item_internal(item_id);
        Ok(serde_wasm_bindgen::to_value(&results)?)
    }

    #[wasm_bindgen]
    pub fn find_npcs(&self, npc_name: &str) -> Result<JsValue, JsValue> {
        let results = self.find_npcs_internal(npc_name);
        Ok(serde_wasm_bindgen::to_value(&results)?)
    }

    #[wasm_bindgen]
    pub fn find_all(&self, tile_ids: JsValue, item_ids: JsValue, npc_names: JsValue) -> Result<JsValue, JsValue> {
        let tile_ids: Vec<i32> = serde_wasm_bindgen::from_value(tile_ids)?;
        let item_ids: Vec<i32> = serde_wasm_bindgen::from_value(item_ids)?;
        let npc_names: Vec<String> = serde_wasm_bindgen::from_value(npc_names)?;

        let mut result = SearchResult {
            tile_positions: Vec::new(),
            chest_results: Vec::new(),
            npc_results: Vec::new(),
        };

        // 查找方块
        if !tile_ids.is_empty() {
            result.tile_positions = self.find_tiles_internal(&tile_ids);
        }

        // 查找箱子
        for item_id in &item_ids {
            let chests = self.find_chests_with_item_internal(*item_id);
            result.chest_results.extend(chests);
        }

        // 查找 NPC
        for npc_name in &npc_names {
            let npcs = self.find_npcs_internal(npc_name);
            result.npc_results.extend(npcs);
        }

        Ok(serde_wasm_bindgen::to_value(&result)?)
    }
}

impl Searcher {
    fn find_tiles_internal(&self, tile_ids: &[i32]) -> Vec<TilePosition> {
        let id_set: HashSet<i32> = tile_ids.iter().cloned().collect();
        let mut results = Vec::new();

        for (idx, tile) in self.world.tiles.iter().enumerate() {
            if tile.is_active && id_set.contains(&tile.tile_id) {
                let x = idx as i32 % self.world.width;
                let y = idx as i32 / self.world.width;
                results.push(TilePosition { x, y });
            }
        }

        results
    }

    fn find_chests_with_item_internal(&self, item_id: i32) -> Vec<ChestResult> {
        let mut results = Vec::new();

        for chest in &self.world.chests {
            let matching_items: Vec<ItemInfo> = chest.items
                .iter()
                .filter(|item| item.id == item_id)
                .map(|item| ItemInfo {
                    id: item.id,
                    name: self.get_item_name(item.id),
                    count: item.stack,
                })
                .collect();

            if !matching_items.is_empty() {
                results.push(ChestResult {
                    chest: chest.clone(),
                    matching_items,
                });
            }
        }

        results
    }

    fn find_npcs_internal(&self, npc_name: &str) -> Vec<NPCResult> {
        let mut results = Vec::new();

        for npc in &self.world.npcs {
            if npc.name.to_lowercase().contains(&npc_name.to_lowercase()) {
                results.push(NPCResult {
                    npc: npc.clone(),
                    distance: 0.0, // TODO: 计算到玩家的距离
                });
            }
        }

        results
    }

    fn get_item_name(&self, item_id: i32) -> String {
        // 简化版物品名称映射
        // 实际实现需要从 names.js 迁移完整的名称映射
        match item_id {
            0 => "Unknown".to_string(),
            1 => "Copper Ore".to_string(),
            2 => "Tin Ore".to_string(),
            3 => "Iron Ore".to_string(),
            4 => "Lead Ore".to_string(),
            5 => "Silver Ore".to_string(),
            6 => "Tungsten Ore".to_string(),
            7 => "Gold Ore".to_string(),
            8 => "Platinum Ore".to_string(),
            _ => format!("Item {}", item_id),
        }
    }
}