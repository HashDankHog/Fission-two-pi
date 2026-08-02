type Side = "top" | "right" | "right_corner" | "bottom" | "left_corner" | "left" | "";
export class Win {
    #isdrag = false;
    #side: Side = "";
    #id: number;
    #prevpos = [0,0];
    #element(type: string, element_class: string): HTMLElement {
        const element = document.createElement(type);
        element.className = element_class;
        element.id = this.#id.toString() + "_" + element_class;
        return element;

    }
    //TODO: rewrite this to use for loops because that would take 1/4 of the lines
    // the only issue is that there might be a performance hit from having a for loop
    // but I am not calling this 200000 per second so I am good    
    constructor(iframe: string, id: number){
        this.#id = id;

        const win = this.#element("div", "window");
        const top = this.#element("div", "top");
        const right = this.#element("div", "right");
        const right_corner = this.#element("div", "right_corner");
        const bottom = this.#element("div", "bottom");
        const left_corner = this.#element("div", "left_corner");
        const left = this.#element("div", "left");
        const frame = this.#element("iframe", "frame");
        const button = this.#element("button", "close");

        button.textContent = "X";
        button.addEventListener("mousedown", () => this.close_window());

        frame.setAttribute("src", iframe);

        top.addEventListener("mousedown", (event) => this.#start_drag(event, "top"));
        right.addEventListener("mousedown", (event) => this.#start_drag(event, "right"));
        right_corner.addEventListener("mousedown", (event) => this.#start_drag(event, "right_corner"));
        bottom.addEventListener("mousedown", (event) => this.#start_drag(event, "bottom"));
        left_corner.addEventListener("mousedown", (event) => this.#start_drag(event, "left_corner"));
        left.addEventListener("mousedown", (event) => this.#start_drag(event, "left"));

        top.addEventListener("mousemove", (event) => this.#drag(event));
        right.addEventListener("mousemove", (event) => this.#drag(event));
        right_corner.addEventListener("mousemove", (event) => this.#drag(event));
        bottom.addEventListener("mousemove", (event) => this.#drag(event));
        left_corner.addEventListener("mousemove", (event) => this.#drag(event));
        left.addEventListener("mousemove", (event) => this.#drag(event));

        top.addEventListener("mouseup", (event) => this.#end_drag(event));
        right.addEventListener("mouseup", (event) => this.#end_drag(event));
        right_corner.addEventListener("mouseup", (event) => this.#end_drag(event));
        bottom.addEventListener("mouseup", (event) => this.#end_drag(event));
        left_corner.addEventListener("mouseup", (event) => this.#end_drag(event));
        left.addEventListener("mouseup", (event) => this.#end_drag(event));

        top.append(button);
        win.append(top);
        win.append(right);
        win.append(right_corner);
        win.append(bottom);
        win.append(left_corner);
        win.append(left);
        win.append(frame);

        document.body.appendChild(win);
        
    }
    place(pos: [number, number], width: number, height: number) {
        console.log("c");
        const win = document.getElementById(this.#id.toString() + "_" + "window") as HTMLCanvasElement;
        win.style.left = pos[0].toString() + "px";
        win.style.top  = pos[1].toString() + "px";

        win.style.width  = width.toString()  + "px";
        win.style.height = height.toString() + "px";
    }
    #start_drag(event: MouseEvent, side: Side) {
        this.#prevpos = [event.x, event.y];
        this.#isdrag = true;
        this.#side = side; 
    }
    #drag(event: MouseEvent) {
        const delta = [event.x - this.#prevpos[0], event.y - this.#prevpos[1]];
        this.#prevpos = [event.x, event.y];
        const rect = (document.getElementById(this.#id.toString() + "_" + "window") as HTMLCanvasElement)
            .getBoundingClientRect();
        if (this.#isdrag == false) {
            return ;
        }
        switch (this.#side) {
            case "top": {
                this.place([rect.x+delta[0],rect.y+delta[1]], rect.width, rect.height);
                break;
            }
            case "right": {
                this.place([rect.x,rect.y], rect.width+delta[0],rect.height);
                break;
            }
            case "right_corner": {
                this.place([rect.x,rect.y], rect.width+delta[0], rect.height+delta[1]);
                break;
            }
            case "bottom": {
                this.place([rect.x,rect.y], rect.width, rect.height+delta[1]);
                break;
            }
            case "left_corner": {
                this.place([rect.x+delta[0],rect.y],rect.width-delta[0],rect.height+delta[1]);
                break;
            }
            case "left": {
                this.place([rect.x+delta[0],rect.y],rect.width-delta[0],rect.height);
                break;
            }
        }
    }
    //TODO: remove the event parameter
    #end_drag(_event: MouseEvent) {
        this.#isdrag = false;
        this.#side = "";
    }
    close_window() {
        const win = document.getElementById(this.#id.toString() + "_" + "window") as HTMLCanvasElement;
        win.remove();
    }
}