(() => {
    const content = document.currentScript.parentNode;
    const logo = content.querySelector(".logo");
    const hamburger = content.querySelector(".hamburger");
    const top = content.querySelector(".top");
    const bottom = content.querySelector(".bottom");
    const bottom_content = bottom.querySelector(".bottom-content");

    top.classList.add("float-in");
    setTimeout(() => top.classList.remove("float-in"), 300);

    window.addEventListener("htmx:afterSettle", e => {
        if (e.detail.target === document.body) {
            bottom.removeAttribute("active");
            window.removeEventListener("click", window_click_handler);
        }
    })

    const window_click_handler = e => {
        if (e.target !== bottom_content && !bottom_content.contains(e.target) && e.target != logo) {
            window.removeEventListener("click", window_click_handler);
            bottom.removeAttribute("active");
        }
    }

    hamburger.addEventListener("click", () => {
        const is_active = bottom.getAttribute("active") != null;

        if (!is_active) {
            bottom.setAttribute("active", "");
            setTimeout(() => window.addEventListener("click", window_click_handler), 0);
        } else bottom.removeAttribute("active");
    })
})()