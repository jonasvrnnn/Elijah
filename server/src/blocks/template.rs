use maud::{html, Markup};

pub fn tree_item(element_id: &str, element_name: &str, child_count: usize, swap_oob: bool) -> Markup {
    let swap_oob = if swap_oob {
        Some(format!("outerHTML:[element-id='{element_id}']"))
    } else {
        None
    };

    html!(
        li element-id=(element_id) hx-swap-oob=[swap_oob] {
            div.move {}
            div.dropzone.up {}
            span hx-get={"/api/blocks/"(element_id)"/properties"} hx-swap="outerHTML" hx-target="#main-content.site-editor > form" { (element_name)"-"(child_count) }
            div.dropzone.down {}
            button.show-children hx-get="/api/blocks/tree" name="parent" value=(element_id) hx-target="closest ul" {}
            button.delete hx-delete={"/api/blocks/delete-element/"(element_id)} hx-swap="delete" hx-target="closest li"  hx-on::before-cleanup-element="refresh_preview()" {}
            input type="hidden" name="element_id" value=(element_id);
        }
    )
}
