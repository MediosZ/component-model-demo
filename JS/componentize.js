import { componentize } from '@bytecodealliance/componentize-js';
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const jsSource = await readFile('bundle/index.bundled.js', 'utf8');

const { component } = await componentize(jsSource, {
    witPath: resolve('../wit/md-renderer.wit'),
    disableFeatures: ['stdio', 'random', 'clocks', 'http']
});

await writeFile('dist/mdrenderer.wasm', component);

console.log('mdrenderer.wasm generated successfully!');