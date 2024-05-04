use super::{
    super::ui::{
        button::ui_button,
        datepicker::{ui_datepicker, DatePickerCfgBuilder},
        theme_toggle::ui_theme_toggle,
        Color,
    },
    login::redirect_login,
};
use crate::{
    auth::User,
    utils::today,
    view::{hx::HxCfg, ui::button::ButtonCfg},
};
use axum::response::IntoResponse;
use maud::{html, Markup};

fn page(profile: User) -> Markup {
    html! {
        div class="flex flex-col justify-start items-center h-screen gap-12 pt-8" {
            (ui_datepicker(DatePickerCfgBuilder::new().with_value(&today()).build()))

            (ui_theme_toggle())


            div class="flex gap-2" {
                (ui_button(html!{"Tooltip Example"},
                    &ButtonCfg::new()
                        .with_tooltip("tooltip-1", html!{"This is the tooltip"}),
                    &HxCfg::new()
                ))

                (ui_button(html!{(profile.email)},
                    &ButtonCfg::new(),
                    &HxCfg::new()
                ))

                (ui_button(html!{"Example"},
                    &ButtonCfg::new()
                        .with_color(Color::Alternative),
                    &HxCfg::new()
                ))


                (ui_button(html!{"Logout"},
                    &ButtonCfg::new()
                        .with_color(Color::PurpleToBlue),
                    &HxCfg::new()
                        .with_post("/logout")
                ))
            }
        }
    }
}

pub async fn page_index(user: Option<User>) -> impl IntoResponse {
    match user {
        Some(user) => crate::view::pages::page("Index", page(user)),
        None => redirect_login(),
    }
}

pub fn redirect_index() -> Markup {
    html! {
        head {
            meta http-equiv="Refresh" content="0; URL=/";
        }
    }
}
