use maud::{Markup, PreEscaped, html};

pub fn project_year(project_id: &str, year: &Option<i64>) -> Markup {
    html!(
        div.year.draft hx-target="closest .year" hx-swap="outerHTML"{
            div.icon style="mask-image: url(/project_pagina/year.svg)" {}
            input placeholder="Year" type="number" autocorrect="off" spellcheck="false" autocomplete="off" step="1" min="2000" max="2035" value=[year] name="year" onkeydown=(PreEscaped("event.keyCode != 38 && event.keyCode != 40 && event.preventDefault()")) hx-patch={"/api/projects/"(project_id)"/year"} hx-trigger="change delay:.5s" {}
            @if year.is_some() {
                button hx-patch={"/api/projects/"(project_id)"/year"} { }
            }
        }
    )
}
