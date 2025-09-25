use maud::{Markup, PreEscaped, html};

use crate::cookies::service::CookieData;

pub fn cookie_items(data: &CookieData, state: &str) -> Markup {
    let analytics_value = if data.analytics { "granted" } else { "denied" };
    
    html!(
        form #cookie-preferences hx-swap-oob="innerHTML" {
            div.item {
                p {
                    "Deze items helpen de website-exploitant begrijpen hoe de website presteert, hoe bezoekers met de site
                    interageren en of er mogelijk technische problemen zijn. Dit opslagtype verzamelt doorgaans geen
                    informatie die een bezoeker identificeert."
                }

                label {
                    "Analytics"
                    input type="checkbox" name="analytics" value="true" checked[data.analytics] autocomplete="off";
                }
            }

            input type="submit" value="Opslaan";

            script {
                (PreEscaped(format!("
                    (() => {{
                        gtag('consent', 'update', {{
                            'analytics_storage': '{analytics_value}'
                        }});
                    }})()
                ")))

                (PreEscaped(format!("cookies.setAttribute('state', '{state}')")))
            }
        }
    )
}
