#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAccountTokensData_ {
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
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_expired: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
}

struct DataAccountTokens_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountTokensData_>,
}

#[derive(Clone)]
pub struct DataAccountTokens(std::rc::Rc<DataAccountTokens_>);

impl DataAccountTokens {
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

    #[doc = "Set the field `account_id`.\nAccount identifier tag."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_expired`.\nWhen true, includes recently-expired tokens in the response."]
    pub fn set_include_expired(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_expired = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_expired` after provisioning.\nWhen true, includes recently-expired tokens in the response."]
    pub fn include_expired(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_expired", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAccountTokens_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountTokens {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountTokens { }

impl terrars::ToListMappable for DataAccountTokens {
    type O = terrars::ListRef<DataAccountTokensRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountTokens_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_account_tokens".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountTokens {
    pub tf_id: std::string::String,
}

impl BuildDataAccountTokens {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountTokens {
        let out = DataAccountTokens(std::rc::Rc::new(DataAccountTokens_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountTokensData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                direction: core::default::Default::default(),
                include_expired: core::default::Default::default(),
                max_items: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountTokensRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountTokensRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountTokensRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include_expired` after provisioning.\nWhen true, includes recently-expired tokens in the response."]
    pub fn include_expired(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_expired", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAccountTokens_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountTokens_Result_Condition_RequestIp {}

impl DataAccountTokens_Result_Condition_RequestIp { }

impl terrars::ToListMappable for DataAccountTokens_Result_Condition_RequestIp {
    type O = terrars::BlockAssignable<DataAccountTokens_Result_Condition_RequestIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountTokens_Result_Condition_RequestIp {}

impl BuildDataAccountTokens_Result_Condition_RequestIp {
    pub fn build(self) -> DataAccountTokens_Result_Condition_RequestIp {
        DataAccountTokens_Result_Condition_RequestIp {}
    }
}

#[allow(dead_code)]
pub struct DataAccountTokens_Result_Condition_RequestIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountTokens_Result_Condition_RequestIpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountTokens_Result_Condition_RequestIpRef {
        DataAccountTokens_Result_Condition_RequestIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountTokens_Result_Condition_RequestIpRef {
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
pub struct DataAccountTokens_Result_Condition {}

impl DataAccountTokens_Result_Condition { }

impl terrars::ToListMappable for DataAccountTokens_Result_Condition {
    type O = terrars::BlockAssignable<DataAccountTokens_Result_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountTokens_Result_Condition {}

impl BuildDataAccountTokens_Result_Condition {
    pub fn build(self) -> DataAccountTokens_Result_Condition {
        DataAccountTokens_Result_Condition {}
    }
}

#[allow(dead_code)]
pub struct DataAccountTokens_Result_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountTokens_Result_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountTokens_Result_ConditionRef {
        DataAccountTokens_Result_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountTokens_Result_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_ip` after provisioning.\nClient IP restrictions."]
    pub fn request_ip(&self) -> DataAccountTokens_Result_Condition_RequestIpRef {
        DataAccountTokens_Result_Condition_RequestIpRef::new(
            self.shared().clone(),
            format!("{}.request_ip", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataAccountTokens_Result_Policies_PermissionGroups_Meta {}

impl DataAccountTokens_Result_Policies_PermissionGroups_Meta { }

impl terrars::ToListMappable for DataAccountTokens_Result_Policies_PermissionGroups_Meta {
    type O = terrars::BlockAssignable<DataAccountTokens_Result_Policies_PermissionGroups_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountTokens_Result_Policies_PermissionGroups_Meta {}

impl BuildDataAccountTokens_Result_Policies_PermissionGroups_Meta {
    pub fn build(self) -> DataAccountTokens_Result_Policies_PermissionGroups_Meta {
        DataAccountTokens_Result_Policies_PermissionGroups_Meta {}
    }
}

#[allow(dead_code)]
pub struct DataAccountTokens_Result_Policies_PermissionGroups_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountTokens_Result_Policies_PermissionGroups_MetaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountTokens_Result_Policies_PermissionGroups_MetaRef {
        DataAccountTokens_Result_Policies_PermissionGroups_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountTokens_Result_Policies_PermissionGroups_MetaRef {
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
pub struct DataAccountTokens_Result_Policies_PermissionGroups {}

impl DataAccountTokens_Result_Policies_PermissionGroups { }

impl terrars::ToListMappable for DataAccountTokens_Result_Policies_PermissionGroups {
    type O = terrars::BlockAssignable<DataAccountTokens_Result_Policies_PermissionGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountTokens_Result_Policies_PermissionGroups {}

impl BuildDataAccountTokens_Result_Policies_PermissionGroups {
    pub fn build(self) -> DataAccountTokens_Result_Policies_PermissionGroups {
        DataAccountTokens_Result_Policies_PermissionGroups {}
    }
}

#[allow(dead_code)]
pub struct DataAccountTokens_Result_Policies_PermissionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountTokens_Result_Policies_PermissionGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountTokens_Result_Policies_PermissionGroupsRef {
        DataAccountTokens_Result_Policies_PermissionGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountTokens_Result_Policies_PermissionGroupsRef {
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
    pub fn meta(&self) -> DataAccountTokens_Result_Policies_PermissionGroups_MetaRef {
        DataAccountTokens_Result_Policies_PermissionGroups_MetaRef::new(
            self.shared().clone(),
            format!("{}.meta", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the permission group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountTokens_Result_Policies {}

impl DataAccountTokens_Result_Policies { }

impl terrars::ToListMappable for DataAccountTokens_Result_Policies {
    type O = terrars::BlockAssignable<DataAccountTokens_Result_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountTokens_Result_Policies {}

impl BuildDataAccountTokens_Result_Policies {
    pub fn build(self) -> DataAccountTokens_Result_Policies {
        DataAccountTokens_Result_Policies {}
    }
}

#[allow(dead_code)]
pub struct DataAccountTokens_Result_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountTokens_Result_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountTokens_Result_PoliciesRef {
        DataAccountTokens_Result_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountTokens_Result_PoliciesRef {
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
    pub fn permission_groups(&self) -> terrars::ListRef<DataAccountTokens_Result_Policies_PermissionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resource names that the policy applies to."]
    pub fn resources(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountTokens_Result {}

impl DataAccountTokens_Result { }

impl terrars::ToListMappable for DataAccountTokens_Result {
    type O = terrars::BlockAssignable<DataAccountTokens_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountTokens_Result {}

impl BuildDataAccountTokens_Result {
    pub fn build(self) -> DataAccountTokens_Result {
        DataAccountTokens_Result {}
    }
}

#[allow(dead_code)]
pub struct DataAccountTokens_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountTokens_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountTokens_ResultRef {
        DataAccountTokens_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountTokens_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> DataAccountTokens_Result_ConditionRef {
        DataAccountTokens_Result_ConditionRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `creator_email_at_creation` after provisioning.\nThe email address of the user who created the token at the time of\ncreation. Only present for Account Owned API Tokens when a creator email\nwas available."]
    pub fn creator_email_at_creation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_email_at_creation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nThe expiration time on or after which the JWT MUST NOT be accepted for processing."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nToken identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issued_on` after provisioning.\nThe time on which the token was created."]
    pub fn issued_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issued_on", self.base))
    }

    #[doc = "Get a reference to the value of field `last_used_on` after provisioning.\nLast time the token was used."]
    pub fn last_used_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_used_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nLast time the token was modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nToken name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `not_before` after provisioning.\nThe time before which the token MUST NOT be accepted for processing."]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.base))
    }

    #[doc =
        "Get a reference to the value of field `policies` after provisioning.\nList of access policies assigned to the token."]
    pub fn policies(&self) -> terrars::ListRef<DataAccountTokens_Result_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioner_id` after provisioning.\nThe identifier of the service that provisioned the token. For an\nOAuth-provisioned token, this is the OAuth client identifier. Present\nwhen `provisioner_type` is present and null when the identifier is\nunavailable."]
    pub fn provisioner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioner_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioner_type` after provisioning.\nThe type of service that provisioned the token. Only present for\nprovisioned Account Owned API Tokens."]
    pub fn provisioner_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioner_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the token.\nAvailable values: \"active\", \"disabled\", \"expired\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
