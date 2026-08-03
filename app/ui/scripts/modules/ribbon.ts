import { openWindow } from "./window.ts";

type Ribbon = {"ribbon": {
    "name": string,
    "item": {
        "name": string,
        "src": string,
        "action": string[]
    }[]
}[]};

export function populateRibbon(obj: Ribbon) {
    const ribbon: any = document.querySelector(".ribbon");
    for (const section of obj.ribbon) {
        const sectionDiv = document.createElement("div");

        sectionDiv.className = "section";
        sectionDiv.id = section.name;

        const headerDiv = document.createElement("div");

        headerDiv.className = "header";
        headerDiv.id = section.name+"Header";

        const sectionHeader = document.createElement("h2");
        sectionHeader.textContent = section.name;
        headerDiv.append(sectionHeader);
        sectionDiv.append(headerDiv);

        const bottomDiv = document.createElement("div");
        bottomDiv.className = "ribbonBottom";
        bottomDiv.id = "bottom";

        for (const item of section.item) {
            const itemDiv = document.createElement("div");

            itemDiv.className = "item";
            itemDiv.id = item.name;

            
            const itemImage = document.createElement("img");
            itemImage.src = item.src;
            itemImage.textContent = item.name;
            
            itemDiv.append(itemImage);

            const actionDiv = document.createElement("div");

            actionDiv.className = "action";
            actionDiv.id = item.name;
            for (const action of item.action) {
                const actionButton = document.createElement("button");

                actionButton.className = "actionButton";
                actionButton.id = action;
                
                actionButton.addEventListener("click", () => openWindow(action));

                actionButton.textContent = action;
                actionDiv.append(actionButton);
                
            }
            itemDiv.append(actionDiv);
            bottomDiv.append(itemDiv);
        }
        sectionDiv.append(bottomDiv);
        ribbon.appendChild(sectionDiv);
    }
}