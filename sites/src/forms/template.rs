
use maud::{html, Markup};

use crate::forms::structs::FormBody;

pub fn form_notification(data: &FormBody) -> Markup {
    html!(
        ul {
            li {
                "Voornaam: "
                strong {
                    (data.first_name)
                }
            }
            li {
                "Familienaam: "
                strong {
                    (data.last_name)
                }
            }
            li {
                "E-mailadres: "
                strong {
                    (data.email)
                }
            }
            li {
                "Telefoonnummer: "
                strong {
                    (data.phone)
                }
            }
            li {
                "Bericht: "
                strong {
                    (data.message)
                }
            }
        }
    )
}