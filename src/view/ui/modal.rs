use axum::http::HeaderMap;
use maud::{html, Markup};

use crate::view::hx::HxHeaderBuilder;
use crate::view::hx::HxSwap;

#[derive(Debug, Default)]
pub struct ModalCfg {
    auto_open: bool,
    toggle_btn: Option<Markup>,
    content: Markup,
    footer: Option<Markup>,
}

impl ModalCfg {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable auto open.
    /// The modal will be opened when the browser completed the request.
    /// It does this by using hyperscript `on load` event handler.
    pub fn auto_open(mut self) -> Self {
        self.auto_open = true;
        self
    }

    /// Add toggle button.
    /// It must contains `data-modal-target` and `data-modal-toggle` with the same value with `id`.
    ///
    /// # Example
    ///
    /// ```
    /// let id = "error-modal";
    ///
    /// let toggle_btn = html! {
    ///     button type="button"
    ///     data-modal-target=(id)
    ///     data-modal-toggle=(id)
    ///     class="hidden block text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
    ///     {
    ///         "Toggle modal"
    ///     }
    /// };
    /// ```
    pub fn with_toggle_btn(mut self, val: Markup) -> Self {
        self.toggle_btn = Some(val);
        self
    }

    pub fn with_content(mut self, val: Markup) -> Self {
        self.content = val;
        self
    }

    pub fn with_footer(mut self, val: Markup) -> Self {
        self.footer = Some(val);
        self
    }
}

/// If `cfg.auto_open == true`, the returned `HeaderMap` must be propagaated.
///
/// # Example
/// ```
/// html! {
///     (ui_modal(
///         &ModalCfg::new()
///             .auto_open()
///             .with_content(html! {
///                 div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4" {
///                     div class="sm:flex sm:items-start"{
///                         div
///                             class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10" {
///                             svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
///                                 stroke="currentColor" aria-hidden="true" {
///                                 path stroke-linecap="round" stroke-linejoin="round"
///                                     d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" {}
///                             }
///                         }
///                         div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left" {
///                             h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title" {
///                                 "Deactivate account"
///                             }
///                             div class="mt-2"{
///                                 p class="text-sm text-gray-500"{
///                                     "Are you sure you want to deactivate your account? All of your data will be permanently removed. This action cannot be undone."
///                                 }
///                             }
///                         }
///                     }
///                 }
///             })
///             .with_footer(html! {
///                 div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6" {
///                     button type="button"
///                         class="inline-flex w-full justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 sm:ml-3 sm:w-auto" {
///                         "Deactivate"
///                     }
///                     button type="button"
///                         x-on:click="open = false"
///                         class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:mt-0 sm:w-auto" {
///                         "Cancel"
///                     }
///                 }
///             })
///             .with_toggle_btn(html! {
///                 (ui_button(
///                     html! {"My Button"},
///                     &ButtonCfg::new()
///                     .with_color(Color::Default)
///                     .on_click("open = true"),
///                     &HxCfg::new())
///                 )
///             }
///         )
///     )).1
/// }
/// ```
pub fn ui_modal(cfg: &ModalCfg) -> (HeaderMap, Markup) {
    let mut headers = HeaderMap::new();
    if cfg.auto_open {
        headers = HxHeaderBuilder::new()
            .with_swap(HxSwap::InnerHTML)
            .with_target("#modal-container")
            .build()
    }

    let modal = html! {
        div x-data="{open: false}" class="relative z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true" {
            @if let Some(ref toggle_btn) = cfg.toggle_btn {
                (toggle_btn)
            } @else if cfg.auto_open {
                button _="on load me.click()" x-on:click="open = true" class="hidden" {}
            }

            div
                x-cloak
                x-show="open"
                x-transition:enter="ease-out duration-200"
                x-transition:enter-start="opacity-0"
                x-transition:enter-end="opacity-100"
                x-transition:leave="ease-in duration-100"
                x-transition:leave-start="opacity-100"
                x-transition:leave-end="opacity-0"
                class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacit"
                {}

            div x-cloak x-show="open" class="fixed inset-0 z-10 w-screen overflow-y-auto"{
                div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0" {
                    div
                        x-transition:enter="ease-out duration-100"
                        x-transition:enter-start="opacity-0 translate-y-48 sm:translate-y-0 sm:scale-95"
                        x-transition:enter-end="opacity-100 translate-y-0 sm:scale-100"
                        x-transition:leave="ease-in duration-200"
                        x-transition:leave-start="opacity-100 translate-y-0 sm:scale-100"
                        x-transition:leave-end="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
                        class="relative transform overflow-hidden rounded-sm bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg" {

                        div {
                            (cfg.content)
                            @if let Some(ref footer) = cfg.footer {
                                (footer)
                            }
                        }
                    }
                }
            }
        }
    };
    (headers, modal)
}
