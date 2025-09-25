use maud::{html, Markup};

use crate::{admin::modules::name::template::name_template, self_replacing_svg};

pub fn project_header(project_id: &str, name: &str, image: &Option<String>, copyright: &Option<String>, image_disabled: bool, name_disabled: bool) -> Markup {
    html!(
        div.main.main-content.draft is="angled-element" {
            div.project {
                (name_template(project_id, name, name_disabled))

                div.image-buttons {
                    @if image_disabled {
                        button.customise hx-post={"/api/projects/"(project_id)"/header-image/customise"} hx-swap="outerHTML" hx-select=".image-buttons" hx-target=".image-buttons" {
                            "Customise"
                        }
                    } @else {
                        button.edit onclick="update_header_photo()" {
                            (self_replacing_svg("/edit.svg"))
                        }
                        button.delete hx-patch={"/api/projects/"(project_id)"/header-image"} hx-target="closest div.main" hx-swap="outerHTML" {
                            (self_replacing_svg("/clear.svg"))
                        }
                    }
                }
            }
            input #header-photo-input type="hidden" name="image" hx-trigger="change" hx-patch={"/api/projects/"(project_id)"/header-image"} hx-target="closest div.main" hx-swap="outerHTML" ;
            img.background src=[image] loading="lazy" {}
            @if let Some(copyright) = copyright {
                span .copyright style="margin-bottom: calc(1rem + var(--angled-padding-bottom));" {
                    { "© " (copyright) }
                }
            }
        }
    )
}