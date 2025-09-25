use maud::{Markup, html};

use crate::self_replacing_svg;

use super::service::CoreNumber;

pub fn core_number_template(project_id: &str, core_number: &CoreNumber) -> Markup {
    html!(
        li.core-number {
            form hx-put={"/api/projects/"(project_id)"/core_numbers/"(core_number.id)} hx-trigger="change" hx-swap="outerHTML" {
                input.core-number-number name="number" value=(core_number.number) placeholder="Number" autocomplete="off";
                textarea.core-number-title is="auto-size" name="title" autocomplete="off" placeholder="Title" { (core_number.title) }
            }
            button.core-number-delete hx-delete={"/api/projects/"(project_id)"/core_numbers/"(core_number.id)} hx-swap="delete" hx-target="closest .core-number" {}
        }
    )
}

pub fn core_numbers(project_id: &str, core_numbers: &Vec<CoreNumber>) -> Markup {
    html!(
        ul #core-numbers {
            @for core_number in core_numbers {
                (core_number_template(project_id, core_number))
            }

            button #add-core-number hx-post={"/api/projects/"(project_id)"/core_numbers"} hx-swap="beforebegin" {
                (self_replacing_svg("/plus.svg"))
            }
        }
    )
}
