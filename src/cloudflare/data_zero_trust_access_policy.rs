#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustAccessPolicyData_ {
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
    policy_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustAccessPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustAccessPolicyData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustAccessPolicy(std::rc::Rc<DataZeroTrustAccessPolicy_>);

impl DataZeroTrustAccessPolicy {
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
    pub fn approval_groups(&self) -> terrars::SetRef<DataZeroTrustAccessPolicy_ApprovalGroupsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approval_groups", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approval_required` after provisioning.\nRequires the user to request access from an administrator at the start of each session."]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_rules` after provisioning.\nThe rules that define how users may connect to targets secured by your application."]
    pub fn connection_rules(&self) -> DataZeroTrustAccessPolicy_ConnectionRulesRef {
        DataZeroTrustAccessPolicy_ConnectionRulesRef::new(
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
    pub fn exclude(&self) -> terrars::SetRef<DataZeroTrustAccessPolicy_ExcludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe UUID of the policy"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nRules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub fn include(&self) -> terrars::SetRef<DataZeroTrustAccessPolicy_IncludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `isolation_required` after provisioning.\nRequire this application to be served in an isolated browser for users matching this policy. 'Client Web Isolation' must be on for the account in order to use this feature."]
    pub fn isolation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolation_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings."]
    pub fn mfa_config(&self) -> DataZeroTrustAccessPolicy_MfaConfigRef {
        DataZeroTrustAccessPolicy_MfaConfigRef::new(
            self.shared().clone(),
            format!("{}.mfa_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Access policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe UUID of the policy"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
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
    pub fn require(&self) -> terrars::SetRef<DataZeroTrustAccessPolicy_RequireRef> {
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

impl terrars::Referable for DataZeroTrustAccessPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustAccessPolicy { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy {
    type O = terrars::ListRef<DataZeroTrustAccessPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustAccessPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustAccessPolicy {
    pub tf_id: std::string::String,
    #[doc = "The UUID of the policy"]
    pub policy_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustAccessPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustAccessPolicy {
        let out = DataZeroTrustAccessPolicy(std::rc::Rc::new(DataZeroTrustAccessPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustAccessPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                policy_id: self.policy_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustAccessPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustAccessPolicyRef {
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
        "Get a reference to the value of field `app_count` after provisioning.\nNumber of access applications currently using this policy."]
    pub fn app_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approval_groups` after provisioning.\nAdministrators who can approve a temporary authentication request."]
    pub fn approval_groups(&self) -> terrars::SetRef<DataZeroTrustAccessPolicy_ApprovalGroupsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approval_groups", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approval_required` after provisioning.\nRequires the user to request access from an administrator at the start of each session."]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_rules` after provisioning.\nThe rules that define how users may connect to targets secured by your application."]
    pub fn connection_rules(&self) -> DataZeroTrustAccessPolicy_ConnectionRulesRef {
        DataZeroTrustAccessPolicy_ConnectionRulesRef::new(
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
    pub fn exclude(&self) -> terrars::SetRef<DataZeroTrustAccessPolicy_ExcludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe UUID of the policy"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nRules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub fn include(&self) -> terrars::SetRef<DataZeroTrustAccessPolicy_IncludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `isolation_required` after provisioning.\nRequire this application to be served in an isolated browser for users matching this policy. 'Client Web Isolation' must be on for the account in order to use this feature."]
    pub fn isolation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolation_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings."]
    pub fn mfa_config(&self) -> DataZeroTrustAccessPolicy_MfaConfigRef {
        DataZeroTrustAccessPolicy_MfaConfigRef::new(
            self.shared().clone(),
            format!("{}.mfa_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Access policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe UUID of the policy"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
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
    pub fn require(&self) -> terrars::SetRef<DataZeroTrustAccessPolicy_RequireRef> {
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
pub struct DataZeroTrustAccessPolicy_ApprovalGroups {}

impl DataZeroTrustAccessPolicy_ApprovalGroups { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_ApprovalGroups {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_ApprovalGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_ApprovalGroups {}

impl BuildDataZeroTrustAccessPolicy_ApprovalGroups {
    pub fn build(self) -> DataZeroTrustAccessPolicy_ApprovalGroups {
        DataZeroTrustAccessPolicy_ApprovalGroups {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_ApprovalGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_ApprovalGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_ApprovalGroupsRef {
        DataZeroTrustAccessPolicy_ApprovalGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_ApprovalGroupsRef {
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
pub struct DataZeroTrustAccessPolicy_ConnectionRules_Rdp {}

impl DataZeroTrustAccessPolicy_ConnectionRules_Rdp { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_ConnectionRules_Rdp {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_ConnectionRules_Rdp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_ConnectionRules_Rdp {}

impl BuildDataZeroTrustAccessPolicy_ConnectionRules_Rdp {
    pub fn build(self) -> DataZeroTrustAccessPolicy_ConnectionRules_Rdp {
        DataZeroTrustAccessPolicy_ConnectionRules_Rdp {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_ConnectionRules_RdpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_ConnectionRules_RdpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_ConnectionRules_RdpRef {
        DataZeroTrustAccessPolicy_ConnectionRules_RdpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_ConnectionRules_RdpRef {
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
pub struct DataZeroTrustAccessPolicy_ConnectionRules {}

impl DataZeroTrustAccessPolicy_ConnectionRules { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_ConnectionRules {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_ConnectionRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_ConnectionRules {}

impl BuildDataZeroTrustAccessPolicy_ConnectionRules {
    pub fn build(self) -> DataZeroTrustAccessPolicy_ConnectionRules {
        DataZeroTrustAccessPolicy_ConnectionRules {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_ConnectionRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_ConnectionRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_ConnectionRulesRef {
        DataZeroTrustAccessPolicy_ConnectionRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_ConnectionRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rdp` after provisioning.\nThe RDP-specific rules that define clipboard behavior for RDP connections."]
    pub fn rdp(&self) -> DataZeroTrustAccessPolicy_ConnectionRules_RdpRef {
        DataZeroTrustAccessPolicy_ConnectionRules_RdpRef::new(self.shared().clone(), format!("{}.rdp", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {}

impl DataZeroTrustAccessPolicy_Exclude_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {
        DataZeroTrustAccessPolicy_Exclude_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Exclude_AuthContext {}

impl DataZeroTrustAccessPolicy_Exclude_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_AuthContext {}

impl BuildDataZeroTrustAccessPolicy_Exclude_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_AuthContext {
        DataZeroTrustAccessPolicy_Exclude_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_AuthContextRef {
        DataZeroTrustAccessPolicy_Exclude_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_AuthContextRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_AuthMethod {}

impl DataZeroTrustAccessPolicy_Exclude_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_AuthMethod {}

impl BuildDataZeroTrustAccessPolicy_Exclude_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_AuthMethod {
        DataZeroTrustAccessPolicy_Exclude_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_AuthMethodRef {
        DataZeroTrustAccessPolicy_Exclude_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_AuthMethodRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_AzureAd {}

impl DataZeroTrustAccessPolicy_Exclude_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_AzureAd {}

impl BuildDataZeroTrustAccessPolicy_Exclude_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_AzureAd {
        DataZeroTrustAccessPolicy_Exclude_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_AzureAdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_AzureAdRef {
        DataZeroTrustAccessPolicy_Exclude_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_AzureAdRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_Certificate {}

impl DataZeroTrustAccessPolicy_Exclude_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_Certificate {}

impl BuildDataZeroTrustAccessPolicy_Exclude_Certificate {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_Certificate {
        DataZeroTrustAccessPolicy_Exclude_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_CertificateRef {
        DataZeroTrustAccessPolicy_Exclude_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {}

impl DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMember { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {}

impl BuildDataZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {
        DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMember {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_CommonName {}

impl DataZeroTrustAccessPolicy_Exclude_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_CommonName {}

impl BuildDataZeroTrustAccessPolicy_Exclude_CommonName {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_CommonName {
        DataZeroTrustAccessPolicy_Exclude_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_CommonNameRef {
        DataZeroTrustAccessPolicy_Exclude_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_CommonNameRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_DevicePosture {}

impl DataZeroTrustAccessPolicy_Exclude_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_DevicePosture {}

impl BuildDataZeroTrustAccessPolicy_Exclude_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_DevicePosture {
        DataZeroTrustAccessPolicy_Exclude_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_DevicePostureRef {
        DataZeroTrustAccessPolicy_Exclude_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_DevicePostureRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_Email {}

impl DataZeroTrustAccessPolicy_Exclude_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_Email {}

impl BuildDataZeroTrustAccessPolicy_Exclude_Email {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_Email {
        DataZeroTrustAccessPolicy_Exclude_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_EmailRef {
        DataZeroTrustAccessPolicy_Exclude_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_EmailRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_EmailDomain {}

impl DataZeroTrustAccessPolicy_Exclude_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_EmailDomain {}

impl BuildDataZeroTrustAccessPolicy_Exclude_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_EmailDomain {
        DataZeroTrustAccessPolicy_Exclude_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_EmailDomainRef {
        DataZeroTrustAccessPolicy_Exclude_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_EmailDomainRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_EmailList {}

impl DataZeroTrustAccessPolicy_Exclude_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_EmailList {}

impl BuildDataZeroTrustAccessPolicy_Exclude_EmailList {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_EmailList {
        DataZeroTrustAccessPolicy_Exclude_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_EmailListRef {
        DataZeroTrustAccessPolicy_Exclude_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_EmailListRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_Everyone {}

impl DataZeroTrustAccessPolicy_Exclude_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_Everyone {}

impl BuildDataZeroTrustAccessPolicy_Exclude_Everyone {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_Everyone {
        DataZeroTrustAccessPolicy_Exclude_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_EveryoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_EveryoneRef {
        DataZeroTrustAccessPolicy_Exclude_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Exclude_ExternalEvaluation {}

impl DataZeroTrustAccessPolicy_Exclude_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_ExternalEvaluation {}

impl BuildDataZeroTrustAccessPolicy_Exclude_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_ExternalEvaluation {
        DataZeroTrustAccessPolicy_Exclude_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
        DataZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_Geo {}

impl DataZeroTrustAccessPolicy_Exclude_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_Geo {}

impl BuildDataZeroTrustAccessPolicy_Exclude_Geo {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_Geo {
        DataZeroTrustAccessPolicy_Exclude_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_GeoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_GeoRef {
        DataZeroTrustAccessPolicy_Exclude_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_GeoRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_GithubOrganization {}

impl DataZeroTrustAccessPolicy_Exclude_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_GithubOrganization {}

impl BuildDataZeroTrustAccessPolicy_Exclude_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_GithubOrganization {
        DataZeroTrustAccessPolicy_Exclude_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
        DataZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_Group {}

impl DataZeroTrustAccessPolicy_Exclude_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_Group {}

impl BuildDataZeroTrustAccessPolicy_Exclude_Group {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_Group {
        DataZeroTrustAccessPolicy_Exclude_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_GroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_GroupRef {
        DataZeroTrustAccessPolicy_Exclude_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_GroupRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_Gsuite {}

impl DataZeroTrustAccessPolicy_Exclude_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_Gsuite {}

impl BuildDataZeroTrustAccessPolicy_Exclude_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_Gsuite {
        DataZeroTrustAccessPolicy_Exclude_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_GsuiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_GsuiteRef {
        DataZeroTrustAccessPolicy_Exclude_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_GsuiteRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_Ip {}

impl DataZeroTrustAccessPolicy_Exclude_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_Ip {}

impl BuildDataZeroTrustAccessPolicy_Exclude_Ip {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_Ip {
        DataZeroTrustAccessPolicy_Exclude_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_IpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_IpRef {
        DataZeroTrustAccessPolicy_Exclude_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_IpRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_IpList {}

impl DataZeroTrustAccessPolicy_Exclude_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_IpList {}

impl BuildDataZeroTrustAccessPolicy_Exclude_IpList {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_IpList {
        DataZeroTrustAccessPolicy_Exclude_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_IpListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_IpListRef {
        DataZeroTrustAccessPolicy_Exclude_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_IpListRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_LinkedAppToken {}

impl DataZeroTrustAccessPolicy_Exclude_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_LinkedAppToken {}

impl BuildDataZeroTrustAccessPolicy_Exclude_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_LinkedAppToken {
        DataZeroTrustAccessPolicy_Exclude_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
        DataZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_LoginMethod {}

impl DataZeroTrustAccessPolicy_Exclude_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_LoginMethod {}

impl BuildDataZeroTrustAccessPolicy_Exclude_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_LoginMethod {
        DataZeroTrustAccessPolicy_Exclude_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_LoginMethodRef {
        DataZeroTrustAccessPolicy_Exclude_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_LoginMethodRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_Oidc {}

impl DataZeroTrustAccessPolicy_Exclude_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_Oidc {}

impl BuildDataZeroTrustAccessPolicy_Exclude_Oidc {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_Oidc {
        DataZeroTrustAccessPolicy_Exclude_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_OidcRef {
        DataZeroTrustAccessPolicy_Exclude_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_OidcRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_Okta {}

impl DataZeroTrustAccessPolicy_Exclude_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_Okta {}

impl BuildDataZeroTrustAccessPolicy_Exclude_Okta {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_Okta {
        DataZeroTrustAccessPolicy_Exclude_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_OktaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_OktaRef {
        DataZeroTrustAccessPolicy_Exclude_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_OktaRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_Saml {}

impl DataZeroTrustAccessPolicy_Exclude_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_Saml {}

impl BuildDataZeroTrustAccessPolicy_Exclude_Saml {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_Saml {
        DataZeroTrustAccessPolicy_Exclude_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_SamlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Exclude_SamlRef {
        DataZeroTrustAccessPolicy_Exclude_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_SamlRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_ServiceToken {}

impl DataZeroTrustAccessPolicy_Exclude_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_ServiceToken {}

impl BuildDataZeroTrustAccessPolicy_Exclude_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_ServiceToken {
        DataZeroTrustAccessPolicy_Exclude_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
        DataZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude_UserRiskScore {}

impl DataZeroTrustAccessPolicy_Exclude_UserRiskScore { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude_UserRiskScore {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude_UserRiskScore {}

impl BuildDataZeroTrustAccessPolicy_Exclude_UserRiskScore {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude_UserRiskScore {
        DataZeroTrustAccessPolicy_Exclude_UserRiskScore {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
        DataZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
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
pub struct DataZeroTrustAccessPolicy_Exclude {}

impl DataZeroTrustAccessPolicy_Exclude { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Exclude {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Exclude {}

impl BuildDataZeroTrustAccessPolicy_Exclude {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Exclude {
        DataZeroTrustAccessPolicy_Exclude {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_ExcludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_ExcludeRef {
        DataZeroTrustAccessPolicy_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicy_Exclude_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessPolicy_Exclude_AuthContextRef {
        DataZeroTrustAccessPolicy_Exclude_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessPolicy_Exclude_AuthMethodRef {
        DataZeroTrustAccessPolicy_Exclude_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessPolicy_Exclude_AzureAdRef {
        DataZeroTrustAccessPolicy_Exclude_AzureAdRef::new(self.shared().clone(), format!("{}.azure_ad", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessPolicy_Exclude_CertificateRef {
        DataZeroTrustAccessPolicy_Exclude_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicy_Exclude_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessPolicy_Exclude_CommonNameRef {
        DataZeroTrustAccessPolicy_Exclude_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessPolicy_Exclude_DevicePostureRef {
        DataZeroTrustAccessPolicy_Exclude_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessPolicy_Exclude_EmailRef {
        DataZeroTrustAccessPolicy_Exclude_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessPolicy_Exclude_EmailDomainRef {
        DataZeroTrustAccessPolicy_Exclude_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessPolicy_Exclude_EmailListRef {
        DataZeroTrustAccessPolicy_Exclude_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessPolicy_Exclude_EveryoneRef {
        DataZeroTrustAccessPolicy_Exclude_EveryoneRef::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef {
        DataZeroTrustAccessPolicy_Exclude_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessPolicy_Exclude_GeoRef {
        DataZeroTrustAccessPolicy_Exclude_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessPolicy_Exclude_GithubOrganizationRef {
        DataZeroTrustAccessPolicy_Exclude_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessPolicy_Exclude_GroupRef {
        DataZeroTrustAccessPolicy_Exclude_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessPolicy_Exclude_GsuiteRef {
        DataZeroTrustAccessPolicy_Exclude_GsuiteRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessPolicy_Exclude_IpRef {
        DataZeroTrustAccessPolicy_Exclude_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessPolicy_Exclude_IpListRef {
        DataZeroTrustAccessPolicy_Exclude_IpListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef {
        DataZeroTrustAccessPolicy_Exclude_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessPolicy_Exclude_LoginMethodRef {
        DataZeroTrustAccessPolicy_Exclude_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessPolicy_Exclude_OidcRef {
        DataZeroTrustAccessPolicy_Exclude_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessPolicy_Exclude_OktaRef {
        DataZeroTrustAccessPolicy_Exclude_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessPolicy_Exclude_SamlRef {
        DataZeroTrustAccessPolicy_Exclude_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessPolicy_Exclude_ServiceTokenRef {
        DataZeroTrustAccessPolicy_Exclude_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> DataZeroTrustAccessPolicy_Exclude_UserRiskScoreRef {
        DataZeroTrustAccessPolicy_Exclude_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Include_AnyValidServiceToken {}

impl DataZeroTrustAccessPolicy_Include_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessPolicy_Include_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_AnyValidServiceToken {
        DataZeroTrustAccessPolicy_Include_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Include_AuthContext {}

impl DataZeroTrustAccessPolicy_Include_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_AuthContext {}

impl BuildDataZeroTrustAccessPolicy_Include_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_AuthContext {
        DataZeroTrustAccessPolicy_Include_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_AuthContextRef {
        DataZeroTrustAccessPolicy_Include_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_AuthContextRef {
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
pub struct DataZeroTrustAccessPolicy_Include_AuthMethod {}

impl DataZeroTrustAccessPolicy_Include_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_AuthMethod {}

impl BuildDataZeroTrustAccessPolicy_Include_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_AuthMethod {
        DataZeroTrustAccessPolicy_Include_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_AuthMethodRef {
        DataZeroTrustAccessPolicy_Include_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_AuthMethodRef {
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
pub struct DataZeroTrustAccessPolicy_Include_AzureAd {}

impl DataZeroTrustAccessPolicy_Include_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_AzureAd {}

impl BuildDataZeroTrustAccessPolicy_Include_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_AzureAd {
        DataZeroTrustAccessPolicy_Include_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_AzureAdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_AzureAdRef {
        DataZeroTrustAccessPolicy_Include_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_AzureAdRef {
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
pub struct DataZeroTrustAccessPolicy_Include_Certificate {}

impl DataZeroTrustAccessPolicy_Include_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_Certificate {}

impl BuildDataZeroTrustAccessPolicy_Include_Certificate {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_Certificate {
        DataZeroTrustAccessPolicy_Include_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_CertificateRef {
        DataZeroTrustAccessPolicy_Include_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Include_CloudflareAccountMember {}

impl DataZeroTrustAccessPolicy_Include_CloudflareAccountMember { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_CloudflareAccountMember {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_CloudflareAccountMember {}

impl BuildDataZeroTrustAccessPolicy_Include_CloudflareAccountMember {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_CloudflareAccountMember {
        DataZeroTrustAccessPolicy_Include_CloudflareAccountMember {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
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
pub struct DataZeroTrustAccessPolicy_Include_CommonName {}

impl DataZeroTrustAccessPolicy_Include_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_CommonName {}

impl BuildDataZeroTrustAccessPolicy_Include_CommonName {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_CommonName {
        DataZeroTrustAccessPolicy_Include_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_CommonNameRef {
        DataZeroTrustAccessPolicy_Include_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_CommonNameRef {
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
pub struct DataZeroTrustAccessPolicy_Include_DevicePosture {}

impl DataZeroTrustAccessPolicy_Include_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_DevicePosture {}

impl BuildDataZeroTrustAccessPolicy_Include_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_DevicePosture {
        DataZeroTrustAccessPolicy_Include_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_DevicePostureRef {
        DataZeroTrustAccessPolicy_Include_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_DevicePostureRef {
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
pub struct DataZeroTrustAccessPolicy_Include_Email {}

impl DataZeroTrustAccessPolicy_Include_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_Email {}

impl BuildDataZeroTrustAccessPolicy_Include_Email {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_Email {
        DataZeroTrustAccessPolicy_Include_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_EmailRef {
        DataZeroTrustAccessPolicy_Include_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_EmailRef {
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
pub struct DataZeroTrustAccessPolicy_Include_EmailDomain {}

impl DataZeroTrustAccessPolicy_Include_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_EmailDomain {}

impl BuildDataZeroTrustAccessPolicy_Include_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_EmailDomain {
        DataZeroTrustAccessPolicy_Include_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_EmailDomainRef {
        DataZeroTrustAccessPolicy_Include_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_EmailDomainRef {
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
pub struct DataZeroTrustAccessPolicy_Include_EmailList {}

impl DataZeroTrustAccessPolicy_Include_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_EmailList {}

impl BuildDataZeroTrustAccessPolicy_Include_EmailList {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_EmailList {
        DataZeroTrustAccessPolicy_Include_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_EmailListRef {
        DataZeroTrustAccessPolicy_Include_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_EmailListRef {
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
pub struct DataZeroTrustAccessPolicy_Include_Everyone {}

impl DataZeroTrustAccessPolicy_Include_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_Everyone {}

impl BuildDataZeroTrustAccessPolicy_Include_Everyone {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_Everyone {
        DataZeroTrustAccessPolicy_Include_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_EveryoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_EveryoneRef {
        DataZeroTrustAccessPolicy_Include_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Include_ExternalEvaluation {}

impl DataZeroTrustAccessPolicy_Include_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_ExternalEvaluation {}

impl BuildDataZeroTrustAccessPolicy_Include_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_ExternalEvaluation {
        DataZeroTrustAccessPolicy_Include_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
        DataZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
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
pub struct DataZeroTrustAccessPolicy_Include_Geo {}

impl DataZeroTrustAccessPolicy_Include_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_Geo {}

impl BuildDataZeroTrustAccessPolicy_Include_Geo {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_Geo {
        DataZeroTrustAccessPolicy_Include_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_GeoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_GeoRef {
        DataZeroTrustAccessPolicy_Include_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_GeoRef {
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
pub struct DataZeroTrustAccessPolicy_Include_GithubOrganization {}

impl DataZeroTrustAccessPolicy_Include_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_GithubOrganization {}

impl BuildDataZeroTrustAccessPolicy_Include_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_GithubOrganization {
        DataZeroTrustAccessPolicy_Include_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_GithubOrganizationRef {
        DataZeroTrustAccessPolicy_Include_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_GithubOrganizationRef {
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
pub struct DataZeroTrustAccessPolicy_Include_Group {}

impl DataZeroTrustAccessPolicy_Include_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_Group {}

impl BuildDataZeroTrustAccessPolicy_Include_Group {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_Group {
        DataZeroTrustAccessPolicy_Include_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_GroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_GroupRef {
        DataZeroTrustAccessPolicy_Include_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_GroupRef {
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
pub struct DataZeroTrustAccessPolicy_Include_Gsuite {}

impl DataZeroTrustAccessPolicy_Include_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_Gsuite {}

impl BuildDataZeroTrustAccessPolicy_Include_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_Gsuite {
        DataZeroTrustAccessPolicy_Include_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_GsuiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_GsuiteRef {
        DataZeroTrustAccessPolicy_Include_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_GsuiteRef {
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
pub struct DataZeroTrustAccessPolicy_Include_Ip {}

impl DataZeroTrustAccessPolicy_Include_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_Ip {}

impl BuildDataZeroTrustAccessPolicy_Include_Ip {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_Ip {
        DataZeroTrustAccessPolicy_Include_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_IpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_IpRef {
        DataZeroTrustAccessPolicy_Include_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_IpRef {
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
pub struct DataZeroTrustAccessPolicy_Include_IpList {}

impl DataZeroTrustAccessPolicy_Include_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_IpList {}

impl BuildDataZeroTrustAccessPolicy_Include_IpList {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_IpList {
        DataZeroTrustAccessPolicy_Include_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_IpListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_IpListRef {
        DataZeroTrustAccessPolicy_Include_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_IpListRef {
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
pub struct DataZeroTrustAccessPolicy_Include_LinkedAppToken {}

impl DataZeroTrustAccessPolicy_Include_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_LinkedAppToken {}

impl BuildDataZeroTrustAccessPolicy_Include_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_LinkedAppToken {
        DataZeroTrustAccessPolicy_Include_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
        DataZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
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
pub struct DataZeroTrustAccessPolicy_Include_LoginMethod {}

impl DataZeroTrustAccessPolicy_Include_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_LoginMethod {}

impl BuildDataZeroTrustAccessPolicy_Include_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_LoginMethod {
        DataZeroTrustAccessPolicy_Include_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_LoginMethodRef {
        DataZeroTrustAccessPolicy_Include_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_LoginMethodRef {
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
pub struct DataZeroTrustAccessPolicy_Include_Oidc {}

impl DataZeroTrustAccessPolicy_Include_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_Oidc {}

impl BuildDataZeroTrustAccessPolicy_Include_Oidc {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_Oidc {
        DataZeroTrustAccessPolicy_Include_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_OidcRef {
        DataZeroTrustAccessPolicy_Include_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_OidcRef {
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
pub struct DataZeroTrustAccessPolicy_Include_Okta {}

impl DataZeroTrustAccessPolicy_Include_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_Okta {}

impl BuildDataZeroTrustAccessPolicy_Include_Okta {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_Okta {
        DataZeroTrustAccessPolicy_Include_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_OktaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_OktaRef {
        DataZeroTrustAccessPolicy_Include_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_OktaRef {
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
pub struct DataZeroTrustAccessPolicy_Include_Saml {}

impl DataZeroTrustAccessPolicy_Include_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_Saml {}

impl BuildDataZeroTrustAccessPolicy_Include_Saml {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_Saml {
        DataZeroTrustAccessPolicy_Include_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_SamlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Include_SamlRef {
        DataZeroTrustAccessPolicy_Include_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_SamlRef {
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
pub struct DataZeroTrustAccessPolicy_Include_ServiceToken {}

impl DataZeroTrustAccessPolicy_Include_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_ServiceToken {}

impl BuildDataZeroTrustAccessPolicy_Include_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_ServiceToken {
        DataZeroTrustAccessPolicy_Include_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_ServiceTokenRef {
        DataZeroTrustAccessPolicy_Include_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_ServiceTokenRef {
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
pub struct DataZeroTrustAccessPolicy_Include_UserRiskScore {}

impl DataZeroTrustAccessPolicy_Include_UserRiskScore { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include_UserRiskScore {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include_UserRiskScore {}

impl BuildDataZeroTrustAccessPolicy_Include_UserRiskScore {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include_UserRiskScore {
        DataZeroTrustAccessPolicy_Include_UserRiskScore {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Include_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Include_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Include_UserRiskScoreRef {
        DataZeroTrustAccessPolicy_Include_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Include_UserRiskScoreRef {
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
pub struct DataZeroTrustAccessPolicy_Include {}

impl DataZeroTrustAccessPolicy_Include { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Include {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Include {}

impl BuildDataZeroTrustAccessPolicy_Include {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Include {
        DataZeroTrustAccessPolicy_Include {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_IncludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_IncludeRef {
        DataZeroTrustAccessPolicy_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicy_Include_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessPolicy_Include_AuthContextRef {
        DataZeroTrustAccessPolicy_Include_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessPolicy_Include_AuthMethodRef {
        DataZeroTrustAccessPolicy_Include_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessPolicy_Include_AzureAdRef {
        DataZeroTrustAccessPolicy_Include_AzureAdRef::new(self.shared().clone(), format!("{}.azure_ad", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessPolicy_Include_CertificateRef {
        DataZeroTrustAccessPolicy_Include_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> DataZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicy_Include_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessPolicy_Include_CommonNameRef {
        DataZeroTrustAccessPolicy_Include_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessPolicy_Include_DevicePostureRef {
        DataZeroTrustAccessPolicy_Include_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessPolicy_Include_EmailRef {
        DataZeroTrustAccessPolicy_Include_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessPolicy_Include_EmailDomainRef {
        DataZeroTrustAccessPolicy_Include_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessPolicy_Include_EmailListRef {
        DataZeroTrustAccessPolicy_Include_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessPolicy_Include_EveryoneRef {
        DataZeroTrustAccessPolicy_Include_EveryoneRef::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessPolicy_Include_ExternalEvaluationRef {
        DataZeroTrustAccessPolicy_Include_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessPolicy_Include_GeoRef {
        DataZeroTrustAccessPolicy_Include_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessPolicy_Include_GithubOrganizationRef {
        DataZeroTrustAccessPolicy_Include_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessPolicy_Include_GroupRef {
        DataZeroTrustAccessPolicy_Include_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessPolicy_Include_GsuiteRef {
        DataZeroTrustAccessPolicy_Include_GsuiteRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessPolicy_Include_IpRef {
        DataZeroTrustAccessPolicy_Include_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessPolicy_Include_IpListRef {
        DataZeroTrustAccessPolicy_Include_IpListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessPolicy_Include_LinkedAppTokenRef {
        DataZeroTrustAccessPolicy_Include_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessPolicy_Include_LoginMethodRef {
        DataZeroTrustAccessPolicy_Include_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessPolicy_Include_OidcRef {
        DataZeroTrustAccessPolicy_Include_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessPolicy_Include_OktaRef {
        DataZeroTrustAccessPolicy_Include_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessPolicy_Include_SamlRef {
        DataZeroTrustAccessPolicy_Include_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessPolicy_Include_ServiceTokenRef {
        DataZeroTrustAccessPolicy_Include_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> DataZeroTrustAccessPolicy_Include_UserRiskScoreRef {
        DataZeroTrustAccessPolicy_Include_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_MfaConfig {}

impl DataZeroTrustAccessPolicy_MfaConfig { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_MfaConfig {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_MfaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_MfaConfig {}

impl BuildDataZeroTrustAccessPolicy_MfaConfig {
    pub fn build(self) -> DataZeroTrustAccessPolicy_MfaConfig {
        DataZeroTrustAccessPolicy_MfaConfig {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_MfaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_MfaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_MfaConfigRef {
        DataZeroTrustAccessPolicy_MfaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_MfaConfigRef {
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
pub struct DataZeroTrustAccessPolicy_Require_AnyValidServiceToken {}

impl DataZeroTrustAccessPolicy_Require_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessPolicy_Require_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_AnyValidServiceToken {
        DataZeroTrustAccessPolicy_Require_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Require_AuthContext {}

impl DataZeroTrustAccessPolicy_Require_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_AuthContext {}

impl BuildDataZeroTrustAccessPolicy_Require_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_AuthContext {
        DataZeroTrustAccessPolicy_Require_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_AuthContextRef {
        DataZeroTrustAccessPolicy_Require_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_AuthContextRef {
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
pub struct DataZeroTrustAccessPolicy_Require_AuthMethod {}

impl DataZeroTrustAccessPolicy_Require_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_AuthMethod {}

impl BuildDataZeroTrustAccessPolicy_Require_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_AuthMethod {
        DataZeroTrustAccessPolicy_Require_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_AuthMethodRef {
        DataZeroTrustAccessPolicy_Require_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_AuthMethodRef {
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
pub struct DataZeroTrustAccessPolicy_Require_AzureAd {}

impl DataZeroTrustAccessPolicy_Require_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_AzureAd {}

impl BuildDataZeroTrustAccessPolicy_Require_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_AzureAd {
        DataZeroTrustAccessPolicy_Require_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_AzureAdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_AzureAdRef {
        DataZeroTrustAccessPolicy_Require_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_AzureAdRef {
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
pub struct DataZeroTrustAccessPolicy_Require_Certificate {}

impl DataZeroTrustAccessPolicy_Require_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_Certificate {}

impl BuildDataZeroTrustAccessPolicy_Require_Certificate {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_Certificate {
        DataZeroTrustAccessPolicy_Require_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_CertificateRef {
        DataZeroTrustAccessPolicy_Require_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Require_CloudflareAccountMember {}

impl DataZeroTrustAccessPolicy_Require_CloudflareAccountMember { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_CloudflareAccountMember {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_CloudflareAccountMember {}

impl BuildDataZeroTrustAccessPolicy_Require_CloudflareAccountMember {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_CloudflareAccountMember {
        DataZeroTrustAccessPolicy_Require_CloudflareAccountMember {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
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
pub struct DataZeroTrustAccessPolicy_Require_CommonName {}

impl DataZeroTrustAccessPolicy_Require_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_CommonName {}

impl BuildDataZeroTrustAccessPolicy_Require_CommonName {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_CommonName {
        DataZeroTrustAccessPolicy_Require_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_CommonNameRef {
        DataZeroTrustAccessPolicy_Require_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_CommonNameRef {
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
pub struct DataZeroTrustAccessPolicy_Require_DevicePosture {}

impl DataZeroTrustAccessPolicy_Require_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_DevicePosture {}

impl BuildDataZeroTrustAccessPolicy_Require_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_DevicePosture {
        DataZeroTrustAccessPolicy_Require_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_DevicePostureRef {
        DataZeroTrustAccessPolicy_Require_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_DevicePostureRef {
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
pub struct DataZeroTrustAccessPolicy_Require_Email {}

impl DataZeroTrustAccessPolicy_Require_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_Email {}

impl BuildDataZeroTrustAccessPolicy_Require_Email {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_Email {
        DataZeroTrustAccessPolicy_Require_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_EmailRef {
        DataZeroTrustAccessPolicy_Require_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_EmailRef {
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
pub struct DataZeroTrustAccessPolicy_Require_EmailDomain {}

impl DataZeroTrustAccessPolicy_Require_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_EmailDomain {}

impl BuildDataZeroTrustAccessPolicy_Require_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_EmailDomain {
        DataZeroTrustAccessPolicy_Require_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_EmailDomainRef {
        DataZeroTrustAccessPolicy_Require_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_EmailDomainRef {
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
pub struct DataZeroTrustAccessPolicy_Require_EmailList {}

impl DataZeroTrustAccessPolicy_Require_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_EmailList {}

impl BuildDataZeroTrustAccessPolicy_Require_EmailList {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_EmailList {
        DataZeroTrustAccessPolicy_Require_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_EmailListRef {
        DataZeroTrustAccessPolicy_Require_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_EmailListRef {
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
pub struct DataZeroTrustAccessPolicy_Require_Everyone {}

impl DataZeroTrustAccessPolicy_Require_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_Everyone {}

impl BuildDataZeroTrustAccessPolicy_Require_Everyone {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_Everyone {
        DataZeroTrustAccessPolicy_Require_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_EveryoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_EveryoneRef {
        DataZeroTrustAccessPolicy_Require_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicy_Require_ExternalEvaluation {}

impl DataZeroTrustAccessPolicy_Require_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_ExternalEvaluation {}

impl BuildDataZeroTrustAccessPolicy_Require_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_ExternalEvaluation {
        DataZeroTrustAccessPolicy_Require_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
        DataZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
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
pub struct DataZeroTrustAccessPolicy_Require_Geo {}

impl DataZeroTrustAccessPolicy_Require_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_Geo {}

impl BuildDataZeroTrustAccessPolicy_Require_Geo {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_Geo {
        DataZeroTrustAccessPolicy_Require_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_GeoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_GeoRef {
        DataZeroTrustAccessPolicy_Require_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_GeoRef {
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
pub struct DataZeroTrustAccessPolicy_Require_GithubOrganization {}

impl DataZeroTrustAccessPolicy_Require_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_GithubOrganization {}

impl BuildDataZeroTrustAccessPolicy_Require_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_GithubOrganization {
        DataZeroTrustAccessPolicy_Require_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_GithubOrganizationRef {
        DataZeroTrustAccessPolicy_Require_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_GithubOrganizationRef {
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
pub struct DataZeroTrustAccessPolicy_Require_Group {}

impl DataZeroTrustAccessPolicy_Require_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_Group {}

impl BuildDataZeroTrustAccessPolicy_Require_Group {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_Group {
        DataZeroTrustAccessPolicy_Require_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_GroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_GroupRef {
        DataZeroTrustAccessPolicy_Require_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_GroupRef {
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
pub struct DataZeroTrustAccessPolicy_Require_Gsuite {}

impl DataZeroTrustAccessPolicy_Require_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_Gsuite {}

impl BuildDataZeroTrustAccessPolicy_Require_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_Gsuite {
        DataZeroTrustAccessPolicy_Require_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_GsuiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_GsuiteRef {
        DataZeroTrustAccessPolicy_Require_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_GsuiteRef {
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
pub struct DataZeroTrustAccessPolicy_Require_Ip {}

impl DataZeroTrustAccessPolicy_Require_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_Ip {}

impl BuildDataZeroTrustAccessPolicy_Require_Ip {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_Ip {
        DataZeroTrustAccessPolicy_Require_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_IpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_IpRef {
        DataZeroTrustAccessPolicy_Require_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_IpRef {
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
pub struct DataZeroTrustAccessPolicy_Require_IpList {}

impl DataZeroTrustAccessPolicy_Require_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_IpList {}

impl BuildDataZeroTrustAccessPolicy_Require_IpList {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_IpList {
        DataZeroTrustAccessPolicy_Require_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_IpListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_IpListRef {
        DataZeroTrustAccessPolicy_Require_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_IpListRef {
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
pub struct DataZeroTrustAccessPolicy_Require_LinkedAppToken {}

impl DataZeroTrustAccessPolicy_Require_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_LinkedAppToken {}

impl BuildDataZeroTrustAccessPolicy_Require_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_LinkedAppToken {
        DataZeroTrustAccessPolicy_Require_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
        DataZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
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
pub struct DataZeroTrustAccessPolicy_Require_LoginMethod {}

impl DataZeroTrustAccessPolicy_Require_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_LoginMethod {}

impl BuildDataZeroTrustAccessPolicy_Require_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_LoginMethod {
        DataZeroTrustAccessPolicy_Require_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_LoginMethodRef {
        DataZeroTrustAccessPolicy_Require_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_LoginMethodRef {
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
pub struct DataZeroTrustAccessPolicy_Require_Oidc {}

impl DataZeroTrustAccessPolicy_Require_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_Oidc {}

impl BuildDataZeroTrustAccessPolicy_Require_Oidc {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_Oidc {
        DataZeroTrustAccessPolicy_Require_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_OidcRef {
        DataZeroTrustAccessPolicy_Require_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_OidcRef {
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
pub struct DataZeroTrustAccessPolicy_Require_Okta {}

impl DataZeroTrustAccessPolicy_Require_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_Okta {}

impl BuildDataZeroTrustAccessPolicy_Require_Okta {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_Okta {
        DataZeroTrustAccessPolicy_Require_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_OktaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_OktaRef {
        DataZeroTrustAccessPolicy_Require_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_OktaRef {
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
pub struct DataZeroTrustAccessPolicy_Require_Saml {}

impl DataZeroTrustAccessPolicy_Require_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_Saml {}

impl BuildDataZeroTrustAccessPolicy_Require_Saml {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_Saml {
        DataZeroTrustAccessPolicy_Require_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_SamlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_Require_SamlRef {
        DataZeroTrustAccessPolicy_Require_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_SamlRef {
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
pub struct DataZeroTrustAccessPolicy_Require_ServiceToken {}

impl DataZeroTrustAccessPolicy_Require_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_ServiceToken {}

impl BuildDataZeroTrustAccessPolicy_Require_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_ServiceToken {
        DataZeroTrustAccessPolicy_Require_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_ServiceTokenRef {
        DataZeroTrustAccessPolicy_Require_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_ServiceTokenRef {
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
pub struct DataZeroTrustAccessPolicy_Require_UserRiskScore {}

impl DataZeroTrustAccessPolicy_Require_UserRiskScore { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require_UserRiskScore {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require_UserRiskScore {}

impl BuildDataZeroTrustAccessPolicy_Require_UserRiskScore {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require_UserRiskScore {
        DataZeroTrustAccessPolicy_Require_UserRiskScore {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_Require_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_Require_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicy_Require_UserRiskScoreRef {
        DataZeroTrustAccessPolicy_Require_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_Require_UserRiskScoreRef {
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
pub struct DataZeroTrustAccessPolicy_Require {}

impl DataZeroTrustAccessPolicy_Require { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicy_Require {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicy_Require>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicy_Require {}

impl BuildDataZeroTrustAccessPolicy_Require {
    pub fn build(self) -> DataZeroTrustAccessPolicy_Require {
        DataZeroTrustAccessPolicy_Require {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicy_RequireRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicy_RequireRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicy_RequireRef {
        DataZeroTrustAccessPolicy_RequireRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicy_RequireRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicy_Require_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessPolicy_Require_AuthContextRef {
        DataZeroTrustAccessPolicy_Require_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessPolicy_Require_AuthMethodRef {
        DataZeroTrustAccessPolicy_Require_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessPolicy_Require_AzureAdRef {
        DataZeroTrustAccessPolicy_Require_AzureAdRef::new(self.shared().clone(), format!("{}.azure_ad", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessPolicy_Require_CertificateRef {
        DataZeroTrustAccessPolicy_Require_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> DataZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicy_Require_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessPolicy_Require_CommonNameRef {
        DataZeroTrustAccessPolicy_Require_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessPolicy_Require_DevicePostureRef {
        DataZeroTrustAccessPolicy_Require_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessPolicy_Require_EmailRef {
        DataZeroTrustAccessPolicy_Require_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessPolicy_Require_EmailDomainRef {
        DataZeroTrustAccessPolicy_Require_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessPolicy_Require_EmailListRef {
        DataZeroTrustAccessPolicy_Require_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessPolicy_Require_EveryoneRef {
        DataZeroTrustAccessPolicy_Require_EveryoneRef::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessPolicy_Require_ExternalEvaluationRef {
        DataZeroTrustAccessPolicy_Require_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessPolicy_Require_GeoRef {
        DataZeroTrustAccessPolicy_Require_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessPolicy_Require_GithubOrganizationRef {
        DataZeroTrustAccessPolicy_Require_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessPolicy_Require_GroupRef {
        DataZeroTrustAccessPolicy_Require_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessPolicy_Require_GsuiteRef {
        DataZeroTrustAccessPolicy_Require_GsuiteRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessPolicy_Require_IpRef {
        DataZeroTrustAccessPolicy_Require_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessPolicy_Require_IpListRef {
        DataZeroTrustAccessPolicy_Require_IpListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessPolicy_Require_LinkedAppTokenRef {
        DataZeroTrustAccessPolicy_Require_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessPolicy_Require_LoginMethodRef {
        DataZeroTrustAccessPolicy_Require_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessPolicy_Require_OidcRef {
        DataZeroTrustAccessPolicy_Require_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessPolicy_Require_OktaRef {
        DataZeroTrustAccessPolicy_Require_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessPolicy_Require_SamlRef {
        DataZeroTrustAccessPolicy_Require_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessPolicy_Require_ServiceTokenRef {
        DataZeroTrustAccessPolicy_Require_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> DataZeroTrustAccessPolicy_Require_UserRiskScoreRef {
        DataZeroTrustAccessPolicy_Require_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}
