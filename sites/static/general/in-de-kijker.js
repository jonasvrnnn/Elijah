(() => {
    let index = 0;

    const carousel = document.currentScript.parentNode;
    const content = carousel.querySelector(".content");
    const previous = carousel.querySelector("[type='previous']");
    const next = carousel.querySelector("[type='next']");
    const items = content.querySelectorAll(".item");
    const count = items.length;

    const update_index = i => {
        if (i == undefined) i = 0;
        else if (i < 0) i = 0;
        else if (i > count - 1) i = count - 1;

        index = i;

        previous.toggleAttribute('hidden', i <= 0);
        next.toggleAttribute('hidden', i >= items.length - 1);

        carousel.style.setProperty("--index", i);
    };

    previous.addEventListener('click', () => update_index(index - 1));
    next.addEventListener('click', () => update_index(index + 1));
    update_index(0);

    document.addEventListener('swiped', e => {
        if (e.target !== carousel && !carousel.contains(e.target)) return;

        switch (e.detail.dir) {
            case "left": update_index(index + 1); break;
            case "right": update_index(index - 1); break;
        }

        this.set_index(this.index[0], new_sub_index)
    });
})()