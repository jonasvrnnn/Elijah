use maud::{Markup, PreEscaped, html};

use crate::{
    admin::modules::{
        banner::template::project_banner, company_list::{service::CompanyLists, template::company_list_template}, content::{service::ContentEntry, template::content_template}, core_numbers::{self, service::CoreNumber}, header::template::project_header, image::{service::ImageData, template::lightbox}, industry::{service::IndustryData, template::project_industries}, introduction::template::introduction_template, labels::{service::Label, template::labels_template}, location::template::project_location, party_list::{service::ProjectPartyData, template::party_list_template}, publiek_privaat::template::publiek_privaat_template, show_in_carousel::template::show_in_carousel_template, status::template::status_template, thumbnail::template::thumbnail_template, tms::service::ProjectTMData, visible::template::visible_template, weight::template::project_weight, year::template::project_year
    },
    self_replacing_svg,
};

use super::service::{ProjectItem, ProjectListItem};

pub fn page_buttons_update_wrapper(project_id: &str, draft: bool, content: &Markup) -> Markup {
    html!(
        (page_buttons(project_id, draft, true));
        (content)
    )
}

pub fn page_buttons(project_id: &str, draft: bool, hx_swap_oob: bool) -> Markup {
    let hx_swap_oob_value = if hx_swap_oob {
        Some(format!("#page-buttons[non-draft]"))
    } else {
        None
    };

    html!(
        div #page-buttons hx-swap-oob=[hx_swap_oob_value] non-draft[hx_swap_oob]  {
            button #revert-changes title="Revert all changes" hx-confirm="Are you sure you want to revert all changes?"  hx-post={"/api/projects/"(project_id)"/revert-changes"} disabled[!draft] hx-target="#page" hx-push-url={"/projects/"(project_id)} hx-disinherit="*" {
                (self_replacing_svg("/undo.svg"))
            }

            button #save-changes hx-post={"/api/projects/"(project_id)"/save-changes"} title="Save all changes" hx-confirm="Are you sure you want to save all changes?" disabled[!draft] hx-target="#page" {}

            button #unpublish hx-delete={"/api/projects/"(project_id)"/unpublish"} title="Unpublish the live site" hx-confirm="Are you sure you want to unpublish the live site? (A draft version will automatically be created)" hx-target="#page" { "UNPUBLISH" }
            
            button #delete hx-delete={"/api/projects/"(project_id)} title="Delete the project" hx-confirm="Are you sure you want to delete this project?" hx-swap="delete" hx-target={"li[data-project-id='"(project_id)"'"} hx-on::after-request="page.close()" { "DELETE" }

            button #close-dialog hx-preserve="true" hx-push-url="true" hx-get="/projects" title="Close page" hx-select="#main-content" hx-swap="outerHTML" hx-target="#main-content" {}
        }
    )
}

pub fn project_template(
    project: &ProjectItem,
    company_name: &Option<String>,
    company_lists: &CompanyLists,
    party_data: &ProjectPartyData,
    tm_data: &ProjectTMData,
    industry_data: &IndustryData,
    content: &Vec<ContentEntry>,
    core_numbers: &Vec<CoreNumber>,
    images: &Vec<ImageData>,
    labels: &Vec<Label>
) -> Markup {
    html!(
        div #bynder_container {}

        div #selects new[!project.published] {
            div #project-settings {
                (project_weight(&project.id, &company_name, &project.weight))
                (publiek_privaat_template(&project.id, &project.publiek_privaat, company_name.is_some()))
                (status_template(&project.id, project.status, company_name.is_some()))
                (show_in_carousel_template(&project.id, project.show_in_carousel, company_name.is_none()))
                (visible_template(&project.id, project.visible, company_name.is_none()))
                (thumbnail_template(&project.id, &project.thumbnail, company_name.is_none()))
                (labels_template(&project.id, labels, company_name.is_some()))
            }

            (company_list_template(&project.id, &company_name, &company_lists.active, &company_lists.non_active))

            (page_buttons(&project.id, project.draft ,false))

            style #selects-extra-styling {}
        }

        div #page-content data-project-id=(&project.id) {
            (project_header(&project.id, &project.name, &project.header_photo, &project.header_photo_copyright, !project.custom_header_photo.unwrap_or(false), company_name.is_some()))

            div.container-0 {
                div.project-introduction {
                    div.project-info disabled[company_name.is_some()] {
                        div.overlay disabled[company_name.is_some()] {}

                        (party_list_template(&project.id, &party_data.clients, &tm_data.clients, "client"))
                        (party_list_template(&project.id, &party_data.architects, &tm_data.architects, "architect"))
                        (party_list_template(&project.id, &party_data.contractors, &tm_data.contractors, "contractor"))
                        hr {}
                        (project_industries(&project.id, industry_data))
                        (project_year(&project.id, &project.year))
                        (project_location(&project.id, &project.location))
                    }

                    (introduction_template(&project.id, company_name, &project.introduction, !project.custom_introduction.unwrap_or(false)))
                }

                (core_numbers::template::core_numbers(&project.id, core_numbers))

                (content_template(&project.id, content, !project.custom_content.unwrap_or(false)))

                (project_banner(&project.id, &project.banner_photo, &project.banner_photo_copyright, !project.custom_banner_photo.unwrap_or(false)))

                (lightbox(&project.id, images, !project.custom_lightbox.unwrap_or(false)))
            }
        }

        script {
            (PreEscaped("
                (() => {
                    const dialog = document.currentScript.parentNode;
                    dialog.showModal();
                })()
            "))
        }
    )
}

pub fn project_list_item(project: &ProjectListItem) -> Markup {
    html!(
        li hx-get={"/projects/"(project.id)} data-project-id=(project.id) hx-target="#page" hx-swap="innerHTML" hx-push-url="true" hx-select="#page > *" {
            div.bar {
                @if !project.published {
                    div.new { "NEW" }
                } @else if project.draft {
                    div.draft { "DRAFT"}
                }
            }
            img src=[&project.header_photo] loading="lazy";
            span {
                (project.name)
            }
        }
    )
}

pub fn project_list(projects: &Vec<ProjectListItem>, page: &Option<Markup>) -> Markup {
    html!(
        section.projects #main-content {
            nav {
                div.search-wrapper {
                    input type="search" autocomplete="off" hx-trigger="input changed delay:500ms" hx-target="#projects-list" hx-select="#projects-list" hx-swap="outerHTML" hx-get="/projects" name="filter";
                    (self_replacing_svg("/search.svg"))
                }
                button {
                    (self_replacing_svg("/filter.svg"))
                }
            }

            ul #projects-list {
                div #project-add hx-preserve="true" hx-prompt="What's the project's name?" hx-post="/api/projects" hx-select="#projects-list" hx-target="#projects-list" hx-swap="outerHTML" hx-disinherit="*" {}
                @for project in projects {
                    (project_list_item(project))
                }
            }

            dialog #page {
                @if let Some(page) = page {
                    (page)
                }
            }
        }
    )
}

pub fn projects(projects: &Vec<ProjectListItem>) -> Markup {
    html!(
        @for project in projects {
                (project_list_item(project))
            }
    )
}
