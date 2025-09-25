use maud::{html, Markup};

pub fn inner_text(field_name: &str, value: &Option<String>, multiline: bool) -> Markup {
    html!(
        @if multiline {
            textarea name=(field_name) {
                (value.as_deref().unwrap_or_default())
            }
        } @else {
            input name=(field_name) placeholder="Type text" value=[value];
        }
    )
}

pub fn number(field_name: &str, value: &Option<f32>, min: Option<f32>, max: Option<f32>, step: Option<f32>) -> Markup {
    html!(
        input type="number" name=(field_name) placeholder="Type text" min=[min] max=[max] value=[value] step=[step];
    )
}

pub fn boolean(field_name: &str, value: &Option<bool>) -> Markup {
    html!(
        input type="checkbox" name=(field_name) checked=[value] value="true";
    )
}

pub fn select(field_name: &str, value: &Option<String>, options: &Vec<&str>) -> Markup {
    html!(
        select name=(field_name) {
            option { }
            @for option in options {
                @let selected = value.as_deref() == Some(option);
                option value=(option) selected[selected] { (option) }
            }
        }
    )
}