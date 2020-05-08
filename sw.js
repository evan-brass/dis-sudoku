const src = './wasm/debug/service_worker.js';
importScripts(src);

wasm_bindgen(src.replace(/\.js$/, '_bg.wasm'));