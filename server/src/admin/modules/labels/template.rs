use maud::{Markup, html};

use crate::admin::modules::labels::service::Label;

pub fn labels_template(
    project_id: &str,
    labels: &Vec<Label>,
    disabled: bool,
) -> Markup {
    html!(
        details #labels {
            summary {

            }
            form hx-put={{"/api/projects/"(project_id)"/labels"}} hx-trigger="change" hx-select="label" autocomplete="off" {
                @for label in labels {
                    label {
                        (label.name)
                        input type="checkbox" name="labels" value=(label.name) checked[label.active];
                    }
                }
            }
            script src="/labels.js" {}
        }
    )
}
