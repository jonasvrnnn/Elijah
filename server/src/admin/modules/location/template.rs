use maud::{html, Markup};

pub fn project_location(project_id: &str, location: &Option<String>) -> Markup {
    html!(
        div.year.draft hx-swap="outerHTML" {
            div.icon style="mask-image: url(/project_pagina/location.svg)" {}
            input placeholder="Location" value=[location] name="location" hx-patch={"/api/projects/"(project_id)"/location"} hx-trigger="change" hx-select="input" {}
        }
    )
}