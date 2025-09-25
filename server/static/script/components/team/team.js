class TeamElement extends HTMLElement {
    constructor() {
        super();

        const shadowRoot = this.attachShadow({
            mode: 'open'
        });

        shadowRoot.innerHTML = `
            <link rel="stylesheet" href="/components/team/team.css" />
        `;

        this.team = [];

        this.info = false;
    }

    generate_element(child) {
        const element = document.createElement("div");
        element.innerHTML = `<div class="person" style='background-image: url("${child.getAttribute('image')}");'>
            <div class="bottom">
                <div class="name">${child.getAttribute('name')}</div>
                <div class="title">${child.getAttribute('title')}</div>
            </div>

            <div class="overlay">${child.textContent}</div>

            <div class="buttons">
                <div class="info">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 3.778 16.999">
                        <path
                            d="M9935.622,64.631a1.253,1.253,0,0,1-.364-.918V55.992a1.275,1.275,0,0,1,.356-.918,1.2,1.2,0,0,1,.9-.369,1.214,1.214,0,0,1,.9.369,1.251,1.251,0,0,1,.366.918v7.722a1.275,1.275,0,0,1-.356.918,1.2,1.2,0,0,1-.9.369A1.224,1.224,0,0,1,9935.622,64.631Zm-.222-13.194a1.324,1.324,0,0,1-.4-.978,1.447,1.447,0,0,1,.4-1.04,1.38,1.38,0,0,1,1.98,0,1.447,1.447,0,0,1,.4,1.04,1.314,1.314,0,0,1-.4.978,1.438,1.438,0,0,1-1.978,0Z"
                            transform="translate(-9934.501 -48.501)" fill="CurrentColor" stroke="rgba(0,0,0,0)"
                            stroke-miterlimit="10" stroke-width="1"></path>
                    </svg>

                    <svg class="close" viewBox="4.58 4.58 14.83 14.83"><path d="M17 7L7 17M7 7L17 17" stroke="CurrentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="--darkreader-inline-stroke: CurrentColor;" data-darkreader-inline-stroke=""></path> </svg>
                </div>
            </div>
        </div>`;

        const return_element = element.children[0];

        const info = return_element.querySelector(".info");
        const buttons = return_element.querySelector(".buttons");

        info.addEventListener("click", () => return_element.toggleAttribute('active'));

        // Check for phone attribute
        if (child.hasAttribute("phone")) {
            const link = document.createElement('a');
            link.href = `tel:${child.getAttribute("phone")}`;
            link.innerHTML = `
                <svg fill="CurrentColor" viewBox="3 3 18 18"><path d="M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2z"></path></svg>`;

            buttons.appendChild(link);
        }

        // Check for email attribute
        if (child.hasAttribute("email")) {
            const link = document.createElement('a');
            link.href = `mailto:${child.getAttribute("email")}`;
            link.innerHTML = `
                <svg fill="CurrentColor" preserveAspectRatio="xMinYMin" viewBox="-0.03 -0.03 20.03 14.03">
                    <path d="M3.598 2l5.747 5.12a1 1 0 0 0 1.33 0L16.423 2H3.598zM18 3.273l-5.994 5.341a3 3 0 0 1-3.992 0L2 3.254V12h16V3.273zM2 0h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2z"></path>
                </svg>`;
            buttons.appendChild(link);
        }

        // Check for email attribute
        if (child.hasAttribute("linkedin")) {
            const link = document.createElement('a');
            link.href = child.getAttribute("linkedin");
            link.innerHTML = `
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" xml:space="preserve">
                <defs></defs>
                <g style="fill: CurrentColor; stroke: none; stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; opacity: 1;" transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)">
                    <path d="M 1.48 29.91 h 18.657 v 60.01 H 1.48 V 29.91 z M 10.809 0.08 c 5.963 0 10.809 4.846 10.809 10.819 c 0 5.967 -4.846 10.813 -10.809 10.813 C 4.832 21.712 0 16.866 0 10.899 C 0 4.926 4.832 0.08 10.809 0.08" style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; opacity: 1;" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round"></path>
                    <path d="M 31.835 29.91 h 17.89 v 8.206 h 0.255 c 2.49 -4.72 8.576 -9.692 17.647 -9.692 C 86.514 28.424 90 40.849 90 57.007 V 89.92 H 71.357 V 60.737 c 0 -6.961 -0.121 -15.912 -9.692 -15.912 c -9.706 0 -11.187 7.587 -11.187 15.412 V 89.92 H 31.835 V 29.91 z" style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; opacity: 1;" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round"></path>
                </g>
            </svg>`;

            buttons.appendChild(link);
        }

        window.addEventListener("click", e => {
            const path = e.composedPath();

            // Check if the click target is outside of the current instance
            if (!path.includes(return_element)) {
                return_element.removeAttribute('active');
            }
        })


        return return_element;
    }

    connectedCallback() {
        setTimeout(() => {
            for (let child of this.children) {
                const element = this.generate_element(child);
                this.shadowRoot.appendChild(element);
            }

            htmx.process(this.shadowRoot);

        }, 0);
    }

    render() {

    }
}

// Define the custom element
customElements.define('team-element', TeamElement);