class CustomAccordion2 extends HTMLElement {
    constructor() {
        super();

        const shadowRoot = this.attachShadow({
            mode: 'open'
        })

        this.index = 0;
        this.items = [];
        this.index_data = new Map();

        shadowRoot.innerHTML = `
            <link rel="stylesheet" href="/css/text.css" />
            <link rel="stylesheet" href="/css/accordion-content.css" />
            <link rel="stylesheet" href="/components/accordion-2/accordion.css" />

            <div class="titles-wrapper">
                <div class="titles"></div>
            </div>

            <div class="items-wrapper">
                <div class="items"></div>
            </div>

            <div class="mobile">
                <div class="subtitles">
                </div>
                
                <div class="title">
                    <svg style="aspect-ratio: 1/1;" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="8.22 5.64 7.78 12.73"> <path d="M13.1719 12L8.22192 7.04999L9.63592 5.63599L15.9999 12L9.63592 18.364L8.22192 16.95L13.1719 12Z" fill="CurrentColor"></path> </svg>
                    
                    <div></div>
                    
                    <svg style="aspect-ratio: 1/1;" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="8.22 5.64 7.78 12.73"> <path d="M13.1719 12L8.22192 7.04999L9.63592 5.63599L15.9999 12L9.63592 18.364L8.22192 16.95L13.1719 12Z" fill="CurrentColor"></path> </svg>
                </div>
            </div>
        `;

        this.title_list = shadowRoot.querySelector(".titles");
        this.item_list = shadowRoot.querySelector(".items");
        this.mobile_title_list = shadowRoot.querySelector(".mobile > .title > div");
        this.mobile_subtitle_list = shadowRoot.querySelector(".mobile > .subtitles");
        this.mobile_title_previous = shadowRoot.querySelector(".mobile > .title > svg");
        this.mobile_title_next = shadowRoot.querySelector(".mobile > .title > svg:nth-of-type(2)");

        this.mobile_title_previous.addEventListener("click", () => this.go_to_sibling_title(false));
        this.mobile_title_next.addEventListener("click", () => this.go_to_sibling_title());
    }

    connectedCallback() {
        setTimeout(() => {
            const children = this.children;

            this.process_items(children);

            this.render();

            this.go_to_index(0);
        })
    }

    render() {
        this.title_list.innerHTML = '';
        this.item_list.innerHTML = '';
        this.mobile_title_list.innerHTML = '';
        this.mobile_subtitle_list.innerHTML = '';

        let item_index = 0;

        this.items.forEach((item, title_index) => {
            const title = document.createElement("div");
            title.classList.add("title");
            title.innerText = item.title;

            let i = parseInt(item_index);
            title.addEventListener("click", () => {
                this.go_to_index(i);
            });

            this.index_data.set(item_index, {
                title: title_index
            })

            const mobile_title = document.createElement("span");
            mobile_title.innerText = item.title;
            this.mobile_title_list.appendChild(mobile_title);

            this.title_list.appendChild(title);

            const item_element = document.createElement("div");
            item_element.classList.add("item");
            item_element.appendChild(item.content);

            this.item_list.appendChild(item_element);

            const mobile_subtitles = document.createElement("div");

            if (item.subitems.length > 0) {
                const sub_item_list = document.createElement("div");
                sub_item_list.classList.add("subtitles");

                title_element.appendChild(sub_item_list);

                item.subitems.forEach((subitem, subitem_index) => {
                    item_index++;
                    let i = parseInt(item_index);

                    const sub_title = document.createElement("div");
                    sub_title.classList.add("subtitle");
                    sub_title.innerText = subitem.title;

                    this.index_data.set(item_index, {
                        title: title_index,
                        subtitle: subitem_index
                    })

                    sub_title.addEventListener("click", () => this.go_to_index(i));

                    sub_item_list.appendChild(sub_title);

                    const item_element = document.createElement("div");
                    item_element.classList.add("item");
                    item_element.replaceChildren(...subitem.content);

                    this.item_list.appendChild(item_element);

                    const mobile_subtitle = document.createElement("span");
                    mobile_subtitle.addEventListener("click", () => this.go_to_index(i));
                    mobile_subtitle.innerText = subitem.title;
                    mobile_subtitles.appendChild(mobile_subtitle);
                })
            }

            this.mobile_subtitle_list.appendChild(mobile_subtitles);

            item_index++;
        })
    }

    go_to_sibling_title(next = true) {
        const map = this.index_data;

        let current_title = map.get(this.index).title;

        map.forEach((value, index) => {
            if (value.title === current_title + (next ? 1 : -1) && (value.subtitle === undefined || value.subtitle == 0)) this.go_to_index(index);
        })
    }

    clear() {
        this.items = [];

        this.render();
    }

    go_to_index(index) {
        this.index = index;

        const titles = this.title_list.querySelectorAll(".title,.subtitle");

        const converted_i = this.index_data.get(index);

        this.mobile_title_previous.toggleAttribute("hidden", converted_i.title == 0);
        this.mobile_title_next.toggleAttribute("hidden", converted_i.title == this.title_list.childElementCount - 1);

        const scroll_width = this.title_list.parentElement.offsetWidth;

        Array.from(titles).forEach((title, i) => {
            title.toggleAttribute("selected", index === i)

            if(index === i) {
                const title_left = title.offsetLeft;
                const title_width = title.offsetWidth;

                this.title_list.style.left = `calc(50% - ${title_left / 1100 * 100}% - ${title_width / 2}px)`;
            }
        });

        Array.from(this.item_list.children).forEach((item, i) => {
            item.toggleAttribute("selected", index === i);

            if (index === i) {
                const carousels = item.querySelectorAll("carousel-element");
                for (const carousel of carousels) {
                    carousel.reset();
                }

                this.item_list.style.left = `calc(-${i * 55}% - ${i * 5}rem)`;
            }
        });

        Array.from(this.mobile_title_list.children).forEach((title, i) => {
            title.toggleAttribute("selected", converted_i.title === i);
        })

        Array.from(this.mobile_subtitle_list.children).forEach((subtitle_container, i) => {
            subtitle_container.toggleAttribute("selected", converted_i.title === i);

            Array.from(subtitle_container.children).forEach((subtitle, subtitle_index) => {
                subtitle.toggleAttribute("selected", converted_i.subtitle === subtitle_index);
            })
        })
    }

    process_items(elements) {
        this.items = [];

        for (const element of elements) {
            let title = element.getAttribute("title");
            let id = element.getAttribute("id");

            const children = element.children;

            let content = children[0] ?? document.createElement("div");

            let content_background_image = content.getAttribute("background-image");

            if (content_background_image) content.style.background = `
                linear-gradient(#f7f8f6, #f7f8f6d9 max(50%, 25rem), #0000),
                url(${content_background_image}) no-repeat
            `;

            let subitems = [];

            for (let i = 1; i < children.length; i++) {
                const sub_element = children[i];

                const sub_title = sub_element.getAttribute("title");
                const sub_id = sub_element.getAttribute("id");

                subitems.push({
                    id: sub_id,
                    title: sub_title,
                    content: sub_element.children
                })
            }

            this.items.push({
                id,
                title,
                content,
                subitems
            })
        }
    }

    disconnectedCallback() {
    }

    attributeChangedCallback(name, oldValue, newValue) {
    }

    static get observedAttributes() {
        return ['my-attribute'];
    }
}

// Define the custom element
customElements.define('custom-accordion2', CustomAccordion2);