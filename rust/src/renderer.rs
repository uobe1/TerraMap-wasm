// 渲染器
// 对应原项目的 MapHelper.js 渲染部分

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::world_loader::{World, Tile};
use crate::colors::Rgb;

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    scale: f64,
}

impl Renderer {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Renderer, JsValue> {
        let ctx = canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Failed to get 2d context"))?
            .dyn_into::<CanvasRenderingContext2d>()?;

        // 禁用图像平滑，保持像素风格
        ctx.set_image_smoothing_enabled(false);

        Ok(Self { ctx, scale: 1.0 })
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    pub fn get_scale(&self) -> f64 {
        self.scale
    }

    pub fn render_world_js(&self, world_js: JsValue) -> Result<(), JsValue> {
        let world: World = serde_wasm_bindgen::from_value(world_js)?;
        self.render_world(&world, None, false, None)
    }

    pub fn render_world_with_highlight_js(
        &self,
        world_js: JsValue,
        highlight_positions_js: JsValue,
        highlight_all: bool,
    ) -> Result<(), JsValue> {
        let world: World = serde_wasm_bindgen::from_value(world_js)?;
        let highlight_positions: Option<Vec<(i32, i32)>> =
            if highlight_positions_js.is_undefined() || highlight_positions_js.is_null() {
                None
            } else {
                let positions: Vec<i32> = serde_wasm_bindgen::from_value(highlight_positions_js)?;
                if positions.len() % 2 != 0 {
                    return Err(JsValue::from_str("Invalid highlight positions"));
                }
                Some(
                    positions
                        .chunks(2)
                        .map(|chunk| (chunk[0], chunk[1]))
                        .collect(),
                )
            };
        self.render_world(&world, highlight_positions, highlight_all, None)
    }

    pub fn render_world_visible_js(
        &self,
        world_js: JsValue,
        visible_area_js: JsValue,
        highlight_positions_js: JsValue,
        highlight_all: bool,
    ) -> Result<(), JsValue> {
        let world: World = serde_wasm_bindgen::from_value(world_js)?;
        let visible_area: Option<(i32, i32, i32, i32)> =
            if visible_area_js.is_undefined() || visible_area_js.is_null() {
                None
            } else {
                let area: Vec<i32> = serde_wasm_bindgen::from_value(visible_area_js)?;
                if area.len() != 4 {
                    return Err(JsValue::from_str("Invalid visible area"));
                }
                Some((area[0], area[1], area[2], area[3])) // x, y, width, height
            };

        let highlight_positions: Option<Vec<(i32, i32)>> =
            if highlight_positions_js.is_undefined() || highlight_positions_js.is_null() {
                None
            } else {
                let positions: Vec<i32> = serde_wasm_bindgen::from_value(highlight_positions_js)?;
                if positions.len() % 2 != 0 {
                    return Err(JsValue::from_str("Invalid highlight positions"));
                }
                Some(
                    positions
                        .chunks(2)
                        .map(|chunk| (chunk[0], chunk[1]))
                        .collect(),
                )
            };

        self.render_world(&world, highlight_positions, highlight_all, visible_area)
    }

    pub fn render_world(
        &self,
        world: &World,
        highlight_positions: Option<Vec<(i32, i32)>>,
        highlight_all: bool,
        visible_area: Option<(i32, i32, i32, i32)>,
    ) -> Result<(), JsValue> {
        // 清空画布
        let width = self.ctx.canvas().unwrap().width();
        let height = self.ctx.canvas().unwrap().height();
        self.ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

        // 创建高亮位置集合
        let highlight_set: std::collections::HashSet<(i32, i32)> = highlight_positions
            .unwrap_or_default()
            .into_iter()
            .collect();

        // 确定渲染范围
        let (start_x, start_y, end_x, end_y) = match visible_area {
            Some((x, y, w, h)) => {
                let start_x = x.max(0) as usize;
                let start_y = y.max(0) as usize;
                let end_x = (x + w).min(world.width) as usize;
                let end_y = (y + h).min(world.height) as usize;
                (start_x, start_y, end_x, end_y)
            }
            None => (0, 0, world.width as usize, world.height as usize),
        };

        // 批量渲染方块
        for y in start_y..end_y {
            for x in start_x..end_x {
                let idx = (y * world.width as usize + x) as usize;
                if idx >= world.tiles.len() {
                    continue;
                }

                let tile = &world.tiles[idx];
                if tile.is_active {
                    let is_highlighted = highlight_set.contains(&(x as i32, y as i32));
                    self.render_tile(x as f64, y as f64, tile, is_highlighted, highlight_all)?;
                }
            }
        }

        Ok(())
    }

    pub fn render_tile_js(&self, x: f64, y: f64, tile_js: JsValue) -> Result<(), JsValue> {
        let tile: Tile = serde_wasm_bindgen::from_value(tile_js)?;
        self.render_tile(x, y, &tile, false, false)
    }

    pub fn render_tile(
        &self,
        x: f64,
        y: f64,
        tile: &Tile,
        is_highlighted: bool,
        highlight_all: bool,
    ) -> Result<(), JsValue> {
        // 获取方块颜色
        let color = crate::colors::TileColors::get_color(tile.tile_id);

        // 设置填充颜色
        let color_string = if is_highlighted && highlight_all {
            // 高亮颜色（黄色）
            "rgba(255, 255, 0, 0.5)".to_string()
        } else if is_highlighted {
            // 单个高亮颜色（半透明黄色）
            "rgba(255, 255, 0, 0.3)".to_string()
        } else {
            // 正常颜色
            color.to_css_string()
        };

        self.ctx.set_fill_style_str(&color_string);

        // 绘制方块
        let size = self.scale;
        self.ctx.fill_rect(x * size, y * size, size, size);

        // 如果高亮但不是全部高亮，绘制边框
        if is_highlighted && !highlight_all {
            self.ctx.set_stroke_style_str("rgba(255, 255, 0, 0.8)");
            self.ctx.set_line_width(2.0 / self.scale);
            self.ctx.stroke_rect(x * size, y * size, size, size);
        }

        Ok(())
    }
}