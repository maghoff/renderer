'use strict';

// Fetch and instantiate our wasm module
fetch("rust.wasm").then(response =>
    response.arrayBuffer()
).then(bytes =>
    WebAssembly.instantiate(bytes, {
        env: {
            cos: Math.cos,
            sin: Math.sin,
            Math_tan: Math.tan,
        }
    })
).then(results => {
    const mod = results.instance.exports;
    const canvas = document.getElementById('screen');

    const width  = canvas.width;
    const height = canvas.height;
    const byteSize = width * height * 4;

    // Create a buffer that's shared between JS and WASM:
    const pointer = mod.alloc(byteSize);
    const buffer = new Uint8ClampedArray(mod.memory.buffer, pointer, byteSize);

    const ctx = canvas.getContext('2d');
    const img = new ImageData(buffer, width, height);

    function step(timestamp) {
        mod.fill(pointer, width, height, timestamp/1000);
        ctx.putImageData(img, 0, 0);
        window.requestAnimationFrame(step);
    }

    step(0);
})
.catch(alert);
