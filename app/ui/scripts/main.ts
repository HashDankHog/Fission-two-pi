/* 
It turns out that I just needed to move the draw_rect function to its own section
beforehand each draw_pixel call was taking roughly a millesecond
but since I was doing so many it added up super quickly
now it is taking roughly 400ms to run update_canvas and 1ms to run draw_rect which is a huge improvement,
which is now all the way down to 1ms
*/

import { populateRibbon } from "./modules/ribbon.ts";
import { updateCanvas } from "./modules/viewport.ts";
//import { dragElement } from "./modules/window.js";

type FixX = {
    "FixX": {
        point: number,
        position: number
    }
};
type FixY = { 
    "FixY": {
        point: number,
        position: number
    }
};
type FixZ = { 
    "FixZ": {
        point: number,
        position: number
    }
};
type Distance = { 
    "Distance": {
        point_1: number,
        point_2: number,
        distance: number
    }
};
type Angle = { 
    "Angle": {
        point_1: number,
        point_2: number,
        point_3: number,
        angle: number
    }
};
type Constraint = FixX | FixY | FixZ | Distance | Angle;

const window_tauri: any = window;
const { invoke } = window_tauri.__TAURI__.core;

var c = document.getElementById("viewport_canvas") as HTMLCanvasElement;
var ctx = c.getContext("2d") as CanvasRenderingContext2D;


invoke("set_screen", {width: ctx.canvas.width, height: ctx.canvas.height});


function resizeIframe(obj: HTMLIFrameElement) {
    obj.style.height = (obj.contentWindow as Window).document.documentElement.scrollHeight + 'px';
}

const myRequest = new Request("json/designRibbon.json");

fetch(myRequest)
  .then((response) => response.json())
  .then((data) => {
        populateRibbon(data);
  })
  .catch(console.error);

//code for draggable window, actually buns af and needs to be rewritten
//entirely
//TODO: fix

// Make the DIV element draggable:
dragElement(document.getElementById("window") as HTMLCanvasElement);
var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
var elmnt =document.getElementById("windowHeader") as HTMLCanvasElement;
var win = document.getElementById("window") as HTMLCanvasElement;
elmnt.style.top="0px";
elmnt.style.left="0px";
win.style.top="0px";
win.style.left="0px";
function dragElement(elmnt: HTMLCanvasElement) {
    const body = document.querySelector('body');
    if (document.getElementById(elmnt.className + "Header")) {
        // if present, the header is where you move the DIV from:
        const header = document.getElementById(elmnt.className + "Header") as HTMLCanvasElement;
        header.addEventListener("onmousedown", () => dragMouseDown);
    } else {
        // otherwise, move the DIV from anywhere inside the DIV:
        elmnt.onmousedown = dragMouseDown;
    }
}
function dragMouseDown(evt: MouseEvent) {
    // get the mouse cursor position at startup:
    pos3 = evt.clientX;
    pos4 = evt.clientY;
    document.onmouseup = closeDragElement;
    // call a function whenever the cursor moves:
    document.onmousemove = elementDrag;
}

function elementDrag(evt: MouseEvent) {
    // calculate the new cursor position:
    pos1 = pos3 - evt.clientX;
    pos2 = pos4 - evt.clientY;
    pos3 = evt.clientX;
    pos4 = evt.clientY;
    // set the element's new position:
    elmnt.style.top = (elmnt.offsetTop - pos2) + "px";
    elmnt.style.left = (elmnt.offsetLeft - pos1) + "px";
    win.style.top = (parseInt(win.style.top)-pos2)+"px";
    win.style.left = (parseInt(win.style.left)-pos1)+"px";
}

function closeDragElement() {
    // stop moving when mouse button is released:
    document.onmouseup = null;
    document.onmousemove = null;
}
function closeWindow() {
    const a = document.getElementById("window") as HTMLCanvasElement;
    a.style.top = "-450px";
}

//updateCanvas(ctx);
let i = 100;
async function animate() {
    
    invoke("draw_rect", {coord: [i,i], size: [100, 100], color: [255,255,255]});
    updateCanvas(ctx);
    if (i >= 900){
        i = 0;
    }
    i +=1;
    requestAnimationFrame(animate);
}
//requestAnimationFrame(animate);

function plotToCanvas(){
    invoke("clear_canvas");
    const windowFrame: any = document.getElementById("windowFrame");
    windowFrame.contentWindow.plot();
    updateCanvas(ctx);
}
var x: Constraint = {"FixX": {"point": 0, "position": 100.0}};
var y: Constraint = {"FixY": {"point": 0, "position": 100.0}};
var z: Constraint = {"FixZ": {"point": 0, "position": 0.0}};

closeWindow();
invoke("add_point", {point: [100.0, 100.0, 0.0]});
invoke("add_constraint", {constraint: x });
invoke("add_constraint", {constraint: y });
invoke("add_constraint", {constraint: z });
updateCanvas(ctx);
const windowClose = document.getElementById("windowClose") as HTMLCanvasElement;
const plot = document.getElementById("plot") as HTMLCanvasElement;
windowClose.addEventListener("click", closeWindow);
plot.addEventListener("click", () => requestAnimationFrame(plotToCanvas));


