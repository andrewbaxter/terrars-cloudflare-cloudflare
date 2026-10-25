#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct EmailSecurityAllowPolicyData_ {
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
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comments: std::option::Option<terrars::PrimField<std::string::String>>,
    is_acceptable_sender: terrars::PrimField<bool>,
    is_exempt_recipient: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_recipient: std::option::Option<terrars::PrimField<bool>>,
    is_regex: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_sender: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_spoof: std::option::Option<terrars::PrimField<bool>>,
    is_trusted_sender: terrars::PrimField<bool>,
    pattern: terrars::PrimField<std::string::String>,
    pattern_type: terrars::PrimField<std::string::String>,
    verify_sender: terrars::PrimField<bool>,
}

struct EmailSecurityAllowPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EmailSecurityAllowPolicyData_>,
}

#[derive(Clone)]
pub struct EmailSecurityAllowPolicy(std::rc::Rc<EmailSecurityAllowPolicy_>);

impl EmailSecurityAllowPolicy {
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

    #[doc = "Set the field `comments`.\n"]
    pub fn set_comments(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comments = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_recipient`.\nDeprecated as of July 1, 2025. Use `is_exempt_recipient` instead. End of life: July 1, 2026."]
    pub fn set_is_recipient(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_recipient = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_sender`.\nDeprecated as of July 1, 2025. Use `is_trusted_sender` instead. End of life: July 1, 2026."]
    pub fn set_is_sender(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_sender = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_spoof`.\nDeprecated as of July 1, 2025. Use `is_acceptable_sender` instead. End of life: July 1, 2026."]
    pub fn set_is_spoof(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_spoof = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comments` after provisioning.\n"]
    pub fn comments(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAllow policy identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_acceptable_sender` after provisioning.\nExempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply."]
    pub fn is_acceptable_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_acceptable_sender", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_exempt_recipient` after provisioning.\nBypasses all detections for messages to this recipient."]
    pub fn is_exempt_recipient(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_exempt_recipient", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_recipient` after provisioning.\nDeprecated as of July 1, 2025. Use `is_exempt_recipient` instead. End of life: July 1, 2026."]
    pub fn is_recipient(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_recipient", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_regex` after provisioning.\n"]
    pub fn is_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_regex", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_sender` after provisioning.\nDeprecated as of July 1, 2025. Use `is_trusted_sender` instead. End of life: July 1, 2026."]
    pub fn is_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_sender", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_spoof` after provisioning.\nDeprecated as of July 1, 2025. Use `is_acceptable_sender` instead. End of life: July 1, 2026."]
    pub fn is_spoof(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_spoof", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_trusted_sender` after provisioning.\nBypasses all detections and link following for messages from this sender."]
    pub fn is_trusted_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_trusted_sender", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nThe pattern value to match. The format depends on `pattern_type`: a valid email address for EMAIL (e.g. `user@example.com`), a valid domain name for DOMAIN (e.g. `example.com`), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. `1.2.3.4`, `1.2.3.0/24`, `2606:4700:4700::1111`, or `2606:4700:4700::/48`); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pattern_type` after provisioning.\nType of pattern matching.\n- EMAIL: matches a full email address (e.g. `user@example.com`)\n- DOMAIN: matches a domain name (e.g. `example.com`)\n- IP: matches a plain IPv4 or IPv6 address (e.g. `1.2.3.4` or `2606:4700:4700::1111`) or CIDR block (e.g. `1.2.3.0/24` or `2606:4700:4700::/48`). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.\n- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.\nAvailable values: \"EMAIL\", \"DOMAIN\", \"IP\", \"UNKNOWN\"."]
    pub fn pattern_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `verify_sender` after provisioning.\nEnforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication."]
    pub fn verify_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_sender", self.extract_ref()))
    }
}

impl terrars::Referable for EmailSecurityAllowPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EmailSecurityAllowPolicy { }

