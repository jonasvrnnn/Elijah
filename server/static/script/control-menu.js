class ControlMenu extends HTMLElement {
    constructor() {
        super();

        this.items = [];
        this.currentItem;
    }

    process_buttons() {
        const buttons = this.querySelectorAll("button[name]");

        for(let button of buttons) {
            const name = button.getAttribute("name");
            const content = this.querySelector(`[content='${name}']`);
        
            let item = {
                button, content
            }

            content.classList.add("content");

            button.addEventListener("click", () => {
                let activate = this.currentItem !== item;
                
                this.currentItem?.button.removeAttribute("active");
                this.currentItem?.content.removeAttribute("active");

                if(activate) {
                    item.button.toggleAttribute("active", true);
                    item.content.toggleAttribute("active", true);
                    this.currentItem = item;
                } else {
                    this.currentItem = null;
                }
            })

            this.items.push(item)
        }
    }

    connectedCallback() {
        setTimeout(() => {
            this.process_buttons();
        })
    }
}

// Define the custom element
customElements.define('control-menu', ControlMenu);