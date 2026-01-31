#!/usr/bin/env node
/**
 * 自动替换 colors.rs 文件的脚本
 * 使用生成的 colors_generated.rs 替换原有的 colors.rs
 */

const fs = require('fs');
const path = require('path');

const colorsGeneratedPath = path.join(__dirname, '..', 'rust', 'src', 'colors_generated.rs');
const colorsPath = path.join(__dirname, '..', 'rust', 'src', 'colors.rs');

console.log('开始替换颜色文件...');

// 检查文件是否存在
if (!fs.existsSync(colorsGeneratedPath)) {
  console.error(`错误: 找不到 ${colorsGeneratedPath}`);
  process.exit(1);
}

// 读取生成的颜色文件
const generatedContent = fs.readFileSync(colorsGeneratedPath, 'utf8');

// 统计颜色数量
const colorMatches = generatedContent.match(/pub fn [a-z_]+\(\) -> Rgb/g);
const colorCount = colorMatches ? colorMatches.length : 0;

console.log(`找到 ${colorCount} 个颜色定义`);

// 备份原有文件（如果存在）
if (fs.existsSync(colorsPath)) {
  const backupPath = colorsPath + '.backup';
  fs.copyFileSync(colorsPath, backupPath);
  console.log(`已备份原有文件到 ${backupPath}`);
}

// 写入新文件
fs.writeFileSync(colorsPath, generatedContent, 'utf8');

console.log(`✓ 成功替换 ${colorsPath}`);
console.log(`✓ 包含 ${colorCount} 个颜色定义`);
console.log('完成！');