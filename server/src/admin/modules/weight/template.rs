use maud::{Markup, html};

pub fn project_weight(
    project_id: &str,
    company_name: &Option<String>,
    weight: &Option<i64>,
) -> Markup {
    let hx_patch = company_name
        .as_deref()
        .map(|cn| format!("/api/projects/{project_id}/weight?company={cn}"));

    html!(
        div #project-weight title="Weight" disabled[company_name.is_none()] {
            label { (weight.map(|w| w.to_string()).unwrap_or("/".to_string())) }
            input type="range" min="0" max="100" step="1" autocomplete="off" oninput="this.previousSibling.innerText = this.value" value=[weight] name="weight" hx-patch=[hx_patch]  hx-trigger="change" hx-swap="none" hx-target="#project-weight";
        }
    )
}
