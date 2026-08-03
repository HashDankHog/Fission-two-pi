const windowTauri: any = window;
const { invoke } = windowTauri.parent.__TAURI__.core;

import { Constraint } from "../modules/constraint.ts";

function angle() {
    const p0 = Number((document.getElementById("p0") as HTMLInputElement).value);
    const p1 = Number((document.getElementById("p1") as HTMLInputElement).value);
    const p2 = Number((document.getElementById("p2") as HTMLInputElement).value);
    const th = Number((document.getElementById("th") as HTMLInputElement).value);
    
    const angle: Constraint = {"Angle": {point_1: p0, point_2: p1, point_3: p2, angle: th}};

    invoke("add_constraint", {constraint: angle});
   
}
(document.querySelector("button") as HTMLButtonElement).addEventListener("click", () => angle());
