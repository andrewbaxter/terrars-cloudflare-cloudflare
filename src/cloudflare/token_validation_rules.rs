#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TokenValidationRulesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    action: terrars::PrimField<std::string::String>,
    description: terrars::PrimField<std::string::String>,
    enabled: terrars::PrimField<bool>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    position: std::option::Option<TokenValidationRules_Position>,
    selector: TokenValidationRules_Selector,
    title: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct TokenValidationRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TokenValidationRulesData_>,
}

#[derive(Clone)]
pub struct TokenValidationRules(std::rc::Rc<TokenValidationRules_>);

impl TokenValidationRules {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `position`.\nUpdate rule order among zone rules."]
    pub fn set_position(self, v: impl std::convert::Into<TokenValidationRules_Position>) -> Self {
        self.0.data.borrow_mut().position = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to take on requests that match operations included in `selector` and fail `expression`.\nAvailable values: \"log\", \"block\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description that gives more details than `title`."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nToggle rule on or off."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nRule expression. Requests that fail to match this expression will be subject to `action`.\n\nFor details on expressions, see the [Cloudflare Docs](https://developers.cloudflare.com/api-shield/security/jwt-validation/)."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\nUpdate rule order among zone rules."]
    pub fn position(&self) -> TokenValidationRules_PositionRef {
        TokenValidationRules_PositionRef::new(self.shared().clone(), format!("{}.position", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `selector` after provisioning.\nSelect operations covered by this rule.\n\nFor details on selectors, see the [Cloudflare Docs](https://developers.cloudflare.com/api-shield/security/jwt-validation/)."]
    pub fn selector(&self) -> TokenValidationRules_SelectorRef {
        TokenValidationRules_SelectorRef::new(self.shared().clone(), format!("{}.selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\nA human-readable name for the rule."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for TokenValidationRules {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TokenValidationRules { }

impl terrars::ToListMappable for TokenValidationRules {
    type O = terrars::ListRef<TokenValidationRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TokenValidationRules_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_token_validation_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTokenValidationRules {
    pub tf_id: std::string::String,
    #[doc =
        "Action to take on requests that match operations included in `selector` and fail `expression`.\nAvailable values: \"log\", \"block\"."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = "A human-readable description that gives more details than `title`."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "Toggle rule on or off."]
    pub enabled: terrars::PrimField<bool>,
    #[doc =
        "Rule expression. Requests that fail to match this expression will be subject to `action`.\n\nFor details on expressions, see the [Cloudflare Docs](https://developers.cloudflare.com/api-shield/security/jwt-validation/)."]
    pub expression: terrars::PrimField<std::string::String>,
    #[doc =
        "Select operations covered by this rule.\n\nFor details on selectors, see the [Cloudflare Docs](https://developers.cloudflare.com/api-shield/security/jwt-validation/)."]
    pub selector: TokenValidationRules_Selector,
    #[doc = "A human-readable name for the rule."]
    pub title: terrars::PrimField<std::string::String>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildTokenValidationRules {
    pub fn build(self, stack: &mut terrars::Stack) -> TokenValidationRules {
        let out = TokenValidationRules(std::rc::Rc::new(TokenValidationRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TokenValidationRulesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action: self.action,
                description: self.description,
                enabled: self.enabled,
                expression: self.expression,
                position: core::default::Default::default(),
                selector: self.selector,
                title: self.title,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TokenValidationRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TokenValidationRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TokenValidationRulesRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to take on requests that match operations included in `selector` and fail `expression`.\nAvailable values: \"log\", \"block\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description that gives more details than `title`."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nToggle rule on or off."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nRule expression. Requests that fail to match this expression will be subject to `action`.\n\nFor details on expressions, see the [Cloudflare Docs](https://developers.cloudflare.com/api-shield/security/jwt-validation/)."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\nUpdate rule order among zone rules."]
    pub fn position(&self) -> TokenValidationRules_PositionRef {
        TokenValidationRules_PositionRef::new(self.shared().clone(), format!("{}.position", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `selector` after provisioning.\nSelect operations covered by this rule.\n\nFor details on selectors, see the [Cloudflare Docs](https://developers.cloudflare.com/api-shield/security/jwt-validation/)."]
    pub fn selector(&self) -> TokenValidationRules_SelectorRef {
        TokenValidationRules_SelectorRef::new(self.shared().clone(), format!("{}.selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\nA human-readable name for the rule."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TokenValidationRules_Position {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index: std::option::Option<terrars::PrimField<f64>>,
}

impl TokenValidationRules_Position {
    #[doc = "Set the field `after`.\nMove rule to after rule with this ID."]
    pub fn set_after(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `before`.\nMove rule to before rule with this ID."]
    pub fn set_before(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index`.\nMove rule to this position"]
    pub fn set_index(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.index = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TokenValidationRules_Position {
    type O = terrars::BlockAssignable<TokenValidationRules_Position>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTokenValidationRules_Position {}

impl BuildTokenValidationRules_Position {
    pub fn build(self) -> TokenValidationRules_Position {
        TokenValidationRules_Position {
            after: core::default::Default::default(),
            before: core::default::Default::default(),
            index: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TokenValidationRules_PositionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TokenValidationRules_PositionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TokenValidationRules_PositionRef {
        TokenValidationRules_PositionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TokenValidationRules_PositionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `after` after provisioning.\nMove rule to after rule with this ID."]
    pub fn after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `before` after provisioning.\nMove rule to before rule with this ID."]
    pub fn before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.before", self.base))
    }

    #[doc = "Get a reference to the value of field `index` after provisioning.\nMove rule to this position"]
    pub fn index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index", self.base))
    }
}

#[derive(Serialize)]
pub struct TokenValidationRules_Selector_Exclude {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl TokenValidationRules_Selector_Exclude {
    #[doc = "Set the field `operation_ids`.\nExcluded operation IDs."]
    pub fn set_operation_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.operation_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TokenValidationRules_Selector_Exclude {
    type O = terrars::BlockAssignable<TokenValidationRules_Selector_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTokenValidationRules_Selector_Exclude {}

impl BuildTokenValidationRules_Selector_Exclude {
    pub fn build(self) -> TokenValidationRules_Selector_Exclude {
        TokenValidationRules_Selector_Exclude { operation_ids: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct TokenValidationRules_Selector_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TokenValidationRules_Selector_ExcludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TokenValidationRules_Selector_ExcludeRef {
        TokenValidationRules_Selector_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TokenValidationRules_Selector_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operation_ids` after provisioning.\nExcluded operation IDs."]
    pub fn operation_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct TokenValidationRules_Selector_Include {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl TokenValidationRules_Selector_Include {
    #[doc = "Set the field `host`.\nIncluded hostnames."]
    pub fn set_host(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TokenValidationRules_Selector_Include {
    type O = terrars::BlockAssignable<TokenValidationRules_Selector_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTokenValidationRules_Selector_Include {}

impl BuildTokenValidationRules_Selector_Include {
    pub fn build(self) -> TokenValidationRules_Selector_Include {
        TokenValidationRules_Selector_Include { host: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct TokenValidationRules_Selector_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TokenValidationRules_Selector_IncludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TokenValidationRules_Selector_IncludeRef {
        TokenValidationRules_Selector_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TokenValidationRules_Selector_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nIncluded hostnames."]
    pub fn host(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }
}

#[derive(Serialize)]
pub struct TokenValidationRules_Selector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<std::vec::Vec<TokenValidationRules_Selector_Exclude>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<std::vec::Vec<TokenValidationRules_Selector_Include>>,
}

impl TokenValidationRules_Selector {
    #[doc = "Set the field `exclude`.\nIgnore operations that were otherwise included by `include`."]
    pub fn set_exclude(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<TokenValidationRules_Selector_Exclude>>,
    ) -> Self {
        self.exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\nSelect all matching operations."]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<TokenValidationRules_Selector_Include>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TokenValidationRules_Selector {
    type O = terrars::BlockAssignable<TokenValidationRules_Selector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTokenValidationRules_Selector {}

impl BuildTokenValidationRules_Selector {
    pub fn build(self) -> TokenValidationRules_Selector {
        TokenValidationRules_Selector {
            exclude: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TokenValidationRules_SelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TokenValidationRules_SelectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TokenValidationRules_SelectorRef {
        TokenValidationRules_SelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TokenValidationRules_SelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nIgnore operations that were otherwise included by `include`."]
    pub fn exclude(&self) -> terrars::ListRef<TokenValidationRules_Selector_ExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\nSelect all matching operations."]
    pub fn include(&self) -> terrars::ListRef<TokenValidationRules_Selector_IncludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}
