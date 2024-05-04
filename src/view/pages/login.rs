use super::super::ui::{
    button::{ui_button, ButtonType},
    input::{ui_input, InputCfgBuilder, InputType},
    Color,
};
use crate::{
    auth::{self, User},
    view::{hx::HxCfg, ui::button::ButtonCfg},
};
use axum::response::{IntoResponse, Redirect};
use maud::{html, Markup};

pub async fn page_login(user: Option<User>) -> impl IntoResponse {
    if user.is_some() {
        return Redirect::to("/").into_response();
    }

    let google_auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&scope={scope}&client_id={client}&redirect_uri={redirect}&access_type=offline",
        client = auth::GOOGLE_OAUTH_CLIENT_ID.as_str(),
        redirect = auth::GOOGLE_OAUTH_REDIRECT_URL.as_str(),
        scope = url_escape::encode_fragment(&auth::GOOGLE_OAUTH_SCOPE.as_str())
    );
    super::page(
        "Login",
        html! {
            div class="flex flex-col justify-center items-center h-screen" {
                div class="w-[350px] mt-10 border border-border bg-background/70 shadow-md z-10 rounded-sm px-6 pt-8 pb-6" {
                    h1 class="text-xl font-bold flex justify-center" {"Login"}
                    (ui_button(html! { "Login with Google" },
                        &ButtonCfg::new()
                            .append_icon("/static/img/icons8-google.svg", "h-5 w-5")
                            .with_cn("mt-10 w-full py-6 border-foreground/20 bg-background-100 hover:bg-background-100/50")
                            .with_color(Color::Alternative)
                            .as_link(&google_auth_url),
                        &HxCfg::new()
                    ))
                }
            }
        },
    ).into_response()
}

pub fn credential_form() -> Markup {
    let script = "on input add .invisible to #error-message";
    let pre_escaped = html! {
        form id="login-form"
            class="mt-8 flex flex-col gap-2"
            hx-post="/login" hx-target="#error-message" hx-swap="outerHTML"
            hx-indicator="#indicator"
            hx-disabled-elt="#submit"
        {
            (ui_input("username", &InputCfgBuilder::new().with_label("Username").with_autocomplete("username").with_script(script).autofocus(true).spellcheck(false).build()))
            (ui_input("password", &InputCfgBuilder::new().with_label("Password").with_autocomplete("current-password").with_script(script).with_ccn("mt-4").with_type(InputType::Password).build()))
            div class="mt-4" {
                (ui_button(
                    html!{
                        div class="flex gap-2" {
                            span { "Login" }
                            div id="indicator" class="htmx-indicator" { "Loading" }
                        }
                    },
                    &ButtonCfg::new().with_color(Color::Default).with_type(ButtonType::Submit).with_id("submit").with_cn("w-24"),
                    &HxCfg::new()
                ))
                p id="error-message" class="text-sm mt-2 text-destructive invisible" {
                    "error message placeholder"
                }
            }
        }
    };
    pre_escaped
}

pub fn redirect_login() -> Markup {
    html! {
        head {
            meta http-equiv="Refresh" content="0; URL=/login";
        }
    }
}
