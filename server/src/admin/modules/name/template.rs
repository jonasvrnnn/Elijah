use maud::{Markup, html};

pub fn name_template(project_id: &str, name: &str, disabled: bool) -> Markup {
    html!(
        textarea.project-name disabled[disabled] autocomplete="off" is="auto-size" onclick="event.stopImmediatePropagation()" hx-patch={"/api/projects/"(project_id)"/name"} name="name" hx-trigger="change changed" hx-target="this" hx-select=".project-name" hx-swap="outerHTML" {
                (name)
            }
    )
}
