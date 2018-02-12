'use strict';

const gridSize = 64;

// https://www.sitepoint.com/how-to-translate-from-dom-to-svg-coordinates-and-back-again/
// translate page to SVG co-ordinate
function svgPoint(element, x, y) {
    const svg = document.querySelector("svg");
    const pt = svg.createSVGPoint();

    pt.x = x;
    pt.y = y;

    return pt.matrixTransform(element.getScreenCTM().inverse());
}

function draggable(node, callback) {
    let dragging = false;

    node.addEventListener("mousedown", ev => {
        ev.preventDefault();
        ev.stopPropagation();

        node.classList.add("drag");
        node.setCapture(true);
        dragging = true;
    });
    node.addEventListener("mousemove", ev => {
        if (!dragging) return;
        ev.preventDefault();
        ev.stopPropagation();

        const tr = svgPoint(node, ev.x, ev.y);
        callback(tr.x, tr.y);
    });
    node.addEventListener("mouseup", ev => {
        if (!dragging) return;
        ev.preventDefault();
        ev.stopPropagation();

        node.classList.remove("drag");
        dragging = false;
    });
}

function initCamera(cameraDom, callback) {
    const focusPoint = {
        x: gridSize * 4.5,
        y: gridSize * 6.5,
    };

    const targetPoint = {
        x: gridSize * 4.5,
        y: gridSize * 3.5,
    };

    let direction = {
        x: 0,
        y: -1,
    };

    const dom = {
        focus: cameraDom.querySelector(".camera--focus"),
        target: cameraDom.querySelector(".camera--target"),
        sightline: cameraDom.querySelector(".camera--sightline"),
        direction: cameraDom.querySelector(".camera--direction"),
    };

    function updateDirection() {
        const dirVec = {
            x: targetPoint.x - focusPoint.x,
            y: targetPoint.y - focusPoint.y,
        };
        const len = Math.sqrt(dirVec.x * dirVec.x + dirVec.y * dirVec.y);
        direction = {
            x: dirVec.x / len,
            y: dirVec.y / len,
        };

        const scale = 64;
        const offset = {
            x: direction.x * scale,
            y: direction.y * scale,
        };

        dom.direction.setAttribute("x1", focusPoint.x);
        dom.direction.setAttribute("y1", focusPoint.y);
        dom.direction.setAttribute("x2", focusPoint.x + offset.x);
        dom.direction.setAttribute("y2", focusPoint.y + offset.y);
    }

    draggable(cameraDom.querySelector(".camera--target"), (x, y) => {
        targetPoint.x = x;
        targetPoint.y = y;

        dom.target.setAttribute("cx", x);
        dom.target.setAttribute("cy", y);
        dom.sightline.setAttribute("x2", x);
        dom.sightline.setAttribute("y2", y);
        updateDirection();

        callback(focusPoint, direction);
    });

    draggable(cameraDom.querySelector(".camera--focus"), (x, y) => {
        focusPoint.x = x;
        focusPoint.y = y;

        dom.focus.setAttribute("cx", x);
        dom.focus.setAttribute("cy", y);
        dom.sightline.setAttribute("x1", x);
        dom.sightline.setAttribute("y1", y);
        updateDirection();

        callback(focusPoint, direction);
    });
}

function drawMap(dom) {
    const map =
        "xxxxxxxxxx" +
        "x   x    x" +
        "x      x x" +
        "x        x" +
        "x        x" +
        "x x      x" +
        "x        x" +
        "x        x" +
        "x        x" +
        "x        x" +
        "x        x" +
        "x      x x" +
        "xx       x" +
        "xxxxxxxxxx";
    const w = 10, h = 14;

    for (let y = 0; y < h; ++y) {
        for (let x = 0; x < w; ++x) {
            const el = document.createElementNS("http://www.w3.org/2000/svg", "rect");
            el.setAttribute("x", gridSize * x);
            el.setAttribute("y", gridSize * y);
            el.setAttribute("width", gridSize);
            el.setAttribute("height", gridSize);
            el.setAttribute("stroke", "none");
            el.setAttribute("fill", map[y*w + x] == "x" ? "#00ff00" : "transparent");
            dom.appendChild(el);
        }
    }
}

function interactiveMap(svg, updateCamera) {
    drawMap(svg.querySelector(".map"));

    initCamera(
        svg.querySelector(".camera"),
        updateCamera
    );
}