impl terrars::ToListMappable for EmailSecurityAllowPolicy {
    type O = terrars::ListRef<EmailSecurityAllowPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EmailSecurityAllowPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_email_security_allow_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmailSecurityAllowPolicy {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Exempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply."]
    pub is_acceptable_sender: terrars::PrimField<bool>,
    #[doc = "Bypasses all detections for messages to this recipient."]
    pub is_exempt_recipient: terrars::PrimField<bool>,
    #[doc = ""]
    pub is_regex: terrars::PrimField<bool>,
    #[doc = "Bypasses all detections and link following for messages from this sender."]
    pub is_trusted_sender: terrars::PrimField<bool>,
    #[doc =
        "The pattern value to match. The format depends on `pattern_type`: a valid email address for EMAIL (e.g. `user@example.com`), a valid domain name for DOMAIN (e.g. `example.com`), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. `1.2.3.4`, `1.2.3.0/24`, `2606:4700:4700::1111`, or `2606:4700:4700::/48`); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents."]
    pub pattern: terrars::PrimField<std::string::String>,
    #[doc =
        "Type of pattern matching.\n- EMAIL: matches a full email address (e.g. `user@example.com`)\n- DOMAIN: matches a domain name (e.g. `example.com`)\n- IP: matches a plain IPv4 or IPv6 address (e.g. `1.2.3.4` or `2606:4700:4700::1111`) or CIDR block (e.g. `1.2.3.0/24` or `2606:4700:4700::/48`). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.\n- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.\nAvailable values: \"EMAIL\", \"DOMAIN\", \"IP\", \"UNKNOWN\"."]
    pub pattern_type: terrars::PrimField<std::string::String>,
    #[doc =
        "Enforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication."]
    pub verify_sender: terrars::PrimField<bool>,
}

impl BuildEmailSecurityAllowPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> EmailSecurityAllowPolicy {
        let out = EmailSecurityAllowPolicy(std::rc::Rc::new(EmailSecurityAllowPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EmailSecurityAllowPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                comments: core::default::Default::default(),
                is_acceptable_sender: self.is_acceptable_sender,
                is_exempt_recipient: self.is_exempt_recipient,
                is_recipient: core::default::Default::default(),
                is_regex: self.is_regex,
                is_sender: core::default::Default::default(),
                is_spoof: core::default::Default::default(),
                is_trusted_sender: self.is_trusted_sender,
                pattern: self.pattern,
                pattern_type: self.pattern_type,
                verify_sender: self.verify_sender,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmailSecurityAllowPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmailSecurityAllowPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmailSecurityAllowPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comments` after provisioning.\n"]
    pub fn comments(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAllow policy identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_acceptable_sender` after provisioning.\nExempts messages from this sender from Spam, Spoof and Bulk dispositions only; Malicious and Suspicious dispositions still apply."]
    pub fn is_acceptable_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_acceptable_sender", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_exempt_recipient` after provisioning.\nBypasses all detections for messages to this recipient."]
    pub fn is_exempt_recipient(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_exempt_recipient", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_recipient` after provisioning.\nDeprecated as of July 1, 2025. Use `is_exempt_recipient` instead. End of life: July 1, 2026."]
    pub fn is_recipient(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_recipient", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_regex` after provisioning.\n"]
    pub fn is_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_regex", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_sender` after provisioning.\nDeprecated as of July 1, 2025. Use `is_trusted_sender` instead. End of life: July 1, 2026."]
    pub fn is_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_sender", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_spoof` after provisioning.\nDeprecated as of July 1, 2025. Use `is_acceptable_sender` instead. End of life: July 1, 2026."]
    pub fn is_spoof(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_spoof", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_trusted_sender` after provisioning.\nBypasses all detections and link following for messages from this sender."]
    pub fn is_trusted_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_trusted_sender", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nThe pattern value to match. The format depends on `pattern_type`: a valid email address for EMAIL (e.g. `user@example.com`), a valid domain name for DOMAIN (e.g. `example.com`), or a plain IPv4 or IPv6 address or CIDR block for IP (e.g. `1.2.3.4`, `1.2.3.0/24`, `2606:4700:4700::1111`, or `2606:4700:4700::/48`); the API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pattern_type` after provisioning.\nType of pattern matching.\n- EMAIL: matches a full email address (e.g. `user@example.com`)\n- DOMAIN: matches a domain name (e.g. `example.com`)\n- IP: matches a plain IPv4 or IPv6 address (e.g. `1.2.3.4` or `2606:4700:4700::1111`) or CIDR block (e.g. `1.2.3.0/24` or `2606:4700:4700::/48`). The API rejects private or unique-local, loopback, link-local, unspecified, and IPv4 broadcast addresses, including their IPv4-mapped IPv6 equivalents.\n- UNKNOWN: deprecated; you cannot use this when creating or updating policies, but it may appear on existing entries.\nAvailable values: \"EMAIL\", \"DOMAIN\", \"IP\", \"UNKNOWN\"."]
    pub fn pattern_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `verify_sender` after provisioning.\nEnforce DMARC, SPF or DKIM authentication. When on, Email Security only honors policies that pass authentication."]
    pub fn verify_sender(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_sender", self.extract_ref()))
    }
}
