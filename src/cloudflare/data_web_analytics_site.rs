#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataWebAnalyticsSiteData_ {
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
    filter: std::option::Option<DataWebAnalyticsSite_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    site_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataWebAnalyticsSite_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWebAnalyticsSiteData_>,
}

#[derive(Clone)]
pub struct DataWebAnalyticsSite(std::rc::Rc<DataWebAnalyticsSite_>);

impl DataWebAnalyticsSite {
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

    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataWebAnalyticsSite_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `site_id`.\nIdentifier."]
    pub fn set_site_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().site_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_install` after provisioning.\nIf enabled, the JavaScript snippet is automatically injected for orange-clouded sites."]
    pub fn auto_install(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_install", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataWebAnalyticsSite_FilterRef {
        DataWebAnalyticsSite_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nA list of rules."]
    pub fn rules(&self) -> terrars::ListRef<DataWebAnalyticsSite_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ruleset` after provisioning.\n"]
    pub fn ruleset(&self) -> DataWebAnalyticsSite_RulesetRef {
        DataWebAnalyticsSite_RulesetRef::new(self.shared().clone(), format!("{}.ruleset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier."]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_tag` after provisioning.\nThe Web Analytics site identifier."]
    pub fn site_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_token` after provisioning.\nThe Web Analytics site token."]
    pub fn site_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snippet` after provisioning.\nEncoded JavaScript snippet."]
    pub fn snippet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snippet", self.extract_ref()))
    }
}

impl terrars::Referable for DataWebAnalyticsSite {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWebAnalyticsSite { }

impl terrars::ToListMappable for DataWebAnalyticsSite {
    type O = terrars::ListRef<DataWebAnalyticsSiteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWebAnalyticsSite_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_web_analytics_site".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWebAnalyticsSite {
    pub tf_id: std::string::String,
}

impl BuildDataWebAnalyticsSite {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWebAnalyticsSite {
        let out = DataWebAnalyticsSite(std::rc::Rc::new(DataWebAnalyticsSite_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWebAnalyticsSiteData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                site_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWebAnalyticsSiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWebAnalyticsSiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWebAnalyticsSiteRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_install` after provisioning.\nIf enabled, the JavaScript snippet is automatically injected for orange-clouded sites."]
    pub fn auto_install(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_install", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataWebAnalyticsSite_FilterRef {
        DataWebAnalyticsSite_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nA list of rules."]
    pub fn rules(&self) -> terrars::ListRef<DataWebAnalyticsSite_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ruleset` after provisioning.\n"]
    pub fn ruleset(&self) -> DataWebAnalyticsSite_RulesetRef {
        DataWebAnalyticsSite_RulesetRef::new(self.shared().clone(), format!("{}.ruleset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier."]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_tag` after provisioning.\nThe Web Analytics site identifier."]
    pub fn site_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_token` after provisioning.\nThe Web Analytics site token."]
    pub fn site_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snippet` after provisioning.\nEncoded JavaScript snippet."]
    pub fn snippet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snippet", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWebAnalyticsSite_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWebAnalyticsSite_Filter {
    #[doc =
        "Set the field `order_by`.\nThe property used to sort the list of results.\nAvailable values: \"host\", \"created\"."]
    pub fn set_order_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order_by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWebAnalyticsSite_Filter {
    type O = terrars::BlockAssignable<DataWebAnalyticsSite_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWebAnalyticsSite_Filter {}

impl BuildDataWebAnalyticsSite_Filter {
    pub fn build(self) -> DataWebAnalyticsSite_Filter {
        DataWebAnalyticsSite_Filter { order_by: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataWebAnalyticsSite_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWebAnalyticsSite_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWebAnalyticsSite_FilterRef {
        DataWebAnalyticsSite_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWebAnalyticsSite_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nThe property used to sort the list of results.\nAvailable values: \"host\", \"created\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWebAnalyticsSite_Rules {}

impl DataWebAnalyticsSite_Rules { }

impl terrars::ToListMappable for DataWebAnalyticsSite_Rules {
    type O = terrars::BlockAssignable<DataWebAnalyticsSite_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWebAnalyticsSite_Rules {}

impl BuildDataWebAnalyticsSite_Rules {
    pub fn build(self) -> DataWebAnalyticsSite_Rules {
        DataWebAnalyticsSite_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataWebAnalyticsSite_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWebAnalyticsSite_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWebAnalyticsSite_RulesRef {
        DataWebAnalyticsSite_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWebAnalyticsSite_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe hostname the rule will be applied to."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe Web Analytics rule identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inclusive` after provisioning.\nWhether the rule includes or excludes traffic from being measured."]
    pub fn inclusive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `is_paused` after provisioning.\nWhether the rule is paused or not."]
    pub fn is_paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_paused", self.base))
    }

    #[doc = "Get a reference to the value of field `paths` after provisioning.\nThe paths the rule will be applied to."]
    pub fn paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWebAnalyticsSite_Ruleset {}

impl DataWebAnalyticsSite_Ruleset { }

impl terrars::ToListMappable for DataWebAnalyticsSite_Ruleset {
    type O = terrars::BlockAssignable<DataWebAnalyticsSite_Ruleset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWebAnalyticsSite_Ruleset {}

impl BuildDataWebAnalyticsSite_Ruleset {
    pub fn build(self) -> DataWebAnalyticsSite_Ruleset {
        DataWebAnalyticsSite_Ruleset {}
    }
}

#[allow(dead_code)]
pub struct DataWebAnalyticsSite_RulesetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWebAnalyticsSite_RulesetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWebAnalyticsSite_RulesetRef {
        DataWebAnalyticsSite_RulesetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWebAnalyticsSite_RulesetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the ruleset is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe Web Analytics ruleset identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_name` after provisioning.\n"]
    pub fn zone_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_name", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_tag` after provisioning.\nThe zone identifier."]
    pub fn zone_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_tag", self.base))
    }
}
