// 世界数据类型定义
export interface Tile {
  tile_id: number;
  wall_id: number;
  liquid: number;
  is_active: boolean;
  is_actuated: boolean;
  color: number;
  u: number;
  v: number;
  brick_style: number;
  full: boolean;
  half_brick: boolean;
  slope: number;
  wire_red: boolean;
  wire_blue: boolean;
  wire_green: boolean;
  wire_yellow: boolean;
  actuator: boolean;
  in_active: boolean;
  wall_color: number;
  wall_u: number;
  wall_v: number;
  wall_full: boolean;
  wall_half_brick: boolean;
  wall_slope: number;
}

export interface ChestItem {
  id: number;
  stack: number;
  prefix: number;
}

export interface Chest {
  x: number;
  y: number;
  name: string;
  items: ChestItem[];
}

export interface NPC {
  id: number;
  name: string;
  sprite_id: number;
  position_x: number;
  position_y: number;
  home_x: number;
  home_y: number;
  direction: number;
  is_homeless: boolean;
}

export interface World {
  name: string;
  width: number;
  height: number;
  world_id: number;
  tiles: Tile[];
  chests: Chest[];
  npcs: NPC[];
  signs: any[];
  tile_entities: any[];
  // ... 其他字段
}