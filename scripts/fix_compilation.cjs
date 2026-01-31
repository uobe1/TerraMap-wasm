#!/usr/bin/env node
/**
 * 自动修复编译错误
 * 将 stream.buffer.len() 替换为 stream.len()
 */

const fs = require('fs');
const path = require('path');

const filePath = path.join(__dirname, '..', 'rust', 'src', 'world_loader.rs');

console.log('开始修复编译错误...');

if (!fs.existsSync(filePath)) {
  console.error(`错误: 找不到 ${filePath}`);
  process.exit(1);
}

let content = fs.readFileSync(filePath, 'utf8');

// 统计替换次数
let count = 0;

// 替换 stream.buffer.len() 为 stream.len()
content = content.replace(/stream\.buffer\.len\(\)/g, () => {
  count++;
  return 'stream.len()';
});

console.log(`修复了 ${count} 处 stream.buffer.len() 的调用`);

// 写回文件
fs.writeFileSync(filePath, content, 'utf8');

console.log('✓ 修复完成');