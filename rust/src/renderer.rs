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
        self.render_world(&world)
    }

    pub fn render_world(&self, world: &World) -> Result<(), JsValue> {
        // 清空画布
        let width = self.ctx.canvas().unwrap().width();
        let height = self.ctx.canvas().unwrap().height();
        self.ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

        // 批量渲染方块
        for y in 0..world.height {
            for x in 0..world.width {
                let idx = (y * world.width + x) as usize;
                if idx >= world.tiles.len() {
                    continue;
                }

                let tile = &world.tiles[idx];
                if tile.is_active {
                    self.render_tile(x as f64, y as f64, tile)?;
                }
            }
        }

        Ok(())
    }

    pub fn render_tile_js(&self, x: f64, y: f64, tile_js: JsValue) -> Result<(), JsValue> {
        let tile: Tile = serde_wasm_bindgen::from_value(tile_js)?;
        self.render_tile(x, y, &tile)
    }

    pub fn render_tile(&self, x: f64, y: f64, tile: &Tile) -> Result<(), JsValue> {
        // 获取方块颜色
        let color = crate::colors::TileColors::get_color(tile.tile_id);

        // 设置填充颜色
        let color_string = color.to_css_string();
        self.ctx.set_fill_style_str(&color_string);

        // 绘制方块
        let size = self.scale;
        self.ctx.fill_rect(x * size, y * size, size, size);

        Ok(())
    }
}