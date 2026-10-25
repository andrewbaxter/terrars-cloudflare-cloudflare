#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustAccessPolicyData_ {
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
    approval_groups: std::option::Option<std::vec::Vec<ZeroTrustAccessPolicy_ApprovalGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approval_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_rules: std::option::Option<ZeroTrustAccessPolicy_ConnectionRules>,
    decision: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<std::vec::Vec<ZeroTrustAccessPolicy_Exclude>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<std::vec::Vec<ZeroTrustAccessPolicy_Include>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    isolation_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mfa_config: std::option::Option<ZeroTrustAccessPolicy_MfaConfig>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    purpose_justification_prompt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    purpose_justification_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require: std::option::Option<std::vec::Vec<ZeroTrustAccessPolicy_Require>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ZeroTrustAccessPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustAccessPolicyData_>,
}

#[derive(Clone)]
pub struct ZeroTrustAccessPolicy(std::rc::Rc<ZeroTrustAccessPolicy_>);

impl ZeroTrustAccessPolicy {
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

    #[doc = "Set the field `approval_groups`.\nAdministrators who can approve a temporary authentication request."]
    pub fn set_approval_groups(
        self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessPolicy_ApprovalGroups>>,
    ) -> Self {
        self.0.data.borrow_mut().approval_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `approval_required`.\nRequires the user to request access from an administrator at the start of each session."]
    pub fn set_approval_required(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().approval_required = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection_rules`.\nThe rules that define how users may connect to targets secured by your application."]
    pub fn set_connection_rules(self, v: impl std::convert::Into<ZeroTrustAccessPolicy_ConnectionRules>) -> Self {
        self.0.data.borrow_mut().connection_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclude`.\nRules evaluated with a NOT logical operator. To match the policy, a user cannot meet any of the Exclude rules."]
    pub fn set_exclude(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessPolicy_Exclude>>) -> Self {
        self.0.data.borrow_mut().exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include`.\nRules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub fn set_include(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessPolicy_Include>>) -> Self {
        self.0.data.borrow_mut().include = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `isolation_required`.\nRequire this application to be served in an isolated browser for users matching this policy. 'Client Web Isolation' must be on for the account in order to use this feature."]
    pub fn set_isolation_required(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().isolation_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mfa_config`.\nConfigures multi-factor authentication (MFA) settings."]
    pub fn set_mfa_config(self, v: impl std::convert::Into<ZeroTrustAccessPolicy_MfaConfig>) -> Self {
        self.0.data.borrow_mut().mfa_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `purpose_justification_prompt`.\nA custom message that will appear on the purpose justification screen."]
    pub fn set_purpose_justification_prompt(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().purpose_justification_prompt = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `purpose_justification_required`.\nRequire users to enter a justification when they log in to the application."]
    pub fn set_purpose_justification_required(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().purpose_justification_required = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require`.\nRules evaluated with an AND logical operator. To match the policy, a user must meet all of the Require rules."]
    pub fn set_require(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessPolicy_Require>>) -> Self {
        self.0.data.borrow_mut().require = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_duration`.\nThe amount of time that tokens issued for the application will be valid. Must be in the format `300ms` or `2h45m`. Valid time units are: ns, us (or µs), ms, s, m, h."]
    pub fn set_session_duration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().session_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_count` after provisioning.\nNumber of access applications currently using this policy."]
    pub fn app_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approval_groups` after provisioning.\nAdministrators who can approve a temporary authentication request."]
    pub fn approval_groups(&self) -> terrars::SetRef<ZeroTrustAccessPolicy_ApprovalGroupsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approval_groups", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approval_required` after provisioning.\nRequires the user to request access from an administrator at the start of each session."]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_rules` after provisioning.\nThe rules that define how users may connect to targets secured by your application."]
    pub fn connection_rules(&self) -> ZeroTrustAccessPolicy_ConnectionRulesRef {
        ZeroTrustAccessPolicy_ConnectionRulesRef::new(
            self.shared().clone(),
            format!("{}.connection_rules", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `decision` after provisioning.\nThe action Access will take if a user matches this policy. Infrastructure application policies can only use the Allow action.\nAvailable values: \"allow\", \"deny\", \"non_identity\", \"bypass\"."]
    pub fn decision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decision", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nRules evaluated with a NOT logical operator. To match the policy, a user cannot meet any of the Exclude rules."]
    pub fn exclude(&self) -> terrars::SetRef<ZeroTrustAccessPolicy_ExcludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe UUID of the policy"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nRules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub fn include(&self) -> terrars::SetRef<ZeroTrustAccessPolicy_IncludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `isolation_required` after provisioning.\nRequire this application to be served in an isolated browser for users matching this policy. 'Client Web Isolation' must be on for the account in order to use this feature."]
    pub fn isolation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolation_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings."]
    pub fn mfa_config(&self) -> ZeroTrustAccessPolicy_MfaConfigRef {
        ZeroTrustAccessPolicy_MfaConfigRef::new(self.shared().clone(), format!("{}.mfa_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Access policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_prompt` after provisioning.\nA custom message that will appear on the purpose justification screen."]
    pub fn purpose_justification_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_prompt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_required` after provisioning.\nRequire users to enter a justification when they log in to the application."]
    pub fn purpose_justification_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.purpose_justification_required", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `require` after provisioning.\nRules evaluated with an AND logical operator. To match the policy, a user must meet all of the Require rules."]
    pub fn require(&self) -> terrars::SetRef<ZeroTrustAccessPolicy_RequireRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.require", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reusable` after provisioning.\n"]
    pub fn reusable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reusable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nThe amount of time that tokens issued for the application will be valid. Must be in the format `300ms` or `2h45m`. Valid time units are: ns, us (or µs), ms, s, m, h."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustAccessPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustAccessPolicy { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy {
    type O = terrars::ListRef<ZeroTrustAccessPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustAccessPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustAccessPolicy {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The action Access will take if a user matches this policy. Infrastructure application policies can only use the Allow action.\nAvailable values: \"allow\", \"deny\", \"non_identity\", \"bypass\"."]
    pub decision: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Access policy."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustAccessPolicy {
        let out = ZeroTrustAccessPolicy(std::rc::Rc::new(ZeroTrustAccessPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustAccessPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                approval_groups: core::default::Default::default(),
                approval_required: core::default::Default::default(),
                connection_rules: core::default::Default::default(),
                decision: self.decision,
                exclude: core::default::Default::default(),
                include: core::default::Default::default(),
                isolation_required: core::default::Default::default(),
                mfa_config: core::default::Default::default(),
                name: self.name,
                purpose_justification_prompt: core::default::Default::default(),
                purpose_justification_required: core::default::Default::default(),
                require: core::default::Default::default(),
                session_duration: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustAccessPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustAccessPolicyRef {
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

    #[doc =
        "Get a reference to the value of field `app_count` after provisioning.\nNumber of access applications currently using this policy."]
    pub fn app_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approval_groups` after provisioning.\nAdministrators who can approve a temporary authentication request."]
    pub fn approval_groups(&self) -> terrars::SetRef<ZeroTrustAccessPolicy_ApprovalGroupsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approval_groups", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approval_required` after provisioning.\nRequires the user to request access from an administrator at the start of each session."]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_rules` after provisioning.\nThe rules that define how users may connect to targets secured by your application."]
    pub fn connection_rules(&self) -> ZeroTrustAccessPolicy_ConnectionRulesRef {
        ZeroTrustAccessPolicy_ConnectionRulesRef::new(
            self.shared().clone(),
            format!("{}.connection_rules", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `decision` after provisioning.\nThe action Access will take if a user matches this policy. Infrastructure application policies can only use the Allow action.\nAvailable values: \"allow\", \"deny\", \"non_identity\", \"bypass\"."]
    pub fn decision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decision", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nRules evaluated with a NOT logical operator. To match the policy, a user cannot meet any of the Exclude rules."]
    pub fn exclude(&self) -> terrars::SetRef<ZeroTrustAccessPolicy_ExcludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe UUID of the policy"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nRules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub fn include(&self) -> terrars::SetRef<ZeroTrustAccessPolicy_IncludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `isolation_required` after provisioning.\nRequire this application to be served in an isolated browser for users matching this policy. 'Client Web Isolation' must be on for the account in order to use this feature."]
    pub fn isolation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolation_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings."]
    pub fn mfa_config(&self) -> ZeroTrustAccessPolicy_MfaConfigRef {
        ZeroTrustAccessPolicy_MfaConfigRef::new(self.shared().clone(), format!("{}.mfa_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Access policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_prompt` after provisioning.\nA custom message that will appear on the purpose justification screen."]
    pub fn purpose_justification_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_prompt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_required` after provisioning.\nRequire users to enter a justification when they log in to the application."]
    pub fn purpose_justification_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.purpose_justification_required", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `require` after provisioning.\nRules evaluated with an AND logical operator. To match the policy, a user must meet all of the Require rules."]
    pub fn require(&self) -> terrars::SetRef<ZeroTrustAccessPolicy_RequireRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.require", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reusable` after provisioning.\n"]
    pub fn reusable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reusable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nThe amount of time that tokens issued for the application will be valid. Must be in the format `300ms` or `2h45m`. Valid time units are: ns, us (or µs), ms, s, m, h."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_ApprovalGroups {
    approvals_needed: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_list_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_ApprovalGroups {
    #[doc = "Set the field `email_addresses`.\nA list of emails that can approve the access request."]
    pub fn set_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_list_uuid`.\nThe UUID of an re-usable email list."]
    pub fn set_email_list_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_list_uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_ApprovalGroups {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_ApprovalGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_ApprovalGroups {
    #[doc = "The number of approvals needed to obtain access."]
    pub approvals_needed: terrars::PrimField<f64>,
}

impl BuildZeroTrustAccessPolicy_ApprovalGroups {
    pub fn build(self) -> ZeroTrustAccessPolicy_ApprovalGroups {
        ZeroTrustAccessPolicy_ApprovalGroups {
            approvals_needed: self.approvals_needed,
            email_addresses: core::default::Default::default(),
            email_list_uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_ApprovalGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_ApprovalGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_ApprovalGroupsRef {
        ZeroTrustAccessPolicy_ApprovalGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_ApprovalGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `approvals_needed` after provisioning.\nThe number of approvals needed to obtain access."]
    pub fn approvals_needed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approvals_needed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_addresses` after provisioning.\nA list of emails that can approve the access request."]
    pub fn email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_list_uuid` after provisioning.\nThe UUID of an re-usable email list."]
    pub fn email_list_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_list_uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_ConnectionRules_Rdp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_clipboard_local_to_remote_formats: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_clipboard_remote_to_local_formats: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ZeroTrustAccessPolicy_ConnectionRules_Rdp {
    #[doc =
        "Set the field `allowed_clipboard_local_to_remote_formats`.\nClipboard formats allowed when copying from local machine to remote RDP session."]
    pub fn set_allowed_clipboard_local_to_remote_formats(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_clipboard_local_to_remote_formats = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_clipboard_remote_to_local_formats`.\nClipboard formats allowed when copying from remote RDP session to local machine."]
    pub fn set_allowed_clipboard_remote_to_local_formats(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_clipboard_remote_to_local_formats = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_ConnectionRules_Rdp {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_ConnectionRules_Rdp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_ConnectionRules_Rdp {}

impl BuildZeroTrustAccessPolicy_ConnectionRules_Rdp {
    pub fn build(self) -> ZeroTrustAccessPolicy_ConnectionRules_Rdp {
        ZeroTrustAccessPolicy_ConnectionRules_Rdp {
            allowed_clipboard_local_to_remote_formats: core::default::Default::default(),
            allowed_clipboard_remote_to_local_formats: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_ConnectionRules_RdpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_ConnectionRules_RdpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_ConnectionRules_RdpRef {
        ZeroTrustAccessPolicy_ConnectionRules_RdpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_ConnectionRules_RdpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_clipboard_local_to_remote_formats` after provisioning.\nClipboard formats allowed when copying from local machine to remote RDP session."]
    pub fn allowed_clipboard_local_to_remote_formats(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_clipboard_local_to_remote_formats", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `allowed_clipboard_remote_to_local_formats` after provisioning.\nClipboard formats allowed when copying from remote RDP session to local machine."]
    pub fn allowed_clipboard_remote_to_local_formats(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_clipboard_remote_to_local_formats", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_ConnectionRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdp: std::option::Option<ZeroTrustAccessPolicy_ConnectionRules_Rdp>,
}

impl ZeroTrustAccessPolicy_ConnectionRules {
    #[doc = "Set the field `rdp`.\nThe RDP-specific rules that define clipboard behavior for RDP connections."]
    pub fn set_rdp(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_ConnectionRules_Rdp>) -> Self {
        self.rdp = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_ConnectionRules {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_ConnectionRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_ConnectionRules {}

impl BuildZeroTrustAccessPolicy_ConnectionRules {
    pub fn build(self) -> ZeroTrustAccessPolicy_ConnectionRules {
        ZeroTrustAccessPolicy_ConnectionRules { rdp: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_ConnectionRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_ConnectionRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_ConnectionRulesRef {
        ZeroTrustAccessPolicy_ConnectionRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_ConnectionRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rdp` after provisioning.\nThe RDP-specific rules that define clipboard behavior for RDP connections."]
    pub fn rdp(&self) -> ZeroTrustAccessPolicy_ConnectionRules_RdpRef {
        ZeroTrustAccessPolicy_ConnectionRules_RdpRef::new(self.shared().clone(), format!("{}.rdp", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {}

impl ZeroTrustAccessPolicy_Exclude_AnyValidServiceToken { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {}

impl BuildZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {
        ZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
        ZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_AuthContext {
    ac_id: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_AuthContext { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_AuthContext {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_AuthContext {
    #[doc = "The ACID of an Authentication context."]
    pub ac_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of an Authentication context."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Azure identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_AuthContext {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_AuthContext {
        ZeroTrustAccessPolicy_Exclude_AuthContext {
            ac_id: self.ac_id,
            id: self.id,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_AuthContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_AuthContextRef {
        ZeroTrustAccessPolicy_Exclude_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_AuthContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ac_id` after provisioning.\nThe ACID of an Authentication context."]
    pub fn ac_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ac_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Authentication context."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_AuthMethod {
    auth_method: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_AuthMethod { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_AuthMethod {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_AuthMethod {
    #[doc = "The type of authentication method https://datatracker.ietf.org/doc/html/rfc8176#section-2."]
    pub auth_method: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_AuthMethod {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_AuthMethod {
        ZeroTrustAccessPolicy_Exclude_AuthMethod { auth_method: self.auth_method }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_AuthMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_AuthMethodRef {
        ZeroTrustAccessPolicy_Exclude_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_AuthMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_method` after provisioning.\nThe type of authentication method https://datatracker.ietf.org/doc/html/rfc8176#section-2."]
    pub fn auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_AzureAd {
    id: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_AzureAd { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_AzureAd {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_AzureAd {
    #[doc = "The ID of an Azure group."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Azure identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_AzureAd {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_AzureAd {
        ZeroTrustAccessPolicy_Exclude_AzureAd {
            id: self.id,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_AzureAdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_AzureAdRef {
        ZeroTrustAccessPolicy_Exclude_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_AzureAdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Azure group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_Certificate {}

impl ZeroTrustAccessPolicy_Exclude_Certificate { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_Certificate {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_Certificate {}

impl BuildZeroTrustAccessPolicy_Exclude_Certificate {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_Certificate {
        ZeroTrustAccessPolicy_Exclude_Certificate {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_CertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_CertificateRef {
        ZeroTrustAccessPolicy_Exclude_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {
    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {}

impl BuildZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {
        ZeroTrustAccessPolicy_Exclude_CloudflareAccountMember { account_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
        ZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_CommonName {
    common_name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_CommonName { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_CommonName {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_CommonName {
    #[doc = "The common name to match."]
    pub common_name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_CommonName {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_CommonName {
        ZeroTrustAccessPolicy_Exclude_CommonName { common_name: self.common_name }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_CommonNameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_CommonNameRef {
        ZeroTrustAccessPolicy_Exclude_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_CommonNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\nThe common name to match."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_DevicePosture {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    integration_uid: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_DevicePosture {
    #[doc = "Set the field `account_id`.\nThe ID of the account that owns the device posture integration."]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_DevicePosture {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_DevicePosture {
    #[doc = "The ID of a device posture integration."]
    pub integration_uid: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_DevicePosture {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_DevicePosture {
        ZeroTrustAccessPolicy_Exclude_DevicePosture {
            account_id: core::default::Default::default(),
            integration_uid: self.integration_uid,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Exclude_DevicePostureRef {
        ZeroTrustAccessPolicy_Exclude_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_DevicePostureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe ID of the account that owns the device posture integration."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `integration_uid` after provisioning.\nThe ID of a device posture integration."]
    pub fn integration_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_Email {
    email: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_Email { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_Email {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_Email {
    #[doc = "The email of the user."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_Email {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_Email {
        ZeroTrustAccessPolicy_Exclude_Email { email: self.email }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_EmailRef {
        ZeroTrustAccessPolicy_Exclude_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_EmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe email of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_EmailDomain {
    domain: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_EmailDomain { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_EmailDomain {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_EmailDomain {
    #[doc = "The email domain to match."]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_EmailDomain {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_EmailDomain {
        ZeroTrustAccessPolicy_Exclude_EmailDomain { domain: self.domain }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_EmailDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_EmailDomainRef {
        ZeroTrustAccessPolicy_Exclude_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_EmailDomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe email domain to match."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_EmailList {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_EmailList { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_EmailList {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_EmailList {
    #[doc = "The ID of a previously created email list."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_EmailList {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_EmailList {
        ZeroTrustAccessPolicy_Exclude_EmailList { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_EmailListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_EmailListRef {
        ZeroTrustAccessPolicy_Exclude_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_EmailListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created email list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_Everyone {}

impl ZeroTrustAccessPolicy_Exclude_Everyone { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_Everyone {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_Everyone {}

impl BuildZeroTrustAccessPolicy_Exclude_Everyone {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_Everyone {
        ZeroTrustAccessPolicy_Exclude_Everyone {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_EveryoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_EveryoneRef {
        ZeroTrustAccessPolicy_Exclude_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_ExternalEvaluation {
    evaluate_url: terrars::PrimField<std::string::String>,
    keys_url: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_ExternalEvaluation { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_ExternalEvaluation {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_ExternalEvaluation {
    #[doc = "The API endpoint containing your business logic."]
    pub evaluate_url: terrars::PrimField<std::string::String>,
    #[doc = "The API endpoint containing the key that Access uses to verify that the response came from your API."]
    pub keys_url: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_ExternalEvaluation {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_ExternalEvaluation {
        ZeroTrustAccessPolicy_Exclude_ExternalEvaluation {
            evaluate_url: self.evaluate_url,
            keys_url: self.keys_url,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
        ZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `evaluate_url` after provisioning.\nThe API endpoint containing your business logic."]
    pub fn evaluate_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keys_url` after provisioning.\nThe API endpoint containing the key that Access uses to verify that the response came from your API."]
    pub fn keys_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_Geo {
    country_code: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_Geo { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_Geo {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_Geo {
    #[doc = "The country code that should be matched."]
    pub country_code: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_Geo {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_Geo {
        ZeroTrustAccessPolicy_Exclude_Geo { country_code: self.country_code }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_GeoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_GeoRef {
        ZeroTrustAccessPolicy_Exclude_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_GeoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `country_code` after provisioning.\nThe country code that should be matched."]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_GithubOrganization {
    identity_provider_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    team: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_Exclude_GithubOrganization {
    #[doc = "Set the field `team`.\nThe name of the team"]
    pub fn set_team(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.team = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_GithubOrganization {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_GithubOrganization {
    #[doc = "The ID of your Github identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the organization."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_GithubOrganization {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_GithubOrganization {
        ZeroTrustAccessPolicy_Exclude_GithubOrganization {
            identity_provider_id: self.identity_provider_id,
            name: self.name,
            team: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
        ZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Github identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `team` after provisioning.\nThe name of the team"]
    pub fn team(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_Group {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_Group { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_Group {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_Group {
    #[doc = "The ID of a previously created Access group."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_Group {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_Group {
        ZeroTrustAccessPolicy_Exclude_Group { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_GroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_GroupRef {
        ZeroTrustAccessPolicy_Exclude_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_GroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created Access group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_Gsuite {
    email: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_Gsuite { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_Gsuite {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_Gsuite {
    #[doc = "The email of the Google Workspace group."]
    pub email: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Google Workspace identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_Gsuite {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_Gsuite {
        ZeroTrustAccessPolicy_Exclude_Gsuite {
            email: self.email,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_GsuiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_GsuiteRef {
        ZeroTrustAccessPolicy_Exclude_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_GsuiteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nThe email of the Google Workspace group."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Google Workspace identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_Ip {
    ip: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_Ip { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_Ip {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_Ip {
    #[doc = "An IPv4 or IPv6 CIDR block."]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_Ip {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_Ip {
        ZeroTrustAccessPolicy_Exclude_Ip { ip: self.ip }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_IpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_IpRef {
        ZeroTrustAccessPolicy_Exclude_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_IpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 or IPv6 CIDR block."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_IpList {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_IpList { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_IpList {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_IpList {
    #[doc = "The ID of a previously created IP list."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_IpList {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_IpList {
        ZeroTrustAccessPolicy_Exclude_IpList { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_IpListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_IpListRef {
        ZeroTrustAccessPolicy_Exclude_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_IpListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created IP list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_LinkedAppToken {
    app_uid: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_LinkedAppToken { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_LinkedAppToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_LinkedAppToken {
    #[doc = "The ID of an Access OIDC SaaS application"]
    pub app_uid: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_LinkedAppToken {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_LinkedAppToken {
        ZeroTrustAccessPolicy_Exclude_LinkedAppToken { app_uid: self.app_uid }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
        ZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_uid` after provisioning.\nThe ID of an Access OIDC SaaS application"]
    pub fn app_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_LoginMethod {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_LoginMethod { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_LoginMethod {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_LoginMethod {
    #[doc = "The ID of an identity provider."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_LoginMethod {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_LoginMethod {
        ZeroTrustAccessPolicy_Exclude_LoginMethod { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_LoginMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_LoginMethodRef {
        ZeroTrustAccessPolicy_Exclude_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_LoginMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an identity provider."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_Oidc {
    claim_name: terrars::PrimField<std::string::String>,
    claim_value: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_Oidc { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_Oidc {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_Oidc {
    #[doc = "The name of the OIDC claim."]
    pub claim_name: terrars::PrimField<std::string::String>,
    #[doc = "The OIDC claim value to look for."]
    pub claim_value: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your OIDC identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_Oidc {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_Oidc {
        ZeroTrustAccessPolicy_Exclude_Oidc {
            claim_name: self.claim_name,
            claim_value: self.claim_value,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_OidcRef {
        ZeroTrustAccessPolicy_Exclude_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_OidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_name` after provisioning.\nThe name of the OIDC claim."]
    pub fn claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `claim_value` after provisioning.\nThe OIDC claim value to look for."]
    pub fn claim_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your OIDC identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_Okta {
    identity_provider_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_Okta { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_Okta {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_Okta {
    #[doc = "The ID of your Okta identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Okta group."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_Okta {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_Okta {
        ZeroTrustAccessPolicy_Exclude_Okta {
            identity_provider_id: self.identity_provider_id,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_OktaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_OktaRef {
        ZeroTrustAccessPolicy_Exclude_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_OktaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Okta identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Okta group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_Saml {
    attribute_name: terrars::PrimField<std::string::String>,
    attribute_value: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_Saml { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_Saml {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_Saml {
    #[doc = "The name of the SAML attribute."]
    pub attribute_name: terrars::PrimField<std::string::String>,
    #[doc = "The SAML attribute value to look for."]
    pub attribute_value: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your SAML identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_Saml {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_Saml {
        ZeroTrustAccessPolicy_Exclude_Saml {
            attribute_name: self.attribute_name,
            attribute_value: self.attribute_value,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_SamlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_SamlRef {
        ZeroTrustAccessPolicy_Exclude_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_SamlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_name` after provisioning.\nThe name of the SAML attribute."]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `attribute_value` after provisioning.\nThe SAML attribute value to look for."]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your SAML identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_ServiceToken {
    token_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Exclude_ServiceToken { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_ServiceToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_ServiceToken {
    #[doc = "The ID of a Service Token."]
    pub token_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Exclude_ServiceToken {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_ServiceToken {
        ZeroTrustAccessPolicy_Exclude_ServiceToken { token_id: self.token_id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
        ZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\nThe ID of a Service Token."]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude_UserRiskScore {
    user_risk_score: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_Exclude_UserRiskScore { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude_UserRiskScore {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude_UserRiskScore {
    #[doc = "A list of risk score levels to match. Values can be low, medium, high, or unscored."]
    pub user_risk_score: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildZeroTrustAccessPolicy_Exclude_UserRiskScore {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude_UserRiskScore {
        ZeroTrustAccessPolicy_Exclude_UserRiskScore { user_risk_score: self.user_risk_score }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
        ZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_risk_score` after provisioning.\nA list of risk score levels to match. Values can be low, medium, high, or unscored."]
    pub fn user_risk_score(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_risk_score", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Exclude {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_valid_service_token: std::option::Option<ZeroTrustAccessPolicy_Exclude_AnyValidServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_context: std::option::Option<ZeroTrustAccessPolicy_Exclude_AuthContext>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_method: std::option::Option<ZeroTrustAccessPolicy_Exclude_AuthMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_ad: std::option::Option<ZeroTrustAccessPolicy_Exclude_AzureAd>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<ZeroTrustAccessPolicy_Exclude_Certificate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_account_member: std::option::Option<ZeroTrustAccessPolicy_Exclude_CloudflareAccountMember>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<ZeroTrustAccessPolicy_Exclude_CommonName>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<ZeroTrustAccessPolicy_Exclude_DevicePosture>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<ZeroTrustAccessPolicy_Exclude_Email>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_domain: std::option::Option<ZeroTrustAccessPolicy_Exclude_EmailDomain>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_list: std::option::Option<ZeroTrustAccessPolicy_Exclude_EmailList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    everyone: std::option::Option<ZeroTrustAccessPolicy_Exclude_Everyone>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_evaluation: std::option::Option<ZeroTrustAccessPolicy_Exclude_ExternalEvaluation>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<ZeroTrustAccessPolicy_Exclude_Geo>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_organization: std::option::Option<ZeroTrustAccessPolicy_Exclude_GithubOrganization>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<ZeroTrustAccessPolicy_Exclude_Group>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gsuite: std::option::Option<ZeroTrustAccessPolicy_Exclude_Gsuite>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<ZeroTrustAccessPolicy_Exclude_Ip>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_list: std::option::Option<ZeroTrustAccessPolicy_Exclude_IpList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_app_token: std::option::Option<ZeroTrustAccessPolicy_Exclude_LinkedAppToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_method: std::option::Option<ZeroTrustAccessPolicy_Exclude_LoginMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<ZeroTrustAccessPolicy_Exclude_Oidc>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta: std::option::Option<ZeroTrustAccessPolicy_Exclude_Okta>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<ZeroTrustAccessPolicy_Exclude_Saml>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_token: std::option::Option<ZeroTrustAccessPolicy_Exclude_ServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_risk_score: std::option::Option<ZeroTrustAccessPolicy_Exclude_UserRiskScore>,
}

impl ZeroTrustAccessPolicy_Exclude {
    #[doc = "Set the field `any_valid_service_token`.\nAn empty object which matches on all service tokens."]
    pub fn set_any_valid_service_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_AnyValidServiceToken>,
    ) -> Self {
        self.any_valid_service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_context`.\n"]
    pub fn set_auth_context(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_AuthContext>) -> Self {
        self.auth_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_AuthMethod>) -> Self {
        self.auth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure_ad`.\n"]
    pub fn set_azure_ad(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_AzureAd>) -> Self {
        self.azure_ad = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_Certificate>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudflare_account_member`.\n"]
    pub fn set_cloudflare_account_member(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_CloudflareAccountMember>,
    ) -> Self {
        self.cloudflare_account_member = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_CommonName>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_DevicePosture>) -> Self {
        self.device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_Email>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_EmailDomain>) -> Self {
        self.email_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_list`.\n"]
    pub fn set_email_list(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_EmailList>) -> Self {
        self.email_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `everyone`.\nAn empty object which matches on all users."]
    pub fn set_everyone(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_Everyone>) -> Self {
        self.everyone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_ExternalEvaluation>,
    ) -> Self {
        self.external_evaluation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_Geo>) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github_organization`.\n"]
    pub fn set_github_organization(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_GithubOrganization>,
    ) -> Self {
        self.github_organization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_Group>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_Gsuite>) -> Self {
        self.gsuite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_Ip>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_IpList>) -> Self {
        self.ip_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_app_token`.\n"]
    pub fn set_linked_app_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_LinkedAppToken>,
    ) -> Self {
        self.linked_app_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_LoginMethod>) -> Self {
        self.login_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_Oidc>) -> Self {
        self.oidc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_Okta>) -> Self {
        self.okta = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_Saml>) -> Self {
        self.saml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_ServiceToken>) -> Self {
        self.service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_risk_score`.\n"]
    pub fn set_user_risk_score(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Exclude_UserRiskScore>,
    ) -> Self {
        self.user_risk_score = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Exclude {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Exclude {}

impl BuildZeroTrustAccessPolicy_Exclude {
    pub fn build(self) -> ZeroTrustAccessPolicy_Exclude {
        ZeroTrustAccessPolicy_Exclude {
            any_valid_service_token: core::default::Default::default(),
            auth_context: core::default::Default::default(),
            auth_method: core::default::Default::default(),
            azure_ad: core::default::Default::default(),
            certificate: core::default::Default::default(),
            cloudflare_account_member: core::default::Default::default(),
            common_name: core::default::Default::default(),
            device_posture: core::default::Default::default(),
            email: core::default::Default::default(),
            email_domain: core::default::Default::default(),
            email_list: core::default::Default::default(),
            everyone: core::default::Default::default(),
            external_evaluation: core::default::Default::default(),
            geo: core::default::Default::default(),
            github_organization: core::default::Default::default(),
            group: core::default::Default::default(),
            gsuite: core::default::Default::default(),
            ip: core::default::Default::default(),
            ip_list: core::default::Default::default(),
            linked_app_token: core::default::Default::default(),
            login_method: core::default::Default::default(),
            oidc: core::default::Default::default(),
            okta: core::default::Default::default(),
            saml: core::default::Default::default(),
            service_token: core::default::Default::default(),
            user_risk_score: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_ExcludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_ExcludeRef {
        ZeroTrustAccessPolicy_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> ZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
        ZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> ZeroTrustAccessPolicy_Exclude_AuthContextRef {
        ZeroTrustAccessPolicy_Exclude_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> ZeroTrustAccessPolicy_Exclude_AuthMethodRef {
        ZeroTrustAccessPolicy_Exclude_AuthMethodRef::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> ZeroTrustAccessPolicy_Exclude_AzureAdRef {
        ZeroTrustAccessPolicy_Exclude_AzureAdRef::new(self.shared().clone(), format!("{}.azure_ad", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> ZeroTrustAccessPolicy_Exclude_CertificateRef {
        ZeroTrustAccessPolicy_Exclude_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> ZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
        ZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> ZeroTrustAccessPolicy_Exclude_CommonNameRef {
        ZeroTrustAccessPolicy_Exclude_CommonNameRef::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ZeroTrustAccessPolicy_Exclude_DevicePostureRef {
        ZeroTrustAccessPolicy_Exclude_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ZeroTrustAccessPolicy_Exclude_EmailRef {
        ZeroTrustAccessPolicy_Exclude_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ZeroTrustAccessPolicy_Exclude_EmailDomainRef {
        ZeroTrustAccessPolicy_Exclude_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> ZeroTrustAccessPolicy_Exclude_EmailListRef {
        ZeroTrustAccessPolicy_Exclude_EmailListRef::new(self.shared().clone(), format!("{}.email_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> ZeroTrustAccessPolicy_Exclude_EveryoneRef {
        ZeroTrustAccessPolicy_Exclude_EveryoneRef::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
        ZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ZeroTrustAccessPolicy_Exclude_GeoRef {
        ZeroTrustAccessPolicy_Exclude_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> ZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
        ZeroTrustAccessPolicy_Exclude_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ZeroTrustAccessPolicy_Exclude_GroupRef {
        ZeroTrustAccessPolicy_Exclude_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ZeroTrustAccessPolicy_Exclude_GsuiteRef {
        ZeroTrustAccessPolicy_Exclude_GsuiteRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ZeroTrustAccessPolicy_Exclude_IpRef {
        ZeroTrustAccessPolicy_Exclude_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ZeroTrustAccessPolicy_Exclude_IpListRef {
        ZeroTrustAccessPolicy_Exclude_IpListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> ZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
        ZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ZeroTrustAccessPolicy_Exclude_LoginMethodRef {
        ZeroTrustAccessPolicy_Exclude_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ZeroTrustAccessPolicy_Exclude_OidcRef {
        ZeroTrustAccessPolicy_Exclude_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ZeroTrustAccessPolicy_Exclude_OktaRef {
        ZeroTrustAccessPolicy_Exclude_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ZeroTrustAccessPolicy_Exclude_SamlRef {
        ZeroTrustAccessPolicy_Exclude_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
        ZeroTrustAccessPolicy_Exclude_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> ZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
        ZeroTrustAccessPolicy_Exclude_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_AnyValidServiceToken {}

impl ZeroTrustAccessPolicy_Include_AnyValidServiceToken { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_AnyValidServiceToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_AnyValidServiceToken {}

impl BuildZeroTrustAccessPolicy_Include_AnyValidServiceToken {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_AnyValidServiceToken {
        ZeroTrustAccessPolicy_Include_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
        ZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_AuthContext {
    ac_id: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_AuthContext { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_AuthContext {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_AuthContext {
    #[doc = "The ACID of an Authentication context."]
    pub ac_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of an Authentication context."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Azure identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_AuthContext {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_AuthContext {
        ZeroTrustAccessPolicy_Include_AuthContext {
            ac_id: self.ac_id,
            id: self.id,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_AuthContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_AuthContextRef {
        ZeroTrustAccessPolicy_Include_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_AuthContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ac_id` after provisioning.\nThe ACID of an Authentication context."]
    pub fn ac_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ac_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Authentication context."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_AuthMethod {
    auth_method: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_AuthMethod { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_AuthMethod {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_AuthMethod {
    #[doc = "The type of authentication method https://datatracker.ietf.org/doc/html/rfc8176#section-2."]
    pub auth_method: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_AuthMethod {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_AuthMethod {
        ZeroTrustAccessPolicy_Include_AuthMethod { auth_method: self.auth_method }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_AuthMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_AuthMethodRef {
        ZeroTrustAccessPolicy_Include_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_AuthMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_method` after provisioning.\nThe type of authentication method https://datatracker.ietf.org/doc/html/rfc8176#section-2."]
    pub fn auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_AzureAd {
    id: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_AzureAd { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_AzureAd {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_AzureAd {
    #[doc = "The ID of an Azure group."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Azure identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_AzureAd {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_AzureAd {
        ZeroTrustAccessPolicy_Include_AzureAd {
            id: self.id,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_AzureAdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_AzureAdRef {
        ZeroTrustAccessPolicy_Include_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_AzureAdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Azure group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_Certificate {}

impl ZeroTrustAccessPolicy_Include_Certificate { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_Certificate {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_Certificate {}

impl BuildZeroTrustAccessPolicy_Include_Certificate {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_Certificate {
        ZeroTrustAccessPolicy_Include_Certificate {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_CertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_CertificateRef {
        ZeroTrustAccessPolicy_Include_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_CloudflareAccountMember {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_Include_CloudflareAccountMember {
    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_CloudflareAccountMember {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_CloudflareAccountMember {}

impl BuildZeroTrustAccessPolicy_Include_CloudflareAccountMember {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_CloudflareAccountMember {
        ZeroTrustAccessPolicy_Include_CloudflareAccountMember { account_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
        ZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_CommonName {
    common_name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_CommonName { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_CommonName {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_CommonName {
    #[doc = "The common name to match."]
    pub common_name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_CommonName {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_CommonName {
        ZeroTrustAccessPolicy_Include_CommonName { common_name: self.common_name }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_CommonNameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_CommonNameRef {
        ZeroTrustAccessPolicy_Include_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_CommonNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\nThe common name to match."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_DevicePosture {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    integration_uid: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_DevicePosture {
    #[doc = "Set the field `account_id`.\nThe ID of the account that owns the device posture integration."]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_DevicePosture {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_DevicePosture {
    #[doc = "The ID of a device posture integration."]
    pub integration_uid: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_DevicePosture {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_DevicePosture {
        ZeroTrustAccessPolicy_Include_DevicePosture {
            account_id: core::default::Default::default(),
            integration_uid: self.integration_uid,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Include_DevicePostureRef {
        ZeroTrustAccessPolicy_Include_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_DevicePostureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe ID of the account that owns the device posture integration."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `integration_uid` after provisioning.\nThe ID of a device posture integration."]
    pub fn integration_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_Email {
    email: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_Email { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_Email {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_Email {
    #[doc = "The email of the user."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_Email {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_Email {
        ZeroTrustAccessPolicy_Include_Email { email: self.email }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_EmailRef {
        ZeroTrustAccessPolicy_Include_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_EmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe email of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_EmailDomain {
    domain: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_EmailDomain { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_EmailDomain {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_EmailDomain {
    #[doc = "The email domain to match."]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_EmailDomain {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_EmailDomain {
        ZeroTrustAccessPolicy_Include_EmailDomain { domain: self.domain }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_EmailDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_EmailDomainRef {
        ZeroTrustAccessPolicy_Include_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_EmailDomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe email domain to match."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_EmailList {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_EmailList { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_EmailList {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_EmailList {
    #[doc = "The ID of a previously created email list."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_EmailList {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_EmailList {
        ZeroTrustAccessPolicy_Include_EmailList { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_EmailListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_EmailListRef {
        ZeroTrustAccessPolicy_Include_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_EmailListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created email list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_Everyone {}

impl ZeroTrustAccessPolicy_Include_Everyone { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_Everyone {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_Everyone {}

impl BuildZeroTrustAccessPolicy_Include_Everyone {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_Everyone {
        ZeroTrustAccessPolicy_Include_Everyone {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_EveryoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_EveryoneRef {
        ZeroTrustAccessPolicy_Include_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_ExternalEvaluation {
    evaluate_url: terrars::PrimField<std::string::String>,
    keys_url: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_ExternalEvaluation { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_ExternalEvaluation {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_ExternalEvaluation {
    #[doc = "The API endpoint containing your business logic."]
    pub evaluate_url: terrars::PrimField<std::string::String>,
    #[doc = "The API endpoint containing the key that Access uses to verify that the response came from your API."]
    pub keys_url: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_ExternalEvaluation {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_ExternalEvaluation {
        ZeroTrustAccessPolicy_Include_ExternalEvaluation {
            evaluate_url: self.evaluate_url,
            keys_url: self.keys_url,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
        ZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `evaluate_url` after provisioning.\nThe API endpoint containing your business logic."]
    pub fn evaluate_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keys_url` after provisioning.\nThe API endpoint containing the key that Access uses to verify that the response came from your API."]
    pub fn keys_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_Geo {
    country_code: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_Geo { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_Geo {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_Geo {
    #[doc = "The country code that should be matched."]
    pub country_code: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_Geo {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_Geo {
        ZeroTrustAccessPolicy_Include_Geo { country_code: self.country_code }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_GeoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_GeoRef {
        ZeroTrustAccessPolicy_Include_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_GeoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `country_code` after provisioning.\nThe country code that should be matched."]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_GithubOrganization {
    identity_provider_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    team: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_Include_GithubOrganization {
    #[doc = "Set the field `team`.\nThe name of the team"]
    pub fn set_team(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.team = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_GithubOrganization {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_GithubOrganization {
    #[doc = "The ID of your Github identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the organization."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_GithubOrganization {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_GithubOrganization {
        ZeroTrustAccessPolicy_Include_GithubOrganization {
            identity_provider_id: self.identity_provider_id,
            name: self.name,
            team: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Include_GithubOrganizationRef {
        ZeroTrustAccessPolicy_Include_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_GithubOrganizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Github identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `team` after provisioning.\nThe name of the team"]
    pub fn team(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_Group {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_Group { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_Group {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_Group {
    #[doc = "The ID of a previously created Access group."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_Group {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_Group {
        ZeroTrustAccessPolicy_Include_Group { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_GroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_GroupRef {
        ZeroTrustAccessPolicy_Include_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_GroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created Access group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_Gsuite {
    email: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_Gsuite { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_Gsuite {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_Gsuite {
    #[doc = "The email of the Google Workspace group."]
    pub email: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Google Workspace identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_Gsuite {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_Gsuite {
        ZeroTrustAccessPolicy_Include_Gsuite {
            email: self.email,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_GsuiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_GsuiteRef {
        ZeroTrustAccessPolicy_Include_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_GsuiteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nThe email of the Google Workspace group."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Google Workspace identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_Ip {
    ip: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_Ip { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_Ip {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_Ip {
    #[doc = "An IPv4 or IPv6 CIDR block."]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_Ip {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_Ip {
        ZeroTrustAccessPolicy_Include_Ip { ip: self.ip }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_IpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_IpRef {
        ZeroTrustAccessPolicy_Include_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_IpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 or IPv6 CIDR block."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_IpList {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_IpList { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_IpList {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_IpList {
    #[doc = "The ID of a previously created IP list."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_IpList {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_IpList {
        ZeroTrustAccessPolicy_Include_IpList { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_IpListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_IpListRef {
        ZeroTrustAccessPolicy_Include_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_IpListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created IP list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_LinkedAppToken {
    app_uid: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_LinkedAppToken { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_LinkedAppToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_LinkedAppToken {
    #[doc = "The ID of an Access OIDC SaaS application"]
    pub app_uid: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_LinkedAppToken {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_LinkedAppToken {
        ZeroTrustAccessPolicy_Include_LinkedAppToken { app_uid: self.app_uid }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
        ZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_uid` after provisioning.\nThe ID of an Access OIDC SaaS application"]
    pub fn app_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_LoginMethod {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_LoginMethod { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_LoginMethod {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_LoginMethod {
    #[doc = "The ID of an identity provider."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_LoginMethod {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_LoginMethod {
        ZeroTrustAccessPolicy_Include_LoginMethod { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_LoginMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_LoginMethodRef {
        ZeroTrustAccessPolicy_Include_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_LoginMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an identity provider."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_Oidc {
    claim_name: terrars::PrimField<std::string::String>,
    claim_value: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_Oidc { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_Oidc {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_Oidc {
    #[doc = "The name of the OIDC claim."]
    pub claim_name: terrars::PrimField<std::string::String>,
    #[doc = "The OIDC claim value to look for."]
    pub claim_value: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your OIDC identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_Oidc {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_Oidc {
        ZeroTrustAccessPolicy_Include_Oidc {
            claim_name: self.claim_name,
            claim_value: self.claim_value,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_OidcRef {
        ZeroTrustAccessPolicy_Include_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_OidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_name` after provisioning.\nThe name of the OIDC claim."]
    pub fn claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `claim_value` after provisioning.\nThe OIDC claim value to look for."]
    pub fn claim_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your OIDC identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_Okta {
    identity_provider_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_Okta { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_Okta {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_Okta {
    #[doc = "The ID of your Okta identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Okta group."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_Okta {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_Okta {
        ZeroTrustAccessPolicy_Include_Okta {
            identity_provider_id: self.identity_provider_id,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_OktaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_OktaRef {
        ZeroTrustAccessPolicy_Include_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_OktaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Okta identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Okta group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_Saml {
    attribute_name: terrars::PrimField<std::string::String>,
    attribute_value: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_Saml { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_Saml {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_Saml {
    #[doc = "The name of the SAML attribute."]
    pub attribute_name: terrars::PrimField<std::string::String>,
    #[doc = "The SAML attribute value to look for."]
    pub attribute_value: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your SAML identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_Saml {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_Saml {
        ZeroTrustAccessPolicy_Include_Saml {
            attribute_name: self.attribute_name,
            attribute_value: self.attribute_value,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_SamlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_SamlRef {
        ZeroTrustAccessPolicy_Include_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_SamlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_name` after provisioning.\nThe name of the SAML attribute."]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `attribute_value` after provisioning.\nThe SAML attribute value to look for."]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your SAML identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_ServiceToken {
    token_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Include_ServiceToken { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_ServiceToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_ServiceToken {
    #[doc = "The ID of a Service Token."]
    pub token_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Include_ServiceToken {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_ServiceToken {
        ZeroTrustAccessPolicy_Include_ServiceToken { token_id: self.token_id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_ServiceTokenRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Include_ServiceTokenRef {
        ZeroTrustAccessPolicy_Include_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_ServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\nThe ID of a Service Token."]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include_UserRiskScore {
    user_risk_score: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_Include_UserRiskScore { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include_UserRiskScore {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include_UserRiskScore {
    #[doc = "A list of risk score levels to match. Values can be low, medium, high, or unscored."]
    pub user_risk_score: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildZeroTrustAccessPolicy_Include_UserRiskScore {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include_UserRiskScore {
        ZeroTrustAccessPolicy_Include_UserRiskScore { user_risk_score: self.user_risk_score }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Include_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Include_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Include_UserRiskScoreRef {
        ZeroTrustAccessPolicy_Include_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Include_UserRiskScoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_risk_score` after provisioning.\nA list of risk score levels to match. Values can be low, medium, high, or unscored."]
    pub fn user_risk_score(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_risk_score", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Include {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_valid_service_token: std::option::Option<ZeroTrustAccessPolicy_Include_AnyValidServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_context: std::option::Option<ZeroTrustAccessPolicy_Include_AuthContext>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_method: std::option::Option<ZeroTrustAccessPolicy_Include_AuthMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_ad: std::option::Option<ZeroTrustAccessPolicy_Include_AzureAd>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<ZeroTrustAccessPolicy_Include_Certificate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_account_member: std::option::Option<ZeroTrustAccessPolicy_Include_CloudflareAccountMember>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<ZeroTrustAccessPolicy_Include_CommonName>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<ZeroTrustAccessPolicy_Include_DevicePosture>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<ZeroTrustAccessPolicy_Include_Email>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_domain: std::option::Option<ZeroTrustAccessPolicy_Include_EmailDomain>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_list: std::option::Option<ZeroTrustAccessPolicy_Include_EmailList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    everyone: std::option::Option<ZeroTrustAccessPolicy_Include_Everyone>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_evaluation: std::option::Option<ZeroTrustAccessPolicy_Include_ExternalEvaluation>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<ZeroTrustAccessPolicy_Include_Geo>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_organization: std::option::Option<ZeroTrustAccessPolicy_Include_GithubOrganization>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<ZeroTrustAccessPolicy_Include_Group>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gsuite: std::option::Option<ZeroTrustAccessPolicy_Include_Gsuite>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<ZeroTrustAccessPolicy_Include_Ip>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_list: std::option::Option<ZeroTrustAccessPolicy_Include_IpList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_app_token: std::option::Option<ZeroTrustAccessPolicy_Include_LinkedAppToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_method: std::option::Option<ZeroTrustAccessPolicy_Include_LoginMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<ZeroTrustAccessPolicy_Include_Oidc>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta: std::option::Option<ZeroTrustAccessPolicy_Include_Okta>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<ZeroTrustAccessPolicy_Include_Saml>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_token: std::option::Option<ZeroTrustAccessPolicy_Include_ServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_risk_score: std::option::Option<ZeroTrustAccessPolicy_Include_UserRiskScore>,
}

impl ZeroTrustAccessPolicy_Include {
    #[doc = "Set the field `any_valid_service_token`.\nAn empty object which matches on all service tokens."]
    pub fn set_any_valid_service_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_AnyValidServiceToken>,
    ) -> Self {
        self.any_valid_service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_context`.\n"]
    pub fn set_auth_context(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_AuthContext>) -> Self {
        self.auth_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_AuthMethod>) -> Self {
        self.auth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure_ad`.\n"]
    pub fn set_azure_ad(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_AzureAd>) -> Self {
        self.azure_ad = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_Certificate>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudflare_account_member`.\n"]
    pub fn set_cloudflare_account_member(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_CloudflareAccountMember>,
    ) -> Self {
        self.cloudflare_account_member = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_CommonName>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_DevicePosture>) -> Self {
        self.device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_Email>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_EmailDomain>) -> Self {
        self.email_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_list`.\n"]
    pub fn set_email_list(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_EmailList>) -> Self {
        self.email_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `everyone`.\nAn empty object which matches on all users."]
    pub fn set_everyone(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_Everyone>) -> Self {
        self.everyone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_ExternalEvaluation>,
    ) -> Self {
        self.external_evaluation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_Geo>) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github_organization`.\n"]
    pub fn set_github_organization(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_GithubOrganization>,
    ) -> Self {
        self.github_organization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_Group>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_Gsuite>) -> Self {
        self.gsuite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_Ip>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_IpList>) -> Self {
        self.ip_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_app_token`.\n"]
    pub fn set_linked_app_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_LinkedAppToken>,
    ) -> Self {
        self.linked_app_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_LoginMethod>) -> Self {
        self.login_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_Oidc>) -> Self {
        self.oidc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_Okta>) -> Self {
        self.okta = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_Saml>) -> Self {
        self.saml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_ServiceToken>) -> Self {
        self.service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_risk_score`.\n"]
    pub fn set_user_risk_score(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Include_UserRiskScore>,
    ) -> Self {
        self.user_risk_score = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Include {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Include {}

impl BuildZeroTrustAccessPolicy_Include {
    pub fn build(self) -> ZeroTrustAccessPolicy_Include {
        ZeroTrustAccessPolicy_Include {
            any_valid_service_token: core::default::Default::default(),
            auth_context: core::default::Default::default(),
            auth_method: core::default::Default::default(),
            azure_ad: core::default::Default::default(),
            certificate: core::default::Default::default(),
            cloudflare_account_member: core::default::Default::default(),
            common_name: core::default::Default::default(),
            device_posture: core::default::Default::default(),
            email: core::default::Default::default(),
            email_domain: core::default::Default::default(),
            email_list: core::default::Default::default(),
            everyone: core::default::Default::default(),
            external_evaluation: core::default::Default::default(),
            geo: core::default::Default::default(),
            github_organization: core::default::Default::default(),
            group: core::default::Default::default(),
            gsuite: core::default::Default::default(),
            ip: core::default::Default::default(),
            ip_list: core::default::Default::default(),
            linked_app_token: core::default::Default::default(),
            login_method: core::default::Default::default(),
            oidc: core::default::Default::default(),
            okta: core::default::Default::default(),
            saml: core::default::Default::default(),
            service_token: core::default::Default::default(),
            user_risk_score: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_IncludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_IncludeRef {
        ZeroTrustAccessPolicy_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> ZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
        ZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> ZeroTrustAccessPolicy_Include_AuthContextRef {
        ZeroTrustAccessPolicy_Include_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> ZeroTrustAccessPolicy_Include_AuthMethodRef {
        ZeroTrustAccessPolicy_Include_AuthMethodRef::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> ZeroTrustAccessPolicy_Include_AzureAdRef {
        ZeroTrustAccessPolicy_Include_AzureAdRef::new(self.shared().clone(), format!("{}.azure_ad", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> ZeroTrustAccessPolicy_Include_CertificateRef {
        ZeroTrustAccessPolicy_Include_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> ZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
        ZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> ZeroTrustAccessPolicy_Include_CommonNameRef {
        ZeroTrustAccessPolicy_Include_CommonNameRef::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ZeroTrustAccessPolicy_Include_DevicePostureRef {
        ZeroTrustAccessPolicy_Include_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ZeroTrustAccessPolicy_Include_EmailRef {
        ZeroTrustAccessPolicy_Include_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ZeroTrustAccessPolicy_Include_EmailDomainRef {
        ZeroTrustAccessPolicy_Include_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> ZeroTrustAccessPolicy_Include_EmailListRef {
        ZeroTrustAccessPolicy_Include_EmailListRef::new(self.shared().clone(), format!("{}.email_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> ZeroTrustAccessPolicy_Include_EveryoneRef {
        ZeroTrustAccessPolicy_Include_EveryoneRef::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
        ZeroTrustAccessPolicy_Include_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ZeroTrustAccessPolicy_Include_GeoRef {
        ZeroTrustAccessPolicy_Include_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> ZeroTrustAccessPolicy_Include_GithubOrganizationRef {
        ZeroTrustAccessPolicy_Include_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ZeroTrustAccessPolicy_Include_GroupRef {
        ZeroTrustAccessPolicy_Include_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ZeroTrustAccessPolicy_Include_GsuiteRef {
        ZeroTrustAccessPolicy_Include_GsuiteRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ZeroTrustAccessPolicy_Include_IpRef {
        ZeroTrustAccessPolicy_Include_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ZeroTrustAccessPolicy_Include_IpListRef {
        ZeroTrustAccessPolicy_Include_IpListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> ZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
        ZeroTrustAccessPolicy_Include_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ZeroTrustAccessPolicy_Include_LoginMethodRef {
        ZeroTrustAccessPolicy_Include_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ZeroTrustAccessPolicy_Include_OidcRef {
        ZeroTrustAccessPolicy_Include_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ZeroTrustAccessPolicy_Include_OktaRef {
        ZeroTrustAccessPolicy_Include_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ZeroTrustAccessPolicy_Include_SamlRef {
        ZeroTrustAccessPolicy_Include_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ZeroTrustAccessPolicy_Include_ServiceTokenRef {
        ZeroTrustAccessPolicy_Include_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> ZeroTrustAccessPolicy_Include_UserRiskScoreRef {
        ZeroTrustAccessPolicy_Include_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_MfaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_authenticators: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mfa_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_MfaConfig {
    #[doc = "Set the field `allowed_authenticators`.\nLists the MFA methods that users can authenticate with."]
    pub fn set_allowed_authenticators(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_authenticators = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mfa_disabled`.\nIndicates whether to disable MFA for this resource. This option is available at the application and policy level."]
    pub fn set_mfa_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mfa_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_duration`.\nDefines the duration of an MFA session. Must be in minutes (m) or hours (h). Minimum: 0m. Maximum: 720h (30 days). Examples:`5m` or `24h`."]
    pub fn set_session_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.session_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_MfaConfig {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_MfaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_MfaConfig {}

impl BuildZeroTrustAccessPolicy_MfaConfig {
    pub fn build(self) -> ZeroTrustAccessPolicy_MfaConfig {
        ZeroTrustAccessPolicy_MfaConfig {
            allowed_authenticators: core::default::Default::default(),
            mfa_disabled: core::default::Default::default(),
            session_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_MfaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_MfaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_MfaConfigRef {
        ZeroTrustAccessPolicy_MfaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_MfaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_authenticators` after provisioning.\nLists the MFA methods that users can authenticate with."]
    pub fn allowed_authenticators(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_authenticators", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mfa_disabled` after provisioning.\nIndicates whether to disable MFA for this resource. This option is available at the application and policy level."]
    pub fn mfa_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mfa_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nDefines the duration of an MFA session. Must be in minutes (m) or hours (h). Minimum: 0m. Maximum: 720h (30 days). Examples:`5m` or `24h`."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_AnyValidServiceToken {}

impl ZeroTrustAccessPolicy_Require_AnyValidServiceToken { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_AnyValidServiceToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_AnyValidServiceToken {}

impl BuildZeroTrustAccessPolicy_Require_AnyValidServiceToken {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_AnyValidServiceToken {
        ZeroTrustAccessPolicy_Require_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
        ZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_AuthContext {
    ac_id: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_AuthContext { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_AuthContext {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_AuthContext {
    #[doc = "The ACID of an Authentication context."]
    pub ac_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of an Authentication context."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Azure identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_AuthContext {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_AuthContext {
        ZeroTrustAccessPolicy_Require_AuthContext {
            ac_id: self.ac_id,
            id: self.id,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_AuthContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_AuthContextRef {
        ZeroTrustAccessPolicy_Require_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_AuthContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ac_id` after provisioning.\nThe ACID of an Authentication context."]
    pub fn ac_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ac_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Authentication context."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_AuthMethod {
    auth_method: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_AuthMethod { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_AuthMethod {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_AuthMethod {
    #[doc = "The type of authentication method https://datatracker.ietf.org/doc/html/rfc8176#section-2."]
    pub auth_method: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_AuthMethod {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_AuthMethod {
        ZeroTrustAccessPolicy_Require_AuthMethod { auth_method: self.auth_method }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_AuthMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_AuthMethodRef {
        ZeroTrustAccessPolicy_Require_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_AuthMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_method` after provisioning.\nThe type of authentication method https://datatracker.ietf.org/doc/html/rfc8176#section-2."]
    pub fn auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_AzureAd {
    id: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_AzureAd { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_AzureAd {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_AzureAd {
    #[doc = "The ID of an Azure group."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Azure identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_AzureAd {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_AzureAd {
        ZeroTrustAccessPolicy_Require_AzureAd {
            id: self.id,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_AzureAdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_AzureAdRef {
        ZeroTrustAccessPolicy_Require_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_AzureAdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an Azure group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Azure identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_Certificate {}

impl ZeroTrustAccessPolicy_Require_Certificate { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_Certificate {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_Certificate {}

impl BuildZeroTrustAccessPolicy_Require_Certificate {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_Certificate {
        ZeroTrustAccessPolicy_Require_Certificate {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_CertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_CertificateRef {
        ZeroTrustAccessPolicy_Require_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_CloudflareAccountMember {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_Require_CloudflareAccountMember {
    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_CloudflareAccountMember {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_CloudflareAccountMember {}

impl BuildZeroTrustAccessPolicy_Require_CloudflareAccountMember {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_CloudflareAccountMember {
        ZeroTrustAccessPolicy_Require_CloudflareAccountMember { account_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
        ZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_CommonName {
    common_name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_CommonName { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_CommonName {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_CommonName {
    #[doc = "The common name to match."]
    pub common_name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_CommonName {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_CommonName {
        ZeroTrustAccessPolicy_Require_CommonName { common_name: self.common_name }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_CommonNameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_CommonNameRef {
        ZeroTrustAccessPolicy_Require_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_CommonNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\nThe common name to match."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_DevicePosture {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    integration_uid: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_DevicePosture {
    #[doc = "Set the field `account_id`.\nThe ID of the account that owns the device posture integration."]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_DevicePosture {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_DevicePosture {
    #[doc = "The ID of a device posture integration."]
    pub integration_uid: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_DevicePosture {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_DevicePosture {
        ZeroTrustAccessPolicy_Require_DevicePosture {
            account_id: core::default::Default::default(),
            integration_uid: self.integration_uid,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Require_DevicePostureRef {
        ZeroTrustAccessPolicy_Require_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_DevicePostureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe ID of the account that owns the device posture integration."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `integration_uid` after provisioning.\nThe ID of a device posture integration."]
    pub fn integration_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_Email {
    email: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_Email { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_Email {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_Email {
    #[doc = "The email of the user."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_Email {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_Email {
        ZeroTrustAccessPolicy_Require_Email { email: self.email }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_EmailRef {
        ZeroTrustAccessPolicy_Require_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_EmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe email of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_EmailDomain {
    domain: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_EmailDomain { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_EmailDomain {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_EmailDomain {
    #[doc = "The email domain to match."]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_EmailDomain {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_EmailDomain {
        ZeroTrustAccessPolicy_Require_EmailDomain { domain: self.domain }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_EmailDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_EmailDomainRef {
        ZeroTrustAccessPolicy_Require_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_EmailDomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe email domain to match."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_EmailList {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_EmailList { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_EmailList {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_EmailList {
    #[doc = "The ID of a previously created email list."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_EmailList {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_EmailList {
        ZeroTrustAccessPolicy_Require_EmailList { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_EmailListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_EmailListRef {
        ZeroTrustAccessPolicy_Require_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_EmailListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created email list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_Everyone {}

impl ZeroTrustAccessPolicy_Require_Everyone { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_Everyone {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_Everyone {}

impl BuildZeroTrustAccessPolicy_Require_Everyone {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_Everyone {
        ZeroTrustAccessPolicy_Require_Everyone {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_EveryoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_EveryoneRef {
        ZeroTrustAccessPolicy_Require_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_ExternalEvaluation {
    evaluate_url: terrars::PrimField<std::string::String>,
    keys_url: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_ExternalEvaluation { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_ExternalEvaluation {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_ExternalEvaluation {
    #[doc = "The API endpoint containing your business logic."]
    pub evaluate_url: terrars::PrimField<std::string::String>,
    #[doc = "The API endpoint containing the key that Access uses to verify that the response came from your API."]
    pub keys_url: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_ExternalEvaluation {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_ExternalEvaluation {
        ZeroTrustAccessPolicy_Require_ExternalEvaluation {
            evaluate_url: self.evaluate_url,
            keys_url: self.keys_url,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
        ZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `evaluate_url` after provisioning.\nThe API endpoint containing your business logic."]
    pub fn evaluate_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keys_url` after provisioning.\nThe API endpoint containing the key that Access uses to verify that the response came from your API."]
    pub fn keys_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_Geo {
    country_code: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_Geo { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_Geo {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_Geo {
    #[doc = "The country code that should be matched."]
    pub country_code: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_Geo {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_Geo {
        ZeroTrustAccessPolicy_Require_Geo { country_code: self.country_code }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_GeoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_GeoRef {
        ZeroTrustAccessPolicy_Require_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_GeoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `country_code` after provisioning.\nThe country code that should be matched."]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_GithubOrganization {
    identity_provider_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    team: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_Require_GithubOrganization {
    #[doc = "Set the field `team`.\nThe name of the team"]
    pub fn set_team(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.team = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_GithubOrganization {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_GithubOrganization {
    #[doc = "The ID of your Github identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the organization."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_GithubOrganization {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_GithubOrganization {
        ZeroTrustAccessPolicy_Require_GithubOrganization {
            identity_provider_id: self.identity_provider_id,
            name: self.name,
            team: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Require_GithubOrganizationRef {
        ZeroTrustAccessPolicy_Require_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_GithubOrganizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Github identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `team` after provisioning.\nThe name of the team"]
    pub fn team(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_Group {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_Group { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_Group {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_Group {
    #[doc = "The ID of a previously created Access group."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_Group {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_Group {
        ZeroTrustAccessPolicy_Require_Group { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_GroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_GroupRef {
        ZeroTrustAccessPolicy_Require_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_GroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created Access group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_Gsuite {
    email: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_Gsuite { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_Gsuite {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_Gsuite {
    #[doc = "The email of the Google Workspace group."]
    pub email: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Google Workspace identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_Gsuite {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_Gsuite {
        ZeroTrustAccessPolicy_Require_Gsuite {
            email: self.email,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_GsuiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_GsuiteRef {
        ZeroTrustAccessPolicy_Require_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_GsuiteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nThe email of the Google Workspace group."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Google Workspace identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_Ip {
    ip: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_Ip { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_Ip {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_Ip {
    #[doc = "An IPv4 or IPv6 CIDR block."]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_Ip {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_Ip {
        ZeroTrustAccessPolicy_Require_Ip { ip: self.ip }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_IpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_IpRef {
        ZeroTrustAccessPolicy_Require_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_IpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 or IPv6 CIDR block."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_IpList {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_IpList { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_IpList {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_IpList {
    #[doc = "The ID of a previously created IP list."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_IpList {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_IpList {
        ZeroTrustAccessPolicy_Require_IpList { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_IpListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_IpListRef {
        ZeroTrustAccessPolicy_Require_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_IpListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of a previously created IP list."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_LinkedAppToken {
    app_uid: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_LinkedAppToken { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_LinkedAppToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_LinkedAppToken {
    #[doc = "The ID of an Access OIDC SaaS application"]
    pub app_uid: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_LinkedAppToken {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_LinkedAppToken {
        ZeroTrustAccessPolicy_Require_LinkedAppToken { app_uid: self.app_uid }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
        ZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_uid` after provisioning.\nThe ID of an Access OIDC SaaS application"]
    pub fn app_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_LoginMethod {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_LoginMethod { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_LoginMethod {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_LoginMethod {
    #[doc = "The ID of an identity provider."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_LoginMethod {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_LoginMethod {
        ZeroTrustAccessPolicy_Require_LoginMethod { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_LoginMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_LoginMethodRef {
        ZeroTrustAccessPolicy_Require_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_LoginMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of an identity provider."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_Oidc {
    claim_name: terrars::PrimField<std::string::String>,
    claim_value: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_Oidc { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_Oidc {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_Oidc {
    #[doc = "The name of the OIDC claim."]
    pub claim_name: terrars::PrimField<std::string::String>,
    #[doc = "The OIDC claim value to look for."]
    pub claim_value: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your OIDC identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_Oidc {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_Oidc {
        ZeroTrustAccessPolicy_Require_Oidc {
            claim_name: self.claim_name,
            claim_value: self.claim_value,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_OidcRef {
        ZeroTrustAccessPolicy_Require_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_OidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_name` after provisioning.\nThe name of the OIDC claim."]
    pub fn claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `claim_value` after provisioning.\nThe OIDC claim value to look for."]
    pub fn claim_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your OIDC identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_Okta {
    identity_provider_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_Okta { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_Okta {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_Okta {
    #[doc = "The ID of your Okta identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Okta group."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_Okta {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_Okta {
        ZeroTrustAccessPolicy_Require_Okta {
            identity_provider_id: self.identity_provider_id,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_OktaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_OktaRef {
        ZeroTrustAccessPolicy_Require_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_OktaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your Okta identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Okta group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_Saml {
    attribute_name: terrars::PrimField<std::string::String>,
    attribute_value: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_Saml { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_Saml {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_Saml {
    #[doc = "The name of the SAML attribute."]
    pub attribute_name: terrars::PrimField<std::string::String>,
    #[doc = "The SAML attribute value to look for."]
    pub attribute_value: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your SAML identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_Saml {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_Saml {
        ZeroTrustAccessPolicy_Require_Saml {
            attribute_name: self.attribute_name,
            attribute_value: self.attribute_value,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_SamlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_SamlRef {
        ZeroTrustAccessPolicy_Require_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_SamlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_name` after provisioning.\nThe name of the SAML attribute."]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `attribute_value` after provisioning.\nThe SAML attribute value to look for."]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of your SAML identity provider."]
    pub fn identity_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_ServiceToken {
    token_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessPolicy_Require_ServiceToken { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_ServiceToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_ServiceToken {
    #[doc = "The ID of a Service Token."]
    pub token_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessPolicy_Require_ServiceToken {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_ServiceToken {
        ZeroTrustAccessPolicy_Require_ServiceToken { token_id: self.token_id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_ServiceTokenRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_Require_ServiceTokenRef {
        ZeroTrustAccessPolicy_Require_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_ServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\nThe ID of a Service Token."]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require_UserRiskScore {
    user_risk_score: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessPolicy_Require_UserRiskScore { }

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require_UserRiskScore {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require_UserRiskScore {
    #[doc = "A list of risk score levels to match. Values can be low, medium, high, or unscored."]
    pub user_risk_score: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildZeroTrustAccessPolicy_Require_UserRiskScore {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require_UserRiskScore {
        ZeroTrustAccessPolicy_Require_UserRiskScore { user_risk_score: self.user_risk_score }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_Require_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_Require_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessPolicy_Require_UserRiskScoreRef {
        ZeroTrustAccessPolicy_Require_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_Require_UserRiskScoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_risk_score` after provisioning.\nA list of risk score levels to match. Values can be low, medium, high, or unscored."]
    pub fn user_risk_score(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_risk_score", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessPolicy_Require {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_valid_service_token: std::option::Option<ZeroTrustAccessPolicy_Require_AnyValidServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_context: std::option::Option<ZeroTrustAccessPolicy_Require_AuthContext>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_method: std::option::Option<ZeroTrustAccessPolicy_Require_AuthMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_ad: std::option::Option<ZeroTrustAccessPolicy_Require_AzureAd>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<ZeroTrustAccessPolicy_Require_Certificate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_account_member: std::option::Option<ZeroTrustAccessPolicy_Require_CloudflareAccountMember>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<ZeroTrustAccessPolicy_Require_CommonName>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<ZeroTrustAccessPolicy_Require_DevicePosture>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<ZeroTrustAccessPolicy_Require_Email>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_domain: std::option::Option<ZeroTrustAccessPolicy_Require_EmailDomain>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_list: std::option::Option<ZeroTrustAccessPolicy_Require_EmailList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    everyone: std::option::Option<ZeroTrustAccessPolicy_Require_Everyone>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_evaluation: std::option::Option<ZeroTrustAccessPolicy_Require_ExternalEvaluation>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<ZeroTrustAccessPolicy_Require_Geo>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_organization: std::option::Option<ZeroTrustAccessPolicy_Require_GithubOrganization>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<ZeroTrustAccessPolicy_Require_Group>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gsuite: std::option::Option<ZeroTrustAccessPolicy_Require_Gsuite>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<ZeroTrustAccessPolicy_Require_Ip>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_list: std::option::Option<ZeroTrustAccessPolicy_Require_IpList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_app_token: std::option::Option<ZeroTrustAccessPolicy_Require_LinkedAppToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_method: std::option::Option<ZeroTrustAccessPolicy_Require_LoginMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<ZeroTrustAccessPolicy_Require_Oidc>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta: std::option::Option<ZeroTrustAccessPolicy_Require_Okta>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<ZeroTrustAccessPolicy_Require_Saml>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_token: std::option::Option<ZeroTrustAccessPolicy_Require_ServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_risk_score: std::option::Option<ZeroTrustAccessPolicy_Require_UserRiskScore>,
}

impl ZeroTrustAccessPolicy_Require {
    #[doc = "Set the field `any_valid_service_token`.\nAn empty object which matches on all service tokens."]
    pub fn set_any_valid_service_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_AnyValidServiceToken>,
    ) -> Self {
        self.any_valid_service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_context`.\n"]
    pub fn set_auth_context(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_AuthContext>) -> Self {
        self.auth_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_AuthMethod>) -> Self {
        self.auth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure_ad`.\n"]
    pub fn set_azure_ad(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_AzureAd>) -> Self {
        self.azure_ad = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_Certificate>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudflare_account_member`.\n"]
    pub fn set_cloudflare_account_member(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_CloudflareAccountMember>,
    ) -> Self {
        self.cloudflare_account_member = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_CommonName>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_DevicePosture>) -> Self {
        self.device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_Email>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_EmailDomain>) -> Self {
        self.email_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_list`.\n"]
    pub fn set_email_list(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_EmailList>) -> Self {
        self.email_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `everyone`.\nAn empty object which matches on all users."]
    pub fn set_everyone(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_Everyone>) -> Self {
        self.everyone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_ExternalEvaluation>,
    ) -> Self {
        self.external_evaluation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_Geo>) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github_organization`.\n"]
    pub fn set_github_organization(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_GithubOrganization>,
    ) -> Self {
        self.github_organization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_Group>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_Gsuite>) -> Self {
        self.gsuite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_Ip>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_IpList>) -> Self {
        self.ip_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_app_token`.\n"]
    pub fn set_linked_app_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_LinkedAppToken>,
    ) -> Self {
        self.linked_app_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_LoginMethod>) -> Self {
        self.login_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_Oidc>) -> Self {
        self.oidc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_Okta>) -> Self {
        self.okta = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_Saml>) -> Self {
        self.saml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_ServiceToken>) -> Self {
        self.service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_risk_score`.\n"]
    pub fn set_user_risk_score(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessPolicy_Require_UserRiskScore>,
    ) -> Self {
        self.user_risk_score = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessPolicy_Require {
    type O = terrars::BlockAssignable<ZeroTrustAccessPolicy_Require>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessPolicy_Require {}

impl BuildZeroTrustAccessPolicy_Require {
    pub fn build(self) -> ZeroTrustAccessPolicy_Require {
        ZeroTrustAccessPolicy_Require {
            any_valid_service_token: core::default::Default::default(),
            auth_context: core::default::Default::default(),
            auth_method: core::default::Default::default(),
            azure_ad: core::default::Default::default(),
            certificate: core::default::Default::default(),
            cloudflare_account_member: core::default::Default::default(),
            common_name: core::default::Default::default(),
            device_posture: core::default::Default::default(),
            email: core::default::Default::default(),
            email_domain: core::default::Default::default(),
            email_list: core::default::Default::default(),
            everyone: core::default::Default::default(),
            external_evaluation: core::default::Default::default(),
            geo: core::default::Default::default(),
            github_organization: core::default::Default::default(),
            group: core::default::Default::default(),
            gsuite: core::default::Default::default(),
            ip: core::default::Default::default(),
            ip_list: core::default::Default::default(),
            linked_app_token: core::default::Default::default(),
            login_method: core::default::Default::default(),
            oidc: core::default::Default::default(),
            okta: core::default::Default::default(),
            saml: core::default::Default::default(),
            service_token: core::default::Default::default(),
            user_risk_score: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessPolicy_RequireRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessPolicy_RequireRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessPolicy_RequireRef {
        ZeroTrustAccessPolicy_RequireRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessPolicy_RequireRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> ZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
        ZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> ZeroTrustAccessPolicy_Require_AuthContextRef {
        ZeroTrustAccessPolicy_Require_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> ZeroTrustAccessPolicy_Require_AuthMethodRef {
        ZeroTrustAccessPolicy_Require_AuthMethodRef::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> ZeroTrustAccessPolicy_Require_AzureAdRef {
        ZeroTrustAccessPolicy_Require_AzureAdRef::new(self.shared().clone(), format!("{}.azure_ad", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> ZeroTrustAccessPolicy_Require_CertificateRef {
        ZeroTrustAccessPolicy_Require_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> ZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
        ZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> ZeroTrustAccessPolicy_Require_CommonNameRef {
        ZeroTrustAccessPolicy_Require_CommonNameRef::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ZeroTrustAccessPolicy_Require_DevicePostureRef {
        ZeroTrustAccessPolicy_Require_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ZeroTrustAccessPolicy_Require_EmailRef {
        ZeroTrustAccessPolicy_Require_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ZeroTrustAccessPolicy_Require_EmailDomainRef {
        ZeroTrustAccessPolicy_Require_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> ZeroTrustAccessPolicy_Require_EmailListRef {
        ZeroTrustAccessPolicy_Require_EmailListRef::new(self.shared().clone(), format!("{}.email_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> ZeroTrustAccessPolicy_Require_EveryoneRef {
        ZeroTrustAccessPolicy_Require_EveryoneRef::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
        ZeroTrustAccessPolicy_Require_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ZeroTrustAccessPolicy_Require_GeoRef {
        ZeroTrustAccessPolicy_Require_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> ZeroTrustAccessPolicy_Require_GithubOrganizationRef {
        ZeroTrustAccessPolicy_Require_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ZeroTrustAccessPolicy_Require_GroupRef {
        ZeroTrustAccessPolicy_Require_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ZeroTrustAccessPolicy_Require_GsuiteRef {
        ZeroTrustAccessPolicy_Require_GsuiteRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ZeroTrustAccessPolicy_Require_IpRef {
        ZeroTrustAccessPolicy_Require_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ZeroTrustAccessPolicy_Require_IpListRef {
        ZeroTrustAccessPolicy_Require_IpListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> ZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
        ZeroTrustAccessPolicy_Require_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ZeroTrustAccessPolicy_Require_LoginMethodRef {
        ZeroTrustAccessPolicy_Require_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ZeroTrustAccessPolicy_Require_OidcRef {
        ZeroTrustAccessPolicy_Require_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ZeroTrustAccessPolicy_Require_OktaRef {
        ZeroTrustAccessPolicy_Require_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ZeroTrustAccessPolicy_Require_SamlRef {
        ZeroTrustAccessPolicy_Require_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ZeroTrustAccessPolicy_Require_ServiceTokenRef {
        ZeroTrustAccessPolicy_Require_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> ZeroTrustAccessPolicy_Require_UserRiskScoreRef {
        ZeroTrustAccessPolicy_Require_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}
