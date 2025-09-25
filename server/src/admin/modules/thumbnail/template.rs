use maud::{Markup, html};

pub fn thumbnail_template(project_id: &str, value: &Option<String>, base_disabled: bool) -> Markup {
    let base_checked = value.as_deref().map(|v| v == "base").unwrap_or(false);
    let headerphoto_checked = value.is_none();
    let custom_checked = value.as_deref().map(|_| !(base_checked || headerphoto_checked) ).unwrap_or(false);

    let value = if base_checked || headerphoto_checked {
        &None
    } else {
        value
    };

    html!(
        details #thumbnail {
            summary {

            }
            form hx-patch={"/api/projects/"(project_id)"/thumbnail"} hx-swap="innerHTML" hx-trigger="change" hx-select="img" hx-target="div.image" {
                label disabled[base_disabled] { "Base" input type="radio" name="source" value="base" checked[base_checked]; }
                label { "Headerphoto" input type="radio" name="source" value="headerphoto" checked[headerphoto_checked]; }
                label { "Custom" input type="radio" name="source" value="custom" checked[custom_checked]; }
                div.image onclick="update_header_photo(this.parentNode)" {
                    img src=[value];
                }
            }
            script src="/thumbnail.js" {}
        }
    )
}