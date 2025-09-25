function update_content_entry(field) {
    const entry = field.closest(".entry");

    if(!entry) { alert("There was an error when updating the content entry.") }

    const project_id = entry.closest("[data-project-id]").getAttribute("data-project-id");
    const entry_id = entry.closest("[data-id]").getAttribute("data-id");
    
    const text = entry.querySelector('.text')?.innerHTML;

    const image = entry.querySelector(".img-wrapper > img")?.getAttribute("src");

    let values = {
        text
    };

    if(image) values.image = image;

    htmx.ajax("PUT", `/api/admin/projects/${project_id}/content/${entry_id}`, {
        target: entry,
        swap: "outerHTML",
        values,
        select: 'div.entry'
    })
}

function clear_image(field) {
    const img = field.parentElement.querySelector("img");
    img.src = '';

    update_content_entry(field);
}

function update_content_entry_image(field) {
    let input = field.parentNode.querySelector("input[name='image']");

    BynderCompactView.open({
        container: bynder_container,
        onSuccess: function (assets) {
            let asset = assets[0];

            let copyright = asset?.metaproperties?.nodes?.find(n => n.name === 'Fotograaf')?.options[0]?.displayLabel;

            input.setAttribute("hx-vals", JSON.stringify({
                image: asset.files.Webp.url,
                copyright
            }))

            let event = new Event("change");
            input.dispatchEvent(event);
            bynder_container.innerHTML = '';
        },
        portal: {
            url: 'mediabox.groepvanroey.be'
        },
        mode: "SingleSelectFile"
    })
}

function move_content_entry(entry, after = true) {
    const sibling = entry.nextElementSibling;

    if(sibling) {
        sibling.insertAdjacentElement("afterend", entry);
    }
}