// Simple test script for WASM module
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function testWasmModule() {
  console.log('Testing WASM module...');

  try {
    // Read test world file
    const testFilePath = path.join(__dirname, 'test.wld');
    if (!fs.existsSync(testFilePath)) {
      console.error('Test file not found:', testFilePath);
      return;
    }

    console.log('Reading test world file...');
    const fileBuffer = fs.readFileSync(testFilePath);
    console.log('File size:', fileBuffer.length, 'bytes');

    // Try to load WASM module
    console.log('Loading WASM module...');
    const wasm = await import('./pkg/terra_map_wasm.js');

    // Load WASM file using initSync for Node.js environment
    const wasmFilePath = path.join(__dirname, 'pkg', 'terra_map_wasm_bg.wasm');
    const wasmBuffer = fs.readFileSync(wasmFilePath);
    wasm.initSync(wasmBuffer);

    console.log('WASM module loaded successfully');
    console.log('Available exports:', Object.keys(wasm));

    // Test WorldLoader
    console.log('\nTesting WorldLoader...');
    const loader = new wasm.WorldLoader();
    const world = loader.load_from_data(new Uint8Array(fileBuffer));

    console.log('World loaded successfully!');
    console.log('World name:', world.name);
    console.log('World id:', world.world_id);
    console.log('World dimensions:', world.width, 'x', world.height);
    console.log('Number of tiles:', world.tiles.length);
    console.log('Number of chests:', world.chests.length);
    console.log('Number of NPCs:', world.npcs.length);

    // 检查是否有任何数据
    if (world.width === 0 || world.height === 0) {
      console.warn('Warning: World dimensions are zero!');
    }
    if (world.tiles.length === 0) {
      console.warn('Warning: No tiles loaded!');
    }
    if (!world.name) {
      console.warn('Warning: World name is empty!');
    }

    // Test Searcher
    console.log('\nTesting Searcher...');
    const searcher = new wasm.Searcher(world);

    // Find some common tiles
    const tiles = searcher.find_tiles([0, 1, 2]);
    console.log('Found tiles:', tiles.length, 'positions');

    // Test Renderer (mock)
    console.log('\nTesting Renderer...');
    console.log('Renderer class available:', typeof wasm.Renderer === 'function');

    console.log('\n✅ All tests passed!');
  } catch (error) {
    console.error('\n❌ Test failed:', error);
    process.exit(1);
  }
}

testWasmModule().then(() => process.exit(0));