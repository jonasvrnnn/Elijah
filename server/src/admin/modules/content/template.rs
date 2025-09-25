use maud::{Markup, PreEscaped, html};


use super::service::ContentEntry;

pub fn content_entry_image(project_id: &str, content_entry_id: &str, image: &Option<String>, copyright: &Option<String>) -> Markup {
    html!(
        div.img-wrapper {
            button onclick="update_header_photo(this)" hx-trigger="change" hx-patch={"/api/projects/"(project_id)"/content/"(content_entry_id)"/image"} hx-swap="outerHTML" hx-target="closest .img-wrapper" hx-select=".img-wrapper" class="edit" {}

            @if image.is_some() {
                button.delete hx-patch={"/api/projects/"(project_id)"/content/"(content_entry_id)"/image"} hx-target="closest .img-wrapper" hx-swap="outerHTML" {}
            }

            input type="hidden" name="image" value=[image] hx-trigger="change" hx-patch={"/api/projects/"(project_id)"/content/"(content_entry_id)"/image"} hx-target="closest .img-wrapper" hx-swap="outerHTML";
            img src=[&image];

            @if let Some(copyright) = copyright {
                span .copyright {
                    { (copyright) }
                }
            }
        }
    )
}

pub fn content_entry_quote(project_id: &str, content_entry_id: &str, quote: &Option<String>) -> Markup {
    html!(
        textarea.quote is="auto-size" placeholder="Type a quote here" name="content" hx-trigger="change" hx-patch={"/api/projects/"(project_id)"/content/"(content_entry_id)"/quote"} {
            @if let Some(quote) = quote {
                (quote)
            }
        }
    )
}

pub fn content_entry_text(project_id: &str, content_entry_id: &str, text: &str) -> Markup {
    html!(
        div.text hx-trigger="change" hx-patch={"/api/projects/"(project_id)"/content/"(content_entry_id)"/text"}{
            div is="text-editor" {
                (PreEscaped(text))
            }
        }
    )
}

pub fn content_entry_template(
    project_id: &str,
    content_entry: &ContentEntry,
    disabled: bool,
) -> Markup {
    html!(
        div.entry.disabled[disabled] data-id=(content_entry.id) {
            div.content-entry-content {
                input type="hidden" name="previous_id" value=(content_entry.id);

                (content_entry_text(project_id, &content_entry.id, &content_entry.text))

                div.config-wrapper {
                    div.config {
                        button hx-delete={"/api/projects/"(project_id)"/content/"(content_entry.id)} hx-swap="delete" hx-target="closest .entry" {
                            "❌"
                        }
                    }
                }

                (content_entry_image(project_id, &content_entry.id, &content_entry.image, &content_entry.image_copyright))

                (content_entry_quote(project_id, &content_entry.id, &content_entry.quote))

                @if disabled {
                    div.overlay {
                        "This page's content is currently not customised for this company."
                        button hx-post={"/api/projects/"(project_id)"/content/customise"} hx-target="#content" hx-swap="outerHTML" {
                            "Customise"
                        }
                    }
                }
            }

            button hx-post={"/api/projects/"(project_id)"/content"} name="previous_id" value=(content_entry.id) hx-swap="afterend" hx-target="closest .entry" class="add-content-entry" {}
        }
    )
}

pub fn content_template(
    project_id: &str,
    content: &Vec<ContentEntry>,
    disabled: bool,
) -> Markup {
    html!(
        section #content {
            button.add-content-entry.first hx-post={"/api/projects/"(project_id)"/content"} hx-swap="afterend" {}

            @for entry in content {
                (content_entry_template(project_id, entry, disabled))
            }
        }
    )
}
