#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataApiTokenData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataApiToken_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataApiToken_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataApiTokenData_>,
}

#[derive(Clone)]
pub struct DataApiToken(std::rc::Rc<DataApiToken_>);

impl DataApiToken {
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
    pub fn set_filter(self, v: impl std::convert::Into<DataApiToken_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_id`.\nToken identifier tag."]
    pub fn set_token_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().token_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> DataApiToken_ConditionRef {
        DataApiToken_ConditionRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataApiToken_FilterRef {
        DataApiToken_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
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
    pub fn policies(&self) -> terrars::ListRef<DataApiToken_PoliciesRef> {
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

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\nToken identifier tag."]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataApiToken {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataApiToken { }

impl terrars::ToListMappable for DataApiToken {
    type O = terrars::ListRef<DataApiTokenRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataApiToken_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_api_token".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApiToken {
    pub tf_id: std::string::String,
}

impl BuildDataApiToken {
    pub fn build(self, stack: &mut terrars::Stack) -> DataApiToken {
        let out = DataApiToken(std::rc::Rc::new(DataApiToken_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataApiTokenData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                token_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataApiTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiTokenRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataApiTokenRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> DataApiToken_ConditionRef {
        DataApiToken_ConditionRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataApiToken_FilterRef {
        DataApiToken_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
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
    pub fn policies(&self) -> terrars::ListRef<DataApiToken_PoliciesRef> {
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

    #[doc = "Get a reference to the value of field `token_id` after provisioning.\nToken identifier tag."]
    pub fn token_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataApiToken_Condition_RequestIp {}

impl DataApiToken_Condition_RequestIp { }

impl terrars::ToListMappable for DataApiToken_Condition_RequestIp {
    type O = terrars::BlockAssignable<DataApiToken_Condition_RequestIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiToken_Condition_RequestIp {}

impl BuildDataApiToken_Condition_RequestIp {
    pub fn build(self) -> DataApiToken_Condition_RequestIp {
        DataApiToken_Condition_RequestIp {}
    }
}

#[allow(dead_code)]
pub struct DataApiToken_Condition_RequestIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiToken_Condition_RequestIpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApiToken_Condition_RequestIpRef {
        DataApiToken_Condition_RequestIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiToken_Condition_RequestIpRef {
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
pub struct DataApiToken_Condition {}

impl DataApiToken_Condition { }

impl terrars::ToListMappable for DataApiToken_Condition {
    type O = terrars::BlockAssignable<DataApiToken_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiToken_Condition {}

impl BuildDataApiToken_Condition {
    pub fn build(self) -> DataApiToken_Condition {
        DataApiToken_Condition {}
    }
}

#[allow(dead_code)]
pub struct DataApiToken_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiToken_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApiToken_ConditionRef {
        DataApiToken_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiToken_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_ip` after provisioning.\nClient IP restrictions."]
    pub fn request_ip(&self) -> DataApiToken_Condition_RequestIpRef {
        DataApiToken_Condition_RequestIpRef::new(self.shared().clone(), format!("{}.request_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiToken_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_expired: std::option::Option<terrars::PrimField<bool>>,
}

impl DataApiToken_Filter {
    #[doc = "Set the field `direction`.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_expired`.\nWhen true, includes recently-expired tokens in the response."]
    pub fn set_include_expired(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_expired = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApiToken_Filter {
    type O = terrars::BlockAssignable<DataApiToken_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiToken_Filter {}

impl BuildDataApiToken_Filter {
    pub fn build(self) -> DataApiToken_Filter {
        DataApiToken_Filter {
            direction: core::default::Default::default(),
            include_expired: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApiToken_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiToken_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApiToken_FilterRef {
        DataApiToken_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiToken_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_expired` after provisioning.\nWhen true, includes recently-expired tokens in the response."]
    pub fn include_expired(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_expired", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiToken_Policies_PermissionGroups_Meta {}

impl DataApiToken_Policies_PermissionGroups_Meta { }

impl terrars::ToListMappable for DataApiToken_Policies_PermissionGroups_Meta {
    type O = terrars::BlockAssignable<DataApiToken_Policies_PermissionGroups_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiToken_Policies_PermissionGroups_Meta {}

impl BuildDataApiToken_Policies_PermissionGroups_Meta {
    pub fn build(self) -> DataApiToken_Policies_PermissionGroups_Meta {
        DataApiToken_Policies_PermissionGroups_Meta {}
    }
}

#[allow(dead_code)]
pub struct DataApiToken_Policies_PermissionGroups_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiToken_Policies_PermissionGroups_MetaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiToken_Policies_PermissionGroups_MetaRef {
        DataApiToken_Policies_PermissionGroups_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiToken_Policies_PermissionGroups_MetaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `category` after provisioning.\nA category used to group permission groups."]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deprecated` after provisioning.\nIndicates whether the permission group is deprecated."]
    pub fn deprecated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAdditional information about the permission group."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `editable` after provisioning.\nIndicates whether the permission group can be edited."]
    pub fn editable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.editable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `eol_at` after provisioning.\nThe planned end-of-life date and time, when provided."]
    pub fn eol_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eol_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nA label identifying the permission group."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nThe scope associated with the permission group."]
    pub fn scopes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `visibility` after provisioning.\nIndicates the permission group's availability or visibility."]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiToken_Policies_PermissionGroups {}

impl DataApiToken_Policies_PermissionGroups { }

impl terrars::ToListMappable for DataApiToken_Policies_PermissionGroups {
    type O = terrars::BlockAssignable<DataApiToken_Policies_PermissionGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiToken_Policies_PermissionGroups {}

impl BuildDataApiToken_Policies_PermissionGroups {
    pub fn build(self) -> DataApiToken_Policies_PermissionGroups {
        DataApiToken_Policies_PermissionGroups {}
    }
}

#[allow(dead_code)]
pub struct DataApiToken_Policies_PermissionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiToken_Policies_PermissionGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApiToken_Policies_PermissionGroupsRef {
        DataApiToken_Policies_PermissionGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiToken_Policies_PermissionGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the permission group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nAttributes associated to the permission group."]
    pub fn meta(&self) -> DataApiToken_Policies_PermissionGroups_MetaRef {
        DataApiToken_Policies_PermissionGroups_MetaRef::new(self.shared().clone(), format!("{}.meta", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the permission group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiToken_Policies {}

impl DataApiToken_Policies { }

impl terrars::ToListMappable for DataApiToken_Policies {
    type O = terrars::BlockAssignable<DataApiToken_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiToken_Policies {}

impl BuildDataApiToken_Policies {
    pub fn build(self) -> DataApiToken_Policies {
        DataApiToken_Policies {}
    }
}

#[allow(dead_code)]
pub struct DataApiToken_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiToken_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApiToken_PoliciesRef {
        DataApiToken_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiToken_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effect` after provisioning.\nAllow or deny operations against the resources.\nAvailable values: \"allow\", \"deny\"."]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nPolicy identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission_groups` after provisioning.\nA set of permission groups that are specified to the policy."]
    pub fn permission_groups(&self) -> terrars::ListRef<DataApiToken_Policies_PermissionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resource names that the policy applies to."]
    pub fn resources(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}
