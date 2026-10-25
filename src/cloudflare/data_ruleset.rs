#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataRulesetData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ruleset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataRuleset_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRulesetData_>,
}

#[derive(Clone)]
pub struct DataRuleset(std::rc::Rc<DataRuleset_>);

impl DataRuleset {
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

    #[doc = "Set the field `account_id`.\nThe unique ID of the account."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nThe unique ID of the ruleset."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ruleset_id`.\nThe unique ID of the ruleset."]
    pub fn set_ruleset_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ruleset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nThe unique ID of the zone."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe unique ID of the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative description of the ruleset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the ruleset."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe kind of the ruleset.\nAvailable values: \"managed\", \"custom\", \"root\", \"zone\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_updated` after provisioning.\nThe timestamp of when the ruleset was last modified."]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe human-readable name of the ruleset."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `phase` after provisioning.\nThe phase of the ruleset.\nAvailable values: \"ddos_l4\", \"ddos_l7\", \"http_config_settings\", \"http_custom_errors\", \"http_log_custom_fields\", \"http_ratelimit\", \"http_request_cache_settings\", \"http_request_dynamic_redirect\", \"http_request_firewall_custom\", \"http_request_firewall_managed\", \"http_request_late_transform\", \"http_request_origin\", \"http_request_redirect\", \"http_request_sanitize\", \"http_request_sbfm\", \"http_request_transform\", \"http_response_cache_settings\", \"http_response_compression\", \"http_response_firewall_managed\", \"http_response_headers_transform\", \"magic_transit\", \"magic_transit_ids_managed\", \"magic_transit_managed\", \"magic_transit_ratelimit\"."]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nThe list of rules in the ruleset."]
    pub fn rules(&self) -> terrars::ListRef<DataRuleset_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ruleset_id` after provisioning.\nThe unique ID of the ruleset."]
    pub fn ruleset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ruleset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the ruleset."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nThe unique ID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataRuleset {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRuleset { }

impl terrars::ToListMappable for DataRuleset {
    type O = terrars::ListRef<DataRulesetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRuleset_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_ruleset".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRuleset {
    pub tf_id: std::string::String,
}

impl BuildDataRuleset {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRuleset {
        let out = DataRuleset(std::rc::Rc::new(DataRuleset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRulesetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                ruleset_id: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRulesetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRulesetRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe unique ID of the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative description of the ruleset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the ruleset."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe kind of the ruleset.\nAvailable values: \"managed\", \"custom\", \"root\", \"zone\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_updated` after provisioning.\nThe timestamp of when the ruleset was last modified."]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe human-readable name of the ruleset."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `phase` after provisioning.\nThe phase of the ruleset.\nAvailable values: \"ddos_l4\", \"ddos_l7\", \"http_config_settings\", \"http_custom_errors\", \"http_log_custom_fields\", \"http_ratelimit\", \"http_request_cache_settings\", \"http_request_dynamic_redirect\", \"http_request_firewall_custom\", \"http_request_firewall_managed\", \"http_request_late_transform\", \"http_request_origin\", \"http_request_redirect\", \"http_request_sanitize\", \"http_request_sbfm\", \"http_request_transform\", \"http_response_cache_settings\", \"http_response_compression\", \"http_response_firewall_managed\", \"http_response_headers_transform\", \"magic_transit\", \"magic_transit_ids_managed\", \"magic_transit_managed\", \"magic_transit_ratelimit\"."]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nThe list of rules in the ruleset."]
    pub fn rules(&self) -> terrars::ListRef<DataRuleset_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ruleset_id` after provisioning.\nThe unique ID of the ruleset."]
    pub fn ruleset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ruleset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the ruleset."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nThe unique ID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Algorithms {}

impl DataRuleset_Rules_ActionParameters_Algorithms { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Algorithms {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Algorithms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Algorithms {}

impl BuildDataRuleset_Rules_ActionParameters_Algorithms {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Algorithms {
        DataRuleset_Rules_ActionParameters_Algorithms {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_AlgorithmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_AlgorithmsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_AlgorithmsRef {
        DataRuleset_Rules_ActionParameters_AlgorithmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_AlgorithmsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the compression algorithm to enable.\nAvailable values: \"none\", \"auto\", \"default\", \"gzip\", \"brotli\", \"zstd\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Autominify {}

impl DataRuleset_Rules_ActionParameters_Autominify { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Autominify {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Autominify>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Autominify {}

impl BuildDataRuleset_Rules_ActionParameters_Autominify {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Autominify {
        DataRuleset_Rules_ActionParameters_Autominify {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_AutominifyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_AutominifyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_AutominifyRef {
        DataRuleset_Rules_ActionParameters_AutominifyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_AutominifyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `css` after provisioning.\nWhether to minify CSS files."]
    pub fn css(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.css", self.base))
    }

    #[doc = "Get a reference to the value of field `html` after provisioning.\nWhether to minify HTML files."]
    pub fn html(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }

    #[doc = "Get a reference to the value of field `js` after provisioning.\nWhether to minify JavaScript files."]
    pub fn js(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.js", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_BrowserTtl {}

impl DataRuleset_Rules_ActionParameters_BrowserTtl { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_BrowserTtl {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_BrowserTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_BrowserTtl {}

impl BuildDataRuleset_Rules_ActionParameters_BrowserTtl {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_BrowserTtl {
        DataRuleset_Rules_ActionParameters_BrowserTtl {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_BrowserTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_BrowserTtlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_BrowserTtlRef {
        DataRuleset_Rules_ActionParameters_BrowserTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_BrowserTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nThe browser TTL (in seconds) if you choose the \"override_origin\" mode."]
    pub fn default(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe browser TTL mode.\nAvailable values: \"respect_origin\", \"bypass_by_default\", \"override_origin\", \"bypass\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {}

impl BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Cookie {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_presence` after provisioning.\nA list of cookies to check for the presence of. The presence of these cookies is included in the cache key."]
    pub fn check_presence(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nA list of cookies to include in the cache key."]
    pub fn include(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Header {}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Header { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Header {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Header {}

impl BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Header {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Header {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Header {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_presence` after provisioning.\nA list of headers to check for the presence of. The presence of these headers is included in the cache key."]
    pub fn check_presence(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nA mapping of header names to a list of values. If a header is present in the request and contains any of the values provided, its value is included in the cache key."]
    pub fn contains(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_origin` after provisioning.\nWhether to exclude the origin header in the cache key."]
    pub fn exclude_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nA list of headers to include in the cache key."]
    pub fn include(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Host {}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Host { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Host {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Host>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Host {}

impl BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Host {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Host {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_Host {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resolved` after provisioning.\nWhether to use the resolved host in the cache key."]
    pub fn resolved(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolved", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {}

impl BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Exclude {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all` after provisioning.\nWhether to exclude all query string parameters from the cache key."]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `list` after provisioning.\nA list of query string parameters to exclude from the cache key."]
    pub fn list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {}

impl BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_Include {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all` after provisioning.\nWhether to include all query string parameters in the cache key."]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `list` after provisioning.\nA list of query string parameters to include in the cache key."]
    pub fn list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {}

impl BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nWhich query string parameters to exclude from the cache key."]
    pub fn exclude(&self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_ExcludeRef::new(
            self.shared().clone(),
            format!("{}.exclude", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nWhich query string parameters to include in the cache key."]
    pub fn include(&self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryString_IncludeRef::new(
            self.shared().clone(),
            format!("{}.include", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_User {}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_User { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_User {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_User>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_User {}

impl BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey_User {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_User {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_User {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `device_type` after provisioning.\nWhether to use the user agent's device type in the cache key."]
    pub fn device_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `geo` after provisioning.\nWhether to use the user agents's country in the cache key."]
    pub fn geo(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lang` after provisioning.\nWhether to use the user agent's language in the cache key."]
    pub fn lang(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lang", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKey {}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKey { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CacheKey_CustomKey {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CacheKey_CustomKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey {}

impl BuildDataRuleset_Rules_ActionParameters_CacheKey_CustomKey {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cookie` after provisioning.\nWhich cookies to include in the cache key."]
    pub fn cookie(&self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_CookieRef::new(
            self.shared().clone(),
            format!("{}.cookie", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `header` after provisioning.\nWhich headers to include in the cache key."]
    pub fn header(&self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HeaderRef::new(
            self.shared().clone(),
            format!("{}.header", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nHow to use the host in the cache key."]
    pub fn host(&self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_HostRef::new(
            self.shared().clone(),
            format!("{}.host", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `query_string` after provisioning.\nWhich query string parameters to include in or exclude from the cache key."]
    pub fn query_string(&self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_QueryStringRef::new(
            self.shared().clone(),
            format!("{}.query_string", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nHow to use characteristics of the request user agent in the cache key."]
    pub fn user(&self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKey_UserRef::new(
            self.shared().clone(),
            format!("{}.user", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CacheKey {}

impl DataRuleset_Rules_ActionParameters_CacheKey { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CacheKey {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CacheKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CacheKey {}

impl BuildDataRuleset_Rules_ActionParameters_CacheKey {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CacheKey {
        DataRuleset_Rules_ActionParameters_CacheKey {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CacheKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CacheKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CacheKeyRef {
        DataRuleset_Rules_ActionParameters_CacheKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CacheKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache_by_device_type` after provisioning.\nWhether to separate cached content based on the visitor's device type."]
    pub fn cache_by_device_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_by_device_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_deception_armor` after provisioning.\nWhether to protect from web cache deception attacks, while allowing static assets to be cached."]
    pub fn cache_deception_armor(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_deception_armor", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_key` after provisioning.\nWhich components of the request are included or excluded from the cache key."]
    pub fn custom_key(&self) -> DataRuleset_Rules_ActionParameters_CacheKey_CustomKeyRef {
        DataRuleset_Rules_ActionParameters_CacheKey_CustomKeyRef::new(
            self.shared().clone(),
            format!("{}.custom_key", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `ignore_query_strings_order` after provisioning.\nWhether to treat requests with the same query parameters the same, regardless of the order those query parameters are in."]
    pub fn ignore_query_strings_order(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_query_strings_order", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CacheReserve {}

impl DataRuleset_Rules_ActionParameters_CacheReserve { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CacheReserve {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CacheReserve>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CacheReserve {}

impl BuildDataRuleset_Rules_ActionParameters_CacheReserve {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CacheReserve {
        DataRuleset_Rules_ActionParameters_CacheReserve {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CacheReserveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CacheReserveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CacheReserveRef {
        DataRuleset_Rules_ActionParameters_CacheReserveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CacheReserveRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `eligible` after provisioning.\nWhether Cache Reserve is enabled. If this is true and a request meets eligibility criteria, Cloudflare will write the resource to Cache Reserve."]
    pub fn eligible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eligible", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_file_size` after provisioning.\nThe minimum file size eligible for storage in Cache Reserve."]
    pub fn minimum_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_file_size", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_CookieFields {}

impl DataRuleset_Rules_ActionParameters_CookieFields { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_CookieFields {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_CookieFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_CookieFields {}

impl BuildDataRuleset_Rules_ActionParameters_CookieFields {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_CookieFields {
        DataRuleset_Rules_ActionParameters_CookieFields {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_CookieFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_CookieFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_CookieFieldsRef {
        DataRuleset_Rules_ActionParameters_CookieFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_CookieFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cookie."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {}

impl DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {}

impl BuildDataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {
        DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRange {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
        DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\nThe lower bound of the range."]
    pub fn from(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\nThe upper bound of the range."]
    pub fn to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {}

impl DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {}

impl BuildDataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {
        DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef {
        DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nA single status code to apply the TTL to."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code_range` after provisioning.\nA range of status codes to apply the TTL to."]
    pub fn status_code_range(&self) -> DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef {
        DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtl_StatusCodeRangeRef::new(
            self.shared().clone(),
            format!("{}.status_code_range", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe time to cache the response for (in seconds). A value of 0 is equivalent to setting the cache control header with the value \"no-cache\". A value of -1 is equivalent to setting the cache control header with the value of \"no-store\"."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_EdgeTtl {}

impl DataRuleset_Rules_ActionParameters_EdgeTtl { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_EdgeTtl {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_EdgeTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_EdgeTtl {}

impl BuildDataRuleset_Rules_ActionParameters_EdgeTtl {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_EdgeTtl {
        DataRuleset_Rules_ActionParameters_EdgeTtl {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_EdgeTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_EdgeTtlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_EdgeTtlRef {
        DataRuleset_Rules_ActionParameters_EdgeTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_EdgeTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nThe edge TTL (in seconds) if you choose the \"override_origin\" mode."]
    pub fn default(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe edge TTL mode.\nAvailable values: \"respect_origin\", \"bypass_by_default\", \"override_origin\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code_ttl` after provisioning.\nA list of TTLs to apply to specific status codes or status code ranges."]
    pub fn status_code_ttl(&self) -> terrars::ListRef<DataRuleset_Rules_ActionParameters_EdgeTtl_StatusCodeTtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status_code_ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_FromList {}

impl DataRuleset_Rules_ActionParameters_FromList { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_FromList {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_FromList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_FromList {}

impl BuildDataRuleset_Rules_ActionParameters_FromList {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_FromList {
        DataRuleset_Rules_ActionParameters_FromList {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_FromListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_FromListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_FromListRef {
        DataRuleset_Rules_ActionParameters_FromListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_FromListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nAn expression that evaluates to the list lookup key."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the list to match against."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_FromValue_TargetUrl {}

impl DataRuleset_Rules_ActionParameters_FromValue_TargetUrl { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_FromValue_TargetUrl {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_FromValue_TargetUrl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_FromValue_TargetUrl {}

impl BuildDataRuleset_Rules_ActionParameters_FromValue_TargetUrl {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_FromValue_TargetUrl {
        DataRuleset_Rules_ActionParameters_FromValue_TargetUrl {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_FromValue_TargetUrlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_FromValue_TargetUrlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_FromValue_TargetUrlRef {
        DataRuleset_Rules_ActionParameters_FromValue_TargetUrlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_FromValue_TargetUrlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nAn expression that evaluates to a URL to redirect the request to."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nA URL to redirect the request to."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_FromValue {}

impl DataRuleset_Rules_ActionParameters_FromValue { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_FromValue {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_FromValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_FromValue {}

impl BuildDataRuleset_Rules_ActionParameters_FromValue {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_FromValue {
        DataRuleset_Rules_ActionParameters_FromValue {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_FromValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_FromValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_FromValueRef {
        DataRuleset_Rules_ActionParameters_FromValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_FromValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `preserve_query_string` after provisioning.\nWhether to keep the query string of the original request."]
    pub fn preserve_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_query_string", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nThe status code to use for the redirect."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc = "Get a reference to the value of field `target_url` after provisioning.\nA URL to redirect the request to."]
    pub fn target_url(&self) -> DataRuleset_Rules_ActionParameters_FromValue_TargetUrlRef {
        DataRuleset_Rules_ActionParameters_FromValue_TargetUrlRef::new(
            self.shared().clone(),
            format!("{}.target_url", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Headers {}

impl DataRuleset_Rules_ActionParameters_Headers { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Headers {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Headers {}

impl BuildDataRuleset_Rules_ActionParameters_Headers {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Headers {
        DataRuleset_Rules_ActionParameters_Headers {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_HeadersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_HeadersRef {
        DataRuleset_Rules_ActionParameters_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nAn expression that evaluates to a value for the header."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform on the header.\nAvailable values: \"add\", \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nA static value for the header."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Immutable {}

impl DataRuleset_Rules_ActionParameters_Immutable { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Immutable {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Immutable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Immutable {}

impl BuildDataRuleset_Rules_ActionParameters_Immutable {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Immutable {
        DataRuleset_Rules_ActionParameters_Immutable {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_ImmutableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_ImmutableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_ImmutableRef {
        DataRuleset_Rules_ActionParameters_ImmutableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_ImmutableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_MatchedData {}

impl DataRuleset_Rules_ActionParameters_MatchedData { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_MatchedData {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_MatchedData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_MatchedData {}

impl BuildDataRuleset_Rules_ActionParameters_MatchedData {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_MatchedData {
        DataRuleset_Rules_ActionParameters_MatchedData {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_MatchedDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_MatchedDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_MatchedDataRef {
        DataRuleset_Rules_ActionParameters_MatchedDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_MatchedDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `public_key` after provisioning.\nThe public key to encrypt matched data logs with."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_MaxAge {}

impl DataRuleset_Rules_ActionParameters_MaxAge { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_MaxAge {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_MaxAge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_MaxAge {}

impl BuildDataRuleset_Rules_ActionParameters_MaxAge {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_MaxAge {
        DataRuleset_Rules_ActionParameters_MaxAge {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_MaxAgeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_MaxAgeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_MaxAgeRef {
        DataRuleset_Rules_ActionParameters_MaxAgeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_MaxAgeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value for the directive in seconds."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_MustRevalidate {}

impl DataRuleset_Rules_ActionParameters_MustRevalidate { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_MustRevalidate {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_MustRevalidate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_MustRevalidate {}

impl BuildDataRuleset_Rules_ActionParameters_MustRevalidate {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_MustRevalidate {
        DataRuleset_Rules_ActionParameters_MustRevalidate {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_MustRevalidateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_MustRevalidateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_MustRevalidateRef {
        DataRuleset_Rules_ActionParameters_MustRevalidateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_MustRevalidateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_MustUnderstand {}

impl DataRuleset_Rules_ActionParameters_MustUnderstand { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_MustUnderstand {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_MustUnderstand>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_MustUnderstand {}

impl BuildDataRuleset_Rules_ActionParameters_MustUnderstand {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_MustUnderstand {
        DataRuleset_Rules_ActionParameters_MustUnderstand {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_MustUnderstandRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_MustUnderstandRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_MustUnderstandRef {
        DataRuleset_Rules_ActionParameters_MustUnderstandRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_MustUnderstandRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_NoCache {}

impl DataRuleset_Rules_ActionParameters_NoCache { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_NoCache {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_NoCache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_NoCache {}

impl BuildDataRuleset_Rules_ActionParameters_NoCache {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_NoCache {
        DataRuleset_Rules_ActionParameters_NoCache {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_NoCacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_NoCacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_NoCacheRef {
        DataRuleset_Rules_ActionParameters_NoCacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_NoCacheRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc = "Get a reference to the value of field `qualifiers` after provisioning.\nThe qualifiers for the directive."]
    pub fn qualifiers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qualifiers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_NoStore {}

impl DataRuleset_Rules_ActionParameters_NoStore { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_NoStore {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_NoStore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_NoStore {}

impl BuildDataRuleset_Rules_ActionParameters_NoStore {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_NoStore {
        DataRuleset_Rules_ActionParameters_NoStore {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_NoStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_NoStoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_NoStoreRef {
        DataRuleset_Rules_ActionParameters_NoStoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_NoStoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_NoTransform {}

impl DataRuleset_Rules_ActionParameters_NoTransform { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_NoTransform {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_NoTransform>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_NoTransform {}

impl BuildDataRuleset_Rules_ActionParameters_NoTransform {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_NoTransform {
        DataRuleset_Rules_ActionParameters_NoTransform {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_NoTransformRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_NoTransformRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_NoTransformRef {
        DataRuleset_Rules_ActionParameters_NoTransformRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_NoTransformRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Origin {}

impl DataRuleset_Rules_ActionParameters_Origin { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Origin {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Origin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Origin {}

impl BuildDataRuleset_Rules_ActionParameters_Origin {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Origin {
        DataRuleset_Rules_ActionParameters_Origin {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_OriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_OriginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_OriginRef {
        DataRuleset_Rules_ActionParameters_OriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_OriginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nA resolved host to route to."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nA destination port to route to."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_OriginRangeRequests {}

impl DataRuleset_Rules_ActionParameters_OriginRangeRequests { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_OriginRangeRequests {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_OriginRangeRequests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_OriginRangeRequests {}

impl BuildDataRuleset_Rules_ActionParameters_OriginRangeRequests {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_OriginRangeRequests {
        DataRuleset_Rules_ActionParameters_OriginRangeRequests {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_OriginRangeRequestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_OriginRangeRequestsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_OriginRangeRequestsRef {
        DataRuleset_Rules_ActionParameters_OriginRangeRequestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_OriginRangeRequestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nWhether to use range requests. `default` is the behaviour the zone gets without this rule.\nAvailable values: \"on\", \"off\", \"default\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Overrides_Categories {}

impl DataRuleset_Rules_ActionParameters_Overrides_Categories { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Overrides_Categories {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Overrides_Categories>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Overrides_Categories {}

impl BuildDataRuleset_Rules_ActionParameters_Overrides_Categories {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Overrides_Categories {
        DataRuleset_Rules_ActionParameters_Overrides_Categories {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_Overrides_CategoriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_Overrides_CategoriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_Overrides_CategoriesRef {
        DataRuleset_Rules_ActionParameters_Overrides_CategoriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_Overrides_CategoriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action to override rules in the category with."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `category` after provisioning.\nThe name of the category to override."]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable execution of rules in the category."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sensitivity_level` after provisioning.\nThe sensitivity level to use for rules in the category. This option is only applicable for DDoS phases.\nAvailable values: \"default\", \"medium\", \"low\", \"eoff\"."]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Overrides_Rules {}

impl DataRuleset_Rules_ActionParameters_Overrides_Rules { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Overrides_Rules {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Overrides_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Overrides_Rules {}

impl BuildDataRuleset_Rules_ActionParameters_Overrides_Rules {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Overrides_Rules {
        DataRuleset_Rules_ActionParameters_Overrides_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_Overrides_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_Overrides_RulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_Overrides_RulesRef {
        DataRuleset_Rules_ActionParameters_Overrides_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_Overrides_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe action to override the rule with."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable execution of the rule."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the rule to override."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_threshold` after provisioning.\nThe score threshold to use for the rule."]
    pub fn score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sensitivity_level` after provisioning.\nThe sensitivity level to use for the rule. This option is only applicable for DDoS phases.\nAvailable values: \"default\", \"medium\", \"low\", \"eoff\"."]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Overrides {}

impl DataRuleset_Rules_ActionParameters_Overrides { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Overrides {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Overrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Overrides {}

impl BuildDataRuleset_Rules_ActionParameters_Overrides {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Overrides {
        DataRuleset_Rules_ActionParameters_Overrides {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_OverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_OverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_OverridesRef {
        DataRuleset_Rules_ActionParameters_OverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_OverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAn action to override all rules with. This option has lower precedence than rule and category overrides."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `categories` after provisioning.\nA list of category-level overrides. This option has the second-highest precedence after rule-level overrides."]
    pub fn categories(&self) -> terrars::ListRef<DataRuleset_Rules_ActionParameters_Overrides_CategoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categories", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable execution of all rules. This option has lower precedence than rule and category overrides."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nA list of rule-level overrides. This option has the highest precedence."]
    pub fn rules(&self) -> terrars::ListRef<DataRuleset_Rules_ActionParameters_Overrides_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sensitivity_level` after provisioning.\nA sensitivity level to set for all rules. This option has lower precedence than rule and category overrides and is only applicable for DDoS phases.\nAvailable values: \"default\", \"medium\", \"low\", \"eoff\"."]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Private {}

impl DataRuleset_Rules_ActionParameters_Private { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Private {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Private>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Private {}

impl BuildDataRuleset_Rules_ActionParameters_Private {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Private {
        DataRuleset_Rules_ActionParameters_Private {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_PrivateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_PrivateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_PrivateRef {
        DataRuleset_Rules_ActionParameters_PrivateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_PrivateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc = "Get a reference to the value of field `qualifiers` after provisioning.\nThe qualifiers for the directive."]
    pub fn qualifiers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qualifiers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_ProxyRevalidate {}

impl DataRuleset_Rules_ActionParameters_ProxyRevalidate { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_ProxyRevalidate {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_ProxyRevalidate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_ProxyRevalidate {}

impl BuildDataRuleset_Rules_ActionParameters_ProxyRevalidate {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_ProxyRevalidate {
        DataRuleset_Rules_ActionParameters_ProxyRevalidate {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_ProxyRevalidateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_ProxyRevalidateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_ProxyRevalidateRef {
        DataRuleset_Rules_ActionParameters_ProxyRevalidateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_ProxyRevalidateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Public {}

impl DataRuleset_Rules_ActionParameters_Public { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Public {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Public>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Public {}

impl BuildDataRuleset_Rules_ActionParameters_Public {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Public {
        DataRuleset_Rules_ActionParameters_Public {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_PublicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_PublicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_PublicRef {
        DataRuleset_Rules_ActionParameters_PublicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_PublicRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_RawResponseFields {}

impl DataRuleset_Rules_ActionParameters_RawResponseFields { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_RawResponseFields {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_RawResponseFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_RawResponseFields {}

impl BuildDataRuleset_Rules_ActionParameters_RawResponseFields {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_RawResponseFields {
        DataRuleset_Rules_ActionParameters_RawResponseFields {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_RawResponseFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_RawResponseFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_RawResponseFieldsRef {
        DataRuleset_Rules_ActionParameters_RawResponseFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_RawResponseFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the response header."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preserve_duplicates` after provisioning.\nWhether to log duplicate values of the same header."]
    pub fn preserve_duplicates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_duplicates", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_RequestFields {}

impl DataRuleset_Rules_ActionParameters_RequestFields { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_RequestFields {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_RequestFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_RequestFields {}

impl BuildDataRuleset_Rules_ActionParameters_RequestFields {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_RequestFields {
        DataRuleset_Rules_ActionParameters_RequestFields {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_RequestFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_RequestFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_RequestFieldsRef {
        DataRuleset_Rules_ActionParameters_RequestFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_RequestFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the header."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Response {}

impl DataRuleset_Rules_ActionParameters_Response { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Response {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Response {}

impl BuildDataRuleset_Rules_ActionParameters_Response {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Response {
        DataRuleset_Rules_ActionParameters_Response {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_ResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_ResponseRef {
        DataRuleset_Rules_ActionParameters_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_ResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe content to return."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nThe type of the content to return."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\nThe status code to return."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_ResponseFields {}

impl DataRuleset_Rules_ActionParameters_ResponseFields { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_ResponseFields {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_ResponseFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_ResponseFields {}

impl BuildDataRuleset_Rules_ActionParameters_ResponseFields {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_ResponseFields {
        DataRuleset_Rules_ActionParameters_ResponseFields {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_ResponseFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_ResponseFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_ResponseFieldsRef {
        DataRuleset_Rules_ActionParameters_ResponseFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_ResponseFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the response header."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preserve_duplicates` after provisioning.\nWhether to log duplicate values of the same header."]
    pub fn preserve_duplicates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_duplicates", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_SMaxage {}

impl DataRuleset_Rules_ActionParameters_SMaxage { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_SMaxage {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_SMaxage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_SMaxage {}

impl BuildDataRuleset_Rules_ActionParameters_SMaxage {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_SMaxage {
        DataRuleset_Rules_ActionParameters_SMaxage {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_SMaxageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_SMaxageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_SMaxageRef {
        DataRuleset_Rules_ActionParameters_SMaxageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_SMaxageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value for the directive in seconds."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_ServeStale {}

impl DataRuleset_Rules_ActionParameters_ServeStale { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_ServeStale {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_ServeStale>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_ServeStale {}

impl BuildDataRuleset_Rules_ActionParameters_ServeStale {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_ServeStale {
        DataRuleset_Rules_ActionParameters_ServeStale {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_ServeStaleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_ServeStaleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_ServeStaleRef {
        DataRuleset_Rules_ActionParameters_ServeStaleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_ServeStaleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_stale_while_updating` after provisioning.\nWhether Cloudflare should disable serving stale content while getting the latest content from the origin."]
    pub fn disable_stale_while_updating(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_stale_while_updating", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Sni {}

impl DataRuleset_Rules_ActionParameters_Sni { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Sni {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Sni>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Sni {}

impl BuildDataRuleset_Rules_ActionParameters_Sni {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Sni {
        DataRuleset_Rules_ActionParameters_Sni {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_SniRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_SniRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_SniRef {
        DataRuleset_Rules_ActionParameters_SniRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_SniRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nA value to override the SNI to."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_StaleIfError {}

impl DataRuleset_Rules_ActionParameters_StaleIfError { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_StaleIfError {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_StaleIfError>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_StaleIfError {}

impl BuildDataRuleset_Rules_ActionParameters_StaleIfError {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_StaleIfError {
        DataRuleset_Rules_ActionParameters_StaleIfError {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_StaleIfErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_StaleIfErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_StaleIfErrorRef {
        DataRuleset_Rules_ActionParameters_StaleIfErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_StaleIfErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value for the directive in seconds."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_StaleWhileRevalidate {}

impl DataRuleset_Rules_ActionParameters_StaleWhileRevalidate { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_StaleWhileRevalidate {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_StaleWhileRevalidate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_StaleWhileRevalidate {}

impl BuildDataRuleset_Rules_ActionParameters_StaleWhileRevalidate {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_StaleWhileRevalidate {
        DataRuleset_Rules_ActionParameters_StaleWhileRevalidate {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_StaleWhileRevalidateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_StaleWhileRevalidateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_StaleWhileRevalidateRef {
        DataRuleset_Rules_ActionParameters_StaleWhileRevalidateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_StaleWhileRevalidateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_only` after provisioning.\nWhether to apply the directive only to Cloudflare's cache."]
    pub fn cloudflare_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform.\nAvailable values: \"set\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value for the directive in seconds."]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_TransformedRequestFields {}

impl DataRuleset_Rules_ActionParameters_TransformedRequestFields { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_TransformedRequestFields {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_TransformedRequestFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_TransformedRequestFields {}

impl BuildDataRuleset_Rules_ActionParameters_TransformedRequestFields {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_TransformedRequestFields {
        DataRuleset_Rules_ActionParameters_TransformedRequestFields {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_TransformedRequestFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_TransformedRequestFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_TransformedRequestFieldsRef {
        DataRuleset_Rules_ActionParameters_TransformedRequestFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_TransformedRequestFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the header."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Uri_Path {}

impl DataRuleset_Rules_ActionParameters_Uri_Path { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Uri_Path {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Uri_Path>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Uri_Path {}

impl BuildDataRuleset_Rules_ActionParameters_Uri_Path {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Uri_Path {
        DataRuleset_Rules_ActionParameters_Uri_Path {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_Uri_PathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_Uri_PathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_Uri_PathRef {
        DataRuleset_Rules_ActionParameters_Uri_PathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_Uri_PathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nAn expression that evaluates to a value to rewrite the URI path to."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nA value to rewrite the URI path to."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Uri_Query {}

impl DataRuleset_Rules_ActionParameters_Uri_Query { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Uri_Query {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Uri_Query>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Uri_Query {}

impl BuildDataRuleset_Rules_ActionParameters_Uri_Query {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Uri_Query {
        DataRuleset_Rules_ActionParameters_Uri_Query {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_Uri_QueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_Uri_QueryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_Uri_QueryRef {
        DataRuleset_Rules_ActionParameters_Uri_QueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_Uri_QueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nAn expression that evaluates to a value to rewrite the URI query to."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nA value to rewrite the URI query to."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Uri {}

impl DataRuleset_Rules_ActionParameters_Uri { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Uri {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Uri>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Uri {}

impl BuildDataRuleset_Rules_ActionParameters_Uri {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Uri {
        DataRuleset_Rules_ActionParameters_Uri {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_UriRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_UriRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_UriRef {
        DataRuleset_Rules_ActionParameters_UriRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_UriRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nWhether to propagate the rewritten URI to origin."]
    pub fn origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nA URI path rewrite."]
    pub fn path(&self) -> DataRuleset_Rules_ActionParameters_Uri_PathRef {
        DataRuleset_Rules_ActionParameters_Uri_PathRef::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\nA URI query rewrite."]
    pub fn query(&self) -> DataRuleset_Rules_ActionParameters_Uri_QueryRef {
        DataRuleset_Rules_ActionParameters_Uri_QueryRef::new(self.shared().clone(), format!("{}.query", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Vary_Default {}

impl DataRuleset_Rules_ActionParameters_Vary_Default { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Vary_Default {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Vary_Default>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Vary_Default {}

impl BuildDataRuleset_Rules_ActionParameters_Vary_Default {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Vary_Default {
        DataRuleset_Rules_ActionParameters_Vary_Default {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_Vary_DefaultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_Vary_DefaultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_Vary_DefaultRef {
        DataRuleset_Rules_ActionParameters_Vary_DefaultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_Vary_DefaultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nHow the header value is treated when building the cache key.\nAvailable values: \"bypass\", \"passthrough\", \"normalize\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Vary_Headers {}

impl DataRuleset_Rules_ActionParameters_Vary_Headers { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Vary_Headers {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Vary_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Vary_Headers {}

impl BuildDataRuleset_Rules_ActionParameters_Vary_Headers {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Vary_Headers {
        DataRuleset_Rules_ActionParameters_Vary_Headers {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_Vary_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_Vary_HeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRuleset_Rules_ActionParameters_Vary_HeadersRef {
        DataRuleset_Rules_ActionParameters_Vary_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_Vary_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nHow the header value is treated when building the cache key.\nAvailable values: \"bypass\", \"passthrough\", \"normalize\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `languages` after provisioning.\nThe set of languages to normalize against. Only valid for the `accept-language` header."]
    pub fn languages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.languages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `media_types` after provisioning.\nThe set of media types to normalize against. Only valid for the `accept` header."]
    pub fn media_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters_Vary {}

impl DataRuleset_Rules_ActionParameters_Vary { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters_Vary {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters_Vary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters_Vary {}

impl BuildDataRuleset_Rules_ActionParameters_Vary {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters_Vary {
        DataRuleset_Rules_ActionParameters_Vary {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParameters_VaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParameters_VaryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParameters_VaryRef {
        DataRuleset_Rules_ActionParameters_VaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParameters_VaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nControls how response Vary headers without a per-header override contribute to the cache key."]
    pub fn default(&self) -> DataRuleset_Rules_ActionParameters_Vary_DefaultRef {
        DataRuleset_Rules_ActionParameters_Vary_DefaultRef::new(
            self.shared().clone(),
            format!("{}.default", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nA mapping of lowercase request header names to their vary configuration."]
    pub fn headers(&self) -> terrars::RecRef<DataRuleset_Rules_ActionParameters_Vary_HeadersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ActionParameters {}

impl DataRuleset_Rules_ActionParameters { }

impl terrars::ToListMappable for DataRuleset_Rules_ActionParameters {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ActionParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ActionParameters {}

impl BuildDataRuleset_Rules_ActionParameters {
    pub fn build(self) -> DataRuleset_Rules_ActionParameters {
        DataRuleset_Rules_ActionParameters {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ActionParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ActionParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ActionParametersRef {
        DataRuleset_Rules_ActionParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ActionParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_cacheable_ports` after provisioning.\nA list of additional ports that caching should be enabled on."]
    pub fn additional_cacheable_ports(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_cacheable_ports", self.base))
    }

    #[doc =
        "Get a reference to the value of field `algorithms` after provisioning.\nCustom order for compression algorithms."]
    pub fn algorithms(&self) -> terrars::ListRef<DataRuleset_Rules_ActionParameters_AlgorithmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.algorithms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `asset_name` after provisioning.\nThe name of a custom asset to serve as the response."]
    pub fn asset_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\nWhether to enable Automatic HTTPS Rewrites."]
    pub fn automatic_https_rewrites(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc =
        "Get a reference to the value of field `autominify` after provisioning.\nWhich file extensions to minify automatically."]
    pub fn autominify(&self) -> DataRuleset_Rules_ActionParameters_AutominifyRef {
        DataRuleset_Rules_ActionParameters_AutominifyRef::new(
            self.shared().clone(),
            format!("{}.autominify", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `bic` after provisioning.\nWhether to enable Browser Integrity Check (BIC)."]
    pub fn bic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `browser_ttl` after provisioning.\nHow long client browsers should cache the response. Cloudflare cache purge will not purge content cached on client browsers, so high browser TTLs may lead to stale content."]
    pub fn browser_ttl(&self) -> DataRuleset_Rules_ActionParameters_BrowserTtlRef {
        DataRuleset_Rules_ActionParameters_BrowserTtlRef::new(
            self.shared().clone(),
            format!("{}.browser_ttl", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `cache` after provisioning.\nWhether the request's response from the origin is eligible for caching. Caching itself will still depend on the cache control header and your other caching configurations."]
    pub fn cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_key` after provisioning.\nWhich components of the request are included in or excluded from the cache key Cloudflare uses to store the response in cache."]
    pub fn cache_key(&self) -> DataRuleset_Rules_ActionParameters_CacheKeyRef {
        DataRuleset_Rules_ActionParameters_CacheKeyRef::new(
            self.shared().clone(),
            format!("{}.cache_key", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `cache_reserve` after provisioning.\nSettings to determine whether the request's response from origin is eligible for Cache Reserve (requires a Cache Reserve add-on plan)."]
    pub fn cache_reserve(&self) -> DataRuleset_Rules_ActionParameters_CacheReserveRef {
        DataRuleset_Rules_ActionParameters_CacheReserveRef::new(
            self.shared().clone(),
            format!("{}.cache_reserve", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe response content."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_converter` after provisioning.\nWhether to enable content conversion (e.g., HTML to Markdown)."]
    pub fn content_converter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_converter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nThe content type header to set with the error response.\nAvailable values: \"application/json\", \"text/html\", \"text/plain\", \"text/xml\"."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cookie_fields` after provisioning.\nThe cookie fields to log."]
    pub fn cookie_fields(&self) -> terrars::ListRef<DataRuleset_Rules_ActionParameters_CookieFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookie_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_apps` after provisioning.\nWhether to disable Cloudflare Apps."]
    pub fn disable_apps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_apps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_rum` after provisioning.\nWhether to disable Real User Monitoring (RUM)."]
    pub fn disable_rum(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_rum", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_zaraz` after provisioning.\nWhether to disable Zaraz."]
    pub fn disable_zaraz(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_zaraz", self.base))
    }

    #[doc =
        "Get a reference to the value of field `edge_ttl` after provisioning.\nHow long the Cloudflare edge network should cache the response."]
    pub fn edge_ttl(&self) -> DataRuleset_Rules_ActionParameters_EdgeTtlRef {
        DataRuleset_Rules_ActionParameters_EdgeTtlRef::new(self.shared().clone(), format!("{}.edge_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_obfuscation` after provisioning.\nWhether to enable Email Obfuscation."]
    pub fn email_obfuscation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nAn expression to generate cache tags for set_cache_tags action."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `fonts` after provisioning.\nWhether to enable Cloudflare Fonts."]
    pub fn fonts(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fonts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `from_list` after provisioning.\nA redirect based on a bulk list lookup."]
    pub fn from_list(&self) -> DataRuleset_Rules_ActionParameters_FromListRef {
        DataRuleset_Rules_ActionParameters_FromListRef::new(
            self.shared().clone(),
            format!("{}.from_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `from_value` after provisioning.\nA redirect based on the request properties."]
    pub fn from_value(&self) -> DataRuleset_Rules_ActionParameters_FromValueRef {
        DataRuleset_Rules_ActionParameters_FromValueRef::new(
            self.shared().clone(),
            format!("{}.from_value", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\nA map of headers to rewrite."]
    pub fn headers(&self) -> terrars::RecRef<DataRuleset_Rules_ActionParameters_HeadersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_header` after provisioning.\nA value to rewrite the HTTP host header to."]
    pub fn host_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hotlink_protection` after provisioning.\nWhether to enable Hotlink Protection."]
    pub fn hotlink_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hotlink_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the ruleset to execute."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `immutable` after provisioning.\nSet the immutable cache control directive."]
    pub fn immutable(&self) -> DataRuleset_Rules_ActionParameters_ImmutableRef {
        DataRuleset_Rules_ActionParameters_ImmutableRef::new(
            self.shared().clone(),
            format!("{}.immutable", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `increment` after provisioning.\nA delta to change the score by, which can be either positive or negative."]
    pub fn increment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.increment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `matched_data` after provisioning.\nThe configuration to use for matched data logging."]
    pub fn matched_data(&self) -> DataRuleset_Rules_ActionParameters_MatchedDataRef {
        DataRuleset_Rules_ActionParameters_MatchedDataRef::new(
            self.shared().clone(),
            format!("{}.matched_data", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nSet the max-age cache control directive."]
    pub fn max_age(&self) -> DataRuleset_Rules_ActionParameters_MaxAgeRef {
        DataRuleset_Rules_ActionParameters_MaxAgeRef::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `mirage` after provisioning.\nWhether to enable Mirage."]
    pub fn mirage(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `must_revalidate` after provisioning.\nSet the must-revalidate cache control directive."]
    pub fn must_revalidate(&self) -> DataRuleset_Rules_ActionParameters_MustRevalidateRef {
        DataRuleset_Rules_ActionParameters_MustRevalidateRef::new(
            self.shared().clone(),
            format!("{}.must_revalidate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `must_understand` after provisioning.\nSet the must-understand cache control directive."]
    pub fn must_understand(&self) -> DataRuleset_Rules_ActionParameters_MustUnderstandRef {
        DataRuleset_Rules_ActionParameters_MustUnderstandRef::new(
            self.shared().clone(),
            format!("{}.must_understand", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `no_cache` after provisioning.\nSet the no-cache cache control directive."]
    pub fn no_cache(&self) -> DataRuleset_Rules_ActionParameters_NoCacheRef {
        DataRuleset_Rules_ActionParameters_NoCacheRef::new(self.shared().clone(), format!("{}.no_cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_store` after provisioning.\nSet the no-store cache control directive."]
    pub fn no_store(&self) -> DataRuleset_Rules_ActionParameters_NoStoreRef {
        DataRuleset_Rules_ActionParameters_NoStoreRef::new(self.shared().clone(), format!("{}.no_store", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_transform` after provisioning.\nSet the no-transform cache control directive."]
    pub fn no_transform(&self) -> DataRuleset_Rules_ActionParameters_NoTransformRef {
        DataRuleset_Rules_ActionParameters_NoTransformRef::new(
            self.shared().clone(),
            format!("{}.no_transform", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe operation to perform for set_cache_tags action.\nAvailable values: \"set\", \"add\", \"remove\"."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `opportunistic_encryption` after provisioning.\nWhether to enable Opportunistic Encryption."]
    pub fn opportunistic_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\nAn origin to route to."]
    pub fn origin(&self) -> DataRuleset_Rules_ActionParameters_OriginRef {
        DataRuleset_Rules_ActionParameters_OriginRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_cache_control` after provisioning.\nWhether Cloudflare will aim to strictly adhere to RFC 7234."]
    pub fn origin_cache_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_cache_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_error_page_passthru` after provisioning.\nWhether to generate Cloudflare error pages for issues from the origin server."]
    pub fn origin_error_page_passthru(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_passthru", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_range_requests` after provisioning.\nControls whether Cloudflare fetches a large asset from the origin as a series of range requests instead of one whole-body request."]
    pub fn origin_range_requests(&self) -> DataRuleset_Rules_ActionParameters_OriginRangeRequestsRef {
        DataRuleset_Rules_ActionParameters_OriginRangeRequestsRef::new(
            self.shared().clone(),
            format!("{}.origin_range_requests", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `overrides` after provisioning.\nA set of overrides to apply to the target ruleset."]
    pub fn overrides(&self) -> DataRuleset_Rules_ActionParameters_OverridesRef {
        DataRuleset_Rules_ActionParameters_OverridesRef::new(
            self.shared().clone(),
            format!("{}.overrides", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `phases` after provisioning.\nA list of phases to skip the execution of. This option is incompatible with the rulesets option.\nAvailable values: \"ddos_l4\", \"ddos_l7\", \"http_config_settings\", \"http_custom_errors\", \"http_log_custom_fields\", \"http_ratelimit\", \"http_request_cache_settings\", \"http_request_dynamic_redirect\", \"http_request_firewall_custom\", \"http_request_firewall_managed\", \"http_request_late_transform\", \"http_request_origin\", \"http_request_redirect\", \"http_request_sanitize\", \"http_request_sbfm\", \"http_request_transform\", \"http_response_cache_settings\", \"http_response_compression\", \"http_response_firewall_managed\", \"http_response_headers_transform\", \"magic_transit\", \"magic_transit_ids_managed\", \"magic_transit_managed\", \"magic_transit_ratelimit\"."]
    pub fn phases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `polish` after provisioning.\nThe Polish level to configure.\nAvailable values: \"off\", \"lossless\", \"lossy\", \"webp\"."]
    pub fn polish(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private` after provisioning.\nSet the private cache control directive."]
    pub fn private(&self) -> DataRuleset_Rules_ActionParameters_PrivateRef {
        DataRuleset_Rules_ActionParameters_PrivateRef::new(self.shared().clone(), format!("{}.private", self.base))
    }

    #[doc =
        "Get a reference to the value of field `products` after provisioning.\nA list of legacy security products to skip the execution of.\nAvailable values: \"bic\", \"hot\", \"rateLimit\", \"securityLevel\", \"uaBlock\", \"waf\", \"zoneLockdown\"."]
    pub fn products(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.products", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_revalidate` after provisioning.\nSet the proxy-revalidate cache control directive."]
    pub fn proxy_revalidate(&self) -> DataRuleset_Rules_ActionParameters_ProxyRevalidateRef {
        DataRuleset_Rules_ActionParameters_ProxyRevalidateRef::new(
            self.shared().clone(),
            format!("{}.proxy_revalidate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `public` after provisioning.\nSet the public cache control directive."]
    pub fn public(&self) -> DataRuleset_Rules_ActionParameters_PublicRef {
        DataRuleset_Rules_ActionParameters_PublicRef::new(self.shared().clone(), format!("{}.public", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_response_fields` after provisioning.\nThe raw response fields to log."]
    pub fn raw_response_fields(&self) -> terrars::ListRef<DataRuleset_Rules_ActionParameters_RawResponseFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.raw_response_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_timeout` after provisioning.\nA timeout value between two successive read operations to use for your origin server. Historically, the timeout value between two read options from Cloudflare to an origin server is 100 seconds. If you are attempting to reduce HTTP 524 errors because of timeouts from an origin server, try increasing this timeout value."]
    pub fn read_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirects_for_ai_training` after provisioning.\nWhether to redirect verified AI training crawlers to canonical URLs."]
    pub fn redirects_for_ai_training(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirects_for_ai_training", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_body_buffering` after provisioning.\nThe request body buffering mode to configure.\nAvailable values: \"none\", \"standard\", \"full\"."]
    pub fn request_body_buffering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_body_buffering", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_fields` after provisioning.\nThe raw request fields to log."]
    pub fn request_fields(&self) -> terrars::ListRef<DataRuleset_Rules_ActionParameters_RequestFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `respect_strong_etags` after provisioning.\nWhether Cloudflare should respect strong ETag (entity tag) headers. If false, Cloudflare converts strong ETag headers to weak ETag headers."]
    pub fn respect_strong_etags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.respect_strong_etags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response` after provisioning.\nThe response to show when the block is applied."]
    pub fn response(&self) -> DataRuleset_Rules_ActionParameters_ResponseRef {
        DataRuleset_Rules_ActionParameters_ResponseRef::new(self.shared().clone(), format!("{}.response", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_body_buffering` after provisioning.\nThe response body buffering mode to configure.\nAvailable values: \"none\", \"standard\"."]
    pub fn response_body_buffering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_body_buffering", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_fields` after provisioning.\nThe transformed response fields to log."]
    pub fn response_fields(&self) -> terrars::ListRef<DataRuleset_Rules_ActionParameters_ResponseFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rocket_loader` after provisioning.\nWhether to enable Rocket Loader."]
    pub fn rocket_loader(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rocket_loader", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nA mapping of ruleset IDs to a list of rule IDs in that ruleset to skip the execution of. This option is incompatible with the ruleset option."]
    pub fn rules(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ruleset` after provisioning.\nA ruleset to skip the execution of. This option is incompatible with the rulesets option.\nAvailable values: \"current\"."]
    pub fn ruleset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ruleset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rulesets` after provisioning.\nA list of ruleset IDs to skip the execution of. This option is incompatible with the ruleset and phases options."]
    pub fn rulesets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rulesets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `s_maxage` after provisioning.\nSet the s-maxage cache control directive."]
    pub fn s_maxage(&self) -> DataRuleset_Rules_ActionParameters_SMaxageRef {
        DataRuleset_Rules_ActionParameters_SMaxageRef::new(self.shared().clone(), format!("{}.s_maxage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_level` after provisioning.\nThe Security Level to configure.\nAvailable values: \"off\", \"essentially_off\", \"low\", \"medium\", \"high\", \"under_attack\"."]
    pub fn security_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serve_stale` after provisioning.\nWhen to serve stale content from cache."]
    pub fn serve_stale(&self) -> DataRuleset_Rules_ActionParameters_ServeStaleRef {
        DataRuleset_Rules_ActionParameters_ServeStaleRef::new(
            self.shared().clone(),
            format!("{}.serve_stale", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `server_side_excludes` after provisioning.\nWhether to enable Server-Side Excludes."]
    pub fn server_side_excludes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_side_excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `sni` after provisioning.\nA Server Name Indication (SNI) override."]
    pub fn sni(&self) -> DataRuleset_Rules_ActionParameters_SniRef {
        DataRuleset_Rules_ActionParameters_SniRef::new(self.shared().clone(), format!("{}.sni", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl` after provisioning.\nThe SSL level to configure.\nAvailable values: \"off\", \"flexible\", \"full\", \"strict\", \"origin_pull\"."]
    pub fn ssl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stale_if_error` after provisioning.\nSet the stale-if-error cache control directive."]
    pub fn stale_if_error(&self) -> DataRuleset_Rules_ActionParameters_StaleIfErrorRef {
        DataRuleset_Rules_ActionParameters_StaleIfErrorRef::new(
            self.shared().clone(),
            format!("{}.stale_if_error", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `stale_while_revalidate` after provisioning.\nSet the stale-while-revalidate cache control directive."]
    pub fn stale_while_revalidate(&self) -> DataRuleset_Rules_ActionParameters_StaleWhileRevalidateRef {
        DataRuleset_Rules_ActionParameters_StaleWhileRevalidateRef::new(
            self.shared().clone(),
            format!("{}.stale_while_revalidate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nThe status code to use for the error."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_etags` after provisioning.\nWhether to strip the ETag header from the response."]
    pub fn strip_etags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_etags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_last_modified` after provisioning.\nWhether to strip the Last-Modified header from the response."]
    pub fn strip_last_modified(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_last_modified", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_set_cookie` after provisioning.\nWhether to strip the Set-Cookie header from the response."]
    pub fn strip_set_cookie(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_set_cookie", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sxg` after provisioning.\nWhether to enable Signed Exchanges (SXG)."]
    pub fn sxg(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sxg", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transformed_request_fields` after provisioning.\nThe transformed request fields to log."]
    pub fn transformed_request_fields(
        &self,
    ) -> terrars::ListRef<DataRuleset_Rules_ActionParameters_TransformedRequestFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformed_request_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nA URI rewrite."]
    pub fn uri(&self) -> DataRuleset_Rules_ActionParameters_UriRef {
        DataRuleset_Rules_ActionParameters_UriRef::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nThe cache tag values for set_cache_tags action."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vary` after provisioning.\nControls how cached responses vary based on request headers. `default` is required and applies to any Vary response header that does not have a per-header override."]
    pub fn vary(&self) -> DataRuleset_Rules_ActionParameters_VaryRef {
        DataRuleset_Rules_ActionParameters_VaryRef::new(self.shared().clone(), format!("{}.vary", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_ExposedCredentialCheck {}

impl DataRuleset_Rules_ExposedCredentialCheck { }

impl terrars::ToListMappable for DataRuleset_Rules_ExposedCredentialCheck {
    type O = terrars::BlockAssignable<DataRuleset_Rules_ExposedCredentialCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_ExposedCredentialCheck {}

impl BuildDataRuleset_Rules_ExposedCredentialCheck {
    pub fn build(self) -> DataRuleset_Rules_ExposedCredentialCheck {
        DataRuleset_Rules_ExposedCredentialCheck {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_ExposedCredentialCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_ExposedCredentialCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_ExposedCredentialCheckRef {
        DataRuleset_Rules_ExposedCredentialCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_ExposedCredentialCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `password_expression` after provisioning.\nAn expression that selects the password used in the credentials check."]
    pub fn password_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username_expression` after provisioning.\nAn expression that selects the user ID used in the credentials check."]
    pub fn username_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_Logging {}

impl DataRuleset_Rules_Logging { }

impl terrars::ToListMappable for DataRuleset_Rules_Logging {
    type O = terrars::BlockAssignable<DataRuleset_Rules_Logging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_Logging {}

impl BuildDataRuleset_Rules_Logging {
    pub fn build(self) -> DataRuleset_Rules_Logging {
        DataRuleset_Rules_Logging {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_LoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_LoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_LoggingRef {
        DataRuleset_Rules_LoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_LoggingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to generate a log when the rule matches."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules_Ratelimit {}

impl DataRuleset_Rules_Ratelimit { }

impl terrars::ToListMappable for DataRuleset_Rules_Ratelimit {
    type O = terrars::BlockAssignable<DataRuleset_Rules_Ratelimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules_Ratelimit {}

impl BuildDataRuleset_Rules_Ratelimit {
    pub fn build(self) -> DataRuleset_Rules_Ratelimit {
        DataRuleset_Rules_Ratelimit {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_Rules_RatelimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_Rules_RatelimitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_Rules_RatelimitRef {
        DataRuleset_Rules_RatelimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_Rules_RatelimitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `characteristics` after provisioning.\nCharacteristics of the request on which the rate limit counter will be incremented."]
    pub fn characteristics(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.characteristics", self.base))
    }

    #[doc =
        "Get a reference to the value of field `counting_expression` after provisioning.\nAn expression that defines when the rate limit counter should be incremented. It defaults to the same as the rule's expression."]
    pub fn counting_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.counting_expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mitigation_timeout` after provisioning.\nPeriod of time in seconds after which the action will be disabled following its first execution."]
    pub fn mitigation_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mitigation_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nPeriod in seconds over which the counter is being incremented."]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests_per_period` after provisioning.\nThe threshold of requests per period after which the action will be executed for the first time."]
    pub fn requests_per_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests_per_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests_to_origin` after provisioning.\nWhether counting is only performed when an origin is reached."]
    pub fn requests_to_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests_to_origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_per_period` after provisioning.\nThe score threshold per period for which the action will be executed the first time."]
    pub fn score_per_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_per_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_response_header_name` after provisioning.\nA response header name provided by the origin, which contains the score to increment rate limit counter with."]
    pub fn score_response_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_response_header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRuleset_Rules {}

impl DataRuleset_Rules { }

impl terrars::ToListMappable for DataRuleset_Rules {
    type O = terrars::BlockAssignable<DataRuleset_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRuleset_Rules {}

impl BuildDataRuleset_Rules {
    pub fn build(self) -> DataRuleset_Rules {
        DataRuleset_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataRuleset_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRuleset_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRuleset_RulesRef {
        DataRuleset_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRuleset_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe action to perform when the rule matches.\nAvailable values: \"block\", \"challenge\", \"compress_response\", \"ddos_dynamic\", \"execute\", \"force_connection_close\", \"js_challenge\", \"log\", \"log_custom_field\", \"managed_challenge\", \"redirect\", \"rewrite\", \"route\", \"score\", \"serve_error\", \"set_cache_control\", \"set_cache_settings\", \"set_cache_tags\", \"set_config\", \"skip\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `action_parameters` after provisioning.\nThe parameters configuring the rule's action."]
    pub fn action_parameters(&self) -> DataRuleset_Rules_ActionParametersRef {
        DataRuleset_Rules_ActionParametersRef::new(self.shared().clone(), format!("{}.action_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `categories` after provisioning.\nThe categories of the rule."]
    pub fn categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categories", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule should be executed."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exposed_credential_check` after provisioning.\nConfiguration for exposed credential checking."]
    pub fn exposed_credential_check(&self) -> DataRuleset_Rules_ExposedCredentialCheckRef {
        DataRuleset_Rules_ExposedCredentialCheckRef::new(
            self.shared().clone(),
            format!("{}.exposed_credential_check", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nThe expression defining which traffic will match the rule."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logging` after provisioning.\nAn object configuring the rule's logging behavior."]
    pub fn logging(&self) -> DataRuleset_Rules_LoggingRef {
        DataRuleset_Rules_LoggingRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ratelimit` after provisioning.\nAn object configuring the rule's rate limit behavior."]
    pub fn ratelimit(&self) -> DataRuleset_Rules_RatelimitRef {
        DataRuleset_Rules_RatelimitRef::new(self.shared().clone(), format!("{}.ratelimit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ref_` after provisioning.\nThe reference of the rule (the rule's ID by default)."]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
}
