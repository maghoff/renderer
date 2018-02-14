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

    const map =
        "xxxxxxxxxxxxxxxx" +
        "x   x          x" +
        "x              x" +
        "x   x          x" +
        "xxxxx          x" +
        "x              x" +
        "x              x" +
        "x              x" +
        "x              x" +
        "x              x" +
        "x              x" +
        "x              x" +
        "x              x" +
        "xxxxxxxxxxxxxxxx";
    const mapWidth = 16, mapHeight = 14;

    // Do allocations up front, as they may invalidate mod.memory.buffer
    const screenByteSize = width * height * 4;
    const screenPtr = mod.alloc(screenByteSize);

    const mapByteSize = mapWidth * mapHeight;
    const mapPtr = mod.alloc(mapByteSize);

    // Data shared between JS and WASM:
    const mapBuf = new Uint8ClampedArray(mod.memory.buffer, mapPtr, mapByteSize);
    for (let i = 0; i < mapByteSize; ++i) mapBuf[i] = map.charCodeAt(i);

    const screenBuf = new Uint8ClampedArray(mod.memory.buffer, screenPtr, screenByteSize);

    // --

    const ctx = canvas.getContext('2d');
    const img = new ImageData(screenBuf, width, height);

    const focusPoint = {
        x: gridSize * 2.5,
        y: gridSize * 2.5,
    };

    const direction = {
        x: 1,
        y: 0,
    };

    let pendingRender = false;
    function render(timestamp) {
        pendingRender = false;

        mod.fill(
            mapPtr, mapWidth, mapHeight,
            screenPtr, width, height,
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
    }

    function writeMap(cell, value) {
        mapBuf[cell.y * mapWidth + cell.x] = value;
        scheduleRender();
    }

    interactiveMap(
        document.querySelector("svg"),
        {
            data: mapBuf,
            width: mapWidth,
            height: mapHeight,
        },
        updateCamera,
        writeMap,
    );
    scheduleRender();
})
.catch(alert);
