(() => {
    const parent = document.currentScript.parentNode;

    const close_details_eventlistener = e => {
        if (e.target !== parent && !parent.contains(e.target) && e.target !== bynder_container && document.contains(e.target)) {
            parent.toggleAttribute("open", false);

            window.removeEventListener("click", close_details_eventlistener);
        }
    }

    parent.addEventListener("toggle", _ => {
        if (parent.open) {
            window.addEventListener("click", close_details_eventlistener);
        }
    })
})()