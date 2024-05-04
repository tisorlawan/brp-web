use axum::http::HeaderMap;
use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct HxCfg<'a> {
    pub post: Option<&'a str>,
    pub get: Option<&'a str>,
    pub trigger: Option<&'a str>,
    pub target: Option<&'a str>,
    pub swap: Option<&'a str>,
    pub select_oob: Option<&'a str>,
    pub vals: Option<&'a str>,
    pub script: Option<&'a str>,
    pub disabled_elt: Option<&'a str>,
}

impl<'a> HxCfg<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_post(mut self, val: &'a str) -> Self {
        self.post = Some(val);
        self
    }

    pub fn with_get(mut self, val: &'a str) -> Self {
        self.get = Some(val);
        self
    }

    pub fn with_trigger(mut self, val: &'a str) -> Self {
        self.trigger = Some(val);
        self
    }

    pub fn with_target(mut self, val: &'a str) -> Self {
        self.target = Some(val);
        self
    }

    pub fn with_swap(mut self, val: &'a str) -> Self {
        self.swap = Some(val);
        self
    }

    pub fn with_select_oob(mut self, val: &'a str) -> Self {
        self.select_oob = Some(val);
        self
    }

    pub fn with_vals(mut self, val: &'a str) -> Self {
        self.vals = Some(val);
        self
    }

    pub fn with_disabled_elt(mut self, val: &'a str) -> Self {
        self.disabled_elt = Some(val);
        self
    }

    pub fn with_script(mut self, val: &'a str) -> Self {
        self.script = Some(val);
        self
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HxSwap {
    #[serde(rename = "innerHTML")]
    InnerHTML,
    #[serde(rename = "outerHTML")]
    OuterHTML,
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd,
    Delete,
    None,
}

#[derive(Debug, Default)]
pub struct HxHeaderBuilder {
    target: Option<&'static str>,
    swap: Option<HxSwap>,
    redirect: Option<&'static str>,
}

impl HxHeaderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_target(mut self, target: &'static str) -> Self {
        self.target = Some(target);
        self
    }

    pub fn with_swap(mut self, swap: HxSwap) -> Self {
        self.swap = Some(swap);
        self
    }

    pub fn with_redirect(mut self, redirect: &'static str) -> Self {
        self.redirect = Some(redirect);
        self
    }

    pub fn build(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        if let Some(val) = self.target {
            headers.insert("Hx-Retarget", val.parse().expect("valid header value"));
        };

        if let Some(ref val) = self.swap {
            let val = serde_variant::to_variant_name(val).expect("valid serde value");
            headers.insert("Hx-Reswap", val.parse().expect("valid header value"));
        }

        if let Some(val) = self.redirect {
            headers.insert("Hx-Redirect", val.parse().expect("valid header value"));
        }

        headers
    }
}
