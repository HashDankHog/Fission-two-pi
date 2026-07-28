const thismightbetheworstlineofcodeiveeverwriten: any = window;
const { invoke } = thismightbetheworstlineofcodeiveeverwriten.__TAURI__.core;

//TODO: this function can be optimized slightly but I dont want to use the version provided by claude because I believe it goes against my learning goals
export async function updateCanvas(ctx: any){
    const img = ctx.getImageData(0, 0, ctx.canvas.width, ctx.canvas.height);
    const test = await invoke("render");
    const data = new Uint8ClampedArray(test);
    for (let i = 0; i < data.length; i += 1) {
        img.data[i] = data[i];
    }
    ctx.putImageData(img,0,0);  
}