use maud::{Markup, html};

use super::super::tms::service::TM;
use super::service::Party;

pub fn party_list_template(
    project_id: &str,
    parties: &Vec<Party>,
    tms: &Vec<TM>,
    r#type: &str,
) -> Markup {
    html!(
        div {
            div.icon style={"mask-image: url(/project_pagina/" (r#type) ".svg)"} {}

            ul {
                @for party in parties {
                    li.draft hx-delete={"/api/projects/"(project_id)"/parties"} hx-target="closest ul" hx-select="ul" hx-swap="outerHTML" hx-vals={
                        r#"{"party":""#(party.name)r#"","type":""#(r#type)r#""}"#
                    } {
                        @if let Some(url) = &party.url {
                            a href=(url) {
                                (party.name)
                            }
                        } @else {
                            (party.name)
                        }

                        input class=(r#type) type="hidden" name="exclude" value=(party.name);
                    }
                }

                @for tm in tms {
                    li.draft hx-delete={"/api/projects/"(project_id)"/tms"} hx-target="closest ul" hx-select="ul" hx-swap="outerHTML" hx-vals={
                        r#"{"party":""#(tm.name)r#"","type":""#(r#type)r#""}"#
                    } {
                        (tm.name)
                    }
                }

                div.party-add-form {
                    input autocorrect="off" spellcheck="false" autocomplete="off" name="filter" placeholder="Type a party name" hx-get={"/api/parties/search"} hx-trigger="focus, input changed delay:500ms" hx-target="next .results" hx-include={"next [name='type'], next [name='project_id'], [name='exclude']."(r#type)} {}
                    input type="hidden" name="type" value=(r#type);
                    input type="hidden" name="project_id" value=(project_id);
                    form.results hx-post={"/api/projects/"(project_id)"/parties"} hx-trigger="change" hx-include="previous [name='type']" hx-select="ul" hx-target="closest ul" hx-swap="outerHTML" {

                    }
                }
            }
        }
    )
}

pub fn full_list(parties: &Vec<Party>) -> Markup {
    html!(
        ul.items #main-content {
            @for party in parties {
                li {
                    (party.name)
                }
            }
        }
    )
}

pub fn search_list_for_existing_project(
    parties: &Vec<String>,
    tms: &Vec<String>,
    name: &Option<String>,
) -> Markup {
    html!(
        label.title { "Parties" }
        @if parties.len() == 0 {
                @if let Some(name) = name {
                    label.item {
                        "Add as new party"
                        input type="radio" name="party" value=(name);
                    }
                }
            }

            @for party in parties {
                label.item {
                    (party)
                    input type="radio" name="party" value=(party);
                }
            }
        hr {}
        label.title { "TM's" }
        @if tms.len() == 0 {
                @if let Some(name) = name {
                    label.item {
                        "Add as new tm"
                        input type="radio" name="tm" value=(name);
                    }
                }
            }

            @for tm in tms {
                label.item {
                    (tm)
                    input type="radio" name="tm" value=(tm);
                }
            }
    )
}
