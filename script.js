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

const testMap = {
    data:
        "0007870000000000" +
        "0     6   0    0" +
        "0     8        0" +
        "0     4   0    0" +
        "00000 0   0000 0" +
        "0   0 0   0  0 0" +
        "0   0 0   0  0 0" +
        "0         0  0 0" +
        "0   0 0   0    0" +
        "0000000000000000",
    width: 16,
    height: 10,
    spawnPos: {
        x: gridSize * 2.5,
        y: gridSize * 2.5,
    },
    spawnDir: {
        x: 1,
        y: 0,
    },
};

const wolfMap = {
    data:
        "0000000000000000000000000000000000000000000000000000000000000000" +
        "0000000000000000000000000000000000000000000000000000000000000000" +
        "0000000000000000000000000000000000000000000000000000000000000000" +
        "0000000000000000000000000000000000000000000000000000000000000000" +
        "0000000000000000000000000000000000000000000000000000000000000000" +
        "0000000000000000000000000000000000000000000000000000000000000000" +
        "0000000000000000000000000000bbbbbbbbbbbbbbb000000000000000000000" +
        "0000000000000000000000000000bbb9bbbbb9bbbbb000000000000000000000" +
        "0000000000000000000000000000b           bbbbbb000000000000000000" +
        "000000    0000000001101120109           9bbbbb000000000000000000" +
        "000000    00000001          b           b   bb000000000000000000" +
        "000000     0000000                          9b000000000000000000" +
        "000000     0000001          b           b   bb000000000000000000" +
        "000000150131050000   0002001a           abbbbb000000000000000000" +
        "000001         100   0000000b           bbbbbb000000000000000000" +
        "000002         201   0100000bbb9bb bb9bbbb0000000000000000000000" +
        "000000         1       00000bbbbb   bbbbbb0000000000000000000000" +
        "000001                 0000000bbb   bbbbbb0000000000000000000000" +
        "111100         0       0000000bba   ab00000000000000000000000000" +
        "010102         201050100100000bbb   bb00000000000000000000000000" +
        "1   00         00000000000bbbbbbb   bbbb000000000000000000000000" +
        "0   101500 005100000000000bbbbbbb   bbbb000000000000000000000000" +
        "0   00000   10000000000000bb  9       bb000000000000000000000000" +
        "10 000002   20000000000000bb  bbb   bbbb000000000000000000000000" +
        "0   00100   10000000000000bb  bbb   bbbb000000000000000000000000" +
        "0   1   0   00000000000000bb  bbb   bbbb000000000000000000000000" +
        "0       2   20000000000000bbbbbb9   9b00000000000000000000000000" +
        "0   0   1   00000000000000bbbbbbb   bb00000000000000888888888888" +
        "1   010100 001000000000000000000b9 9b111111100000000877877788878" +
        "2   20         00000000000101021     02010010000000087        78" +
        "1   01         200000000000               118888888887        88" +
        "0   01         000000000005               587778777887        78" +
        "0    0         100000000000                8         7        88" +
        "0              200000000001                                   48" +
        "0    0         000000000001                7         8        88" +
        "0   00         110000000005               5777    8877        78" +
        "1   00         200000000001               11177  77778        88" +
        "0   00         00000000000003010     0130101078  87787        78" +
        "3   301001 00110000000000000000077 877111111078  7788887 87 8788" +
        "0   10000   000000000000000000077   87000000077  878887888888888" +
        "0   00001   100000000000000000078   87000000078  778888888888888" +
        "1   0   0   000000000000000000077   87000000077  877777777777700" +
        "0       2   200000000000000000078   77000000078  778 7 8 8 78700" +
        "1   1   1   100000000000000000078   87000000077   7         7700" +
        "1   00100   100001000000000000077   77000000077             4700" +
        "0   10101   010112010510100000078   78000000078   7         7700" +
        "1                1      2k0000077   8700000007787778 7 7 7 78700" +
        "1                         k000078   8700000007777777777777777700" +
        "0                0      2k0000078   7700000000000000000000000000" +
        "011011011030001000010501001077777   8777777000000000000000000000" +
        "0000000000k2  0  0  00000007787877 77777877000000000000000000000" +
        "000000000k    0  0 100000078    7   7    87000000000000000000000" +
        "0000000000k2     00000000077             87000000000000000000000" +
        "0000000000000000000000000077    7   7    87000000000000000000000" +
        "0000000000000  0000000000078    8   8    87000000000000000000000" +
        "0000000000000  000000000007778778   7777777000000000000000000000" +
        "0000000000000000000000000077    7   7    77000000000000000000000" +
        "0000000000000000000000000078             87000000000000000000000" +
        "0000000000000000000000000077    7   7    77000000000000000000000" +
        "000000000000000000000000007788778   8778777000000000000000000000" +
        "0000000000000000000000000077             77000000000000000000000" +
        "0000000000000000000000000078             77000000000000000000000" +
        "0000000000000000000000000077             77000000000000000000000" +
        "0000000000000000000000000077487478678487477000000000000000000000" +
        "",
    width: 64,
    height: 64,
    spawnPos: {
        x: gridSize * 29.5,
        y: gridSize * 57.5,
    },
    spawnDir: {
        x: 1,
        y: 0,
    },
};

Promise.all([wasm, textures, testMap]).then(([wasm, textures, mapSpec]) => {
    const mod = wasm.instance.exports;
    const canvas = document.getElementById('screen');

    const width  = canvas.width;
    const height = canvas.height;

    const map = mapSpec.data;
    const mapWidth = mapSpec.width, mapHeight = mapSpec.height;

    // Do allocations up front, as they may invalidate mod.memory.buffer
    const screenByteSize = width * height * 4;
    const screenPtr = mod.alloc(screenByteSize);

    const mapByteSize = mapWidth * mapHeight;
    const mapPtr = mod.alloc(mapByteSize);

    const texturesByteSize = textures.width * textures.height * 4;
    const texturesPtr = mod.alloc(texturesByteSize);

    // Data shared between JS and WASM:
    const mapBuf = new Uint8ClampedArray(mod.memory.buffer, mapPtr, mapByteSize);
    for (let i = 0; i < mapByteSize; ++i) {
        let c = map[i].charCodeAt(0);
        if ('0'.charCodeAt(0) <= c && c <= '9'.charCodeAt(0)) {
            mapBuf[i] = c - '0'.charCodeAt(0) + 1;
        } else if ('a'.charCodeAt(0) <= c && c <= 'z'.charCodeAt(0)) {
            mapBuf[i] = c - 'a'.charCodeAt(0) + 10 + 1;
        } else if ('A'.charCodeAt(0) <= c && c <= 'Z'.charCodeAt(0)) {
            mapBuf[i] = c - 'A'.charCodeAt(0) + 36 + 1;
        } else {
            mapBuf[i] = 0;
        }
    }

    const screenBuf = new Uint8ClampedArray(mod.memory.buffer, screenPtr, screenByteSize);
    const img = new ImageData(screenBuf, width, height);

    const texturesBuf = new Uint8ClampedArray(mod.memory.buffer, texturesPtr, texturesByteSize);
    texturesBuf.set(textures.data);

    // --

    const ctx = canvas.getContext('2d');

    const focusPoint = mapSpec.spawnPos;
    const direction = mapSpec.spawnDir;

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
