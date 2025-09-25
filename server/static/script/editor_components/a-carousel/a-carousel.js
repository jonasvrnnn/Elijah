class ACarousel extends HTMLElement {
    constructor() {
        super();

        this.index = 0;

        setTimeout(() => {
            const stylesheet = document.createElement("link");
            stylesheet.rel="stylesheet";
            stylesheet.href="/components/a-carousel/a-carousel.css";
            document.head.appendChild(stylesheet);
        })

        this.lock = false;

        const list = document.createElement("div");
        list.classList.add("content");
        this.list = list;

        this.list.append(...this.process_children());

        const modal = document.createElement("dialog");
        this.modal = modal;

        const modal_content = document.createElement("section");
        this.modal.appendChild(modal_content);
        this.modal_content = modal_content;

        const close = document.createElement("button");
        close.classList.add("close");
        close.addEventListener("click", () => {
            modal.close();
        })
        modal.appendChild(close);

        const controls = document.createElement("nav");
        const left = document.createElement("button");
        left.addEventListener("click", () => this.set_index(false));
        const right = document.createElement("button");
        right.addEventListener("click", () => this.set_index());
        controls.appendChild(left);
        controls.appendChild(right);

        this.innerHTML = '';
        this.appendChild(list);
        this.appendChild(modal);
        this.appendChild(controls);

        new ResizeObserver(() => {
            let element = Array.from(list.children)[this.index];

            if (!element) return;

            let left = element.offsetLeft;

            list.style.left = `-${left}px`;
        }).observe(this);

        modal.addEventListener("click", e => {
            if (e.target !== modal_content && !modal_content.contains(e.target)) {
                modal.close();
            }
        })

        this.modal.addEventListener("close", () => {
            document.body.style.overflow = '';
            this.modal_content.innerHTML = '';
        })

        let wheel_active = true;
        this.addEventListener("wheel", e => {
            e.preventDefault();

            if (!wheel_active) return;

            const isTouchPad = e.wheelDeltaY ? e.wheelDeltaY === -3 * e.deltaY : e.deltaMode === 0

            const delta = isTouchPad ? e.deltaX : e.deltaY;

            if ((!isTouchPad && e.shiftKey) || isTouchPad) {
                wheel_active = false;

                this.set_index(delta > 0);

                setTimeout(() => wheel_active = true, isTouchPad ? 150 : 100);
            }
        })

        this.addEventListener('swiped', e => {
            e.preventDefault();
            if (!wheel_active) return;

            switch (e.detail.dir) {
                case "left": this.set_index(); break;
                case "right": this.set_index(false); break;
            }

            setTimeout(() => wheel_active = true, 150);
        });
    }

    set_index(up = true) {
        if (up) {
            this.index++;
        } else {
            this.index--;
        }

        if (this.index < 0) this.index = 0;
        if (this.index >= this.list.childElementCount) this.index = this.list.childElementCount - 1;

        let element = Array.from(this.list.children)[this.index];

        let left = element.offsetLeft;

        this.list.style.left = `-${left}px`;
    }

    process_children() {
        return Array.from(this.children).map((child, index) => {
            const title = child.getAttribute("title");
            const sub_title = child.getAttribute("subtitle");
            const background_image = child.getAttribute("image");

            const popup_background = child.getAttribute("popup-background");

            const text = child.innerHTML;

            const item = document.createElement("div");
            item.classList.add("item");
            item.style.backgroundImage = `url(${background_image})`;

            const tab = document.createElement("div");
            tab.classList.add("title");
            tab.innerHTML = title;
            item.appendChild(tab);

            if (sub_title) {
                const title = document.createElement("div");
                title.classList.add("subtitle");
                title.innerHTML = sub_title;
                item.appendChild(title);
            }

            const plus = document.createElement("button");
            plus.classList.add("plus");
            item.appendChild(plus);

            plus.addEventListener("click", e => {
                e.preventDefault();

                console.log(popup_background);

                this.modal.classList.toggle('background-white', popup_background == null);
                if (popup_background != null) {
                    this.modal_content.style.backgroundImage = `url(${popup_background})`;
                } else {
                    this.modal_content.style.backgroundImage = ``;
                }

                this.modal_content.innerHTML = child.innerHTML;
                setTimeout(() => this.modal_content.scrollTop = 0, 0);
                this.modal.showModal();
            });

            return item;
        })
    }

    connectedCallback() {
        this.set_index(0);
    }
}

// Define the custom element
customElements.define('a-carousel', ACarousel);