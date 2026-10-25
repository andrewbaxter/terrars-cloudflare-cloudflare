#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustAccessGroupData_ {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<std::vec::Vec<ZeroTrustAccessGroup_Exclude>>,
    include: std::vec::Vec<ZeroTrustAccessGroup_Include>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_default: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require: std::option::Option<std::vec::Vec<ZeroTrustAccessGroup_Require>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ZeroTrustAccessGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustAccessGroupData_>,
}

#[derive(Clone)]
pub struct ZeroTrustAccessGroup(std::rc::Rc<ZeroTrustAccessGroup_>);

impl ZeroTrustAccessGroup {
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

    #[doc =
        "Set the field `account_id`.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclude`.\nRules evaluated with a NOT logical operator. To match a policy, a user cannot meet any of the Exclude rules."]
    pub fn set_exclude(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessGroup_Exclude>>) -> Self {
        self.0.data.borrow_mut().exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_default`.\nWhether this is the default group"]
    pub fn set_is_default(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_default = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require`.\nRules evaluated with an AND logical operator. To match a policy, a user must meet all of the Require rules."]
    pub fn set_require(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessGroup_Require>>) -> Self {
        self.0.data.borrow_mut().require = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nRules evaluated with a NOT logical operator. To match a policy, a user cannot meet any of the Exclude rules."]
    pub fn exclude(&self) -> terrars::ListRef<ZeroTrustAccessGroup_ExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nRules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub fn include(&self) -> terrars::ListRef<ZeroTrustAccessGroup_IncludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_default` after provisioning.\nWhether this is the default group"]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Access group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require` after provisioning.\nRules evaluated with an AND logical operator. To match a policy, a user must meet all of the Require rules."]
    pub fn require(&self) -> terrars::ListRef<ZeroTrustAccessGroup_RequireRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.require", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustAccessGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustAccessGroup { }

impl terrars::ToListMappable for ZeroTrustAccessGroup {
    type O = terrars::ListRef<ZeroTrustAccessGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustAccessGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustAccessGroup {
    pub tf_id: std::string::String,
    #[doc = "Rules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub include: std::vec::Vec<ZeroTrustAccessGroup_Include>,
    #[doc = "The name of the Access group."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustAccessGroup {
        let out = ZeroTrustAccessGroup(std::rc::Rc::new(ZeroTrustAccessGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustAccessGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                exclude: core::default::Default::default(),
                include: self.include,
                is_default: core::default::Default::default(),
                name: self.name,
                require: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustAccessGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustAccessGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nRules evaluated with a NOT logical operator. To match a policy, a user cannot meet any of the Exclude rules."]
    pub fn exclude(&self) -> terrars::ListRef<ZeroTrustAccessGroup_ExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nRules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub fn include(&self) -> terrars::ListRef<ZeroTrustAccessGroup_IncludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_default` after provisioning.\nWhether this is the default group"]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Access group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require` after provisioning.\nRules evaluated with an AND logical operator. To match a policy, a user must meet all of the Require rules."]
    pub fn require(&self) -> terrars::ListRef<ZeroTrustAccessGroup_RequireRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.require", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessGroup_Exclude_AnyValidServiceToken {}

impl ZeroTrustAccessGroup_Exclude_AnyValidServiceToken { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_AnyValidServiceToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_AnyValidServiceToken {}

impl BuildZeroTrustAccessGroup_Exclude_AnyValidServiceToken {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_AnyValidServiceToken {
        ZeroTrustAccessGroup_Exclude_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef {
        ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessGroup_Exclude_AuthContext {
    ac_id: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_AuthContext { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_AuthContext {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_AuthContext {
    #[doc = "The ACID of an Authentication context."]
    pub ac_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of an Authentication context."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Azure identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_AuthContext {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_AuthContext {
        ZeroTrustAccessGroup_Exclude_AuthContext {
            ac_id: self.ac_id,
            id: self.id,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_AuthContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_AuthContextRef {
        ZeroTrustAccessGroup_Exclude_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_AuthContextRef {
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
pub struct ZeroTrustAccessGroup_Exclude_AuthMethod {
    auth_method: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_AuthMethod { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_AuthMethod {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_AuthMethod {
    #[doc = "The type of authentication method https://datatracker.ietf.org/doc/html/rfc8176#section-2."]
    pub auth_method: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_AuthMethod {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_AuthMethod {
        ZeroTrustAccessGroup_Exclude_AuthMethod { auth_method: self.auth_method }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_AuthMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_AuthMethodRef {
        ZeroTrustAccessGroup_Exclude_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_AuthMethodRef {
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
pub struct ZeroTrustAccessGroup_Exclude_AzureAd {
    id: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_AzureAd { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_AzureAd {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_AzureAd {
    #[doc = "The ID of an Azure group."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Azure identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_AzureAd {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_AzureAd {
        ZeroTrustAccessGroup_Exclude_AzureAd {
            id: self.id,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_AzureAdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_AzureAdRef {
        ZeroTrustAccessGroup_Exclude_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_AzureAdRef {
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
pub struct ZeroTrustAccessGroup_Exclude_Certificate {}

impl ZeroTrustAccessGroup_Exclude_Certificate { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_Certificate {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_Certificate {}

impl BuildZeroTrustAccessGroup_Exclude_Certificate {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_Certificate {
        ZeroTrustAccessGroup_Exclude_Certificate {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_CertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_CertificateRef {
        ZeroTrustAccessGroup_Exclude_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessGroup_Exclude_CloudflareAccountMember {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessGroup_Exclude_CloudflareAccountMember {
    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_CloudflareAccountMember {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_CloudflareAccountMember {}

impl BuildZeroTrustAccessGroup_Exclude_CloudflareAccountMember {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_CloudflareAccountMember {
        ZeroTrustAccessGroup_Exclude_CloudflareAccountMember { account_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef {
        ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef {
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
pub struct ZeroTrustAccessGroup_Exclude_CommonName {
    common_name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_CommonName { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_CommonName {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_CommonName {
    #[doc = "The common name to match."]
    pub common_name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_CommonName {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_CommonName {
        ZeroTrustAccessGroup_Exclude_CommonName { common_name: self.common_name }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_CommonNameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_CommonNameRef {
        ZeroTrustAccessGroup_Exclude_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_CommonNameRef {
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
pub struct ZeroTrustAccessGroup_Exclude_DevicePosture {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    integration_uid: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_DevicePosture {
    #[doc = "Set the field `account_id`.\nThe ID of the account that owns the device posture integration."]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_DevicePosture {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_DevicePosture {
    #[doc = "The ID of a device posture integration."]
    pub integration_uid: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_DevicePosture {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_DevicePosture {
        ZeroTrustAccessGroup_Exclude_DevicePosture {
            account_id: core::default::Default::default(),
            integration_uid: self.integration_uid,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_DevicePostureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_DevicePostureRef {
        ZeroTrustAccessGroup_Exclude_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_DevicePostureRef {
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
pub struct ZeroTrustAccessGroup_Exclude_Email {
    email: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_Email { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_Email {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_Email {
    #[doc = "The email of the user."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_Email {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_Email {
        ZeroTrustAccessGroup_Exclude_Email { email: self.email }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_EmailRef {
        ZeroTrustAccessGroup_Exclude_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_EmailRef {
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
pub struct ZeroTrustAccessGroup_Exclude_EmailDomain {
    domain: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_EmailDomain { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_EmailDomain {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_EmailDomain {
    #[doc = "The email domain to match."]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_EmailDomain {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_EmailDomain {
        ZeroTrustAccessGroup_Exclude_EmailDomain { domain: self.domain }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_EmailDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_EmailDomainRef {
        ZeroTrustAccessGroup_Exclude_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_EmailDomainRef {
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
pub struct ZeroTrustAccessGroup_Exclude_EmailList {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_EmailList { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_EmailList {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_EmailList {
    #[doc = "The ID of a previously created email list."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_EmailList {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_EmailList {
        ZeroTrustAccessGroup_Exclude_EmailList { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_EmailListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_EmailListRef {
        ZeroTrustAccessGroup_Exclude_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_EmailListRef {
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
pub struct ZeroTrustAccessGroup_Exclude_Everyone {}

impl ZeroTrustAccessGroup_Exclude_Everyone { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_Everyone {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_Everyone {}

impl BuildZeroTrustAccessGroup_Exclude_Everyone {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_Everyone {
        ZeroTrustAccessGroup_Exclude_Everyone {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_EveryoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_EveryoneRef {
        ZeroTrustAccessGroup_Exclude_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessGroup_Exclude_ExternalEvaluation {
    evaluate_url: terrars::PrimField<std::string::String>,
    keys_url: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_ExternalEvaluation { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_ExternalEvaluation {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_ExternalEvaluation {
    #[doc = "The API endpoint containing your business logic."]
    pub evaluate_url: terrars::PrimField<std::string::String>,
    #[doc = "The API endpoint containing the key that Access uses to verify that the response came from your API."]
    pub keys_url: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_ExternalEvaluation {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_ExternalEvaluation {
        ZeroTrustAccessGroup_Exclude_ExternalEvaluation {
            evaluate_url: self.evaluate_url,
            keys_url: self.keys_url,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef {
        ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef {
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
pub struct ZeroTrustAccessGroup_Exclude_Geo {
    country_code: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_Geo { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_Geo {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_Geo {
    #[doc = "The country code that should be matched."]
    pub country_code: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_Geo {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_Geo {
        ZeroTrustAccessGroup_Exclude_Geo { country_code: self.country_code }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_GeoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_GeoRef {
        ZeroTrustAccessGroup_Exclude_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_GeoRef {
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
pub struct ZeroTrustAccessGroup_Exclude_GithubOrganization {
    identity_provider_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    team: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessGroup_Exclude_GithubOrganization {
    #[doc = "Set the field `team`.\nThe name of the team"]
    pub fn set_team(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.team = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_GithubOrganization {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_GithubOrganization {
    #[doc = "The ID of your Github identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the organization."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_GithubOrganization {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_GithubOrganization {
        ZeroTrustAccessGroup_Exclude_GithubOrganization {
            identity_provider_id: self.identity_provider_id,
            name: self.name,
            team: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessGroup_Exclude_GithubOrganizationRef {
        ZeroTrustAccessGroup_Exclude_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_GithubOrganizationRef {
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
pub struct ZeroTrustAccessGroup_Exclude_Group {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_Group { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_Group {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_Group {
    #[doc = "The ID of a previously created Access group."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_Group {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_Group {
        ZeroTrustAccessGroup_Exclude_Group { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_GroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_GroupRef {
        ZeroTrustAccessGroup_Exclude_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_GroupRef {
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
pub struct ZeroTrustAccessGroup_Exclude_Gsuite {
    email: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_Gsuite { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_Gsuite {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_Gsuite {
    #[doc = "The email of the Google Workspace group."]
    pub email: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your Google Workspace identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_Gsuite {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_Gsuite {
        ZeroTrustAccessGroup_Exclude_Gsuite {
            email: self.email,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_GsuiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_GsuiteRef {
        ZeroTrustAccessGroup_Exclude_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_GsuiteRef {
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
pub struct ZeroTrustAccessGroup_Exclude_Ip {
    ip: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_Ip { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_Ip {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_Ip {
    #[doc = "An IPv4 or IPv6 CIDR block."]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_Ip {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_Ip {
        ZeroTrustAccessGroup_Exclude_Ip { ip: self.ip }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_IpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_IpRef {
        ZeroTrustAccessGroup_Exclude_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_IpRef {
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
pub struct ZeroTrustAccessGroup_Exclude_IpList {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_IpList { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_IpList {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_IpList {
    #[doc = "The ID of a previously created IP list."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_IpList {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_IpList {
        ZeroTrustAccessGroup_Exclude_IpList { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_IpListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_IpListRef {
        ZeroTrustAccessGroup_Exclude_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_IpListRef {
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
pub struct ZeroTrustAccessGroup_Exclude_LinkedAppToken {
    app_uid: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_LinkedAppToken { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_LinkedAppToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_LinkedAppToken {
    #[doc = "The ID of an Access OIDC SaaS application"]
    pub app_uid: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_LinkedAppToken {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_LinkedAppToken {
        ZeroTrustAccessGroup_Exclude_LinkedAppToken { app_uid: self.app_uid }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef {
        ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef {
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
pub struct ZeroTrustAccessGroup_Exclude_LoginMethod {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_LoginMethod { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_LoginMethod {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_LoginMethod {
    #[doc = "The ID of an identity provider."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_LoginMethod {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_LoginMethod {
        ZeroTrustAccessGroup_Exclude_LoginMethod { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_LoginMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_LoginMethodRef {
        ZeroTrustAccessGroup_Exclude_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_LoginMethodRef {
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
pub struct ZeroTrustAccessGroup_Exclude_Oidc {
    claim_name: terrars::PrimField<std::string::String>,
    claim_value: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_Oidc { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_Oidc {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_Oidc {
    #[doc = "The name of the OIDC claim."]
    pub claim_name: terrars::PrimField<std::string::String>,
    #[doc = "The OIDC claim value to look for."]
    pub claim_value: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your OIDC identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_Oidc {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_Oidc {
        ZeroTrustAccessGroup_Exclude_Oidc {
            claim_name: self.claim_name,
            claim_value: self.claim_value,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_OidcRef {
        ZeroTrustAccessGroup_Exclude_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_OidcRef {
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
pub struct ZeroTrustAccessGroup_Exclude_Okta {
    identity_provider_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_Okta { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_Okta {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_Okta {
    #[doc = "The ID of your Okta identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Okta group."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_Okta {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_Okta {
        ZeroTrustAccessGroup_Exclude_Okta {
            identity_provider_id: self.identity_provider_id,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_OktaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_OktaRef {
        ZeroTrustAccessGroup_Exclude_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_OktaRef {
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
pub struct ZeroTrustAccessGroup_Exclude_Saml {
    attribute_name: terrars::PrimField<std::string::String>,
    attribute_value: terrars::PrimField<std::string::String>,
    identity_provider_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_Saml { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_Saml {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_Saml {
    #[doc = "The name of the SAML attribute."]
    pub attribute_name: terrars::PrimField<std::string::String>,
    #[doc = "The SAML attribute value to look for."]
    pub attribute_value: terrars::PrimField<std::string::String>,
    #[doc = "The ID of your SAML identity provider."]
    pub identity_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_Saml {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_Saml {
        ZeroTrustAccessGroup_Exclude_Saml {
            attribute_name: self.attribute_name,
            attribute_value: self.attribute_value,
            identity_provider_id: self.identity_provider_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_SamlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_SamlRef {
        ZeroTrustAccessGroup_Exclude_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_SamlRef {
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
pub struct ZeroTrustAccessGroup_Exclude_ServiceToken {
    token_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessGroup_Exclude_ServiceToken { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_ServiceToken {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_ServiceToken {
    #[doc = "The ID of a Service Token."]
    pub token_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessGroup_Exclude_ServiceToken {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_ServiceToken {
        ZeroTrustAccessGroup_Exclude_ServiceToken { token_id: self.token_id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_ServiceTokenRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_ServiceTokenRef {
        ZeroTrustAccessGroup_Exclude_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_ServiceTokenRef {
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
pub struct ZeroTrustAccessGroup_Exclude_UserRiskScore {
    user_risk_score: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessGroup_Exclude_UserRiskScore { }

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude_UserRiskScore {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude_UserRiskScore {
    #[doc = "A list of risk score levels to match. Values can be low, medium, high, or unscored."]
    pub user_risk_score: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildZeroTrustAccessGroup_Exclude_UserRiskScore {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude_UserRiskScore {
        ZeroTrustAccessGroup_Exclude_UserRiskScore { user_risk_score: self.user_risk_score }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessGroup_Exclude_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_Exclude_UserRiskScoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_Exclude_UserRiskScoreRef {
        ZeroTrustAccessGroup_Exclude_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_Exclude_UserRiskScoreRef {
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
pub struct ZeroTrustAccessGroup_Exclude {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_valid_service_token: std::option::Option<ZeroTrustAccessGroup_Exclude_AnyValidServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_context: std::option::Option<ZeroTrustAccessGroup_Exclude_AuthContext>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_method: std::option::Option<ZeroTrustAccessGroup_Exclude_AuthMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_ad: std::option::Option<ZeroTrustAccessGroup_Exclude_AzureAd>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<ZeroTrustAccessGroup_Exclude_Certificate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_account_member: std::option::Option<ZeroTrustAccessGroup_Exclude_CloudflareAccountMember>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<ZeroTrustAccessGroup_Exclude_CommonName>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<ZeroTrustAccessGroup_Exclude_DevicePosture>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<ZeroTrustAccessGroup_Exclude_Email>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_domain: std::option::Option<ZeroTrustAccessGroup_Exclude_EmailDomain>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_list: std::option::Option<ZeroTrustAccessGroup_Exclude_EmailList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    everyone: std::option::Option<ZeroTrustAccessGroup_Exclude_Everyone>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_evaluation: std::option::Option<ZeroTrustAccessGroup_Exclude_ExternalEvaluation>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<ZeroTrustAccessGroup_Exclude_Geo>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_organization: std::option::Option<ZeroTrustAccessGroup_Exclude_GithubOrganization>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<ZeroTrustAccessGroup_Exclude_Group>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gsuite: std::option::Option<ZeroTrustAccessGroup_Exclude_Gsuite>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<ZeroTrustAccessGroup_Exclude_Ip>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_list: std::option::Option<ZeroTrustAccessGroup_Exclude_IpList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_app_token: std::option::Option<ZeroTrustAccessGroup_Exclude_LinkedAppToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_method: std::option::Option<ZeroTrustAccessGroup_Exclude_LoginMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<ZeroTrustAccessGroup_Exclude_Oidc>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta: std::option::Option<ZeroTrustAccessGroup_Exclude_Okta>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<ZeroTrustAccessGroup_Exclude_Saml>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_token: std::option::Option<ZeroTrustAccessGroup_Exclude_ServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_risk_score: std::option::Option<ZeroTrustAccessGroup_Exclude_UserRiskScore>,
}

impl ZeroTrustAccessGroup_Exclude {
    #[doc = "Set the field `any_valid_service_token`.\nAn empty object which matches on all service tokens."]
    pub fn set_any_valid_service_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AnyValidServiceToken>,
    ) -> Self {
        self.any_valid_service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_context`.\n"]
    pub fn set_auth_context(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AuthContext>) -> Self {
        self.auth_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AuthMethod>) -> Self {
        self.auth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure_ad`.\n"]
    pub fn set_azure_ad(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AzureAd>) -> Self {
        self.azure_ad = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Certificate>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudflare_account_member`.\n"]
    pub fn set_cloudflare_account_member(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_CloudflareAccountMember>,
    ) -> Self {
        self.cloudflare_account_member = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_CommonName>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_DevicePosture>) -> Self {
        self.device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Email>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_EmailDomain>) -> Self {
        self.email_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_list`.\n"]
    pub fn set_email_list(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_EmailList>) -> Self {
        self.email_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `everyone`.\nAn empty object which matches on all users."]
    pub fn set_everyone(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Everyone>) -> Self {
        self.everyone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_ExternalEvaluation>,
    ) -> Self {
        self.external_evaluation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Geo>) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github_organization`.\n"]
    pub fn set_github_organization(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_GithubOrganization>,
    ) -> Self {
        self.github_organization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Group>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Gsuite>) -> Self {
        self.gsuite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Ip>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_IpList>) -> Self {
        self.ip_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_app_token`.\n"]
    pub fn set_linked_app_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_LinkedAppToken>,
    ) -> Self {
        self.linked_app_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_LoginMethod>) -> Self {
        self.login_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Oidc>) -> Self {
        self.oidc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Okta>) -> Self {
        self.okta = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Saml>) -> Self {
        self.saml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_ServiceToken>) -> Self {
        self.service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_risk_score`.\n"]
    pub fn set_user_risk_score(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_UserRiskScore>) -> Self {
        self.user_risk_score = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessGroup_Exclude {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Exclude {}

impl BuildZeroTrustAccessGroup_Exclude {
    pub fn build(self) -> ZeroTrustAccessGroup_Exclude {
        ZeroTrustAccessGroup_Exclude {
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
pub struct ZeroTrustAccessGroup_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_ExcludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_ExcludeRef {
        ZeroTrustAccessGroup_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef {
        ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> ZeroTrustAccessGroup_Exclude_AuthContextRef {
        ZeroTrustAccessGroup_Exclude_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> ZeroTrustAccessGroup_Exclude_AuthMethodRef {
        ZeroTrustAccessGroup_Exclude_AuthMethodRef::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> ZeroTrustAccessGroup_Exclude_AzureAdRef {
        ZeroTrustAccessGroup_Exclude_AzureAdRef::new(self.shared().clone(), format!("{}.azure_ad", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> ZeroTrustAccessGroup_Exclude_CertificateRef {
        ZeroTrustAccessGroup_Exclude_CertificateRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef {
        ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> ZeroTrustAccessGroup_Exclude_CommonNameRef {
        ZeroTrustAccessGroup_Exclude_CommonNameRef::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ZeroTrustAccessGroup_Exclude_DevicePostureRef {
        ZeroTrustAccessGroup_Exclude_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ZeroTrustAccessGroup_Exclude_EmailRef {
        ZeroTrustAccessGroup_Exclude_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ZeroTrustAccessGroup_Exclude_EmailDomainRef {
        ZeroTrustAccessGroup_Exclude_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> ZeroTrustAccessGroup_Exclude_EmailListRef {
        ZeroTrustAccessGroup_Exclude_EmailListRef::new(self.shared().clone(), format!("{}.email_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> ZeroTrustAccessGroup_Exclude_EveryoneRef {
        ZeroTrustAccessGroup_Exclude_EveryoneRef::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef {
        ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ZeroTrustAccessGroup_Exclude_GeoRef {
        ZeroTrustAccessGroup_Exclude_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> ZeroTrustAccessGroup_Exclude_GithubOrganizationRef {
        ZeroTrustAccessGroup_Exclude_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ZeroTrustAccessGroup_Exclude_GroupRef {
        ZeroTrustAccessGroup_Exclude_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ZeroTrustAccessGroup_Exclude_GsuiteRef {
        ZeroTrustAccessGroup_Exclude_GsuiteRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ZeroTrustAccessGroup_Exclude_IpRef {
        ZeroTrustAccessGroup_Exclude_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ZeroTrustAccessGroup_Exclude_IpListRef {
        ZeroTrustAccessGroup_Exclude_IpListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef {
        ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ZeroTrustAccessGroup_Exclude_LoginMethodRef {
        ZeroTrustAccessGroup_Exclude_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ZeroTrustAccessGroup_Exclude_OidcRef {
        ZeroTrustAccessGroup_Exclude_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ZeroTrustAccessGroup_Exclude_OktaRef {
        ZeroTrustAccessGroup_Exclude_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ZeroTrustAccessGroup_Exclude_SamlRef {
        ZeroTrustAccessGroup_Exclude_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ZeroTrustAccessGroup_Exclude_ServiceTokenRef {
        ZeroTrustAccessGroup_Exclude_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> ZeroTrustAccessGroup_Exclude_UserRiskScoreRef {
        ZeroTrustAccessGroup_Exclude_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessGroup_Include {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_valid_service_token: std::option::Option<ZeroTrustAccessGroup_Exclude_AnyValidServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_context: std::option::Option<ZeroTrustAccessGroup_Exclude_AuthContext>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_method: std::option::Option<ZeroTrustAccessGroup_Exclude_AuthMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_ad: std::option::Option<ZeroTrustAccessGroup_Exclude_AzureAd>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<ZeroTrustAccessGroup_Exclude_Certificate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_account_member: std::option::Option<ZeroTrustAccessGroup_Exclude_CloudflareAccountMember>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<ZeroTrustAccessGroup_Exclude_CommonName>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<ZeroTrustAccessGroup_Exclude_DevicePosture>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<ZeroTrustAccessGroup_Exclude_Email>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_domain: std::option::Option<ZeroTrustAccessGroup_Exclude_EmailDomain>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_list: std::option::Option<ZeroTrustAccessGroup_Exclude_EmailList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    everyone: std::option::Option<ZeroTrustAccessGroup_Exclude_Everyone>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_evaluation: std::option::Option<ZeroTrustAccessGroup_Exclude_ExternalEvaluation>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<ZeroTrustAccessGroup_Exclude_Geo>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_organization: std::option::Option<ZeroTrustAccessGroup_Exclude_GithubOrganization>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<ZeroTrustAccessGroup_Exclude_Group>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gsuite: std::option::Option<ZeroTrustAccessGroup_Exclude_Gsuite>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<ZeroTrustAccessGroup_Exclude_Ip>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_list: std::option::Option<ZeroTrustAccessGroup_Exclude_IpList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_app_token: std::option::Option<ZeroTrustAccessGroup_Exclude_LinkedAppToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_method: std::option::Option<ZeroTrustAccessGroup_Exclude_LoginMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<ZeroTrustAccessGroup_Exclude_Oidc>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta: std::option::Option<ZeroTrustAccessGroup_Exclude_Okta>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<ZeroTrustAccessGroup_Exclude_Saml>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_token: std::option::Option<ZeroTrustAccessGroup_Exclude_ServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_risk_score: std::option::Option<ZeroTrustAccessGroup_Exclude_UserRiskScore>,
}

impl ZeroTrustAccessGroup_Include {
    #[doc = "Set the field `any_valid_service_token`.\nAn empty object which matches on all service tokens."]
    pub fn set_any_valid_service_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AnyValidServiceToken>,
    ) -> Self {
        self.any_valid_service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_context`.\n"]
    pub fn set_auth_context(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AuthContext>) -> Self {
        self.auth_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AuthMethod>) -> Self {
        self.auth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure_ad`.\n"]
    pub fn set_azure_ad(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AzureAd>) -> Self {
        self.azure_ad = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Certificate>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudflare_account_member`.\n"]
    pub fn set_cloudflare_account_member(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_CloudflareAccountMember>,
    ) -> Self {
        self.cloudflare_account_member = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_CommonName>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_DevicePosture>) -> Self {
        self.device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Email>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_EmailDomain>) -> Self {
        self.email_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_list`.\n"]
    pub fn set_email_list(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_EmailList>) -> Self {
        self.email_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `everyone`.\nAn empty object which matches on all users."]
    pub fn set_everyone(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Everyone>) -> Self {
        self.everyone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_ExternalEvaluation>,
    ) -> Self {
        self.external_evaluation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Geo>) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github_organization`.\n"]
    pub fn set_github_organization(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_GithubOrganization>,
    ) -> Self {
        self.github_organization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Group>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Gsuite>) -> Self {
        self.gsuite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Ip>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_IpList>) -> Self {
        self.ip_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_app_token`.\n"]
    pub fn set_linked_app_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_LinkedAppToken>,
    ) -> Self {
        self.linked_app_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_LoginMethod>) -> Self {
        self.login_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Oidc>) -> Self {
        self.oidc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Okta>) -> Self {
        self.okta = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Saml>) -> Self {
        self.saml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_ServiceToken>) -> Self {
        self.service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_risk_score`.\n"]
    pub fn set_user_risk_score(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_UserRiskScore>) -> Self {
        self.user_risk_score = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessGroup_Include {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Include {}

impl BuildZeroTrustAccessGroup_Include {
    pub fn build(self) -> ZeroTrustAccessGroup_Include {
        ZeroTrustAccessGroup_Include {
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
pub struct ZeroTrustAccessGroup_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_IncludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_IncludeRef {
        ZeroTrustAccessGroup_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef {
        ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> ZeroTrustAccessGroup_Exclude_AuthContextRef {
        ZeroTrustAccessGroup_Exclude_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> ZeroTrustAccessGroup_Exclude_AuthMethodRef {
        ZeroTrustAccessGroup_Exclude_AuthMethodRef::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> ZeroTrustAccessGroup_Exclude_AzureAdRef {
        ZeroTrustAccessGroup_Exclude_AzureAdRef::new(self.shared().clone(), format!("{}.azure_ad", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> ZeroTrustAccessGroup_Exclude_CertificateRef {
        ZeroTrustAccessGroup_Exclude_CertificateRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef {
        ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> ZeroTrustAccessGroup_Exclude_CommonNameRef {
        ZeroTrustAccessGroup_Exclude_CommonNameRef::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ZeroTrustAccessGroup_Exclude_DevicePostureRef {
        ZeroTrustAccessGroup_Exclude_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ZeroTrustAccessGroup_Exclude_EmailRef {
        ZeroTrustAccessGroup_Exclude_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ZeroTrustAccessGroup_Exclude_EmailDomainRef {
        ZeroTrustAccessGroup_Exclude_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> ZeroTrustAccessGroup_Exclude_EmailListRef {
        ZeroTrustAccessGroup_Exclude_EmailListRef::new(self.shared().clone(), format!("{}.email_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> ZeroTrustAccessGroup_Exclude_EveryoneRef {
        ZeroTrustAccessGroup_Exclude_EveryoneRef::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef {
        ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ZeroTrustAccessGroup_Exclude_GeoRef {
        ZeroTrustAccessGroup_Exclude_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> ZeroTrustAccessGroup_Exclude_GithubOrganizationRef {
        ZeroTrustAccessGroup_Exclude_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ZeroTrustAccessGroup_Exclude_GroupRef {
        ZeroTrustAccessGroup_Exclude_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ZeroTrustAccessGroup_Exclude_GsuiteRef {
        ZeroTrustAccessGroup_Exclude_GsuiteRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ZeroTrustAccessGroup_Exclude_IpRef {
        ZeroTrustAccessGroup_Exclude_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ZeroTrustAccessGroup_Exclude_IpListRef {
        ZeroTrustAccessGroup_Exclude_IpListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef {
        ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ZeroTrustAccessGroup_Exclude_LoginMethodRef {
        ZeroTrustAccessGroup_Exclude_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ZeroTrustAccessGroup_Exclude_OidcRef {
        ZeroTrustAccessGroup_Exclude_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ZeroTrustAccessGroup_Exclude_OktaRef {
        ZeroTrustAccessGroup_Exclude_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ZeroTrustAccessGroup_Exclude_SamlRef {
        ZeroTrustAccessGroup_Exclude_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ZeroTrustAccessGroup_Exclude_ServiceTokenRef {
        ZeroTrustAccessGroup_Exclude_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> ZeroTrustAccessGroup_Exclude_UserRiskScoreRef {
        ZeroTrustAccessGroup_Exclude_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessGroup_Require {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_valid_service_token: std::option::Option<ZeroTrustAccessGroup_Exclude_AnyValidServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_context: std::option::Option<ZeroTrustAccessGroup_Exclude_AuthContext>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_method: std::option::Option<ZeroTrustAccessGroup_Exclude_AuthMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_ad: std::option::Option<ZeroTrustAccessGroup_Exclude_AzureAd>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<ZeroTrustAccessGroup_Exclude_Certificate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_account_member: std::option::Option<ZeroTrustAccessGroup_Exclude_CloudflareAccountMember>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<ZeroTrustAccessGroup_Exclude_CommonName>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_posture: std::option::Option<ZeroTrustAccessGroup_Exclude_DevicePosture>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<ZeroTrustAccessGroup_Exclude_Email>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_domain: std::option::Option<ZeroTrustAccessGroup_Exclude_EmailDomain>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_list: std::option::Option<ZeroTrustAccessGroup_Exclude_EmailList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    everyone: std::option::Option<ZeroTrustAccessGroup_Exclude_Everyone>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_evaluation: std::option::Option<ZeroTrustAccessGroup_Exclude_ExternalEvaluation>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<ZeroTrustAccessGroup_Exclude_Geo>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_organization: std::option::Option<ZeroTrustAccessGroup_Exclude_GithubOrganization>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<ZeroTrustAccessGroup_Exclude_Group>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gsuite: std::option::Option<ZeroTrustAccessGroup_Exclude_Gsuite>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<ZeroTrustAccessGroup_Exclude_Ip>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_list: std::option::Option<ZeroTrustAccessGroup_Exclude_IpList>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_app_token: std::option::Option<ZeroTrustAccessGroup_Exclude_LinkedAppToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_method: std::option::Option<ZeroTrustAccessGroup_Exclude_LoginMethod>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<ZeroTrustAccessGroup_Exclude_Oidc>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta: std::option::Option<ZeroTrustAccessGroup_Exclude_Okta>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<ZeroTrustAccessGroup_Exclude_Saml>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_token: std::option::Option<ZeroTrustAccessGroup_Exclude_ServiceToken>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_risk_score: std::option::Option<ZeroTrustAccessGroup_Exclude_UserRiskScore>,
}

impl ZeroTrustAccessGroup_Require {
    #[doc = "Set the field `any_valid_service_token`.\nAn empty object which matches on all service tokens."]
    pub fn set_any_valid_service_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AnyValidServiceToken>,
    ) -> Self {
        self.any_valid_service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_context`.\n"]
    pub fn set_auth_context(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AuthContext>) -> Self {
        self.auth_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AuthMethod>) -> Self {
        self.auth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure_ad`.\n"]
    pub fn set_azure_ad(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_AzureAd>) -> Self {
        self.azure_ad = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Certificate>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudflare_account_member`.\n"]
    pub fn set_cloudflare_account_member(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_CloudflareAccountMember>,
    ) -> Self {
        self.cloudflare_account_member = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_CommonName>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_DevicePosture>) -> Self {
        self.device_posture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Email>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_EmailDomain>) -> Self {
        self.email_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_list`.\n"]
    pub fn set_email_list(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_EmailList>) -> Self {
        self.email_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `everyone`.\nAn empty object which matches on all users."]
    pub fn set_everyone(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Everyone>) -> Self {
        self.everyone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_ExternalEvaluation>,
    ) -> Self {
        self.external_evaluation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Geo>) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github_organization`.\n"]
    pub fn set_github_organization(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_GithubOrganization>,
    ) -> Self {
        self.github_organization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Group>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Gsuite>) -> Self {
        self.gsuite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Ip>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_IpList>) -> Self {
        self.ip_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_app_token`.\n"]
    pub fn set_linked_app_token(
        mut self,
        v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_LinkedAppToken>,
    ) -> Self {
        self.linked_app_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_LoginMethod>) -> Self {
        self.login_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Oidc>) -> Self {
        self.oidc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Okta>) -> Self {
        self.okta = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_Saml>) -> Self {
        self.saml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_ServiceToken>) -> Self {
        self.service_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_risk_score`.\n"]
    pub fn set_user_risk_score(mut self, v: impl std::convert::Into<ZeroTrustAccessGroup_Exclude_UserRiskScore>) -> Self {
        self.user_risk_score = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessGroup_Require {
    type O = terrars::BlockAssignable<ZeroTrustAccessGroup_Require>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessGroup_Require {}

impl BuildZeroTrustAccessGroup_Require {
    pub fn build(self) -> ZeroTrustAccessGroup_Require {
        ZeroTrustAccessGroup_Require {
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
pub struct ZeroTrustAccessGroup_RequireRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessGroup_RequireRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessGroup_RequireRef {
        ZeroTrustAccessGroup_RequireRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessGroup_RequireRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef {
        ZeroTrustAccessGroup_Exclude_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> ZeroTrustAccessGroup_Exclude_AuthContextRef {
        ZeroTrustAccessGroup_Exclude_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> ZeroTrustAccessGroup_Exclude_AuthMethodRef {
        ZeroTrustAccessGroup_Exclude_AuthMethodRef::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> ZeroTrustAccessGroup_Exclude_AzureAdRef {
        ZeroTrustAccessGroup_Exclude_AzureAdRef::new(self.shared().clone(), format!("{}.azure_ad", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> ZeroTrustAccessGroup_Exclude_CertificateRef {
        ZeroTrustAccessGroup_Exclude_CertificateRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef {
        ZeroTrustAccessGroup_Exclude_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> ZeroTrustAccessGroup_Exclude_CommonNameRef {
        ZeroTrustAccessGroup_Exclude_CommonNameRef::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ZeroTrustAccessGroup_Exclude_DevicePostureRef {
        ZeroTrustAccessGroup_Exclude_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ZeroTrustAccessGroup_Exclude_EmailRef {
        ZeroTrustAccessGroup_Exclude_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ZeroTrustAccessGroup_Exclude_EmailDomainRef {
        ZeroTrustAccessGroup_Exclude_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> ZeroTrustAccessGroup_Exclude_EmailListRef {
        ZeroTrustAccessGroup_Exclude_EmailListRef::new(self.shared().clone(), format!("{}.email_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> ZeroTrustAccessGroup_Exclude_EveryoneRef {
        ZeroTrustAccessGroup_Exclude_EveryoneRef::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef {
        ZeroTrustAccessGroup_Exclude_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ZeroTrustAccessGroup_Exclude_GeoRef {
        ZeroTrustAccessGroup_Exclude_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> ZeroTrustAccessGroup_Exclude_GithubOrganizationRef {
        ZeroTrustAccessGroup_Exclude_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ZeroTrustAccessGroup_Exclude_GroupRef {
        ZeroTrustAccessGroup_Exclude_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ZeroTrustAccessGroup_Exclude_GsuiteRef {
        ZeroTrustAccessGroup_Exclude_GsuiteRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ZeroTrustAccessGroup_Exclude_IpRef {
        ZeroTrustAccessGroup_Exclude_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ZeroTrustAccessGroup_Exclude_IpListRef {
        ZeroTrustAccessGroup_Exclude_IpListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef {
        ZeroTrustAccessGroup_Exclude_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ZeroTrustAccessGroup_Exclude_LoginMethodRef {
        ZeroTrustAccessGroup_Exclude_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ZeroTrustAccessGroup_Exclude_OidcRef {
        ZeroTrustAccessGroup_Exclude_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ZeroTrustAccessGroup_Exclude_OktaRef {
        ZeroTrustAccessGroup_Exclude_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ZeroTrustAccessGroup_Exclude_SamlRef {
        ZeroTrustAccessGroup_Exclude_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ZeroTrustAccessGroup_Exclude_ServiceTokenRef {
        ZeroTrustAccessGroup_Exclude_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> ZeroTrustAccessGroup_Exclude_UserRiskScoreRef {
        ZeroTrustAccessGroup_Exclude_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}
