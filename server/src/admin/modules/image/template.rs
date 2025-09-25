use maud::{Markup, html};

use crate::admin::modules::image::service::ImageData;

pub fn lightbox_image(project_id: &str, image: &ImageData) -> Markup {
    html!(
        div.lightbox-image-wrapper {
            img src=[&image.image] copyright=[&image.image_copyright];
            button.delete hx-delete={"/api/projects/"(project_id)"/lightbox/"(image.id.as_deref().unwrap_or_default())} hx-swap="delete" hx-target="closest .lightbox-image-wrapper" {}
        }
    )
}

pub fn lightbox(project_id: &str, images: &Vec<ImageData>, disabled: bool) -> Markup {
    html!(
        section #lightbox is="light-box" disabled[disabled] {
            @if disabled {
                div class="overlay" {
                    button hx-post={"/api/projects/"(project_id)"/lightbox/customise"} hx-target="#lightbox" hx-swap="outerHTML" { "Customise for this company" }
                }
            }


            @for image in images {
                (lightbox_image(project_id, image))
            }

            button #lightbox-add onclick="update_header_photo(this, true)" hx-ext="form-json" hx-trigger="change" hx-post={"/api/projects/"(project_id)"/lightbox"} hx-swap="beforebegin" hx-target="this" {}
        }
    )
}
