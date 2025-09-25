function update_header_photo(element, multi = false) {
    let input = element ?? document.body.querySelector("#header-photo-input");

    if (!multi) {
        BynderCompactView.open({
            container: bynder_container,
            onSuccess: function (assets, selectedFile) {
                let asset = assets[0];

                let data = {
                    image: selectedFile.selectedFile.url
                }

                let copyright = asset?.copyright;

                if (copyright) {
                    data.copyright = copyright;
                }

                input.setAttribute("hx-vals", JSON.stringify(data))

                let event = new Event("change");
                input.dispatchEvent(event);
                bynder_container.innerHTML = '';
            },
            portal: {
                url: 'mediabox.groepvanroey.be'
            },
            mode: "SingleSelectFile",
            assetFieldSelection: `
            files
            copyright
        `
        })
    } else {
        BynderCompactView.open({
            container: bynder_container,
            onSuccess: function (assets) {
                let list = assets.map(asset => ({
                    image: (asset.files.Webp ?? asset.files.Presentatie ?? asset.files.original)?.url,
                    copyright: asset.copyright ?? undefined
                }))

                let data = {
                    images: list
                }

                input.setAttribute("hx-vals", JSON.stringify(data))

                let event = new Event("change");
                input.dispatchEvent(event);
                bynder_container.innerHTML = '';
            },
            portal: {
                url: 'mediabox.groepvanroey.be'
            },
            mode: multi ? "MultiSelect" : "SingleSelectFile",
            assetFieldSelection: `
            files
            copyright
        `
        })
    }
}