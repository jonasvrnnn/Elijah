use maud::{Markup, PreEscaped, html};

use crate::self_replacing_svg;

use super::service::CompanyListItem;

pub fn company_list_template<'a>(
    project_id: &'a str,
    current: &'a Option<String>,
    active: &'a Vec<CompanyListItem>,
    non_active: &'a Vec<CompanyListItem>,
) -> Markup {
    html!(
        details #company-select hx-target="#company-select" {
            summary title="Companies" {
                (current.as_deref().unwrap_or("BASE"))
            }

            div #company-select-items {
                @if current.is_some() {
                    button.base hx-get={"/projects/"(project_id)} hx-target="#page" hx-select="#page > *" hx-push-url="true"{
                        "Base"
                    }
                }

                @for company in active {
                    div.active-wrapper {
                        button.active name="company" value=(company.name) hx-get={"/projects/"(project_id)} hx-target="#page" hx-select="#page > *" hx-swap="innerHTML" hx-push-url="true" {
                            (company.name)
                        }
                        button.x hx-delete={"/api/projects/"(project_id)"/companies/"(company.name)} hx-target="#page" hx-select="#page > *" hx-swap="innerHTML" hx-push-url={"/projects/"(project_id)} name="current_company" value=[current] hx-confirm="Remove this company?" hx-disinherit="hx-confirm hx-select hx-push-url" {
                            (self_replacing_svg("/close.svg"))
                        }
                    }
                }
                hr {}
                @for company in non_active {
                    button.non-active hx-confirm="Add this company?" name="company" hx-get={"/projects/"(project_id)} hx-target="#page" hx-select="#page > *" hx-push-url="true" hx-swap="innerHTML" value=(company.name) {
                        (company.name)
                    }
                }

                script {
                    (PreEscaped("
                        (() => {
                            const parent = document.currentScript.parentNode;
                            document.addEventListener('click', e => {
                                if(e.target !== parent && !parent.contains(e.target)) {
                                    parent.parentNode.open = false;
                                }
                            });
                    })();"))
                }
            }
        }
    )
}

pub fn full_list(companies: &Vec<String>) -> Markup {
    html!(
        section.companies #main-content {
            control-menu #projects-nav hx-preserve="true" {
                button name="add" {
                    "➕"
                }

                form autocomplete="off" #projects-add content="add" hx-post="/api/projects" hx-target="#projects-list" hx-select="#projects-list" hx-swap="outerHTML" {
                    input type="text" placeholder="Choose a project name..." name="name";
                    input type="submit" value="➤";
                }

                button name="search" {
                    "🔎"
                }

                form autocomplete="off" #projects-filter content="search" {
                    input type="text" placeholder="Type a search filter..." hx-trigger="input changed delay:500ms" hx-target="#projects-list" hx-select="#projects-list" hx-swap="outerHTML" hx-get="/api/projects" name="filter";
                }
            }

            ul.companies #companies-list {
                @for company in companies {
                    li {
                        div.img-wrapper {
                            img.favicon;
                            img.logo;
                        }
                        span {
                            (company)
                        }
                    }
                }
            }

            div #company-menu {

            }
        }
    )
}

pub fn search_list(companies: &Vec<String>) -> Markup {
    html!(
        @for company in companies {
            label.item {
                (company)
                input type="radio" name="company" value=(company);
            }
        }
    )
}
