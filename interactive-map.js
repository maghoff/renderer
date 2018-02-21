'use strict';

const OPEN = 0;
const WALL = 1;

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

    node.addEventListener("click", ev => {
        ev.preventDefault();
        ev.stopPropagation();
    });
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

function initCamera(cameraDom, initialState, callback) {
    const arrowSize = 64;

    const focusPoint = {
        x: initialState.focusPoint.x,
        y: initialState.focusPoint.y,
    };

    let direction = {
        x: initialState.direction.x,
        y: initialState.direction.y,
    };

    const targetPoint = {
        x: focusPoint.x + direction.x * 3 * gridSize,
        y: focusPoint.y + direction.y * 3 * gridSize,
    };

    const dom = {
        focus: cameraDom.querySelector(".camera--focus"),
        target: cameraDom.querySelector(".camera--target"),
        sightline: cameraDom.querySelector(".camera--sightline"),
        direction: cameraDom.querySelector(".camera--direction"),
        fovLeft: cameraDom.querySelector(".camera--fov--left"),
        fovRight: cameraDom.querySelector(".camera--fov--right"),
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

        const offset = {
            x: direction.x * arrowSize,
            y: direction.y * arrowSize,
        };

        dom.direction.setAttribute("x1", focusPoint.x);
        dom.direction.setAttribute("y1", focusPoint.y);
        dom.direction.setAttribute("x2", focusPoint.x + offset.x);
        dom.direction.setAttribute("y2", focusPoint.y + offset.y);

        const TAU = Math.PI * 2;
        const projection_plane_width = 320.;
        const fov = 60. * TAU / 360.;
        const projection_plane_half_width = projection_plane_width / 2.;
        const distance_to_projection_plane = projection_plane_half_width / Math.tan(fov / 2.);

        const side = {
            x: -direction.y,
            y: direction.x,
        }

        dom.fovLeft.setAttribute("x1", focusPoint.x);
        dom.fovLeft.setAttribute("y1", focusPoint.y);
        dom.fovLeft.setAttribute("x2", focusPoint.x + direction.x * distance_to_projection_plane - side.x * projection_plane_half_width);
        dom.fovLeft.setAttribute("y2", focusPoint.y + direction.y * distance_to_projection_plane - side.y * projection_plane_half_width);

        dom.fovRight.setAttribute("x1", focusPoint.x);
        dom.fovRight.setAttribute("y1", focusPoint.y);
        dom.fovRight.setAttribute("x2", focusPoint.x + direction.x * distance_to_projection_plane + side.x * projection_plane_half_width);
        dom.fovRight.setAttribute("y2", focusPoint.y + direction.y * distance_to_projection_plane + side.y * projection_plane_half_width);
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

    dom.focus.setAttribute("cx", focusPoint.x);
    dom.focus.setAttribute("cy", focusPoint.y);
    dom.sightline.setAttribute("x1", focusPoint.x);
    dom.sightline.setAttribute("y1", focusPoint.y);

    dom.target.setAttribute("cx", targetPoint.x);
    dom.target.setAttribute("cy", targetPoint.y);
    dom.sightline.setAttribute("x2", targetPoint.x);
    dom.sightline.setAttribute("y2", targetPoint.y);

    updateDirection();
}

function drawMap(dom, map) {
    for (let y = 0; y < map.height; ++y) {
        const row = document.createElementNS("http://www.w3.org/2000/svg", "g");
        row.setAttribute("class", "map--row");
        dom.appendChild(row);

        for (let x = 0; x < map.width; ++x) {
            const cellClass = map.data[y*map.width + x] != OPEN ? "map--cell--wall" : "map--cell--open";

            const el = document.createElementNS("http://www.w3.org/2000/svg", "rect");
            el.setAttribute("x", gridSize * x);
            el.setAttribute("y", gridSize * y);
            el.setAttribute("width", gridSize);
            el.setAttribute("height", gridSize);
            el.setAttribute("class", "map--cell " + cellClass);
            row.appendChild(el);
        }
    }
}

function drawGrid(dom, map) {
    for (let x = 0; x < map.width; ++x) {
        const el = document.createElementNS("http://www.w3.org/2000/svg", "line");
        el.setAttribute("x1", gridSize * x);
        el.setAttribute("y1", gridSize * 0);
        el.setAttribute("x2", gridSize * x);
        el.setAttribute("y2", gridSize * map.height);
        dom.appendChild(el);
    }
    for (let y = 0; y < map.height; ++y) {
        const el = document.createElementNS("http://www.w3.org/2000/svg", "line");
        el.setAttribute("x1", gridSize * 0);
        el.setAttribute("y1", gridSize * y);
        el.setAttribute("x2", gridSize * map.width);
        el.setAttribute("y2", gridSize * y);
        dom.appendChild(el);
    }
}

function mapEditor(dom, map, writeMap) {
    dom.addEventListener("click", ev => {
        ev.preventDefault();
        ev.stopPropagation();

        const tr = svgPoint(dom, ev.x, ev.y);
        const cell = {
            x: (tr.x / gridSize) | 0,
            y: (tr.y / gridSize) | 0,
        };

        const prev = map.data[cell.y * map.width + cell.x];
        const next = prev == WALL ? OPEN : WALL;
        writeMap(cell, next);
    });
}

function interactiveMap(svg, map, camera, updateCamera, writeMap) {
    drawMap(svg.querySelector(".map"), map);
    drawGrid(svg.querySelector(".grid"), map);

    mapEditor(svg, map, function (cell, value) {
        const cellClass = value == WALL ? "map--cell--wall" : "map--cell--open";
        const sq = svg.querySelector(`.map>:nth-child(${cell.y + 1})>:nth-child(${cell.x + 1})`);
        sq.setAttribute("class", "map--cell " + cellClass);

        writeMap(cell, value);
    });

    initCamera(
        svg.querySelector(".camera"),
        camera,
        updateCamera
    );
}
