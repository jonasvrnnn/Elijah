const open_image = (target, path) => {
    BynderCompactView.open({
        container: bynder_container,
        onSuccess: function (assets) {
            htmx.ajax('PATCH', path, {
                swap: `outerHTML`,
                target: target,
                values: {
                    src: assets[0].files.Webp.url
                }
            })
            .then(() => bynder_container.innerHTML = '')
        },
        portal: {
            url: 'mediabox.groepvanroey.be'
        },
        mode: "SingleSelectFile"
    })
}