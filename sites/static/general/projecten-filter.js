(() => {
    const form = document.currentScript.parentNode;

    document.body.addEventListener("htmx:configRequest", e => {
        const cleanedData = new FormData();

        for (const [key, value] of e.detail.parameters.entries()) {
            if (value !== undefined && value !== '') {
                cleanedData.append(key, value);
            }
        }

        e.detail.parameters = cleanedData;
    })

    let params = new URLSearchParams(window.location.search);

    params.forEach((value, key) => {
        const elements = form.querySelectorAll(`[name='${key}']`);

        if (elements.length == 0) return;

        for (let element of elements) {
            switch (element.tagName) {
                case "SELECT": {
                    let options = element.options;
                    const index = Array.from(options).findIndex(e => e.value == value);
                    element.selectedIndex = index;
                }; break;
                case "INPUT": {
                    switch (element.getAttribute("type")) {
                        case "checkbox": {
                            if(element.getAttribute("value") == value) element.toggleAttribute("checked", true);
                            break;
                        }
                    }
                }; break;
            }
        }
    })

    const selects_wrapper = form.querySelector(".selects");
    const selects = selects_wrapper.querySelectorAll("select");
    const selects_background = selects_wrapper.querySelector(".background");
    const search_toggle = form.querySelector(".toggle");
    const search_input = form.querySelector(".search > input");

    form.style.setProperty("--select-count", selects.length);
    form.style.setProperty("--select-index", 0);

    for (let i = 0; i < selects.length; i++) {
        const select = selects[i];

        select.toggleAttribute("selected", i === 0)

        select.addEventListener("focus", () => {
            search_input.value = null;
            form.style.setProperty("--select-index", i);

            for (let j = 0; j < selects.length; j++) {
                let sub_select = selects[j];

                if (i !== j) sub_select.removeAttribute("selected");
                else sub_select.setAttribute("selected", '');
            }
        })
    }

    search_toggle.addEventListener('click', () => {
        form.toggleAttribute("search-active");

        if (form.hasAttribute("search-active")) {
            search_input.focus();
        }
    })

    search_input.addEventListener("focus", () => {
        form.reset();

        form.dispatchEvent(new Event('change'));
    })
})();