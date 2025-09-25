use maud::{Markup, html};

use super::service;

pub fn publiek_privaat_template(project_id: &str, r#type: &str, disabled: bool) -> Markup {
    let (current, next) = service::get_current_and_next(&r#type);
    
    html!(        
        button #publiek-privaat title="Aanbesteding" disabled[disabled] hx-patch={"/api/projects/"(project_id)"/publiek-privaat"} name="type" hx-swap="outerHTML" value=(next.1) {
            (current.0)
        }
    )
}