use maud::{Markup, html};


pub fn status_template(project_id: &str, checked: bool, disabled: bool) -> Markup {
    html!(
        button #status autocomplete="off" title="Status" hx-patch={"/api/projects/"(project_id)"/status"} disabled[disabled] name="status" value=(!checked) hx-swap="outerHTML" {
            
        }
    )
}