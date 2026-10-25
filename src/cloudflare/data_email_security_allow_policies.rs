#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataEmailSecurityAllowPoliciesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_acceptable_sender: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_exempt_recipient: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_trusted_sender: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify_sender: std::option::Option<terrars::PrimField<bool>>,
}

struct DataEmailSecurityAllowPolicies_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmailSecurityAllowPoliciesData_>,
}

#[derive(Clone)]
pub struct DataEmailSecurityAllowPolicies(std::rc::Rc<DataEmailSecurityAllowPolicies_>);

impl DataEmailSecurityAllowPolicies {
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

    #[doc = "Set the field `direction`.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_acceptable_sender`.\nFilter to show only policies where messages from the sender are exempted from Spam, Spoof, and Bulk dispositions (not Malicious or Suspicious)."]
    pub fn set_is_acceptable_sender(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_acceptable_sender = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_exempt_recipient`.\nFilter to show only policies where messages to the recipient bypass all detections."]
    pub fn set_is_exempt_recipient(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_exempt_recipient = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_trusted_sender`.\nFilter to show only policies where messages from the sender bypass all detections and link following."]
    pub fn set_is_trusted_sender(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_trusted_sender = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\nField to sort by.\nAvailable values: \"pattern\", \"created_at\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pattern`.\nFilter by exact pattern value."]
    pub fn set_pattern(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pattern = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pattern_type`.\nFilter by pattern type.\nAvailable values: \"EMAIL\", \"DOMAIN\", \"IP\", \"UNKNOWN\"."]
    pub fn set_pattern_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pattern_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nSearch term for filtering records. Behavior may change."]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `verify_sender`.\nFilter to show only policies that enforce DMARC, SPF, or DKIM authentication."]
    pub fn set_verify_sender(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().verify_sender = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_acceptable_sender` after provisioning.\nFilter to show only policies where messages from the sender are exempted from Spam, Spoof, and Bulk dispositions (not Malicious or Suspicious)."]
    pub fn is_acceptable_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_acceptable_sender", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_exempt_recipient` after provisioning.\nFilter to show only policies where messages to the recipient bypass all detections."]
    pub fn is_exempt_recipient(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_exempt_recipient", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_trusted_sender` after provisioning.\nFilter to show only policies where messages from the sender bypass all detections and link following."]
    pub fn is_trusted_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_trusted_sender", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to sort by.\nAvailable values: \"pattern\", \"created_at\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\nFilter by exact pattern value."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pattern_type` after provisioning.\nFilter by pattern type.\nAvailable values: \"EMAIL\", \"DOMAIN\", \"IP\", \"UNKNOWN\"."]
    pub fn pattern_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataEmailSecurityAllowPolicies_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch term for filtering records. Behavior may change."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `verify_sender` after provisioning.\nFilter to show only policies that enforce DMARC, SPF, or DKIM authentication."]
    pub fn verify_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_sender", self.extract_ref()))
    }
}

impl terrars::Referable for DataEmailSecurityAllowPolicies {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmailSecurityAllowPolicies { }

impl terrars::ToListMappable for DataEmailSecurityAllowPolicies {
    type O = terrars::ListRef<DataEmailSecurityAllowPoliciesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmailSecurityAllowPolicies_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_email_security_allow_policies".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmailSecurityAllowPolicies {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataEmailSecurityAllowPolicies {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmailSecurityAllowPolicies {
        let out = DataEmailSecurityAllowPolicies(std::rc::Rc::new(DataEmailSecurityAllowPolicies_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEmailSecurityAllowPoliciesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                direction: core::default::Default::default(),
                is_acceptable_sender: core::default::Default::default(),
                is_exempt_recipient: core::default::Default::default(),
                is_trusted_sender: core::default::Default::default(),
                max_items: core::default::Default::default(),
                order: core::default::Default::default(),
                pattern: core::default::Default::default(),
                pattern_type: core::default::Default::default(),
                search: core::default::Default::default(),
                verify_sender: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmailSecurityAllowPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityAllowPoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmailSecurityAllowPoliciesRef {
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
        "Get a reference to the value of field `direction` after provisioning.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_acceptable_sender` after provisioning.\nFilter to show only policies where messages from the sender are exempted from Spam, Spoof, and Bulk dispositions (not Malicious or Suspicious)."]
    pub fn is_acceptable_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_acceptable_sender", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_exempt_recipient` after provisioning.\nFilter to show only policies where messages to the recipient bypass all detections."]
    pub fn is_exempt_recipient(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_exempt_recipient", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_trusted_sender` after provisioning.\nFilter to show only policies where messages from the sender bypass all detections and link following."]
    pub fn is_trusted_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_trusted_sender", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to sort by.\nAvailable values: \"pattern\", \"created_at\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\nFilter by exact pattern value."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pattern_type` after provisioning.\nFilter by pattern type.\nAvailable values: \"EMAIL\", \"DOMAIN\", \"IP\", \"UNKNOWN\"."]
    pub fn pattern_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataEmailSecurityAllowPolicies_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch term for filtering records. Behavior may change."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `verify_sender` after provisioning.\nFilter to show only policies that enforce DMARC, SPF, or DKIM authentication."]
    pub fn verify_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_sender", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmailSecurityAllowPolicies_Result {}

impl DataEmailSecurityAllowPolicies_Result { }

impl terrars::ToListMappable for DataEmailSecurityAllowPolicies_Result {
    type O = terrars::BlockAssignable<DataEmailSecurityAllowPolicies_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailSecurityAllowPolicies_Result {}

impl BuildDataEmailSecurityAllowPolicies_Result {
    pub fn build(self) -> DataEmailSecurityAllowPolicies_Result {
        DataEmailSecurityAllowPolicies_Result {}
    }
}

#[allow(dead_code)]
pub struct DataEmailSecurityAllowPolicies_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityAllowPolicies_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailSecurityAllowPolicies_ResultRef {
        DataEmailSecurityAllowPolicies_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailSecurityAllowPolicies_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comments` after provisioning.\n"]
    pub fn comments(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comments", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAllow policy identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_acceptable_sender` after provisioning.\nExempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply."]
    pub fn is_acceptable_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_acceptable_sender", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_exempt_recipient` after provisioning.\nBypasses all detections for messages to this recipient."]
    pub fn is_exempt_recipient(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_exempt_recipient", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_recipient` after provisioning.\nDeprecated as of July 1, 2025. Use `is_exempt_recipient` instead. End of life: July 1, 2026."]
    pub fn is_recipient(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_recipient", self.base))
    }

    #[doc = "Get a reference to the value of field `is_regex` after provisioning.\n"]
    pub fn is_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_sender` after provisioning.\nDeprecated as of July 1, 2025. Use `is_trusted_sender` instead. End of life: July 1, 2026."]
    pub fn is_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_sender", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_spoof` after provisioning.\nDeprecated as of July 1, 2025. Use `is_acceptable_sender` instead. End of life: July 1, 2026."]
    pub fn is_spoof(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_spoof", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_trusted_sender` after provisioning.\nBypasses all detections and link following for messages from this sender."]
    pub fn is_trusted_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_trusted_sender", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nThe pattern value to match. The format depends on `pattern_type`: a valid email address for EMAIL (e.g. `user@example.com`), a valid domain name for DOMAIN (e.g. `example.com`), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. `1.2.3.4`, `1.2.3.0/24`, `2606:4700:4700::1111`, or `2606:4700:4700::/48`); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pattern_type` after provisioning.\nType of pattern matching.\n- EMAIL: matches a full email address (e.g. `user@example.com`)\n- DOMAIN: matches a domain name (e.g. `example.com`)\n- IP: matches a plain IPv4 or IPv6 address (e.g. `1.2.3.4` or `2606:4700:4700::1111`) or CIDR block (e.g. `1.2.3.0/24` or `2606:4700:4700::/48`). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.\n- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.\nAvailable values: \"EMAIL\", \"DOMAIN\", \"IP\", \"UNKNOWN\"."]
    pub fn pattern_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `verify_sender` after provisioning.\nEnforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication."]
    pub fn verify_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_sender", self.base))
    }
}
