use maud::{html, Markup};


use super::service::{Company, Party, TM};

pub fn tm_input(input: &Option<String>, tms: &Vec<TM>, is_existing: bool, swap_oob: bool) -> Markup {
    let swap_oob = if swap_oob {
        Some("true")
    } else {
        None
    };

    html!(
        div #tms hx-post="/tms" {
            input autocorrect="off" spellcheck="false" autocomplete="off" placeholder="Name" #tm-select value=[input] hx-trigger="input changed" hx-get="/api/tms/search" name="input" hx-target="#tms" hx-swap="none";
                
            button.active[!is_existing && input.is_some()] #tm-add hx-swap-oob=[swap_oob] { "+" }

            form #tms-datalist hx-swap-oob=[swap_oob] hx-get="/tms" hx-select="#main-content" hx-swap="outerHTML" hx-target="#main-content" hx-trigger="change" hx-push-url="true" {
                @for tm in tms {
                    label {
                        (tm.name)
                        input type="checkbox" name="tm" value=(tm.name);
                    }
                }
            }
        }
    )
}

pub fn list_companies(tm: &str, companies_non_active: &Vec<Company>) -> Markup {
    html!(
        ul #companies hx-swap="outerHTML" hx-target="#companies" {
            @for company in companies_non_active {
                li hx-post={"/tms/"(tm)"/companies"} hx-include="this" {
                    input type="hidden" name="company" value=(company.name);
                    (company.name)
                }
            }
        }
    )
}

pub fn list_parties(tm: &str, parties_non_active: &Vec<Party>) -> Markup {
    html!(
        ul #parties hx-swap="outerHTML" hx-target="#parties" {
            @for party in parties_non_active {
                li hx-post={"/tms/"(tm)"/parties"} hx-include="this" {
                    input type="hidden" name="party" value=(party.name);
                    (party.name)
                }
            }
        }
    )
}

pub fn tm_list_companies(tm: &str, companies_active: &Vec<Company>, swap_oob: bool) -> Markup {
    let swap_oob = if swap_oob {
        Some("true")
    } else {
        None
    };
    
    html!(
        ul #tm-companies hx-swap-oob=[swap_oob] hx-swap="outerHTML" hx-target="#companies" {
            @for company in companies_active {
                li hx-delete={"/tms/"(tm)"/companies"} hx-include="this" {
                    input type="hidden" name="company" value=(company.name);
                    (company.name)
                }
            }
        }
    )
}

pub fn tm_list_parties(tm: &str, parties_active: &Vec<Party>, swap_oob: bool) -> Markup {
    let swap_oob = if swap_oob {
        Some("true")
    } else {
        None
    };
    
    html!(
        ul #tm-parties hx-swap-oob=[swap_oob] hx-swap="outerHTML" hx-target="#parties" {
            @for party in parties_active {
                li hx-delete={"/tms/"(tm)"/parties"} hx-include="this" {
                    input type="hidden" name="party" value=(party.name);
                    (party.name)
                }
            }
        }
    )
}

pub fn main(tm: &Option<String>, (companies_non_active, companies_active): &(Vec<Company>, Vec<Company>), (parties_non_active, parties_active): &(Vec<Party>, Vec<Party>), tms: &Vec<TM>) -> Markup {
    html!(
        div.tms #main-content {
            h2 style="text-align: right;" { "Groepsbedrijven" }
            (tm_input(&tm, tms, tm.is_some(), false))
            h2 style="text-align: left;" { "Externe partijen" }
            @if let Some(tm) = tm {
                (list_companies(tm, companies_non_active))
                (tm_list_companies(tm, companies_active, false))
                (tm_list_parties(tm, parties_active, false))
                (list_parties(tm, parties_non_active))
            }
        }
    )
}