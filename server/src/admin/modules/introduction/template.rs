use maud::{html, Markup, PreEscaped};

pub fn introduction_template(project_id: &str, company_name: &Option<String>, introduction: &Option<String>, disabled: bool) -> Markup {
    html! {
        div #introduction hx-trigger="change" hx-swap="none" hx-patch={"/api/projects/"(project_id)"/introduction"(company_name.as_deref().map(|c| format!("?company={c}")).unwrap_or_default())} {
            @if disabled {
                div class="overlay" {
                    button hx-post={"/api/projects/"(project_id)"/introduction/customise"} hx-swap="outerHTML" hx-target="#introduction" { "Customise" }
                }
            }

            div.content-0 is="text-editor" {
                @if let Some(content) = introduction {
                    (PreEscaped(content))
                }
            }
        }
    }
}