use maud::{Markup, PreEscaped, html};


use super::models::{Permission, UserBase};

pub fn totp_verify_form(email: &str, svg: Option<String>) -> Markup {
    html!(
        form hx-post="/auth/totp/verify" {
            @if let Some(svg) = svg {
                (PreEscaped(svg))
            }
            input type="hidden" name="email" value=(email);
            input name="code" placeholder="code";
            input type="submit" value="verify";
        }
    )
}


pub fn user_list(users: &Vec<UserBase>) -> Markup {
    html!(
        @for user in users {
            (user_template(user))
        }
    )
}

pub fn user_template(user: &UserBase) -> Markup {
    html!(
        details {
            summary.name {
                (user.first_name) " " (user.last_name)
            }

            div.user-content {
                form style="display: contents;" hx-put={"/api/auth/users/"(user.id)} hx-select="form" hx-swap="outerHTML" hx-trigger="change" autocomplete="off" {
                    label {
                        "Email: "
                        input name="email" value=(user.email);
                    }

                    label {
                        "Role: "
                        select name="role" {
                            option value="admin" selected[user.role == "admin"] {
                                "Admin"
                            }
                            option value="user" selected[user.role == "user"] {
                                "User"
                            }
                        }
                    }
                }

                div.toggles {
                    label { "Permissions" }

                    div.table {
                        span{}
                        span style="text-align: center" {
                            "Create"
                        }
                        span style="text-align: center" {
                            "Edit"
                        }

                        div.table hx-trigger={"click from:closest details"} hx-get={"/api/auth//users/"(user.id)"/permissions"} hx-swap="outerHTML" {}
                    }
                }

                button hx-delete={"/api/auth/users/"(user.id)} hx-target="closest details" hx-swap="delete" hx-confirm="Do you really want to delete this user?" {
                    "DELETE"
                }
            }
        }
    )
}

pub fn permission_row(permission: &Permission, user_id: &str, company_name: &str, company_value: &Option<String>) -> Markup {
    html!(
        form hx-patch={"/api/auth//users/"(user_id)"/permissions"(company_value.as_deref().map(|c| format!("?company={c}")).unwrap_or_default())} hx-swap="outerHTML" hx-trigger="change" {
            span {
                (company_name)
            }
            input name="create" type="checkbox" value="true" checked[permission.create.unwrap_or(false)];

            input name="edit" type="checkbox" value="true" checked[permission.edit.unwrap_or(false)];

        }
    )
}

pub fn permissions(permissions: &Vec<Permission>, user_id: &str) -> Markup {
    html!(
        @for permission in permissions {
            @let company = permission.company.as_deref().unwrap_or("Base");
            (permission_row(&permission, user_id, &company, &permission.company))
        }
    )
}

pub fn login_form() -> Markup {
    html!(
        form hx-post="/api/auth/login" hx-swap="outerHTML" {
            h1 { "Groep Van Roey Website Editor" }
            input type="email" name="email" placeholder="email";
            input type="password" name="password" placeholder="password";
            input type="submit" value="Log in";
        }
    )
}

pub fn twofa_form(id: &str, svg: Option<String>) -> Markup {
    html!(
        form hx-post="/api/auth/totp/verify" hx-swap="outerHTML" {
            h1 { "Groep Van Roey Website Editor"}
            @if let Some(svg) = svg {
                (PreEscaped(svg))
            }
            input type="hidden" name="id" value=(id);
            input name="code" placeholder="code";
            input type="submit" value="verify";
        }
    )
}