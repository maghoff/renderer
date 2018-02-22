'use strict';

function loadImage(src) {
    return new Promise((resolve, reject) => {
        const img = new Image();
        img.onload = () => resolve(img);
        img.onerror = reject;
        img.src = src;
    });
}

function fpsControls(dom, pos, dir, update) {
    let active = false;

    dom.addEventListener("click", ev => {
        ev.preventDefault();
        ev.stopPropagation();
        dom.requestPointerLock();
    });

    function mousemove(ev) {
        ev.preventDefault();
        ev.stopPropagation();

        const d = dir();
        const ang = ev.movementX / 90;

        update(
            pos(),
            {
                x: d.x*Math.cos(ang) - d.y*Math.sin(ang),
                y: d.x*Math.sin(ang) + d.y*Math.cos(ang),
            }
        )
    }

    const held = {
        'w': false,
        's': false,
        'a': false,
        'd': false,
    };

    let animating = false;
    let prevTimer = null;
    function animate() {
        if (animating) return;
        animating = true;
        prevTimer = performance.now();
        requestAnimationFrame(animationFrame);
    }

    function animationFrame(timer) {
        const fwd = (held['w'] ? 1 : 0) + (held['s'] ? -1 : 0);
        const rig = (held['d'] ? 1 : 0) + (held['a'] ? -1 : 0);
        if (fwd == 0 && rig == 0) {
            animating = false;
            return;
        }

        const dt = timer - prevTimer;
        const l = dt * 0.3;

        const p = pos();
        const d = dir();
        const s = { x: -d.y, y: d.x };
        update(
            {
                x: p.x + fwd * l * d.x + rig * l * s.x,
                y: p.y + fwd * l * d.y + rig * l * s.y,
            },
            d
        )

        prevTimer = timer;
        requestAnimationFrame(animationFrame);
    }

    function keydown(ev) {
        const k = ev.key.toLowerCase();
        if (k != 'w' && k != 's' && k != 'a' && k != 'd') return;

        ev.preventDefault();
        ev.stopPropagation();

        held[k] = true;

        animate();
    }

    function keyup(ev) {
        const k = ev.key.toLowerCase();
        if (k != 'w' && k != 's' && k != 'a' && k != 'd') return;

        ev.preventDefault();
        ev.stopPropagation();

        held[k] = false;

        animate();
    }

    function lockChangeAlert() {
        const el = document.pointerLockElement || document.mozPointerLockElement;

        if (el === dom) {
            document.addEventListener("mousemove", mousemove, false);
            document.addEventListener("keydown", keydown, false);
            document.addEventListener("keyup", keyup, false);
        } else {
            document.removeEventListener("mousemove", mousemove, false);
            document.removeEventListener("keydown", keydown, false);
            document.removeEventListener("keyup", keyup, false);
            held['w'] = held['s'] = held['a'] = held['d'] = false;
        }
    }

    document.addEventListener('pointerlockchange', lockChangeAlert, false);
    document.addEventListener('mozpointerlockchange', lockChangeAlert, false);
}

const wasm =
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
    );

const textures = loadImage("textures.png")
    .then(img => {
        const canvas = document.createElement("canvas");
        canvas.width = img.width;
        canvas.height = img.height;
        const ctx = canvas.getContext("2d");
        ctx.drawImage(img, 0, 0);
        return ctx.getImageData(0, 0, img.width, img.height);
    });

Promise.all([wasm, textures]).then(([wasm, textures]) => {
    const mod = wasm.instance.exports;
    const canvas = document.getElementById('screen');

    const width  = canvas.width;
    const height = canvas.height;

    const map =
        "xxxxxxxxxxxxxxxx" +
        "x   x x   x    x" +
        "x     x        x" +
        "x   x x   x    x" +
        "xxxxx x   xxxx x" +
        "x   x x   x  x x" +
        "x   x x   x  x x" +
        "x         x  x x" +
        "x   x x   x    x" +
        "xxxxxxxxxxxxxxxx";
    const mapWidth = 16, mapHeight = 10;

    // Do allocations up front, as they may invalidate mod.memory.buffer
    const screenByteSize = width * height * 4;
    const screenPtr = mod.alloc(screenByteSize);

    const mapByteSize = mapWidth * mapHeight;
    const mapPtr = mod.alloc(mapByteSize);

    const texturesByteSize = textures.width * textures.height * 4;
    const texturesPtr = mod.alloc(texturesByteSize);

    // Data shared between JS and WASM:
    const mapBuf = new Uint8ClampedArray(mod.memory.buffer, mapPtr, mapByteSize);
    for (let i = 0; i < mapByteSize; ++i) mapBuf[i] = (map[i] == 'x') ? 1 : 0;

    const screenBuf = new Uint8ClampedArray(mod.memory.buffer, screenPtr, screenByteSize);
    const img = new ImageData(screenBuf, width, height);

    const texturesBuf = new Uint8ClampedArray(mod.memory.buffer, texturesPtr, texturesByteSize);
    texturesBuf.set(textures.data);

    // --

    const ctx = canvas.getContext('2d');

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
            texturesPtr, textures.width, textures.height,
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

    let mapApi = interactiveMap(
        document.querySelector("svg"),
        {
            data: mapBuf,
            width: mapWidth,
            height: mapHeight,
        },
        {
            focusPoint,
            direction
        },
        updateCamera,
        writeMap,
    );

    fpsControls(
        canvas,
        () => focusPoint,
        () => direction,
        (focusPoint, direction) => {
            mapApi.updateCamera(focusPoint, direction);
            updateCamera(focusPoint, direction);
        }
    );

    scheduleRender();
})
.catch(alert);
