// 世界文件加载器
// 对应原项目的 WorldLoader.js

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::data_stream::DataStream;

// 错误类型
#[derive(Debug)]
pub enum WorldLoadError {
    InvalidData { message: String },
    UnsupportedVersion { version: i32 },
    CorruptedData { position: usize, message: String },
    InvalidFormat { expected: String, found: String },
}

impl std::fmt::Display for WorldLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldLoadError::InvalidData { message } => {
                write!(f, "Invalid data: {}", message)
            }
            WorldLoadError::UnsupportedVersion { version } => {
                write!(f, "Unsupported world version: {}", version)
            }
            WorldLoadError::CorruptedData { position, message } => {
                write!(f, "Corrupted data at position {}: {}", position, message)
            }
            WorldLoadError::InvalidFormat { expected, found } => {
                write!(f, "Invalid format: expected '{}', found '{}'", expected, found)
            }
        }
    }
}

impl std::error::Error for WorldLoadError {}

impl From<WorldLoadError> for String {
    fn from(error: WorldLoadError) -> Self {
        error.to_string()
    }
}

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
        // 验证数据不为空
        if data.is_empty() {
            return Err(WorldLoadError::InvalidData {
                message: "World file is empty".to_string(),
            }.into());
        }

        let mut stream = DataStream::new(data);
        let _initial_position = stream.position();

        // 读取文件格式头
        let _positions = self.read_file_format_header(&mut stream).map_err(|e| {
            format!("Failed to read file format header: {}", e)
        })?;

        // 读取世界头
        let (width, height, world_id, name) = self.read_header(&mut stream).map_err(|e| {
            format!("Failed to read world header: {}", e)
        })?;

        // 验证世界尺寸
        if width <= 0 || height <= 0 {
            return Err(WorldLoadError::InvalidData {
                message: format!("Invalid world dimensions: {} x {}", width, height),
            }.into());
        }

        if width > 10000 || height > 10000 {
            return Err(WorldLoadError::InvalidData {
                message: format!("World dimensions too large: {} x {}", width, height),
            }.into());
        }

        // 验证数据长度是否足够
        let expected_tile_count = (width * height) as usize;
        let remaining_bytes = stream.len() - stream.position();
        let estimated_tile_size = 8; // 估算每个方块的平均大小（字节）
        let expected_min_bytes = expected_tile_count * estimated_tile_size;

        if remaining_bytes < expected_min_bytes {
            return Err(WorldLoadError::CorruptedData {
                position: stream.position(),
                message: format!(
                    "Insufficient data for tiles: expected at least {} bytes, found {} bytes",
                    expected_min_bytes, remaining_bytes
                ),
            }.into());
        }

        // 读取方块数据
        let mut tiles = Vec::with_capacity(expected_tile_count);
        for i in 0..(width * height) {
            let tile_pos = stream.position();
            tiles.push(self.read_tile(&mut stream));
            
            // 定期检查是否有足够数据
            if i % 1000000 == 0 {
                let remaining = stream.len() - stream.position();
                let tiles_remaining = (width * height - i) as usize;
                if remaining < tiles_remaining {
                    return Err(WorldLoadError::CorruptedData {
                        position: tile_pos,
                        message: format!(
                            "Unexpected end of data while reading tiles: {} tiles remaining but only {} bytes left",
                            tiles_remaining, remaining
                        ),
                    }.into());
                }
            }
        }

        // 读取其他数据（简化版）
        let chests: Vec<Chest> = Vec::new();
        let npcs: Vec<NPC> = Vec::new();
        let signs: Vec<Sign> = Vec::new();
        let tile_entities: Vec<TileEntity> = Vec::new();

        // 验证读取的方块数量
        if tiles.len() != expected_tile_count {
            return Err(WorldLoadError::CorruptedData {
                position: stream.position(),
                message: format!(
                    "Tile count mismatch: expected {}, found {}",
                    expected_tile_count, tiles.len()
                ),
            }.into());
        }

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

    fn read_file_format_header(&self, stream: &mut DataStream) -> Result<Vec<i32>, String> {
        // 读取版本号
        let _version = stream.read_int32();

        // read file metadata
        // TODO: implement read_uint64()
        let _meta1 = stream.read_uint32();
        let _meta2 = stream.read_uint32();

        // revision
        let _revision = stream.read_uint32();

        // isFavorite
        let _fav1 = stream.read_uint32();
        let _fav2 = stream.read_uint32();

        // read positions
        let positions_length = stream.read_int16();
        let mut positions = Vec::with_capacity(positions_length as usize);
        for _ in 0..positions_length {
            positions.push(stream.read_int32());
        }

        // read importance
        let importance_length = stream.read_int16();

        // 重要性位图处理：每 7 个重要性位存储在一个字节中
        let mut _b = 0u8;
        let mut b2 = 128u8;
        for _ in 0..importance_length {
            if b2 == 128 {
                _b = stream.read_byte();
                b2 = 1;
            } else {
                b2 = b2 << 1;
            }
            // 不存储重要性值，只读取
        }

        Ok(positions)
    }

    fn read_header(&self, stream: &mut DataStream) -> Result<(i32, i32, i32, String), String> {
// name
        let name = stream.read_string();

        // seed
        let _seed = stream.read_string();

        // worldGeneratorVersion
        let _ver1 = stream.read_uint32();
        let _ver2 = stream.read_uint32();

        // UUID (16 bytes)
        let mut _uuid = Vec::new();
        for _ in 0..16 {
            _uuid.push(stream.read_byte());
        }

        // id
        let world_id = stream.read_int32();

        // bounds
        let _left = stream.read_int32();
        let _right = stream.read_int32();
        let _top = stream.read_int32();
        let _bottom = stream.read_int32();

        // height
        let height = stream.read_int32();

        // width
        let width = stream.read_int32();

        // gameMode (version >= 209)
        let _game_mode = stream.read_int32();

        Ok((width, height, world_id, name))
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