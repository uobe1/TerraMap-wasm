// 世界文件加载器
// 对应原项目的 WorldLoader.js

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::data_stream::DataStream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub tile_id: i32,
    pub wall_id: i32,
    pub liquid: i32,
    pub is_active: bool,
    pub is_actuated: bool,
    pub color: i32,
    pub u: i32,
    pub v: i32,
    pub brick_style: i32,
    pub full: bool,
    pub half_brick: bool,
    pub slope: i32,
    pub wire_red: bool,
    pub wire_blue: bool,
    pub wire_green: bool,
    pub wire_yellow: bool,
    pub actuator: bool,
    pub in_active: bool,
    pub wall_color: i32,
    pub wall_u: i32,
    pub wall_v: i32,
    pub wall_full: bool,
    pub wall_half_brick: bool,
    pub wall_slope: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChestItem {
    pub id: i32,
    pub stack: i32,
    pub prefix: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chest {
    pub x: i32,
    pub y: i32,
    pub name: String,
    pub items: Vec<ChestItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPC {
    pub id: i32,
    pub name: String,
    pub sprite_id: i32,
    pub position_x: f32,
    pub position_y: f32,
    pub home_x: i32,
    pub home_y: i32,
    pub direction: i32,
    pub is_homeless: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub world_id: i32,
    pub tiles: Vec<Tile>,
    pub chests: Vec<Chest>,
    pub npcs: Vec<NPC>,
    pub signs: Vec<Sign>,
    pub tile_entities: Vec<TileEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sign {
    pub x: i32,
    pub y: i32,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileEntity {
    pub id: i32,
    pub position_x: i32,
    pub position_y: i32,
    pub entity_type: i32,
    // 其他字段待添加
}

#[wasm_bindgen]
pub struct WorldLoader {
    // 可以在这里添加状态
}

#[wasm_bindgen]
impl WorldLoader {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[wasm_bindgen]
    pub fn load_from_data(&self, data: Vec<u8>) -> Result<JsValue, JsValue> {
        match self.parse_world(data) {
            Ok(world) => Ok(serde_wasm_bindgen::to_value(&world)?),
            Err(e) => Err(JsValue::from_str(&format!("Failed to load world: {}", e))),
        }
    }
}

impl WorldLoader {
    fn parse_world(&self, data: Vec<u8>) -> Result<World, String> {
        let mut stream = DataStream::new(data);

        // 读取文件头
        self.read_header(&mut stream)?;

        // 读取方块数据
        let width = stream.read_int32();
        let height = stream.read_int32();
        let world_id = stream.read_int32();
        let name = stream.read_string();

        let mut tiles = Vec::with_capacity((width * height) as usize);
        for _ in 0..(width * height) {
            tiles.push(self.read_tile(&mut stream));
        }

        // 读取其他数据（简化版）
        let chests = Vec::new();
        let npcs = Vec::new();
        let signs = Vec::new();
        let tile_entities = Vec::new();

        Ok(World {
            name,
            width,
            height,
            world_id,
            tiles,
            chests,
            npcs,
            signs,
            tile_entities,
        })
    }

    fn read_header(&self, stream: &mut DataStream) -> Result<(), String> {
        // 读取版本号和元数据
        let version = stream.read_int32();
        let file_type = stream.read_string();

        if file_type != "terraria" {
            return Err("Invalid world file format".to_string());
        }

        // 读取更多元数据
        stream.skip(8); // 跳过 revision

        Ok(())
    }

    fn read_tile(&self, stream: &mut DataStream) -> Tile {
        let is_active = stream.read_bool();

        if !is_active {
            return Tile {
                tile_id: 0,
                wall_id: 0,
                liquid: 0,
                is_active: false,
                is_actuated: false,
                color: 0,
                u: 0,
                v: 0,
                brick_style: 0,
                full: false,
                half_brick: false,
                slope: 0,
                wire_red: false,
                wire_blue: false,
                wire_green: false,
                wire_yellow: false,
                actuator: false,
                in_active: false,
                wall_color: 0,
                wall_u: 0,
                wall_v: 0,
                wall_full: false,
                wall_half_brick: false,
                wall_slope: 0,
            };
        }

        let tile_id = stream.read_uint16() as i32;
        let wall_id = stream.read_uint16() as i32;

        // 读取液体
        let mut liquid = 0;
        let has_liquid = stream.read_bool();
        if has_liquid {
            liquid = stream.read_uint16() as i32;
        }

        let is_actuated = stream.read_bool();

        let color = if stream.read_bool() {
            stream.read_byte() as i32
        } else {
            0
        };

        let wall_color = if stream.read_bool() {
            stream.read_byte() as i32
        } else {
            0
        };

        // 读取方块样式信息
        let u = stream.read_uint16() as i32;
        let v = stream.read_uint16() as i32;
        let brick_style = stream.read_byte() as i32;

        let full = stream.read_bool();
        let half_brick = stream.read_bool();
        let slope = stream.read_byte() as i32;

        // 读取电线
        let wire_red = stream.read_bool();
        let wire_blue = stream.read_bool();
        let wire_green = stream.read_bool();
        let wire_yellow = stream.read_bool();

        let actuator = stream.read_bool();
        let in_active = stream.read_bool();

        // 读取墙体样式信息
        let wall_u = stream.read_uint16() as i32;
        let wall_v = stream.read_uint16() as i32;
        let wall_full = stream.read_bool();
        let wall_half_brick = stream.read_bool();
        let wall_slope = stream.read_byte() as i32;

        Tile {
            tile_id,
            wall_id,
            liquid,
            is_active,
            is_actuated,
            color,
            u,
            v,
            brick_style,
            full,
            half_brick,
            slope,
            wire_red,
            wire_blue,
            wire_green,
            wire_yellow,
            actuator,
            in_active,
            wall_color,
            wall_u,
            wall_v,
            wall_full,
            wall_half_brick,
            wall_slope,
        }
    }
}