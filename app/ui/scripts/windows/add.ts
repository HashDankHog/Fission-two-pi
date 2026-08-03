const windowTauri: any = window;
const { invoke } = windowTauri.parent.__TAURI__.core;

import { Constraint } from "../modules/constraint.ts";

function add() {
    const x = Number((document.getElementById("xi") as HTMLInputElement).value);
    const y = Number((document.getElementById("yi") as HTMLInputElement).value);
    const z = Number((document.getElementById("zi") as HTMLInputElement).value);
    invoke("add_point", {point: [x,y,z]});
}  
(document.querySelector("button") as HTMLButtonElement).addEventListener("click", () => add());