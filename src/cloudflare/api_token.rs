#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ApiTokenData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<ApiToken_Condition>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expires_on: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_before: std::option::Option<terrars::PrimField<std::string::String>>,
    policies: std::vec::Vec<ApiToken_Policies>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ApiToken_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApiTokenData_>,
}

#[derive(Clone)]
pub struct ApiToken(std::rc::Rc<ApiToken_>);

impl ApiToken {
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

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(self, v: impl std::convert::Into<ApiToken_Condition>) -> Self {
        self.0.data.borrow_mut().condition = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expires_on`.\nThe expiration time on or after which the JWT MUST NOT be accepted for processing."]
    pub fn set_expires_on(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expires_on = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_before`.\nThe time before which the token MUST NOT be accepted for processing."]
    pub fn set_not_before(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().not_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\nStatus of the token.\nAvailable values: \"active\", \"disabled\", \"expired\"."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ApiToken_ConditionRef {
        ApiToken_ConditionRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator_email_at_creation` after provisioning.\nThe email address of the user who created the token at the time of\ncreation. Only present for Account Owned API Tokens when a creator email\nwas available."]
    pub fn creator_email_at_creation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_email_at_creation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nThe expiration time on or after which the JWT MUST NOT be accepted for processing."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nToken identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issued_on` after provisioning.\nThe time on which the token was created."]
    pub fn issued_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issued_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_used_on` after provisioning.\nLast time the token was used."]
    pub fn last_used_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_used_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nLast time the token was modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nToken name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `not_before` after provisioning.\nThe time before which the token MUST NOT be accepted for processing."]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policies` after provisioning.\nList of access policies assigned to the token."]
    pub fn policies(&self) -> terrars::ListRef<ApiToken_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioner_id` after provisioning.\nThe identifier of the service that provisioned the token. For an\nOAuth-provisioned token, this is the OAuth client identifier. Present\nwhen `provisioner_type` is present and null when the identifier is\nunavailable."]
    pub fn provisioner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioner_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioner_type` after provisioning.\nThe type of service that provisioned the token. Only present for\nprovisioned Account Owned API Tokens."]
    pub fn provisioner_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioner_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the token.\nAvailable values: \"active\", \"disabled\", \"expired\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe token value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

impl terrars::Referable for ApiToken {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApiToken { }

impl terrars::ToListMappable for ApiToken {
    type O = terrars::ListRef<ApiTokenRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApiToken_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_api_token".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiToken {
    pub tf_id: std::string::String,
    #[doc = "Token name."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "List of access policies assigned to the token."]
    pub policies: std::vec::Vec<ApiToken_Policies>,
}

impl BuildApiToken {
    pub fn build(self, stack: &mut terrars::Stack) -> ApiToken {
        let out = ApiToken(std::rc::Rc::new(ApiToken_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApiTokenData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                condition: core::default::Default::default(),
                expires_on: core::default::Default::default(),
                name: self.name,
                not_before: core::default::Default::default(),
                policies: self.policies,
                status: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiTokenRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiTokenRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ApiToken_ConditionRef {
        ApiToken_ConditionRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator_email_at_creation` after provisioning.\nThe email address of the user who created the token at the time of\ncreation. Only present for Account Owned API Tokens when a creator email\nwas available."]
    pub fn creator_email_at_creation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_email_at_creation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nThe expiration time on or after which the JWT MUST NOT be accepted for processing."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nToken identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issued_on` after provisioning.\nThe time on which the token was created."]
    pub fn issued_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issued_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_used_on` after provisioning.\nLast time the token was used."]
    pub fn last_used_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_used_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nLast time the token was modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nToken name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `not_before` after provisioning.\nThe time before which the token MUST NOT be accepted for processing."]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policies` after provisioning.\nList of access policies assigned to the token."]
    pub fn policies(&self) -> terrars::ListRef<ApiToken_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioner_id` after provisioning.\nThe identifier of the service that provisioned the token. For an\nOAuth-provisioned token, this is the OAuth client identifier. Present\nwhen `provisioner_type` is present and null when the identifier is\nunavailable."]
    pub fn provisioner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioner_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioner_type` after provisioning.\nThe type of service that provisioned the token. Only present for\nprovisioned Account Owned API Tokens."]
    pub fn provisioner_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioner_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the token.\nAvailable values: \"active\", \"disabled\", \"expired\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe token value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiToken_Condition_RequestIp {
    #[serde(rename = "in", skip_serializing_if = "std::option::Option::is_none")]
    in_: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_in: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ApiToken_Condition_RequestIp {
    #[doc = "Set the field `in_`.\nList of IPv4/IPv6 CIDR addresses."]
    pub fn set_in(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.in_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_in`.\nList of IPv4/IPv6 CIDR addresses."]
    pub fn set_not_in(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_in = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApiToken_Condition_RequestIp {
    type O = terrars::BlockAssignable<ApiToken_Condition_RequestIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiToken_Condition_RequestIp {}

impl BuildApiToken_Condition_RequestIp {
    pub fn build(self) -> ApiToken_Condition_RequestIp {
        ApiToken_Condition_RequestIp {
            in_: core::default::Default::default(),
            not_in: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApiToken_Condition_RequestIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiToken_Condition_RequestIpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiToken_Condition_RequestIpRef {
        ApiToken_Condition_RequestIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiToken_Condition_RequestIpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `in_` after provisioning.\nList of IPv4/IPv6 CIDR addresses."]
    pub fn in_(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.in", self.base))
    }

    #[doc = "Get a reference to the value of field `not_in` after provisioning.\nList of IPv4/IPv6 CIDR addresses."]
    pub fn not_in(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_in", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiToken_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_ip: std::option::Option<ApiToken_Condition_RequestIp>,
}

impl ApiToken_Condition {
    #[doc = "Set the field `request_ip`.\nClient IP restrictions."]
    pub fn set_request_ip(mut self, v: impl std::convert::Into<ApiToken_Condition_RequestIp>) -> Self {
        self.request_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApiToken_Condition {
    type O = terrars::BlockAssignable<ApiToken_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiToken_Condition {}

impl BuildApiToken_Condition {
    pub fn build(self) -> ApiToken_Condition {
        ApiToken_Condition { request_ip: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApiToken_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiToken_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiToken_ConditionRef {
        ApiToken_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiToken_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_ip` after provisioning.\nClient IP restrictions."]
    pub fn request_ip(&self) -> ApiToken_Condition_RequestIpRef {
        ApiToken_Condition_RequestIpRef::new(self.shared().clone(), format!("{}.request_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiToken_Policies_PermissionGroups {
    id: terrars::PrimField<std::string::String>,
}

impl ApiToken_Policies_PermissionGroups { }

impl terrars::ToListMappable for ApiToken_Policies_PermissionGroups {
    type O = terrars::BlockAssignable<ApiToken_Policies_PermissionGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiToken_Policies_PermissionGroups {
    #[doc = "Identifier of the permission group."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildApiToken_Policies_PermissionGroups {
    pub fn build(self) -> ApiToken_Policies_PermissionGroups {
        ApiToken_Policies_PermissionGroups { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ApiToken_Policies_PermissionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiToken_Policies_PermissionGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiToken_Policies_PermissionGroupsRef {
        ApiToken_Policies_PermissionGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiToken_Policies_PermissionGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the permission group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiToken_Policies {
    effect: terrars::PrimField<std::string::String>,
    permission_groups: std::vec::Vec<ApiToken_Policies_PermissionGroups>,
    resources: terrars::PrimField<std::string::String>,
}

impl ApiToken_Policies { }

impl terrars::ToListMappable for ApiToken_Policies {
    type O = terrars::BlockAssignable<ApiToken_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiToken_Policies {
    #[doc = "Allow or deny operations against the resources.\nAvailable values: \"allow\", \"deny\"."]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = "A set of permission groups that are specified to the policy."]
    pub permission_groups: std::vec::Vec<ApiToken_Policies_PermissionGroups>,
    #[doc = "A json object representing the resources that are specified to the policy."]
    pub resources: terrars::PrimField<std::string::String>,
}

impl BuildApiToken_Policies {
    pub fn build(self) -> ApiToken_Policies {
        ApiToken_Policies {
            effect: self.effect,
            permission_groups: self.permission_groups,
            resources: self.resources,
        }
    }
}

#[allow(dead_code)]
pub struct ApiToken_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiToken_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiToken_PoliciesRef {
        ApiToken_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiToken_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effect` after provisioning.\nAllow or deny operations against the resources.\nAvailable values: \"allow\", \"deny\"."]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission_groups` after provisioning.\nA set of permission groups that are specified to the policy."]
    pub fn permission_groups(&self) -> terrars::ListRef<ApiToken_Policies_PermissionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA json object representing the resources that are specified to the policy."]
    pub fn resources(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}
