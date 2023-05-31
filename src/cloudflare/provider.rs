use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderCloudflareData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_base_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_client_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_user_service_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_backoff: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_backoff: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rps: Option<PrimField<f64>>,
}

struct ProviderCloudflare_ {
    data: RefCell<ProviderCloudflareData>,
}

pub struct ProviderCloudflare(Rc<ProviderCloudflare_>);

impl ProviderCloudflare {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "cloudflare", alias)
        } else {
            "cloudflare".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `api_base_path`.\nConfigure the base path used by the API client. Alternatively, can be configured using the `CLOUDFLARE_API_BASE_PATH` environment variable."]
    pub fn set_api_base_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_base_path = Some(v.into());
        self
    }

    #[doc= "Set the field `api_client_logging`.\nWhether to print logs from the API client (using the default log library logger). Alternatively, can be configured using the `CLOUDFLARE_API_CLIENT_LOGGING` environment variable."]
    pub fn set_api_client_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().api_client_logging = Some(v.into());
        self
    }

    #[doc= "Set the field `api_hostname`.\nConfigure the hostname used by the API client. Alternatively, can be configured using the `CLOUDFLARE_API_HOSTNAME` environment variable."]
    pub fn set_api_hostname(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_hostname = Some(v.into());
        self
    }

    #[doc= "Set the field `api_key`.\nThe API key for operations. Alternatively, can be configured using the `CLOUDFLARE_API_KEY` environment variable. API keys are [now considered legacy by Cloudflare](https://developers.cloudflare.com/fundamentals/api/get-started/keys/#limitations), API tokens should be used instead. Must provide only one of `api_key`, `api_token`, `api_user_service_key`."]
    pub fn set_api_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_key = Some(v.into());
        self
    }

    #[doc= "Set the field `api_token`.\nThe API Token for operations. Alternatively, can be configured using the `CLOUDFLARE_API_TOKEN` environment variable. Must provide only one of `api_key`, `api_token`, `api_user_service_key`."]
    pub fn set_api_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_token = Some(v.into());
        self
    }

    #[doc= "Set the field `api_user_service_key`.\nA special Cloudflare API key good for a restricted set of endpoints. Alternatively, can be configured using the `CLOUDFLARE_API_USER_SERVICE_KEY` environment variable. Must provide only one of `api_key`, `api_token`, `api_user_service_key`."]
    pub fn set_api_user_service_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_user_service_key = Some(v.into());
        self
    }

    #[doc= "Set the field `email`.\nA registered Cloudflare email address. Alternatively, can be configured using the `CLOUDFLARE_EMAIL` environment variable. Required when using `api_key`. Conflicts with `api_token`."]
    pub fn set_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().email = Some(v.into());
        self
    }

    #[doc= "Set the field `max_backoff`.\nMaximum backoff period in seconds after failed API calls. Alternatively, can be configured using the `CLOUDFLARE_MAX_BACKOFF` environment variable."]
    pub fn set_max_backoff(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_backoff = Some(v.into());
        self
    }

    #[doc= "Set the field `min_backoff`.\nMinimum backoff period in seconds after failed API calls. Alternatively, can be configured using the `CLOUDFLARE_MIN_BACKOFF` environment variable."]
    pub fn set_min_backoff(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_backoff = Some(v.into());
        self
    }

    #[doc= "Set the field `retries`.\nMaximum number of retries to perform when an API request fails. Alternatively, can be configured using the `CLOUDFLARE_RETRIES` environment variable."]
    pub fn set_retries(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retries = Some(v.into());
        self
    }

    #[doc= "Set the field `rps`.\nRPS limit to apply when making calls to the API. Alternatively, can be configured using the `CLOUDFLARE_RPS` environment variable."]
    pub fn set_rps(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().rps = Some(v.into());
        self
    }
}

impl Provider for ProviderCloudflare_ {
    fn extract_type_tf_id(&self) -> String {
        "cloudflare".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "cloudflare/cloudflare",
            "version": "4.7.1",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderCloudflare {}

impl BuildProviderCloudflare {
    pub fn build(self, stack: &mut Stack) -> ProviderCloudflare {
        let out = ProviderCloudflare(Rc::new(ProviderCloudflare_ { data: RefCell::new(ProviderCloudflareData {
            alias: None,
            api_base_path: core::default::Default::default(),
            api_client_logging: core::default::Default::default(),
            api_hostname: core::default::Default::default(),
            api_key: core::default::Default::default(),
            api_token: core::default::Default::default(),
            api_user_service_key: core::default::Default::default(),
            email: core::default::Default::default(),
            max_backoff: core::default::Default::default(),
            min_backoff: core::default::Default::default(),
            retries: core::default::Default::default(),
            rps: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
