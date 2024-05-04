use super::Color;
use maud::{html, Markup};
use serde::Serialize;
use serde_variant::to_variant_name;

#[derive(Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InputType {
    #[default]
    Text,
    Password,
}

pub struct InputCfg {
    label: Option<&'static str>,
    placeholder: Option<&'static str>,
    color: Color,
    typ: InputType,
    required: bool,
    script: Option<&'static str>,
    spellcheck: Option<&'static str>,
    autofocus: bool,
    autocomplete: Option<&'static str>,
    ccn: Option<&'static str>,
}

impl Default for InputCfg {
    fn default() -> Self {
        Self {
            label: Some("label placeholder"),
            placeholder: None,
            color: Color::default(),
            typ: InputType::default(),
            required: false,
            script: None,
            spellcheck: None,
            autofocus: false,
            autocomplete: None,
            ccn: None,
        }
    }
}

#[derive(Default)]
pub struct InputCfgBuilder {
    cfg: InputCfg,
}

impl InputCfgBuilder {
    pub fn new() -> Self {
        InputCfgBuilder::default()
    }

    pub fn with_label(mut self, val: &'static str) -> Self {
        self.cfg.label = Some(val);
        self
    }

    pub fn with_no_label(mut self) -> Self {
        self.cfg.label = None;
        self
    }

    pub fn with_type(mut self, val: InputType) -> Self {
        self.cfg.typ = val;
        self
    }

    pub fn with_placeholder(mut self, val: &'static str) -> Self {
        self.cfg.placeholder = Some(val);
        self
    }

    pub fn with_color(mut self, val: Color) -> Self {
        self.cfg.color = val;
        self
    }

    pub fn with_script(mut self, val: &'static str) -> Self {
        self.cfg.script = Some(val);
        self
    }

    pub fn with_ccn(mut self, val: &'static str) -> Self {
        self.cfg.ccn = Some(val);
        self
    }

    pub fn autofocus(mut self, val: bool) -> Self {
        self.cfg.autofocus = val;
        self
    }

    pub fn with_autocomplete(mut self, val: &'static str) -> Self {
        self.cfg.autocomplete = Some(val);
        self
    }

    pub fn spellcheck(mut self, val: bool) -> Self {
        if val {
            self.cfg.spellcheck = None;
        } else {
            self.cfg.spellcheck = Some("false");
        }
        self
    }

    pub fn required(mut self) -> Self {
        self.cfg.required = true;
        self
    }

    pub fn build(&self) -> &InputCfg {
        &self.cfg
    }
}

fn input_color_cn(color: &Color) -> &'static str {
    match color {
        Color::Default => "text-foreground bg-background-100 focus:ring-0 dark:placeholder-gray-400 focus:outline-none outline-none border-border focus:border-border-focus",
        _ => unimplemented!(),
    }
}

pub fn ui_input(id: &'static str, cfg: &InputCfg) -> Markup {
    let cn_init = "block mb-1 w-full text-sm font-medium font-normal";
    let cn_color = input_color_cn(&cfg.color);
    let cn = format!("{cn_init} {cn_color}");

    html! {
        div class=[cfg.ccn] {
            @if let Some(label) = cfg.label {
                label
                    for=(id)
                    class="block mb-1 font-medium font-semibold text-xs" {
                    (label)
                }
            }
            input
                type=(to_variant_name(&cfg.typ).expect("valid input typ"))
                name=(id)
                id=(id)
                class=(cn)
                placeholder=[cfg.placeholder] required[cfg.required]
                autofocus[cfg.autofocus]
                spellcheck=[cfg.spellcheck]
                autocomplete=[cfg.autocomplete]
                _=[cfg.script]
                ;
        }
    }
}
