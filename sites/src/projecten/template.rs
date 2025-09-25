use maud::{Markup, PreEscaped, html};

use crate::projecten::{
    endpoint::ProjectItem,
    service::{CarouselProject, ContentEntry, CoreNumber, GroupCompany, ImageData, Project},
};

pub fn basic_list(parties: &Vec<String>, r#type: &str) -> Markup {
    html!(
        div.item {
            div style={"mask-image: url(/assets/project_page/"(r#type)".svg"} {}
            ul {
                @for party in parties {
                    li {
                        (party)
                    }
                }
            }
        }
    )
}

pub fn basic_field(value: &str, r#type: &str) -> Markup {
    html!(
        div.item {
            div style={"mask-image: url(/assets/project_page/"(r#type)".svg"} {}
            span { (value) }
        }
    )
}

pub fn group_company_list(group_companies: &Vec<GroupCompany>) -> Markup {
    html!(
        div.item {
            img src="https://mediabox.groepvanroey.be/m/6f677e9897d1f760/original/corporate-emblem.png";
            ul {
                @for group_company in group_companies {
                    @if let Some(name) = &group_company.name {
                        li {
                            (name)
                        }
                    }
                }
            }
        }
    )
}

pub fn project_template(
    project: &ProjectItem,
    group_companies: &Vec<GroupCompany>,
    core_numbers: &Vec<CoreNumber>,
    content_entries: &Vec<ContentEntry>,
    images: &Vec<ImageData>,
) -> Markup {
    html!(
        title { (project.name) }
        angled-element {
            section #main class="main-0" style={"background-image: url(\""(project.header_photo.as_deref().unwrap_or_default())"\")"} {
                h1.project { (project.name )}
                @if let Some(copyright) = &project.header_photo_copyright {
                    span.copyright { (copyright) }
                }
            }
        }
        div.container-0 {
            section #introduction {
                @if project.clients.0.len() > 0 { (basic_list(&project.clients.0, "client")) }
                @if project.architects.0.len() > 0 { (basic_list(&project.architects.0, "architect")) }
                @if project.contractors.0.len() > 0 { (basic_list(&project.contractors.0, "contractor")) }
                (group_company_list(group_companies))
                hr;
                (basic_list(&project.industries.0, "industry"))
                @if let Some(year) = project.year {
                    (basic_field(&year.to_string(), "year"))
                }
                @if let Some(location) = &project.location {
                    (basic_field(&location, "location"))
                }

                div.content {
                    (PreEscaped(project.introduction.as_deref().unwrap_or_default()))
                }
            }

            @if core_numbers.len() > 0 {
                ul #core-numbers {
                    @for core_number in core_numbers {
                        li {
                            span.number { (core_number.number) }
                            span.title { (core_number.title) }
                        }
                    }
                }
            }

            section #content {
                @for entry in content_entries {
                    div.entry {
                        @if let Some(image) = &entry.image {
                            div.img-wrapper {
                                img src=(image) loading="lazy";
                                @if let Some(copyright) = &entry.image_copyright {
                                    span.copyright { (copyright) }
                                }
                            }
                        }
                        div.content {
                            (PreEscaped(entry.text.as_deref().unwrap_or_default()))
                        }
                        @if let Some(quote) = &entry.quote {
                            blockquote { (quote) }
                        }
                    }
                }

                @if images.len() > 0 {
                    section is="light-box" {
                        @for image in images {
                            img src=[&image.image] copyright=[&image.image_copyright];
                        }
                    }
                }
            }

            @if let Some(banner_image) = &project.banner_photo {
                angled-element #project-banner angle-top="0" triangle-bottom="false" angle-bottom="0" triangle-top triangle-top-size="45" {
                    img loading="lazy" src=(banner_image);
                    @if let Some(copyright) = &project.banner_photo_copyright {
                        span.copyright { (copyright) }
                    }
                }
            }

            (PreEscaped(r#"<!--#include virtual="/map.html" -->"#))
        }
    )
}

pub fn carousel(projects: &Vec<CarouselProject>) -> Markup {
    html!(
        @for project in projects  {
            div.main-content {
                img.background src=[&project.header_photo];
                div.project-info {
                    span.name {
                        (project.name)
                    }
                    @if let Some(location) = &project.location {
                        (location)
                    }
                    a href={"/projecten/"(project.slug.as_deref().unwrap_or_default())} { "Bekijk project" }
                }
            }
        }
    )
}

pub fn in_de_kijker(projects: &Vec<CarouselProject>) -> Markup {
    html!(
        section.project-carousel {
            div.content {
                @for project in projects  {
                    a.item href={"/projecten/"(project.slug.as_deref().unwrap_or_default())} {
                        img src=(project.header_photo.as_deref().unwrap_or_default());
                        span { (project.name)}
                    }
                }
            }

            button type="previous" {}
            button type="next" {}

            script src="/in-de-kijker.js" {}
        }
    )
}

pub fn projects(projects: &Vec<Project>, limit: u8, query_string: &str) -> Markup {
    html!(
        @for project in projects {
            a.project href={"/projecten/"(project.slug)} {
                img src=[&project.thumbnail] loading="lazy";
                div.name {(project.name)}
                div.location { (project.location.as_deref().unwrap_or_default()) }
            }
        }

        @if projects.len() == limit as usize {
                div style="display: hidden;"
                    hx-get={"/api/projecten?"(query_string)}
                    hx-target="this"
                    hx-swap="outerHTML"
                    hx-trigger="revealed" {}
        }
    )
}
