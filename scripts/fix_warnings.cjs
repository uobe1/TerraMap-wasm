#!/usr/bin/env node
/**
 * 自动修复编译警告
 * 将未使用的变量 initial_position 改为 _initial_position
 */

const fs = require('fs');
const path = require('path');

const filePath = path.join(__dirname, '..', 'rust', 'src', 'world_loader.rs');

console.log('开始修复编译警告...');

if (!fs.existsSync(filePath)) {
  console.error(`错误: 找不到 ${filePath}`);
  process.exit(1);
}

let content = fs.readFileSync(filePath, 'utf8');

// 替换未使用的变量
content = content.replace(/let initial_position = stream\.position\(\);/g, 'let _initial_position = stream.position();');

console.log('✓ 修复了未使用的变量警告');

// 写回文件
fs.writeFileSync(filePath, content, 'utf8');

console.log('✓ 修复完成');