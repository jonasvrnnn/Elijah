class TabsElement extends HTMLElement {
    constructor() {
        super();

        this.index = 0;

        const shadowRoot = this.attachShadow({
            mode: 'closed'
        })

        this.lock = false;

        shadowRoot.innerHTML = `
            <link rel="stylesheet" href="/components/tabs/tabs.css" />

            <div class="controls"><div class="background"></div></div>

            <div class="content"></div>
        `;

        this.controls = shadowRoot.querySelector(".controls");
        this.background = this.controls.querySelector(".background");
        this.content = shadowRoot.querySelector(".content");

        this.data = [];
    }

    go_to_index(index) {
        const length = this.data.length;
        const tabs = Array.from(this.controls.querySelectorAll("span"));

        if (index < 0) index = 0;
        if (index >= length) index = length - 1;

        this.index = index;

        this.data.forEach((item, i) => {
            item.tab_element.toggleAttribute("active", i === this.index);
            item.content.toggleAttribute("active", i === this.index);
        })

        this.background.style.left = `${100 / length * this.index}%`;
    }

    connectedCallback() {
        setTimeout(() => {
            Array.from(this.children).forEach((value, i) => {
                const tab = value.getAttribute("tab-title");
                const title = value.getAttribute("title");
                const image = value.getAttribute("image");
                const text = value.innerHTML;

                const item_data = {
                    tab,
                    title,
                    image,
                    text
                }

                this.data.push(item_data);
            });

            this.render();
        }, 0);
    }

    clear() {
        for(let i = 1; i < this.controls.childElementCount; i++) {
            this.controls.children[i].remove();
        }

        this.content.innerHTML = '';
    }

    render() {
        this.clear();

        this.data.forEach((item, index) => {
            const tab = document.createElement("span");
            tab.innerHTML = item.tab;
            tab.addEventListener("click", () => this.go_to_index(index))

            this.controls.appendChild(tab);

            const item_element = document.createElement("div");
            const item_element_content = document.createElement("div");
            item_element.classList.add("item");
            item_element_content.classList.add("item-content");

            item_element_content.innerHTML = item.text;
            item_element.appendChild(item_element_content);

            if(item.title) {
                const title_element = document.createElement("h3");
                title_element.innerText = item.title;
                item_element.insertAdjacentElement("afterbegin", title_element);
            }

            if(item.image) {
                const image = document.createElement("img");
                image.loading = 'lazy';
                image.src = item.image;
                item_element.appendChild(image);
            }

            this.content.appendChild(item_element);

            item.tab_element = tab;
            item.content = item_element;
        })

        this.go_to_index(0);

        this.background.style.width = `${100 / this.data.length}%`;
    }
}

// Define the custom element
customElements.define('tabs-element', TabsElement);