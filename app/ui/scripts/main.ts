/* 
It turns out that I just needed to move the draw_rect function to its own section
beforehand each draw_pixel call was taking roughly a millesecond
but since I was doing so many it added up super quickly
now it is taking roughly 400ms to run update_canvas and 1ms to run draw_rect which is a huge improvement,
which is now all the way down to 1ms
*/

import { populateRibbon } from "./modules/ribbon.ts";
import { updateCanvas } from "./modules/viewport.ts";
import { Constraint } from "./modules/constraint.ts";
import { openWindow } from "./modules/window.ts";
const window_tauri: any = window;
const { invoke } = window_tauri.__TAURI__.core;

var c = document.getElementById("viewport_canvas") as HTMLCanvasElement;
var ctx = c.getContext("2d") as CanvasRenderingContext2D;
invoke("set_screen", {width: ctx.canvas.width, height: ctx.canvas.height});


const myRequest = new Request("json/designRibbon.json");
fetch(myRequest)
  .then((response) => response.json())
  .then((data) => {
        populateRibbon(data);
  })
  .catch(console.error);

function plotToCanvas(){
    invoke("clear_canvas");
    updateCanvas(ctx);
}

const windowClose = document.getElementById("windowClose") as HTMLCanvasElement;
const plot = document.getElementById("plot") as HTMLCanvasElement;
plot.addEventListener("click", () => requestAnimationFrame(plotToCanvas));


