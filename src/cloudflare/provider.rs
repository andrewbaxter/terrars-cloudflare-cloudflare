#![allow(non_camel_case_types)]

use serde::Serialize;

#[derive(Serialize)]
struct ProviderCloudflareData_ {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_user_service_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_agent_operator_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ProviderCloudflare_ {
    data: std::cell::RefCell<ProviderCloudflareData_>,
}

pub struct ProviderCloudflare(std::rc::Rc<ProviderCloudflare_>);

impl ProviderCloudflare {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "cloudflare", alias)
        } else {
            "cloudflare".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }

    #[doc =
        "Set the field `api_key`.\nThe API key for operations. Alternatively, can be configured using the `CLOUDFLARE_API_KEY` environment variable. API keys are [now considered legacy by Cloudflare](https://developers.cloudflare.com/fundamentals/api/get-started/keys/#limitations), API tokens should be used instead. Must provide only one of `api_key`, `api_token`, `api_user_service_key`."]
    pub fn set_api_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `api_token`.\nThe API Token for operations. Alternatively, can be configured using the `CLOUDFLARE_API_TOKEN` environment variable. Must provide only one of `api_key`, `api_token`, `api_user_service_key`."]
    pub fn set_api_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().api_token = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `api_user_service_key`.\nA special Cloudflare API key good for a restricted set of endpoints. Alternatively, can be configured using the `CLOUDFLARE_API_USER_SERVICE_KEY` environment variable. Must provide only one of `api_key`, `api_token`, `api_user_service_key`."]
    pub fn set_api_user_service_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().api_user_service_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `base_url`.\nValue to override the default HTTP client base URL. Alternatively, can be configured using the `base_url` environment variable."]
    pub fn set_base_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().base_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `email`.\nA registered Cloudflare email address. Alternatively, can be configured using the `CLOUDFLARE_EMAIL` environment variable. Required when using `api_key`. Conflicts with `api_token`."]
    pub fn set_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_agent_operator_suffix`.\nA value to append to the HTTP User Agent for all API calls. This value is not something most users need to modify however, if you are using a non-standard provider or operator configuration, this is recommended to assist in uniquely identifying your traffic. **Setting this value will remove the Terraform version from the HTTP User Agent string and may have unintended consequences**. Alternatively, can be configured using the `CLOUDFLARE_USER_AGENT_OPERATOR_SUFFIX` environment variable."]
    pub fn set_user_agent_operator_suffix(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().user_agent_operator_suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::Provider for ProviderCloudflare_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "cloudflare".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "cloudflare/cloudflare",
            "version": "5.28.1",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderCloudflare {}

impl BuildProviderCloudflare {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderCloudflare {
        let out =
            ProviderCloudflare(
                std::rc::Rc::new(ProviderCloudflare_ { data: std::cell::RefCell::new(ProviderCloudflareData_ {
                    alias: std::option::Option::None,
                    api_key: core::default::Default::default(),
                    api_token: core::default::Default::default(),
                    api_user_service_key: core::default::Default::default(),
                    base_url: core::default::Default::default(),
                    email: core::default::Default::default(),
                    user_agent_operator_suffix: core::default::Default::default(),
                }) }),
            );
        stack.add_provider(out.0.clone());
        out
    }
}
