(() => {
    const form = document.currentScript.parentNode.querySelector("form");
    const submit = form.querySelector("[type='submit']");

    form.addEventListener("submit", e => {
        e.preventDefault();

        grecaptcha.ready(function () {
            grecaptcha.execute('6LfWllorAAAAAB6A_wUGzcCqWWBiF5GMHl6xcbQI', { action: 'submit' }).then(function (token) {
                htmx.ajax("POST", "/api/forms", {
                    source: form,
                    target: form.closest(".container-0"),
                    headers: {
                        "recaptcha": token
                    }
                })
            });
        });
    })
})()