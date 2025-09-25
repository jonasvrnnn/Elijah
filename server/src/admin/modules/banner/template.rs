use maud::{Markup, html};

pub fn project_banner(
    project_id: &str,
    image: &Option<String>,
    copyright: &Option<String>,/* 
    alt: &Option<String> */
    disabled: bool
) -> Markup {
    html!(
        div #project-banner is="angled-element" hx-on::after-settle="this.render()" angle-top="0" triangle-bottom="false" angle-bottom="0" triangle-top triangle-top-size="45" hx-target="#project-banner" hx-swap="outerHTML" {
            @if disabled {
                div class="overlay" {
                    button hx-post={"/api/projects/"(project_id)"/banner-image/customise"} hx-swap="outerHTML" hx-target="#project-banner" { "Customise" }
                }
            }
            
            img loading="lazy" src=[image] /* alt=[alt] */;
            button.edit hx-trigger="change" hx-patch={"/api/projects/"(project_id)"/banner-image"} onclick="update_header_photo(this)"{}
            @if image.is_some(){
                button.clear hx-patch={"/api/projects/"(project_id)"/banner-image"} {}
            }

            @if let Some(copyright) = copyright {
                span .copyright {
                    (copyright)
                }
            }
        }
    )
}
