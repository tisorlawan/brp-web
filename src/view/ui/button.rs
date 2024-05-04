use super::super::hx::HxCfg;
use super::Color;
use maud::{html, Markup, Render};

const BTN_BASE_CSS: &str =
    "focus:outline-none text-sm font-medium rounded-xs px-4 py-2 focus:ring-2 transition-opacity shadow-sm inline-flex h-9 items-center flex justify-center cursor:pointer";

pub fn ui_button(mut child: Markup, cfg: &ButtonCfg, hx: &HxCfg) -> Markup {
    let mut cn = format!("{} {}", BTN_BASE_CSS, cfg.color.cn_btn());
    if let Some(cn_extra) = cfg.cn {
        cn = format!("{cn} {cn_extra}");
    }

    if let Some((src, cn)) = cfg.append_icon {
        child = html! {
            div class="flex gap-2 items-center" {
                span class="font-semibold" { (child) }
                img src=(src) class=(cn);
            }
        }
    }

    let btn = if let Some(link) = cfg.link {
        html! {
            a
                id=[cfg.id]
                class=(cn)
                href=(link)
                type="button"
                disabled[cfg.is_disabled]
                hx-get=[hx.get]
                hx-post=[hx.post]
                hx-trigger=[hx.trigger]
                hx-select-oob=[hx.select_oob]
                hx-target=[hx.target]
                hx-swap=[hx.swap]
                hx-vals=[hx.vals]
                hx-disabled-elt=[hx.disabled_elt]
                "x-on:click"=[cfg.x_on_click]
                "x-on:click.outside"=[cfg.x_on_click_outside]
                data-tooltip-target=[cfg.tooltip.as_ref().map(|s| s.0)]
                _=[hx.script.as_ref()]{
                (child)
            }
        }
    } else {
        html! {
            button
                id=[cfg.id]
                class=(cn)
                type=(cfg.typ)
                disabled[cfg.is_disabled]
                hx-get=[hx.get]
                hx-post=[hx.post]
                hx-trigger=[hx.trigger]
                hx-target=[hx.target]
                hx-swap=[hx.swap]
                hx-select-oob=[hx.select_oob]
                hx-vals=[hx.vals]
                hx-disabled-elt=[hx.disabled_elt]
                "x-on:click"=[cfg.x_on_click]
                "x-on:click.outside"=[cfg.x_on_click_outside]
                data-tooltip-target=[cfg.tooltip.as_ref().map(|s| s.0)]
                _=[hx.script.as_ref()]{
                (child)
            }
        }
    };

    if let Some((id, content)) = &cfg.tooltip {
        html! {
            div {
                (btn)

                div
                    id=(id)
                    role="tooltip"
                    class="absolute z-10 invisible inline-block px-3 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-xs shadow-sm opacity-0 tooltip"
                {
                    (content)
                    div class="tooltip-arrow" data-popper-arrow {}
                }
            }
        }
    } else {
        btn
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

impl Render for ButtonType {
    fn render(&self) -> Markup {
        match self {
            ButtonType::Button => html! {"button"},
            ButtonType::Submit => html! {"submit"},
            ButtonType::Reset => html! {"reset"},
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ButtonCfg<'a> {
    id: Option<&'a str>,
    color: Color,
    cn: Option<&'a str>,
    typ: ButtonType,
    is_disabled: bool,
    x_on_click: Option<&'a str>,
    x_on_click_outside: Option<&'a str>,
    tooltip: Option<(&'a str, Markup)>,
    link: Option<&'a str>,
    append_icon: Option<(&'a str, &'a str)>, // (src, class)
}

impl<'a> ButtonCfg<'a> {
    pub fn new() -> ButtonCfg<'a> {
        Self::default()
    }

    pub fn with_id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_cn(mut self, cn: &'a str) -> Self {
        self.cn = Some(cn);
        self
    }

    pub fn with_type(mut self, typ: ButtonType) -> Self {
        self.typ = typ;
        self
    }

    pub fn with_tooltip(mut self, id: &'a str, content: Markup) -> Self {
        self.tooltip = Some((id, content));
        self
    }

    pub fn disable(mut self) -> Self {
        self.is_disabled = true;
        self
    }

    pub fn as_link(mut self, val: &'a str) -> Self {
        self.link = Some(val);
        self
    }

    pub fn append_icon(mut self, val: &'static str, cn: &'static str) -> Self {
        self.append_icon = Some((val, cn));
        self
    }

    pub fn on_click(mut self, val: &'static str) -> Self {
        self.x_on_click = Some(val);
        self
    }

    pub fn on_click_outside(mut self, val: &'static str) -> Self {
        self.x_on_click_outside = Some(val);
        self
    }
}
