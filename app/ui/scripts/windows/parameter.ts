const thismightbetheworstlineofcodeiveeverwriten: any = window;
const { invoke } = thismightbetheworstlineofcodeiveeverwriten.parent.__TAURI__.core;
var segments = [];
var parameter_num=0;
function add_parameter() {
    const row: any = document.createElement("tr");
    
    const body: any = document.querySelector("table");
    const shitCode: any = document.getElementById(String(parameter_num));
    row.innerHTML = shitCode.innerHTML;
    const value = document.createElement("P");
    value.textContent = "0";
    value.className = "value";
    const godIamBadAtCoding: any = document.querySelector(".addParam");
    godIamBadAtCoding.replaceWith(value);
    parameter_num++;
    row.id=String(parameter_num);
    row.className="parameter";
    row.querySelector(".paramNum").textContent=String(parameter_num);

    body.append(row);
}


async function update() {
    var expressions = [];
    const inputFields: any = document.getElementsByClassName("expression");
    for (let i = 0; i < inputFields.length; i+= 1) {
        var value: any = inputFields[i].value;
        if ( value != "") {
            
            expressions.push(inputFields[i].value);
        } else {
            expressions.push("0")
        }
    }
    var values = Array.from(await invoke("update_parameter", {expressions: expressions}));
    var parameter_values = document.getElementsByClassName("value");
    for (let i = 0; i < values.length; i+= 1) {
        parameter_values[i].innerHTML = String(values[i]);
    }
}
function add_segment(){
    var seg_pos = document.getElementsByClassName("parameter").length;
    segments.push(seg_pos);
    invoke("add_segment", {position: seg_pos});
    add_parameter();
    add_parameter();
}
const ouushi: any = document.getElementById("update");
const godhelpusall: any = document.getElementById("Add Segment");
ouushi.addEventListener("click", update);
godhelpusall.addEventListener("click", add_segment);

thismightbetheworstlineofcodeiveeverwriten.plot = function plot(){
    var expressions = [];
    const inputFields: any = document.getElementsByClassName("expression");
    for (let i = 0; i < inputFields.length; i+= 1) {
        var value: any = inputFields[i].value;
        if ( value != "") {
            
            expressions.push(inputFields[i].value);
        } else {
            expressions.push("0")
        }
    }
    for (let segment = 0; segment < segments.length; segment += 1){
        invoke("plot", {segment: segment, expressions: expressions, color: [255,255,255]});
    }
}

