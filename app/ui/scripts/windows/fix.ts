const windowTauri: any = window;
const { invoke } = windowTauri.parent.__TAURI__.core;

import { Constraint } from "../modules/constraint.ts";

function fix() {
    
    const p = Number((document.getElementById("p") as HTMLInputElement).value);

    const x_str = (document.getElementById("x") as HTMLInputElement).value;
    const x = Number(x_str);

    const y_str = (document.getElementById("y") as HTMLInputElement).value;
    const y = Number(y_str);

    const z_str = (document.getElementById("z") as HTMLInputElement).value;
    const z = Number(z_str);
    
    
    const fixX: Constraint = {"FixX": {point: p, position: x}};
    const fixY: Constraint = {"FixY": {point: p, position: y}};
    const fixZ: Constraint = {"FixZ": {point: p, position: z}};
    if (x_str != ""){
        invoke("add_constraint", {constraint: fixX});
    }
    if (y_str != ""){
    invoke("add_constraint", {constraint: fixY});
    }
    if (z_str != ""){
    invoke("add_constraint", {constraint: fixZ});
    }
}
(document.querySelector("button") as HTMLButtonElement).addEventListener("click", () => fix());
