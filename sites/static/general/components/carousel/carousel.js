class CarouselElement extends HTMLElement {
    constructor() {
        super();

        this.index = 0;

        const shadowRoot = this.attachShadow({
            mode: 'open'
        })

        this.lock = false;

        shadowRoot.innerHTML = `
            <link rel="stylesheet" href="/components/carousel/carousel.css" />
            <link rel="stylesheet" href="/css/accordion-content.css" />
            <link rel="stylesheet" href="/css/main-content.css" />

            <div class="slide-wrapper"></div>

            <div class="controls" part="controls">
                <div class="pills" part="pills"></div>
                <div class="playpause" part="playpause">
                    <svg class="play" style="vertical-align: middle;fill: currentColor;overflow: hidden;" viewBox="0 0 1024 1024" version="1.1"><path d="M852.727563 392.447107C956.997809 458.473635 956.941389 565.559517 852.727563 631.55032L281.888889 993.019655C177.618644 1059.046186 93.090909 1016.054114 93.090909 897.137364L93.090909 126.860063C93.090909 7.879206 177.675064-35.013033 281.888889 30.977769L852.727563 392.447107 852.727563 392.447107Z"></path></svg>
                    <svg class="pause" fill="CurrentColor" viewBox="4.73 0 38.15 47.61"><g><path d="M17.991,40.976c0,3.662-2.969,6.631-6.631,6.631l0,0c-3.662,0-6.631-2.969-6.631-6.631V6.631C4.729,2.969,7.698,0,11.36,0   l0,0c3.662,0,6.631,2.969,6.631,6.631V40.976z"></path>	<path d="M42.877,40.976c0,3.662-2.969,6.631-6.631,6.631l0,0c-3.662,0-6.631-2.969-6.631-6.631V6.631   C29.616,2.969,32.585,0,36.246,0l0,0c3.662,0,6.631,2.969,6.631,6.631V40.976z"></path></g></svg>
                </div>
            </div>
        `;

        this.slides = [];

        this.controls = shadowRoot.querySelector(".controls");
        this.pills = shadowRoot.querySelector(".pills");
        this.playpause = shadowRoot.querySelector(".playpause");

        this.playpause.addEventListener("click", () => this.toggle_play());
        
        this.slide_wrapper = this.shadowRoot.querySelector(".slide-wrapper");
                
        this.base = Date.now();
        this.remaining_time = 5000;

        this.observer = new MutationObserver(this.process_children.bind(this));

        this.observeChildren();
    }

    observeChildren() {
        this.observer.observe(this, {
            childList: true,
            subtree: false
        });
    }

    process_children() {

        let elements = this.children;
        let items = [];
        this.reset();

        for(let i = 0; i < elements.length; i++) {
            const element = elements[i];

            const slide = document.createElement("div");
            slide.classList.add("slide");
            
            const slot = document.createElement("slot");
            slot.setAttribute("name", i);

            slide.appendChild(slot);

            element.setAttribute("slot", i);

            items.push(slide);
        }

        this.slides = items;

        this.slides.forEach((slide, i) => {
            this.slide_wrapper.appendChild(slide);

            const pill = document.createElement("div");
            pill.classList.add("pill");
            if (i == this.index) pill.toggleAttribute("active");

            pill.addEventListener("click", () => {
                this.go_to_index(parseInt(i));
            })

            this.pills.appendChild(pill);
        })

        this.controls.classList.toggle("hidden", items.length <= 1);

        this.slide_wrapper.replaceChildren(...items);
   }

    static get observedAttributes() {
        return [

        ];
    }

    toggle_play() {
        const is_paused = this.playpause.getAttribute("paused") !== null;

        if (is_paused) this.play(); else this.pause();
    }

    pause() {
        const current_time = Date.now();
        const time_lost = current_time - this.base;

        this.remaining_time -= time_lost;
        clearTimeout(this.timer);

        this.playpause.setAttribute("paused", '');
        const active_pill = this.pills.querySelector(".pill[active]");
        active_pill.setAttribute("paused", true);
    }

    play() {        
        this.base = Date.now();
        this.timer = setTimeout(() => this.go_to_index(), this.remaining_time);


        this.playpause.removeAttribute("paused");
        const active_pill = this.pills.querySelector(".pill[active]");
        active_pill?.removeAttribute("paused");
    }

    connectedCallback() {
        setTimeout(() => {
            this.process_children();

            this.play();
    
            this.render();
        }, 0);
    }

    clear() {
        this.slide_wrapper.innerHTML = '';
        this.pills.innerHTML = '';
    }

    go_to_index(index = this.index + 1) {
        clearTimeout(this.timer);

        if (this.lock) return;
        

        if (index < 0) index = this.slides.length - 1;
        if (index >= this.slides.length) index = 0;
        if (index === this.index) return;

        this.lock = true;

        this.index = index;

        this.slide_wrapper.children[0].insertAdjacentElement("afterend", this.slides[index]);

        this.slide_wrapper.setAttribute("moving", "");

        this.remaining_time = 5000;
        this.play();

        setTimeout(() => {
            this.lock = false;

            this.slide_wrapper.removeAttribute("moving");

            this.slide_wrapper.appendChild(this.slide_wrapper.children[0])
        }, 500);

        Array.from(this.pills.children).forEach((pill, i) => {
            pill.toggleAttribute("active", i === this.index);
        })
    }

    reset() {
        this.index = 0;

        clearTimeout(this.timer);

        this.render();
        this.remaining_time = 5000;
        this.base = Date.now();
        this.play();

    }

    render() {        
        this.pills.innerHTML = '';

        this.slides.forEach((slide, i) => {
            this.slide_wrapper.appendChild(slide);

            const pill = document.createElement("div");
            pill.classList.add("pill");
            if (i == this.index) pill.toggleAttribute("active");

            pill.addEventListener("click", () => {
                this.go_to_index(parseInt(i));
            })

            this.pills.appendChild(pill);
        })
        
        const current_slide = this.slides[this.index];

        if (current_slide) this.slide_wrapper.insertAdjacentElement("afterbegin", current_slide);
    }
}

// Define the custom element
customElements.define('carousel-element', CarouselElement);
