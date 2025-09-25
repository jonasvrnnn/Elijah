class ProjectFilter extends HTMLElement {
    constructor() {
        super();

        const shadowRoot = this.attachShadow({
            mode: 'open'
        })

        shadowRoot.innerHTML = `
            <link rel="stylesheet" href="/components/project-filter/project-filter.css" />

            <form>
                <div class="row-1">
                    <div class="selects">
                        <slot name="select"></slot>
                        <div class="background"></div>
                    </div>

                    <slot name="checkbox-group-0"></slot>

                    <div class="search">
                        <input placeholder="Zoeken..." />
                        <div class="toggle">
                            <svg viewBox="-0.58 0 58.719 58.719" overflow="visible">
                                <path id="Path_15" data-name="Path 15" d="M683.547,267.547l-18.838-17.8a22.476,22.476,0,1,0-2.274,1.978l19.051,18a1.5,1.5,0,0,0,2.061-2.181Zm-54.1-33.692a19.438,19.438,0,1,1,19.438,19.438A19.46,19.46,0,0,1,629.449,233.855Z" transform="translate(-626.449 -211.418)" fill="CurrentColor" stroke-width="2" stroke="CurrentColor"></path>
                            </svg>
                            <svg fill="none" class="close" viewBox="4.58 4.58 14.83 14.83"><path d="M17 7L7 17M7 7L17 17" stroke="CurrentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </svg>
                        </div>
                    </div>
                </div>

                <div class="row-1">
                    <slot name="checkbox-group-1"></slot>
                </div>
            </form>
        `;

        const form = shadowRoot.querySelector("form");
        this.form = form;

        this.selects_wrapper = form.querySelector(".selects");
        this.selects_slot = form.querySelector("slot[name='select']");
        this.checkbox_group_0_slot = form.querySelector("slot[name='checkbox-group-0']");
        this.checkbox_group_1_slot = form.querySelector("slot[name='checkbox-group-1']");

        const search_button = form.querySelector(".search > .toggle");
        search_button.addEventListener("click", () => {
            this.toggleAttribute('search-active');
            form.reset();
        })

        this.selects_background = this.selects_wrapper.querySelector(".background");
    }

    replace_selects() {
        const selects = this.selects_slot.assignedNodes();

        let items = [];

        for(let i = 0; i < selects.length; i++) {
            const select = selects[i];

            select.removeAttribute("slot");

            items.push(select);
        }

        this.selects_slot.replaceWith(...items);
    }

    replace_checkboxes() {
        const process_checkbox = (checkbox, first) => {
            const new_checkbox = document.createElement("label");
            new_checkbox.removeAttribute("slot");
            new_checkbox.textContent = checkbox.textContent;

            if(first) {
                new_checkbox.classList.add("first");
            }

            const toggle = document.createElement("input");
            toggle.setAttribute("type", "checkbox");
            toggle.setAttribute("name", checkbox.getAttribute("name"))
            new_checkbox.appendChild(toggle);
            
            return new_checkbox;
        }

        const process_checkbox_group = slot => {
            const checkboxes = slot.assignedNodes()

            let items = [];

            for(let i = 0; i < checkboxes.length; i++) {
                const checkbox = checkboxes[i];

                items.push(process_checkbox(checkbox, i === 0));
            }

            slot.replaceWith(...items);
        }

        const checkboxes_0 = this.checkbox_group_0_slot;
        const checkboxes_1 = this.checkbox_group_1_slot;
        
        process_checkbox_group(checkboxes_0);
        process_checkbox_group(checkboxes_1);
    }

    connectedCallback() {
        setTimeout(() => {
            const selects = this.selects_slot.assignedNodes();
          
            this.selects_background.style.width = `${100 / (selects.length)}%`;
            
            for(let i = 0; i < selects.length; i++) {
                let select = selects[i];
    
                if(i === 0) select.toggleAttribute("selected");
                
                select.addEventListener("focus", () => {
                    this.selects_background.style.left = `${i * 100 / (selects.length)}%`;
    
                    for(let j = 0; j < selects.length; j++) {
                        let sub_select = selects[j];
    
                        if(i !== j) sub_select.removeAttribute("selected");
                        else sub_select.setAttribute("selected", '');
                    }
                })
            }

            this.replace_selects();
            this.replace_checkboxes();
        }, 0);
    }
}

// Define the custom element
customElements.define('project-filter', ProjectFilter);