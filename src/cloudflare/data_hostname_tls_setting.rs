#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataHostnameTlsSettingData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    hostname: terrars::PrimField<std::string::String>,
    setting_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataHostnameTlsSetting_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataHostnameTlsSettingData_>,
}

#[derive(Clone)]
pub struct DataHostnameTlsSetting(std::rc::Rc<DataHostnameTlsSetting_>);

impl DataHostnameTlsSetting {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThis is the time the tls setting was originally created for this hostname."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe hostname for which the tls settings are set."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `setting_id` after provisioning.\nThe TLS Setting name.\nThe value type depends on the setting:\n- `ciphers`: value is an array of cipher suite strings (e.g., `[\"ECDHE-RSA-AES128-GCM-SHA256\", \"AES128-GCM-SHA256\"]`).\n- `min_tls_version`: value is a TLS version string (`\"1.0\"`, `\"1.1\"`, `\"1.2\"`, or `\"1.3\"`).\n- `http2`: value is `\"on\"` or `\"off\"`.\nAvailable values: \"ciphers\", \"min_tls_version\", \"http2\"."]
    pub fn setting_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.setting_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nDeployment status for the given tls setting."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nThis is the time the tls setting was updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe TLS setting value.\nThe type depends on the `setting_id` used in the request path:\n- `ciphers`: an array of allowed cipher suite strings in BoringSSL format (e.g., `[\"ECDHE-RSA-AES128-GCM-SHA256\", \"AES128-GCM-SHA256\"]`).\n- `min_tls_version`: a string indicating the minimum TLS version — one of `\"1.0\"`, `\"1.1\"`, `\"1.2\"`, or `\"1.3\"` (e.g., `\"1.2\"`).\n- `http2`: a string indicating whether HTTP/2 is enabled — `\"on\"` or `\"off\"` (e.g., `\"on\"`).\nAvailable values: \"1.0\", \"1.1\", \"1.2\", \"1.3\", \"on\", \"off\"."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataHostnameTlsSetting {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataHostnameTlsSetting { }

impl terrars::ToListMappable for DataHostnameTlsSetting {
    type O = terrars::ListRef<DataHostnameTlsSettingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataHostnameTlsSetting_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_hostname_tls_setting".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataHostnameTlsSetting {
    pub tf_id: std::string::String,
    #[doc = "The hostname for which the tls settings are set."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc =
        "The TLS Setting name.\nThe value type depends on the setting:\n- `ciphers`: value is an array of cipher suite strings (e.g., `[\"ECDHE-RSA-AES128-GCM-SHA256\", \"AES128-GCM-SHA256\"]`).\n- `min_tls_version`: value is a TLS version string (`\"1.0\"`, `\"1.1\"`, `\"1.2\"`, or `\"1.3\"`).\n- `http2`: value is `\"on\"` or `\"off\"`.\nAvailable values: \"ciphers\", \"min_tls_version\", \"http2\"."]
    pub setting_id: terrars::PrimField<std::string::String>,
}

impl BuildDataHostnameTlsSetting {
    pub fn build(self, stack: &mut terrars::Stack) -> DataHostnameTlsSetting {
        let out = DataHostnameTlsSetting(std::rc::Rc::new(DataHostnameTlsSetting_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataHostnameTlsSettingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                hostname: self.hostname,
                setting_id: self.setting_id,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataHostnameTlsSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataHostnameTlsSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataHostnameTlsSettingRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThis is the time the tls setting was originally created for this hostname."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe hostname for which the tls settings are set."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `setting_id` after provisioning.\nThe TLS Setting name.\nThe value type depends on the setting:\n- `ciphers`: value is an array of cipher suite strings (e.g., `[\"ECDHE-RSA-AES128-GCM-SHA256\", \"AES128-GCM-SHA256\"]`).\n- `min_tls_version`: value is a TLS version string (`\"1.0\"`, `\"1.1\"`, `\"1.2\"`, or `\"1.3\"`).\n- `http2`: value is `\"on\"` or `\"off\"`.\nAvailable values: \"ciphers\", \"min_tls_version\", \"http2\"."]
    pub fn setting_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.setting_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nDeployment status for the given tls setting."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nThis is the time the tls setting was updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe TLS setting value.\nThe type depends on the `setting_id` used in the request path:\n- `ciphers`: an array of allowed cipher suite strings in BoringSSL format (e.g., `[\"ECDHE-RSA-AES128-GCM-SHA256\", \"AES128-GCM-SHA256\"]`).\n- `min_tls_version`: a string indicating the minimum TLS version — one of `\"1.0\"`, `\"1.1\"`, `\"1.2\"`, or `\"1.3\"` (e.g., `\"1.2\"`).\n- `http2`: a string indicating whether HTTP/2 is enabled — `\"on\"` or `\"off\"` (e.g., `\"on\"`).\nAvailable values: \"1.0\", \"1.1\", \"1.2\", \"1.3\", \"on\", \"off\"."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
