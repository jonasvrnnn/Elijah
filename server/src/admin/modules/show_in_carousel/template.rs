use maud::{Markup, html};


pub fn show_in_carousel_template(project_id: &str, checked: bool, disabled: bool) -> Markup {
    html!(
        button #show-in-carousel autocomplete="off" title="Show in carousel" checked[checked] disabled[disabled] hx-patch={"/api/projects/"(project_id)"/show-in-carousel"} name="show_in_carousel" value=(!checked) hx-swap="outerHTML" {}
    )
}