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

    const focusPoint = {
        x: gridSize * 4.5,
        y: gridSize * 6.5,
    };

    const direction = {
        x: 0,
        y: -1,
    };

    let pendingRender = false;
    function render(timestamp) {
        pendingRender = false;

        mod.fill(
            pointer, width, height,
            focusPoint.x, focusPoint.y,
            direction.x, direction.y
        );
        ctx.putImageData(img, 0, 0);
    }

    function scheduleRender() {
        if (pendingRender) return;
        pendingRender = true;
        window.requestAnimationFrame(render);
    }

    function updateCamera(newFocusPoint, newDirection) {
        focusPoint.x = newFocusPoint.x;
        focusPoint.y = newFocusPoint.y;
        direction.x = newDirection.x;
        direction.y = newDirection.y;
        scheduleRender();
    };

    interactiveMap(document.querySelector("svg"), updateCamera);
    scheduleRender();
})
.catch(alert);
