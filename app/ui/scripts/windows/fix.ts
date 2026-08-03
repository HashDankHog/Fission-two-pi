import { Constraint } from "../modules/constraint";

function fix() {
    const p = Number((document.getElementById("p") as HTMLInputElement).value);
    const x = Number((document.getElementById("x") as HTMLInputElement).value);
    const y = Number((document.getElementById("y") as HTMLInputElement).value);
    const z = Number((document.getElementById("z") as HTMLInputElement).value);
    
    const fixX: Constraint = {"FixX": {point: p, position: x}};
    const fixY: Constraint = {"FixY": {point: p, position: y}};
    const fixZ: Constraint = {"FixZ": {point: p, position: z}};

    invoke("add_constraint", {constraint: fixX});
    invoke("add_constraint", {constraint: fixY});
    invoke("add_constraint", {constraint: fixZ});
}
(document.querySelector("button") as HTMLButtonElement).addEventListener("click", () => fix());
