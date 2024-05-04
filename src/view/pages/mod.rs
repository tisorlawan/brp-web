use axum::http::HeaderMap;
use maud::{html, Markup, PreEscaped};

pub mod index;
pub mod login;

pub fn page(title: &'static str, child: Markup) -> Markup {
    html! {
        html lang="en-US" {
            head {
                meta charset="UTF-8";
                title {(title)}
                link rel="icon" type="image/svg+xml" href="/static/img/favicon.ico";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="robots" content="index, follow";
                meta name="revisit-after" content="7 days";
                meta name="language" content="English";

                link rel="stylesheet" href="/static/css/styles.css";
                script src="/static/js/htmx.min.js" {}
                script src="/static/js/_hyperscript.min.js" {}
                script src="/static/js/flowbite.min.js" {}
                script src="/static/js/datepicker.min.js" {}
                script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}

                script {
                   r#"htmx.onLoad(function(content) {
                       initFlowbite();
                   })"#
                }

            }

            body {
                (child)
                div id="modal-container" {};
            }

            (script())
        }
    }
}

pub fn redirect(path: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Hx-Redirect", path.parse().unwrap());
    headers
}

fn script() -> Markup {
    html! {
            script {
            (PreEscaped(
r##"
htmx.on("htmx:beforeSwap", function (evt) {
    var status = evt.detail.xhr.status
    if (status === 400 || status == 401) {
        evt.detail.shouldSwap = true;
        evt.detail.isError = false;
    }
})

var themeToggleDarkIcon = document.getElementById('theme-toggle-dark-icon');
var themeToggleLightIcon = document.getElementById("theme-toggle-light-icon");

if (localStorage.getItem("theme") === "dark" || (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches)) {
    themeToggleLightIcon !== null && themeToggleLightIcon.classList.remove("hidden");
} else {
    themeToggleDarkIcon !== null && themeToggleDarkIcon.classList.remove("hidden");
}

var themeToggleBtn = document.getElementById("theme-toggle");

if (localStorage.getItem("theme") === "dark") {
    document.documentElement.classList.add("dark");
}
themeToggleBtn !== null && themeToggleBtn.addEventListener("click", function() {
    themeToggleDarkIcon !== null && themeToggleDarkIcon.classList.toggle("hidden");
    themeToggleLightIcon !== null && themeToggleLightIcon.classList.toggle("hidden");

    if (localStorage.getItem("theme")) {
        if (localStorage.getItem("theme") === "light") {
            document.documentElement.classList.add("dark");
            localStorage.setItem("theme", "dark");
        } else {
            document.documentElement.classList.remove("dark");
            localStorage.setItem("theme", "light");
        }
    } else {
        if (document.documentElement.classList.contains("dark")) {
            document.documentElement.classList.remove("dark");
            localStorage.setItem("theme", "light");
        } else {
            document.documentElement.classList.add("dark");
            localStorage.setItem("theme", "dark");
        }
    }
});
"##))
            }
    }
}
