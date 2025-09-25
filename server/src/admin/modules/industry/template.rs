use maud::{html, Markup};

use super::service::IndustryData;

pub fn project_industries(project_id: &str, data: &IndustryData) -> Markup {
    html!(
        div.industries.draft {
            div.icon style="mask-image: url(/project_pagina/industry.svg)" {}
            ul hx-swap="outerHTML" hx-select="ul" hx-target="closest ul" {
                @for industry in &data.active {
                    @let name = &industry.industry;
                    li hx-delete={ "/api/projects/" (project_id) "/industries/" (name) } {
                        (name)
                    }
                }

                select hx-post={ "/api/projects/" (project_id) "/industries" } hx-trigger="change" name="industry" {
                    option { "Selecteer een sector" }

                    @for industry in &data.non_active {
                        @let name = &industry.industry;
                        option value=(name) {
                            (name)
                        }
                    }
                }
            }
        }
    )
}

pub fn full_list(industries: &Vec<String>) -> Markup {
    html! (
        ul.items #main-content {
            @for industry in industries {
                li {
                    (industry)
                }
            }
        }
    )
}

pub fn search_list_for_existing_project(
    industries: &Vec<String>
) -> Markup {
    html!(
        @for industry in industries {
            label.item {
                (industry)
                input type="radio" name="industry" value=(industry);
            }
        }
    )
}
