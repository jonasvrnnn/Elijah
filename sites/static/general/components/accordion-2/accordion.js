class CustomAccordion2 extends HTMLElement {
    constructor() {
        super();

        const shadowRoot = this.attachShadow({
            mode: 'open'
        })

        this.index = 0;
        this.items = [];
        this.index_data = new Map();
        this.index = [0];

        shadowRoot.innerHTML = `
            <link rel="stylesheet" href="/css/text.css" />
            <link rel="stylesheet" href="/css/accordion-content.css" />
            <link rel="stylesheet" href="/components/accordion-2/accordion.css" />
        `;

        const title_list_wrapper = document.createElement("div");
        title_list_wrapper.classList.add("titles-wrapper");
        shadowRoot.appendChild(title_list_wrapper);

        this.title_list = document.createElement("div");
        this.title_list.classList.add("titles");
        title_list_wrapper.appendChild(this.title_list);

        const content_wrapper = document.createElement("div");
        content_wrapper.classList.add("content-wrapper");
        shadowRoot.appendChild(content_wrapper);

        this.subtitle_list_wrapper_wrapper = document.createElement("div");
        this.subtitle_list_wrapper_wrapper.classList.add("subtitles-wrapper-wrapper");
        content_wrapper.appendChild(this.subtitle_list_wrapper_wrapper);

        this.items_wrapper_wrapper = document.createElement("div");
        this.items_wrapper_wrapper.classList.add("items-wrapper-wrapper");
        content_wrapper.appendChild(this.items_wrapper_wrapper);

        const observer = new MutationObserver(mutationslist => {
            for (let mutation of mutationslist) {
                if (mutation.type === 'childList') {
                    this.items = this.process_entries();
                    this.render();
                }
            }
        });

        observer.observe(this, {
            childList: true
        });

        document.addEventListener('swiped', e => {
            if (e.target !== this) return;


            let new_sub_index;

            switch (e.detail.dir) {
                case "left": new_sub_index = (this.index[1] ?? -1) + 1; break;
                case "right": if (this.index[1] != undefined) new_sub_index = this.index[1] - 1; break;
            }

            if (new_sub_index < -1) return;
            if (new_sub_index == -1) new_sub_index = undefined;
            if (new_sub_index > Array.from(this.items_wrapper_wrapper.children)[this.index[0]].children.length - 2) return;

            this.set_index(this.index[0], new_sub_index, false)
        });
    }

    process_entries(el = this) {
        let items = [];

        for (let entry of el.querySelectorAll(":scope > [title]")) {
            const sub_items = this.process_entries(entry);

            let item = {
                title: entry.getAttribute("title"),
                content: entry.children[0],
                sub_items
            };

            items.push(item);
        }

        return items;
    }

    set_index(index, sub_index, scroll = true) {
        Array.from(this.title_list.children).forEach((title, i) => {
            title.toggleAttribute("active", index === i);

            if (index === i && scroll) {
                title.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' });
            }
        })

        Array.from(this.subtitle_list_wrapper_wrapper.children).forEach((subtitle_list_wrapper, i) => {
            subtitle_list_wrapper.toggleAttribute("active", index === i);

            if (index === i) {
                if (scroll) subtitle_list_wrapper.children[sub_index ?? 0]?.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' });

                Array.from(subtitle_list_wrapper.children).forEach((subtitle, j) => {
                    subtitle.toggleAttribute("active", sub_index === j);
                })
            }
        })

        Array.from(this.items_wrapper_wrapper.children).forEach((items_wrapper, i) => {
            items_wrapper.toggleAttribute("active", index === i);

            if (index === i) {
                let sub_index_new = sub_index != undefined ? sub_index + 1 : 0;
                if (scroll) items_wrapper.children[sub_index_new]?.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' });
            }
        })

        this.index = [index, sub_index];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        console.log(`Attribute ${name} changed from ${oldValue} to ${newValue}`);
    }

    connectedCallback() {
        setTimeout(() => {
            this.items = this.process_entries(this);
            this.render();

            this.set_index(0, null, false);
        })
    }

    render() {
        this.title_list.innerHTML = '';
        this.subtitle_list_wrapper_wrapper.innerHTML = '';
        this.items_wrapper_wrapper.innerHTML = '';

        this.items.forEach((item, index) => {
            let title = document.createElement("span");
            title.innerText = item.title;
            title.setAttribute("index", index)
            title.addEventListener("click", () => {
                this.set_index(index);
            })
            this.title_list.appendChild(title);

            const subtitle_wrapper = document.createElement("div");
            subtitle_wrapper.classList.add("subtitles-wrapper");
            this.subtitle_list_wrapper_wrapper.appendChild(subtitle_wrapper);

            const items_wrapper = document.createElement("div");
            items_wrapper.classList.add("items-wrapper");
            this.items_wrapper_wrapper.appendChild(items_wrapper);

            items_wrapper.appendChild(item.content.cloneNode(true));

            item.sub_items.forEach((si, sub_index) => {
                const sub_title = document.createElement("span");
                sub_title.innerText = si.title;
                sub_title.addEventListener("click", () => {
                    this.set_index(index, sub_index);
                })
                sub_title.setAttribute("index", sub_index);
                subtitle_wrapper.appendChild(sub_title);

                if (si.content) items_wrapper.appendChild(si.content.cloneNode(true));
            })
        })
    }
}

// Define the custom element
customElements.define('custom-accordion2', CustomAccordion2);