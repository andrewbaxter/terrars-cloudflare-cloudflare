#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZoneLockdownData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataZoneLockdown_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lock_downs_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZoneLockdown_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZoneLockdownData_>,
}

#[derive(Clone)]
pub struct DataZoneLockdown(std::rc::Rc<DataZoneLockdown_>);

impl DataZoneLockdown {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataZoneLockdown_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lock_downs_id`.\nThe unique identifier of the Zone Lockdown rule."]
    pub fn set_lock_downs_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().lock_downs_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nDefines an identifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `configurations` after provisioning.\nA list of IP addresses or CIDR ranges that will be allowed to access the URLs specified in the Zone Lockdown rule. You can include any number of `ip` or `ip_range` configurations."]
    pub fn configurations(&self) -> terrars::ListRef<DataZoneLockdown_ConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configurations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe timestamp of when the rule was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZoneLockdown_FilterRef {
        DataZoneLockdown_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the Zone Lockdown rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lock_downs_id` after provisioning.\nThe unique identifier of the Zone Lockdown rule."]
    pub fn lock_downs_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lock_downs_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the rule was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nWhen true, indicates that the rule is currently paused."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urls` after provisioning.\nThe URLs to include in the rule definition. You can use wildcards. Each entered URL will be escaped before use, which means you can only use simple wildcard patterns."]
    pub fn urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nDefines an identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZoneLockdown {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZoneLockdown { }

impl terrars::ToListMappable for DataZoneLockdown {
    type O = terrars::ListRef<DataZoneLockdownRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZoneLockdown_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zone_lockdown".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZoneLockdown {
    pub tf_id: std::string::String,
}

impl BuildDataZoneLockdown {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZoneLockdown {
        let out = DataZoneLockdown(std::rc::Rc::new(DataZoneLockdown_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZoneLockdownData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                lock_downs_id: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZoneLockdownRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneLockdownRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZoneLockdownRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `configurations` after provisioning.\nA list of IP addresses or CIDR ranges that will be allowed to access the URLs specified in the Zone Lockdown rule. You can include any number of `ip` or `ip_range` configurations."]
    pub fn configurations(&self) -> terrars::ListRef<DataZoneLockdown_ConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configurations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe timestamp of when the rule was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn informative summary of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZoneLockdown_FilterRef {
        DataZoneLockdown_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the Zone Lockdown rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lock_downs_id` after provisioning.\nThe unique identifier of the Zone Lockdown rule."]
    pub fn lock_downs_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lock_downs_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the rule was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nWhen true, indicates that the rule is currently paused."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urls` after provisioning.\nThe URLs to include in the rule definition. You can use wildcards. Each entered URL will be escaped before use, which means you can only use simple wildcard patterns."]
    pub fn urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nDefines an identifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZoneLockdown_Configurations {}

impl DataZoneLockdown_Configurations { }

impl terrars::ToListMappable for DataZoneLockdown_Configurations {
    type O = terrars::BlockAssignable<DataZoneLockdown_Configurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZoneLockdown_Configurations {}

impl BuildDataZoneLockdown_Configurations {
    pub fn build(self) -> DataZoneLockdown_Configurations {
        DataZoneLockdown_Configurations {}
    }
}

#[allow(dead_code)]
pub struct DataZoneLockdown_ConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneLockdown_ConfigurationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZoneLockdown_ConfigurationsRef {
        DataZoneLockdown_ConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZoneLockdown_ConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nThe configuration target. You must set the target to `ip` when specifying an IP address in the Zone Lockdown rule.\nAvailable values: \"ip\", \"ip_range\"."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe IP address to match. This address will be compared to the IP address of incoming requests."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZoneLockdown_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_on: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description_search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_range_search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modified_on: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri_search: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZoneLockdown_Filter {
    #[doc = "Set the field `created_on`.\nThe timestamp of when the rule was created."]
    pub fn set_created_on(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_on = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA string to search for in the description of existing rules."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description_search`.\nA string to search for in the description of existing rules."]
    pub fn set_description_search(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.description_search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\nA single IP address to search for in existing rules."]
    pub fn set_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_range_search`.\nA single IP address range to search for in existing rules."]
    pub fn set_ip_range_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_range_search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_search`.\nA single IP address to search for in existing rules."]
    pub fn set_ip_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `modified_on`.\nThe timestamp of when the rule was last modified."]
    pub fn set_modified_on(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.modified_on = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `priority`.\nThe priority of the rule to control the processing order. A lower number indicates higher priority. If not provided, any rules with a configured priority will be processed before rules without a priority."]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri_search`.\nA single URI to search for in the list of URLs of existing rules."]
    pub fn set_uri_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri_search = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZoneLockdown_Filter {
    type O = terrars::BlockAssignable<DataZoneLockdown_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZoneLockdown_Filter {}

impl BuildDataZoneLockdown_Filter {
    pub fn build(self) -> DataZoneLockdown_Filter {
        DataZoneLockdown_Filter {
            created_on: core::default::Default::default(),
            description: core::default::Default::default(),
            description_search: core::default::Default::default(),
            ip: core::default::Default::default(),
            ip_range_search: core::default::Default::default(),
            ip_search: core::default::Default::default(),
            modified_on: core::default::Default::default(),
            priority: core::default::Default::default(),
            uri_search: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZoneLockdown_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneLockdown_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZoneLockdown_FilterRef {
        DataZoneLockdown_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZoneLockdown_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe timestamp of when the rule was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA string to search for in the description of existing rules."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description_search` after provisioning.\nA string to search for in the description of existing rules."]
    pub fn description_search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description_search", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nA single IP address to search for in existing rules."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_range_search` after provisioning.\nA single IP address range to search for in existing rules."]
    pub fn ip_range_search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_range_search", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_search` after provisioning.\nA single IP address to search for in existing rules."]
    pub fn ip_search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_search", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the rule was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nThe priority of the rule to control the processing order. A lower number indicates higher priority. If not provided, any rules with a configured priority will be processed before rules without a priority."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri_search` after provisioning.\nA single URI to search for in the list of URLs of existing rules."]
    pub fn uri_search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri_search", self.base))
    }
}
