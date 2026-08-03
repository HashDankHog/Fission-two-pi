const windowTauri: any = window;
const { invoke } = windowTauri.parent.__TAURI__.core;

import { Constraint } from "../modules/constraint.ts";

function distance() {
    const p0 = Number((document.getElementById("p0") as HTMLInputElement).value);
    const p1 = Number((document.getElementById("p1") as HTMLInputElement).value);
    const dr = Number((document.getElementById("dr") as HTMLInputElement).value);
    
    const distance: Constraint = {"Distance": {point_1: p0, point_2: p1, distance: dr}};

    invoke("add_constraint", {constraint: distance});
   
}
(document.querySelector("button") as HTMLButtonElement).addEventListener("click", () => distance());