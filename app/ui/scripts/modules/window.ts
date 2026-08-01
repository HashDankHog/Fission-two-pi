type Side = "top" | "right" | "right_corner" | "bottom" | "left_corner" | "left" | "";
export class SubWindow {
    #isdrag = false;
    #side: Side = "";
    #id: number;
    #prevpos = [0,0];    
    constructor(iframe: string, id: number){
        this.#id = id;
    }
    #resize(left_corner: [number, number], right_corner: [number, number]) {

    }
    place(pos: [x: number, y: number]) {
        
    }
    #start_drag(event: MouseEvent, side: Side) {
        this.#prevpos = [event.x, event.y];
        this.#isdrag = true;
        this.#side = side; 
    }
    #drag(event: MouseEvent) {

    }
    #end_drag(event: MouseEvent) {

    }
    close_window() {
        
    }
}