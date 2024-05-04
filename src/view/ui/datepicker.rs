use maud::{html, Markup};

#[derive(Default)]
pub struct DatePickerCfg<'a> {
    id: Option<&'a str>,
    name: Option<&'a str>,
    value: Option<&'a str>,
    placeholder: Option<&'a str>,
    script: Option<String>,
}

#[derive(Default)]
pub struct DatePickerCfgBuilder<'a> {
    cfg: DatePickerCfg<'a>,
}

impl<'a> DatePickerCfgBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, val: &'a str) -> Self {
        self.cfg.id = Some(val);
        self.cfg.name = Some(val);
        self
    }

    pub fn with_name(mut self, val: &'a str) -> Self {
        self.cfg.name = Some(val);
        self
    }

    pub fn with_value(mut self, val: &'a str) -> Self {
        self.cfg.value = Some(val);
        self
    }

    pub fn with_placeholder(mut self, val: &'a str) -> Self {
        self.cfg.placeholder = Some(val);
        self
    }

    pub fn with_script(mut self, val: impl AsRef<str>) -> Self {
        self.cfg.script = Some(val.as_ref().to_string());
        self
    }

    pub fn build(&self) -> &DatePickerCfg {
        &self.cfg
    }
}

pub fn ui_datepicker(cfg: &DatePickerCfg) -> Markup {
    html! {
        div class="relative max-w-sm" {
            div class="absolute inset-y-0 start-0 flex items-center ps-3.5 pointer-events-none" {
                svg class="w-4 h-4 text-gray-500" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20" {
                    path d="M20 4a2 2 0 0 0-2-2h-2V1a1 1 0 0 0-2 0v1h-3V1a1 1 0 0 0-2 0v1H6V1a1 1 0 0 0-2 0v1H2a2 2 0 0 0-2 2v2h20V4ZM0 18a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8H0v10Zm5-8h10a1 1 0 0 1 0 2H5a1 1 0 0 1 0-2Z";
                }
            }
          input
              name=[cfg.name]
              id=[cfg.id]
              datepicker
              datepicker-buttons
              datepicker-autohide
              datepicker-autoselect-today
              datepicker-format="dd MM yyyy"
              type="text"
              value=[cfg.value]
              _=[&cfg.script]
              class="text-sm rounded-sm text-foreground bg-foreground/5 focus:ring-0 dark:placeholder-gray-400 focus:outline-none outline-none border-border focus:border-border-focus block w-full ps-10 p-2.5"
              placeholder=[cfg.placeholder];
        }
    }
}
