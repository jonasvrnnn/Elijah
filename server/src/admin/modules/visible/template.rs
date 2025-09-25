use maud::{Markup, html};

pub fn visible_template(project_id: &str, checked: bool, disabled: bool) -> Markup {
    html!(
        button #visible autocomplete="off" title="Visible" checked[checked] disabled[disabled] hx-patch={"/api/projects/"(project_id)"/visible"} name="visible" value=(!checked) hx-swap="outerHTML" {}
    )
}