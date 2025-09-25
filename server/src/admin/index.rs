use axum::response::IntoResponse;
use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::{static_script, static_stylesheet};

fn nav(title: &str, show_log_out: bool) -> Markup {
    html!(
        nav #main-nav {
            h1 { (title) }
            @if show_log_out {
                button.button-0 hx-delete="/api/auth/logout" {
                    "LOGOUT"
                }
            }
        } 
    )
}

pub async fn index_page() -> impl IntoResponse {
    index(&None,&None)
}

pub fn index(list: &Option<Markup>, content: &Option<Markup>) -> Markup {
    html!(
        (DOCTYPE)
        html lang="en" {
            head {
                (static_stylesheet("/index.css"));
                (static_stylesheet("tms.css"));
                (static_stylesheet("main-content.css"));
                (static_stylesheet("project_page.css"));
                (static_stylesheet("page.css"));
                (static_stylesheet("control-menu.css"));
                (static_script("components/angled/angled.js", false))
                (static_script("htmx.js", false))
                (static_script("bynder.js", false))
                (static_script("yaz.js", false))
                (static_script("open_image.js", false))
                (static_script("sound-effect.js", false))
                (static_script("header-photo-update.js", false))
                (static_script("htmx-head.js", false))
                (static_script("autosize.js", false))
                (static_script("text-editor.js", false))
                (static_script("content-entry-update.js", false))
                (static_script("components/textarea-autosize.js", false))
                (static_script("control-menu.js", false))
                (static_script("components/carousel/carousel.js", false))
                (static_script("selectlist/selectlist.js", true))
                (static_script("selectlist/option.js", true))
                (static_script("components/lightbox.js", true))
                script src="https://unpkg.com/tiny-editor/dist/bundle.js" defer {}
            }

            body hx-boost {
                (nav("GROEP VAN ROEY - CONTENT EDITOR", true))

                ul #collections hx-target="#main-content" hx-swap="outerHTML" hx-push-url="true" hx-select="#main-content" {
                    li #collection-projects hx-get="/projects" {
                        div.icon style="mask-image: url(/static/assets/nav/projects.svg);" {}
                        "Projects"
                    }
                    li #collection-projects hx-get="/users" {
                        div.icon style="mask-image: url(/static/assets/nav/users.svg);" {}
                        "Users"
                    }
/*                     li hx-get="/companies" {
                        div.icon style="mask-image: url(/static/assets/nav/group_companies.svg);"{}
                        "Group companies"
                    }
                    li hx-get="/tms" {
                        div.icon style="mask-image: url(/static/assets/nav/tm.svg);"{}
                        "TM's"
                    }
                    li hx-get="/parties" {
                        div.icon style="mask-image: url(/static/assets/nav/external_parties.svg);"{}
                        "External parties"
                    }
                    li hx-get="/industries" {
                        div.icon style="mask-image: url(/static/assets/nav/industries.svg);"{}
                        "Industries"
                    } */

                    script {(PreEscaped(r#"
                        (() => {
                            const list = document.currentScript.parentNode;

                            list.querySelectorAll('li').forEach((item, index) => {
                                item.addEventListener('click', () => {
                                    console.log('test');
                                    list.style.setProperty('--index', index);
                                })
                            });
                        })();
                    "#))}
                }

                @if let Some(list) = list {
                    (list)
                }

                @if let Some(content) = content {
                    (content)
                }
            }
        }
    )
}