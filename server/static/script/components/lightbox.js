class LightBox extends HTMLElement {
    constructor() {
        super();

        const dialog = document.createElement("dialog");
        const img_wrapper = document.createElement("div");
        img_wrapper.classList.add("img-wrapper");
        const copyright = document.createElement("span");
        copyright.classList.add('copyright');
        const main_img = document.createElement("img");
        const left = document.createElement("button");
        left.classList.add("left");
        const right = document.createElement("button");
        right.classList.add("right");

        img_wrapper.appendChild(main_img);
        this.appendChild(dialog);
        dialog.appendChild(left);
        dialog.appendChild(right);
        dialog.appendChild(img_wrapper);
        img_wrapper.appendChild(copyright)

        this.main_img = main_img;
        this.dialog = dialog;
        this.copyright = copyright;
        let images = [];

        left.addEventListener("click", () => {
            this.set_index(this.index - 1);
        });

        right.addEventListener("click", () => {
            this.set_index(this.index + 1);
        });

        this.index = 0;

        dialog.addEventListener('keydown', e => {
            if (!dialog.open) return;

            switch (e.key) {
                case "ArrowLeft": {
                    this.set_index(this.index - 1);
                } break;
                case "ArrowRight": {
                    this.set_index(this.index + 1);
                } break;
            }
        })

        dialog.addEventListener('click', function (event) {
            if (event.target === left || event.target === right) return;
            var rect = dialog.getBoundingClientRect();
            var isInDialog = (rect.top <= event.clientY && event.clientY <= rect.top + rect.height &&
                rect.left <= event.clientX && event.clientX <= rect.left + rect.width);
            if (!isInDialog) {
                dialog.close();
            }
        });

        const observer = new MutationObserver(mutationslist => {
            for (let mutation of mutationslist) {
                if (mutation.type === 'childList') {
                    this.refresh_images();
                }
            }
        });

        observer.observe(this, {
            childList: true
        });
    }

    refresh_images() {
        this.images = Array.from(this.querySelectorAll(":scope > .lightbox-image-wrapper > img"));
        console.log(this.images)
        for (let index in this.images) {
            const image = this.images[index];
            image.addEventListener("click", () => {
                this.set_index(index);
            })
        }
    }

    connectedCallback() {
        setTimeout(() => {
            this.refresh_images();
        }, 0)
    }

    set_index(index) {
        index = parseInt(index);

        if (index < 0) {
            index = this.images.length - 1;
        } else if (index >= this.images.length) {
            index = 0;
        }

        this.index = index;

        let image = this.images[this.index];

        this.main_img.src = image.src;
        this.copyright.innerText = image.getAttribute("copyright");

        if (!this.dialog.open) this.dialog.showModal();
    }
}

customElements.define(
    "light-box",
    LightBox,
    { extends: "section" }
);
