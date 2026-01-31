/**
 * 颜色提取脚本
 * 从 MapHelper.js 中提取所有颜色定义，并转换为 Rust 格式
 */

const fs = require('fs');
const path = require('path');

// 读取 MapHelper.js 文件
const mapHelperPath = path.join(__dirname, '../resources/js/MapHelper.js');
const mapHelperContent = fs.readFileSync(mapHelperPath, 'utf-8');

// 提取所有颜色定义
const tileColors = {};

// 第一遍：提取所有颜色定义（包括变量引用）
const colorPattern = /tileColors\[(\d+)\]\[(\d+)\]\s*=\s*(.+?);/g;
let match;

while ((match = colorPattern.exec(mapHelperContent)) !== null) {
    const id = parseInt(match[1]);
    const variant = parseInt(match[2]);
    const value = match[3].trim();

    if (!tileColors[id]) {
        tileColors[id] = {};
    }
    tileColors[id][variant] = value;
}

// 提取所有变量定义
const variables = {};
const varPattern = /var\s+(\w+)\s*=\s*(.+?);/g;

while ((match = varPattern.exec(mapHelperContent)) !== null) {
    const varName = match[1];
    const value = match[2].trim();
    variables[varName] = value;
}

// 解析颜色值的辅助函数
function parseColorValue(value, visited = new Set()) {
    // 防止循环引用
    if (typeof value !== 'string') {
        if (value && typeof value === 'object' && 'r' in value && 'g' in value && 'b' in value) {
            return value; // 已经是解析后的颜色对象
        }
        return null;
    }
    
    const cacheKey = `${value}`;
    if (visited.has(cacheKey)) {
        return null; // 循环引用
    }
    visited.add(cacheKey);
    
    // 如果是变量引用
    if (variables[value]) {
        return parseColorValue(variables[value], visited);
    }
    
    // 如果是 tileColors 引用
    const tileRefMatch = value.match(/tileColors\[(\d+)\]\[(\d+)\]/);
    if (tileRefMatch) {
        const refId = parseInt(tileRefMatch[1]);
        const refVariant = parseInt(tileRefMatch[2]);
        if (tileColors[refId] && tileColors[refId][refVariant]) {
            return parseColorValue(tileColors[refId][refVariant], visited);
        }
    }
    
    // 如果是 rgb() 函数
    const rgbMatch = value.match(/rgb\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)/);
    if (rgbMatch) {
        return {
            r: parseInt(rgbMatch[1]),
            g: parseInt(rgbMatch[2]),
            b: parseInt(rgbMatch[3])
        };
    }
    
    return null;
}

// 解析所有颜色值
for (const id in tileColors) {
    for (const variant in tileColors[id]) {
        const value = tileColors[id][variant];
        const color = parseColorValue(value);
        if (color) {
            tileColors[id][variant] = color;
        } else {
            console.warn(`无法解析颜色: tileColors[${id}][${variant}] = ${value}`);
        }
    }
}

// 生成 Rust 代码
function generateRustCode(tileColors) {
    let code = '// 颜色定义\n';
    code += '// 从 MapHelper.js 迁移（自动生成）\n\n';

    // Rgb 结构体
    code += '#[derive(Debug, Clone, Copy)]\n';
    code += 'pub struct Rgb {\n';
    code += '    pub r: u8,\n';
    code += '    pub g: u8,\n';
    code += '    pub b: u8,\n';
    code += '}\n\n';

    code += 'impl Rgb {\n';
    code += '    pub const fn new(r: u8, g: u8, b: u8) -> Self {\n';
    code += '        Self { r, g, b }\n';
    code += '    }\n\n';

    code += '    pub fn to_css_string(&self) -> String {\n';
    code += '        format!("rgb({}, {}, {})", self.r, self.g, self.b)\n';
    code += '    }\n';
    code += '}\n\n';

    // TileColors 常量定义
    code += 'pub struct TileColors;\n\n';
    code += 'impl TileColors {\n';

    // 生成 get_color 方法
    code += '    pub fn get_color(tile_id: i32) -> Rgb {\n';
    code += '        match tile_id {\n';

    // 提取所有有颜色定义的方块 ID
    const allIds = Object.keys(tileColors).map(id => parseInt(id)).sort((a, b) => a - b);

    // 为每个 ID 生成匹配分支
    for (const id of allIds) {
        if (tileColors[id] && tileColors[id][0]) {
            const color = tileColors[id][0];
            code += `            ${id} => Rgb::new(${color.r}, ${color.g}, ${color.b}),\n`;
        }
    }

    code += '            _ => Rgb::new(0, 0, 0),\n';
    code += '        }\n';
    code += '    }\n';

    // 生成 get_color_variant 方法，用于获取特定变体的颜色
    code += '\n';
    code += '    pub fn get_color_variant(tile_id: i32, variant: i32) -> Option<Rgb> {\n';
    code += '        match tile_id {\n';

    for (const id of allIds) {
        if (tileColors[id]) {
            code += `            ${id} => {\n`;
            code += `                match variant {\n`;
            
            // 提取该方块的所有变体
            const variants = Object.keys(tileColors[id]).map(v => parseInt(v)).sort((a, b) => a - b);
            
            for (const variant of variants) {
                const color = tileColors[id][variant];
                code += `                    ${variant} => Some(Rgb::new(${color.r}, ${color.g}, ${color.b})),\n`;
            }
            
            code += '                    _ => None,\n';
            code += '                }\n';
            code += '            }\n';
        }
    }

    code += '            _ => None,\n';
    code += '        }\n';
    code += '    }\n';
    code += '}\n';

    return code;
}

// 生成并输出 Rust 代码
const rustCode = generateRustCode(tileColors);

// 保存到文件
const outputPath = path.join(__dirname, '../rust/src/colors_generated.rs');
fs.writeFileSync(outputPath, rustCode);

console.log('颜色定义已提取并保存到:', outputPath);
console.log('提取的颜色数量:', Object.keys(tileColors).length);

// 统计信息
let totalVariants = 0;
for (const id in tileColors) {
    for (const variant in tileColors[id]) {
        totalVariants++;
    }
}

// 检查哪些 ID 在 0-752 范围内但没有颜色定义
const missingIds = [];
for (let i = 0; i < 753; i++) {
    if (!tileColors[i]) {
        missingIds.push(i);
    }
}

console.log('总颜色变体数量:', totalVariants);
console.log('缺失颜色定义的 ID (前20个):', missingIds.slice(0, 20));
console.log('缺失颜色定义的总数:', missingIds.length);

// 检查是否有颜色值为 (0, 0, 0) 的
let zeroColors = 0;
for (const id in tileColors) {
    for (const variant in tileColors[id]) {
        const color = tileColors[id][variant];
        if (color.r === 0 && color.g === 0 && color.b === 0) {
            zeroColors++;
        }
    }
}
console.log('纯黑色 (0,0,0) 的颜色数量:', zeroColors);