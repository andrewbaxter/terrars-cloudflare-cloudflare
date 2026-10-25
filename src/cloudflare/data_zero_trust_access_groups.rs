#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustAccessGroupsData_ {
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
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustAccessGroups_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustAccessGroupsData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustAccessGroups(std::rc::Rc<DataZeroTrustAccessGroups_>);

impl DataZeroTrustAccessGroups {
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

    #[doc =
        "Set the field `account_id`.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the group."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nSearch for groups by other listed query parameters."]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustAccessGroups_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch for groups by other listed query parameters."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustAccessGroups {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustAccessGroups { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups {
    type O = terrars::ListRef<DataZeroTrustAccessGroupsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustAccessGroups_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_groups".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustAccessGroups {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustAccessGroups {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustAccessGroups {
        let out = DataZeroTrustAccessGroups(std::rc::Rc::new(DataZeroTrustAccessGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustAccessGroupsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                name: core::default::Default::default(),
                search: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustAccessGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustAccessGroupsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustAccessGroups_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch for groups by other listed query parameters."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceToken {}

impl DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceToken {
        DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_AuthContext {}

impl DataZeroTrustAccessGroups_Result_Exclude_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_AuthContext {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_AuthContext {
        DataZeroTrustAccessGroups_Result_Exclude_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_AuthContextRef {
        DataZeroTrustAccessGroups_Result_Exclude_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_AuthContextRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_AuthMethod {}

impl DataZeroTrustAccessGroups_Result_Exclude_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_AuthMethod {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_AuthMethod {
        DataZeroTrustAccessGroups_Result_Exclude_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_AuthMethodRef {
        DataZeroTrustAccessGroups_Result_Exclude_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_AuthMethodRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_AzureAd {}

impl DataZeroTrustAccessGroups_Result_Exclude_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_AzureAd {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_AzureAd {
        DataZeroTrustAccessGroups_Result_Exclude_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_AzureAdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_AzureAdRef {
        DataZeroTrustAccessGroups_Result_Exclude_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_AzureAdRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_Certificate {}

impl DataZeroTrustAccessGroups_Result_Exclude_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_Certificate {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_Certificate {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_Certificate {
        DataZeroTrustAccessGroups_Result_Exclude_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_CertificateRef {
        DataZeroTrustAccessGroups_Result_Exclude_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMember {}

impl DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMember { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMember {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMember {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMember {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMember {
        DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMember {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMemberRef {
        DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMemberRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_CommonName {}

impl DataZeroTrustAccessGroups_Result_Exclude_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_CommonName {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_CommonName {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_CommonName {
        DataZeroTrustAccessGroups_Result_Exclude_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_CommonNameRef {
        DataZeroTrustAccessGroups_Result_Exclude_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_CommonNameRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_DevicePosture {}

impl DataZeroTrustAccessGroups_Result_Exclude_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_DevicePosture {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_DevicePosture {
        DataZeroTrustAccessGroups_Result_Exclude_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_DevicePostureRef {
        DataZeroTrustAccessGroups_Result_Exclude_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_DevicePostureRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_Email {}

impl DataZeroTrustAccessGroups_Result_Exclude_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_Email {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_Email {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_Email {
        DataZeroTrustAccessGroups_Result_Exclude_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_EmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_EmailRef {
        DataZeroTrustAccessGroups_Result_Exclude_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_EmailRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_EmailDomain {}

impl DataZeroTrustAccessGroups_Result_Exclude_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_EmailDomain {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_EmailDomain {
        DataZeroTrustAccessGroups_Result_Exclude_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_EmailDomainRef {
        DataZeroTrustAccessGroups_Result_Exclude_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_EmailDomainRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_EmailList {}

impl DataZeroTrustAccessGroups_Result_Exclude_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_EmailList {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_EmailList {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_EmailList {
        DataZeroTrustAccessGroups_Result_Exclude_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_EmailListRef {
        DataZeroTrustAccessGroups_Result_Exclude_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_EmailListRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_Everyone {}

impl DataZeroTrustAccessGroups_Result_Exclude_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_Everyone {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_Everyone {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_Everyone {
        DataZeroTrustAccessGroups_Result_Exclude_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_EveryoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_EveryoneRef {
        DataZeroTrustAccessGroups_Result_Exclude_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluation {}

impl DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluation {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluation {
        DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluationRef {
        DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluationRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_Geo {}

impl DataZeroTrustAccessGroups_Result_Exclude_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_Geo {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_Geo {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_Geo {
        DataZeroTrustAccessGroups_Result_Exclude_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_GeoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_GeoRef {
        DataZeroTrustAccessGroups_Result_Exclude_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_GeoRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_GithubOrganization {}

impl DataZeroTrustAccessGroups_Result_Exclude_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_GithubOrganization {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_GithubOrganization {
        DataZeroTrustAccessGroups_Result_Exclude_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_GithubOrganizationRef {
        DataZeroTrustAccessGroups_Result_Exclude_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_GithubOrganizationRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_Group {}

impl DataZeroTrustAccessGroups_Result_Exclude_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_Group {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_Group {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_Group {
        DataZeroTrustAccessGroups_Result_Exclude_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_GroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_GroupRef {
        DataZeroTrustAccessGroups_Result_Exclude_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_GroupRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_Gsuite {}

impl DataZeroTrustAccessGroups_Result_Exclude_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_Gsuite {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_Gsuite {
        DataZeroTrustAccessGroups_Result_Exclude_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_GsuiteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_GsuiteRef {
        DataZeroTrustAccessGroups_Result_Exclude_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_GsuiteRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_Ip {}

impl DataZeroTrustAccessGroups_Result_Exclude_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_Ip {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_Ip {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_Ip {
        DataZeroTrustAccessGroups_Result_Exclude_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_IpRef {
        DataZeroTrustAccessGroups_Result_Exclude_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_IpRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_IpList {}

impl DataZeroTrustAccessGroups_Result_Exclude_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_IpList {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_IpList {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_IpList {
        DataZeroTrustAccessGroups_Result_Exclude_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_IpListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_IpListRef {
        DataZeroTrustAccessGroups_Result_Exclude_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_IpListRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_LinkedAppToken {}

impl DataZeroTrustAccessGroups_Result_Exclude_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_LinkedAppToken {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_LinkedAppToken {
        DataZeroTrustAccessGroups_Result_Exclude_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_LinkedAppTokenRef {
        DataZeroTrustAccessGroups_Result_Exclude_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_LinkedAppTokenRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_LoginMethod {}

impl DataZeroTrustAccessGroups_Result_Exclude_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_LoginMethod {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_LoginMethod {
        DataZeroTrustAccessGroups_Result_Exclude_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_LoginMethodRef {
        DataZeroTrustAccessGroups_Result_Exclude_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_LoginMethodRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_Oidc {}

impl DataZeroTrustAccessGroups_Result_Exclude_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_Oidc {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_Oidc {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_Oidc {
        DataZeroTrustAccessGroups_Result_Exclude_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_OidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_OidcRef {
        DataZeroTrustAccessGroups_Result_Exclude_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_OidcRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_Okta {}

impl DataZeroTrustAccessGroups_Result_Exclude_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_Okta {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_Okta {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_Okta {
        DataZeroTrustAccessGroups_Result_Exclude_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_OktaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_OktaRef {
        DataZeroTrustAccessGroups_Result_Exclude_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_OktaRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_Saml {}

impl DataZeroTrustAccessGroups_Result_Exclude_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_Saml {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_Saml {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_Saml {
        DataZeroTrustAccessGroups_Result_Exclude_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_SamlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_SamlRef {
        DataZeroTrustAccessGroups_Result_Exclude_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_SamlRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_ServiceToken {}

impl DataZeroTrustAccessGroups_Result_Exclude_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_ServiceToken {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_ServiceToken {
        DataZeroTrustAccessGroups_Result_Exclude_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_ServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Exclude_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_ServiceTokenRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude_UserRiskScore {}

impl DataZeroTrustAccessGroups_Result_Exclude_UserRiskScore { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude_UserRiskScore {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude_UserRiskScore {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude_UserRiskScore {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude_UserRiskScore {
        DataZeroTrustAccessGroups_Result_Exclude_UserRiskScore {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Exclude_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Exclude_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Exclude_UserRiskScoreRef {
        DataZeroTrustAccessGroups_Result_Exclude_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Exclude_UserRiskScoreRef {
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
pub struct DataZeroTrustAccessGroups_Result_Exclude {}

impl DataZeroTrustAccessGroups_Result_Exclude { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Exclude {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Exclude {}

impl BuildDataZeroTrustAccessGroups_Result_Exclude {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Exclude {
        DataZeroTrustAccessGroups_Result_Exclude {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_ExcludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessGroups_Result_ExcludeRef {
        DataZeroTrustAccessGroups_Result_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Exclude_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessGroups_Result_Exclude_AuthContextRef {
        DataZeroTrustAccessGroups_Result_Exclude_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessGroups_Result_Exclude_AuthMethodRef {
        DataZeroTrustAccessGroups_Result_Exclude_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessGroups_Result_Exclude_AzureAdRef {
        DataZeroTrustAccessGroups_Result_Exclude_AzureAdRef::new(
            self.shared().clone(),
            format!("{}.azure_ad", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessGroups_Result_Exclude_CertificateRef {
        DataZeroTrustAccessGroups_Result_Exclude_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMemberRef {
        DataZeroTrustAccessGroups_Result_Exclude_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessGroups_Result_Exclude_CommonNameRef {
        DataZeroTrustAccessGroups_Result_Exclude_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessGroups_Result_Exclude_DevicePostureRef {
        DataZeroTrustAccessGroups_Result_Exclude_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessGroups_Result_Exclude_EmailRef {
        DataZeroTrustAccessGroups_Result_Exclude_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessGroups_Result_Exclude_EmailDomainRef {
        DataZeroTrustAccessGroups_Result_Exclude_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessGroups_Result_Exclude_EmailListRef {
        DataZeroTrustAccessGroups_Result_Exclude_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessGroups_Result_Exclude_EveryoneRef {
        DataZeroTrustAccessGroups_Result_Exclude_EveryoneRef::new(
            self.shared().clone(),
            format!("{}.everyone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluationRef {
        DataZeroTrustAccessGroups_Result_Exclude_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessGroups_Result_Exclude_GeoRef {
        DataZeroTrustAccessGroups_Result_Exclude_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessGroups_Result_Exclude_GithubOrganizationRef {
        DataZeroTrustAccessGroups_Result_Exclude_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessGroups_Result_Exclude_GroupRef {
        DataZeroTrustAccessGroups_Result_Exclude_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessGroups_Result_Exclude_GsuiteRef {
        DataZeroTrustAccessGroups_Result_Exclude_GsuiteRef::new(
            self.shared().clone(),
            format!("{}.gsuite", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessGroups_Result_Exclude_IpRef {
        DataZeroTrustAccessGroups_Result_Exclude_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessGroups_Result_Exclude_IpListRef {
        DataZeroTrustAccessGroups_Result_Exclude_IpListRef::new(
            self.shared().clone(),
            format!("{}.ip_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessGroups_Result_Exclude_LinkedAppTokenRef {
        DataZeroTrustAccessGroups_Result_Exclude_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessGroups_Result_Exclude_LoginMethodRef {
        DataZeroTrustAccessGroups_Result_Exclude_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessGroups_Result_Exclude_OidcRef {
        DataZeroTrustAccessGroups_Result_Exclude_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessGroups_Result_Exclude_OktaRef {
        DataZeroTrustAccessGroups_Result_Exclude_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessGroups_Result_Exclude_SamlRef {
        DataZeroTrustAccessGroups_Result_Exclude_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessGroups_Result_Exclude_ServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Exclude_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> DataZeroTrustAccessGroups_Result_Exclude_UserRiskScoreRef {
        DataZeroTrustAccessGroups_Result_Exclude_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Include_AnyValidServiceToken {}

impl DataZeroTrustAccessGroups_Result_Include_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessGroups_Result_Include_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_AnyValidServiceToken {
        DataZeroTrustAccessGroups_Result_Include_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_AnyValidServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Include_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Include_AuthContext {}

impl DataZeroTrustAccessGroups_Result_Include_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_AuthContext {}

impl BuildDataZeroTrustAccessGroups_Result_Include_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_AuthContext {
        DataZeroTrustAccessGroups_Result_Include_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_AuthContextRef {
        DataZeroTrustAccessGroups_Result_Include_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_AuthContextRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_AuthMethod {}

impl DataZeroTrustAccessGroups_Result_Include_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_AuthMethod {}

impl BuildDataZeroTrustAccessGroups_Result_Include_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_AuthMethod {
        DataZeroTrustAccessGroups_Result_Include_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_AuthMethodRef {
        DataZeroTrustAccessGroups_Result_Include_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_AuthMethodRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_AzureAd {}

impl DataZeroTrustAccessGroups_Result_Include_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_AzureAd {}

impl BuildDataZeroTrustAccessGroups_Result_Include_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_AzureAd {
        DataZeroTrustAccessGroups_Result_Include_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_AzureAdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_AzureAdRef {
        DataZeroTrustAccessGroups_Result_Include_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_AzureAdRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_Certificate {}

impl DataZeroTrustAccessGroups_Result_Include_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_Certificate {}

impl BuildDataZeroTrustAccessGroups_Result_Include_Certificate {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_Certificate {
        DataZeroTrustAccessGroups_Result_Include_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_CertificateRef {
        DataZeroTrustAccessGroups_Result_Include_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMember {}

impl DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMember { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMember {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_CloudflareAccountMember {}

impl BuildDataZeroTrustAccessGroups_Result_Include_CloudflareAccountMember {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMember {
        DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMember {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMemberRef {
        DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMemberRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_CommonName {}

impl DataZeroTrustAccessGroups_Result_Include_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_CommonName {}

impl BuildDataZeroTrustAccessGroups_Result_Include_CommonName {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_CommonName {
        DataZeroTrustAccessGroups_Result_Include_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_CommonNameRef {
        DataZeroTrustAccessGroups_Result_Include_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_CommonNameRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_DevicePosture {}

impl DataZeroTrustAccessGroups_Result_Include_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_DevicePosture {}

impl BuildDataZeroTrustAccessGroups_Result_Include_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_DevicePosture {
        DataZeroTrustAccessGroups_Result_Include_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_DevicePostureRef {
        DataZeroTrustAccessGroups_Result_Include_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_DevicePostureRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_Email {}

impl DataZeroTrustAccessGroups_Result_Include_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_Email {}

impl BuildDataZeroTrustAccessGroups_Result_Include_Email {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_Email {
        DataZeroTrustAccessGroups_Result_Include_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_EmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_EmailRef {
        DataZeroTrustAccessGroups_Result_Include_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_EmailRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_EmailDomain {}

impl DataZeroTrustAccessGroups_Result_Include_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_EmailDomain {}

impl BuildDataZeroTrustAccessGroups_Result_Include_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_EmailDomain {
        DataZeroTrustAccessGroups_Result_Include_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_EmailDomainRef {
        DataZeroTrustAccessGroups_Result_Include_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_EmailDomainRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_EmailList {}

impl DataZeroTrustAccessGroups_Result_Include_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_EmailList {}

impl BuildDataZeroTrustAccessGroups_Result_Include_EmailList {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_EmailList {
        DataZeroTrustAccessGroups_Result_Include_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_EmailListRef {
        DataZeroTrustAccessGroups_Result_Include_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_EmailListRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_Everyone {}

impl DataZeroTrustAccessGroups_Result_Include_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_Everyone {}

impl BuildDataZeroTrustAccessGroups_Result_Include_Everyone {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_Everyone {
        DataZeroTrustAccessGroups_Result_Include_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_EveryoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_EveryoneRef {
        DataZeroTrustAccessGroups_Result_Include_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Include_ExternalEvaluation {}

impl DataZeroTrustAccessGroups_Result_Include_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_ExternalEvaluation {}

impl BuildDataZeroTrustAccessGroups_Result_Include_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_ExternalEvaluation {
        DataZeroTrustAccessGroups_Result_Include_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_ExternalEvaluationRef {
        DataZeroTrustAccessGroups_Result_Include_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_ExternalEvaluationRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_Geo {}

impl DataZeroTrustAccessGroups_Result_Include_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_Geo {}

impl BuildDataZeroTrustAccessGroups_Result_Include_Geo {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_Geo {
        DataZeroTrustAccessGroups_Result_Include_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_GeoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_GeoRef {
        DataZeroTrustAccessGroups_Result_Include_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_GeoRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_GithubOrganization {}

impl DataZeroTrustAccessGroups_Result_Include_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_GithubOrganization {}

impl BuildDataZeroTrustAccessGroups_Result_Include_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_GithubOrganization {
        DataZeroTrustAccessGroups_Result_Include_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_GithubOrganizationRef {
        DataZeroTrustAccessGroups_Result_Include_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_GithubOrganizationRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_Group {}

impl DataZeroTrustAccessGroups_Result_Include_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_Group {}

impl BuildDataZeroTrustAccessGroups_Result_Include_Group {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_Group {
        DataZeroTrustAccessGroups_Result_Include_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_GroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_GroupRef {
        DataZeroTrustAccessGroups_Result_Include_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_GroupRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_Gsuite {}

impl DataZeroTrustAccessGroups_Result_Include_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_Gsuite {}

impl BuildDataZeroTrustAccessGroups_Result_Include_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_Gsuite {
        DataZeroTrustAccessGroups_Result_Include_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_GsuiteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_GsuiteRef {
        DataZeroTrustAccessGroups_Result_Include_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_GsuiteRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_Ip {}

impl DataZeroTrustAccessGroups_Result_Include_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_Ip {}

impl BuildDataZeroTrustAccessGroups_Result_Include_Ip {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_Ip {
        DataZeroTrustAccessGroups_Result_Include_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_IpRef {
        DataZeroTrustAccessGroups_Result_Include_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_IpRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_IpList {}

impl DataZeroTrustAccessGroups_Result_Include_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_IpList {}

impl BuildDataZeroTrustAccessGroups_Result_Include_IpList {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_IpList {
        DataZeroTrustAccessGroups_Result_Include_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_IpListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_IpListRef {
        DataZeroTrustAccessGroups_Result_Include_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_IpListRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_LinkedAppToken {}

impl DataZeroTrustAccessGroups_Result_Include_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_LinkedAppToken {}

impl BuildDataZeroTrustAccessGroups_Result_Include_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_LinkedAppToken {
        DataZeroTrustAccessGroups_Result_Include_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_LinkedAppTokenRef {
        DataZeroTrustAccessGroups_Result_Include_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_LinkedAppTokenRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_LoginMethod {}

impl DataZeroTrustAccessGroups_Result_Include_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_LoginMethod {}

impl BuildDataZeroTrustAccessGroups_Result_Include_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_LoginMethod {
        DataZeroTrustAccessGroups_Result_Include_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_LoginMethodRef {
        DataZeroTrustAccessGroups_Result_Include_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_LoginMethodRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_Oidc {}

impl DataZeroTrustAccessGroups_Result_Include_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_Oidc {}

impl BuildDataZeroTrustAccessGroups_Result_Include_Oidc {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_Oidc {
        DataZeroTrustAccessGroups_Result_Include_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_OidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_OidcRef {
        DataZeroTrustAccessGroups_Result_Include_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_OidcRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_Okta {}

impl DataZeroTrustAccessGroups_Result_Include_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_Okta {}

impl BuildDataZeroTrustAccessGroups_Result_Include_Okta {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_Okta {
        DataZeroTrustAccessGroups_Result_Include_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_OktaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_OktaRef {
        DataZeroTrustAccessGroups_Result_Include_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_OktaRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_Saml {}

impl DataZeroTrustAccessGroups_Result_Include_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_Saml {}

impl BuildDataZeroTrustAccessGroups_Result_Include_Saml {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_Saml {
        DataZeroTrustAccessGroups_Result_Include_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_SamlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_SamlRef {
        DataZeroTrustAccessGroups_Result_Include_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_SamlRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_ServiceToken {}

impl DataZeroTrustAccessGroups_Result_Include_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_ServiceToken {}

impl BuildDataZeroTrustAccessGroups_Result_Include_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_ServiceToken {
        DataZeroTrustAccessGroups_Result_Include_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_ServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Include_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_ServiceTokenRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include_UserRiskScore {}

impl DataZeroTrustAccessGroups_Result_Include_UserRiskScore { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include_UserRiskScore {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include_UserRiskScore {}

impl BuildDataZeroTrustAccessGroups_Result_Include_UserRiskScore {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include_UserRiskScore {
        DataZeroTrustAccessGroups_Result_Include_UserRiskScore {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Include_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Include_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Include_UserRiskScoreRef {
        DataZeroTrustAccessGroups_Result_Include_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Include_UserRiskScoreRef {
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
pub struct DataZeroTrustAccessGroups_Result_Include {}

impl DataZeroTrustAccessGroups_Result_Include { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Include {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Include {}

impl BuildDataZeroTrustAccessGroups_Result_Include {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Include {
        DataZeroTrustAccessGroups_Result_Include {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IncludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessGroups_Result_IncludeRef {
        DataZeroTrustAccessGroups_Result_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessGroups_Result_Include_AnyValidServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Include_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessGroups_Result_Include_AuthContextRef {
        DataZeroTrustAccessGroups_Result_Include_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessGroups_Result_Include_AuthMethodRef {
        DataZeroTrustAccessGroups_Result_Include_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessGroups_Result_Include_AzureAdRef {
        DataZeroTrustAccessGroups_Result_Include_AzureAdRef::new(
            self.shared().clone(),
            format!("{}.azure_ad", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessGroups_Result_Include_CertificateRef {
        DataZeroTrustAccessGroups_Result_Include_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMemberRef {
        DataZeroTrustAccessGroups_Result_Include_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessGroups_Result_Include_CommonNameRef {
        DataZeroTrustAccessGroups_Result_Include_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessGroups_Result_Include_DevicePostureRef {
        DataZeroTrustAccessGroups_Result_Include_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessGroups_Result_Include_EmailRef {
        DataZeroTrustAccessGroups_Result_Include_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessGroups_Result_Include_EmailDomainRef {
        DataZeroTrustAccessGroups_Result_Include_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessGroups_Result_Include_EmailListRef {
        DataZeroTrustAccessGroups_Result_Include_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessGroups_Result_Include_EveryoneRef {
        DataZeroTrustAccessGroups_Result_Include_EveryoneRef::new(
            self.shared().clone(),
            format!("{}.everyone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessGroups_Result_Include_ExternalEvaluationRef {
        DataZeroTrustAccessGroups_Result_Include_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessGroups_Result_Include_GeoRef {
        DataZeroTrustAccessGroups_Result_Include_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessGroups_Result_Include_GithubOrganizationRef {
        DataZeroTrustAccessGroups_Result_Include_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessGroups_Result_Include_GroupRef {
        DataZeroTrustAccessGroups_Result_Include_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessGroups_Result_Include_GsuiteRef {
        DataZeroTrustAccessGroups_Result_Include_GsuiteRef::new(
            self.shared().clone(),
            format!("{}.gsuite", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessGroups_Result_Include_IpRef {
        DataZeroTrustAccessGroups_Result_Include_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessGroups_Result_Include_IpListRef {
        DataZeroTrustAccessGroups_Result_Include_IpListRef::new(
            self.shared().clone(),
            format!("{}.ip_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessGroups_Result_Include_LinkedAppTokenRef {
        DataZeroTrustAccessGroups_Result_Include_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessGroups_Result_Include_LoginMethodRef {
        DataZeroTrustAccessGroups_Result_Include_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessGroups_Result_Include_OidcRef {
        DataZeroTrustAccessGroups_Result_Include_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessGroups_Result_Include_OktaRef {
        DataZeroTrustAccessGroups_Result_Include_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessGroups_Result_Include_SamlRef {
        DataZeroTrustAccessGroups_Result_Include_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessGroups_Result_Include_ServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Include_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> DataZeroTrustAccessGroups_Result_Include_UserRiskScoreRef {
        DataZeroTrustAccessGroups_Result_Include_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceToken {}

impl DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceToken {
        DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceTokenRef {
        DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_AuthContext {}

impl DataZeroTrustAccessGroups_Result_IsDefault_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_AuthContext {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_AuthContext {
        DataZeroTrustAccessGroups_Result_IsDefault_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_AuthContextRef {
        DataZeroTrustAccessGroups_Result_IsDefault_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_AuthContextRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_AuthMethod {}

impl DataZeroTrustAccessGroups_Result_IsDefault_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_AuthMethod {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_AuthMethod {
        DataZeroTrustAccessGroups_Result_IsDefault_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_AuthMethodRef {
        DataZeroTrustAccessGroups_Result_IsDefault_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_AuthMethodRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_AzureAd {}

impl DataZeroTrustAccessGroups_Result_IsDefault_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_AzureAd {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_AzureAd {
        DataZeroTrustAccessGroups_Result_IsDefault_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_AzureAdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_AzureAdRef {
        DataZeroTrustAccessGroups_Result_IsDefault_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_AzureAdRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_Certificate {}

impl DataZeroTrustAccessGroups_Result_IsDefault_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_Certificate {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_Certificate {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_Certificate {
        DataZeroTrustAccessGroups_Result_IsDefault_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_CertificateRef {
        DataZeroTrustAccessGroups_Result_IsDefault_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMember {}

impl DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMember { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMember {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMember {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMember {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMember {
        DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMember {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMemberRef {
        DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMemberRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_CommonName {}

impl DataZeroTrustAccessGroups_Result_IsDefault_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_CommonName {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_CommonName {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_CommonName {
        DataZeroTrustAccessGroups_Result_IsDefault_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_CommonNameRef {
        DataZeroTrustAccessGroups_Result_IsDefault_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_CommonNameRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_DevicePosture {}

impl DataZeroTrustAccessGroups_Result_IsDefault_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_DevicePosture {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_DevicePosture {
        DataZeroTrustAccessGroups_Result_IsDefault_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_DevicePostureRef {
        DataZeroTrustAccessGroups_Result_IsDefault_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_DevicePostureRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_Email {}

impl DataZeroTrustAccessGroups_Result_IsDefault_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_Email {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_Email {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_Email {
        DataZeroTrustAccessGroups_Result_IsDefault_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_EmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_EmailRef {
        DataZeroTrustAccessGroups_Result_IsDefault_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_EmailRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_EmailDomain {}

impl DataZeroTrustAccessGroups_Result_IsDefault_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_EmailDomain {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_EmailDomain {
        DataZeroTrustAccessGroups_Result_IsDefault_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_EmailDomainRef {
        DataZeroTrustAccessGroups_Result_IsDefault_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_EmailDomainRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_EmailList {}

impl DataZeroTrustAccessGroups_Result_IsDefault_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_EmailList {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_EmailList {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_EmailList {
        DataZeroTrustAccessGroups_Result_IsDefault_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_EmailListRef {
        DataZeroTrustAccessGroups_Result_IsDefault_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_EmailListRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_Everyone {}

impl DataZeroTrustAccessGroups_Result_IsDefault_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_Everyone {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_Everyone {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_Everyone {
        DataZeroTrustAccessGroups_Result_IsDefault_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_EveryoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_EveryoneRef {
        DataZeroTrustAccessGroups_Result_IsDefault_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluation {}

impl DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluation {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluation {
        DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluationRef {
        DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluationRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_Geo {}

impl DataZeroTrustAccessGroups_Result_IsDefault_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_Geo {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_Geo {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_Geo {
        DataZeroTrustAccessGroups_Result_IsDefault_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_GeoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_GeoRef {
        DataZeroTrustAccessGroups_Result_IsDefault_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_GeoRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganization {}

impl DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_GithubOrganization {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganization {
        DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganizationRef {
        DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganizationRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_Group {}

impl DataZeroTrustAccessGroups_Result_IsDefault_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_Group {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_Group {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_Group {
        DataZeroTrustAccessGroups_Result_IsDefault_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_GroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_GroupRef {
        DataZeroTrustAccessGroups_Result_IsDefault_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_GroupRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_Gsuite {}

impl DataZeroTrustAccessGroups_Result_IsDefault_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_Gsuite {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_Gsuite {
        DataZeroTrustAccessGroups_Result_IsDefault_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_GsuiteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_GsuiteRef {
        DataZeroTrustAccessGroups_Result_IsDefault_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_GsuiteRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_Ip {}

impl DataZeroTrustAccessGroups_Result_IsDefault_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_Ip {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_Ip {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_Ip {
        DataZeroTrustAccessGroups_Result_IsDefault_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_IpRef {
        DataZeroTrustAccessGroups_Result_IsDefault_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_IpRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_IpList {}

impl DataZeroTrustAccessGroups_Result_IsDefault_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_IpList {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_IpList {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_IpList {
        DataZeroTrustAccessGroups_Result_IsDefault_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_IpListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_IpListRef {
        DataZeroTrustAccessGroups_Result_IsDefault_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_IpListRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppToken {}

impl DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_LinkedAppToken {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppToken {
        DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppTokenRef {
        DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppTokenRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_LoginMethod {}

impl DataZeroTrustAccessGroups_Result_IsDefault_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_LoginMethod {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_LoginMethod {
        DataZeroTrustAccessGroups_Result_IsDefault_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_LoginMethodRef {
        DataZeroTrustAccessGroups_Result_IsDefault_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_LoginMethodRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_Oidc {}

impl DataZeroTrustAccessGroups_Result_IsDefault_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_Oidc {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_Oidc {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_Oidc {
        DataZeroTrustAccessGroups_Result_IsDefault_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_OidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_OidcRef {
        DataZeroTrustAccessGroups_Result_IsDefault_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_OidcRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_Okta {}

impl DataZeroTrustAccessGroups_Result_IsDefault_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_Okta {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_Okta {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_Okta {
        DataZeroTrustAccessGroups_Result_IsDefault_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_OktaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_OktaRef {
        DataZeroTrustAccessGroups_Result_IsDefault_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_OktaRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_Saml {}

impl DataZeroTrustAccessGroups_Result_IsDefault_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_Saml {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_Saml {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_Saml {
        DataZeroTrustAccessGroups_Result_IsDefault_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_SamlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_SamlRef {
        DataZeroTrustAccessGroups_Result_IsDefault_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_SamlRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_ServiceToken {}

impl DataZeroTrustAccessGroups_Result_IsDefault_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_ServiceToken {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_ServiceToken {
        DataZeroTrustAccessGroups_Result_IsDefault_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_ServiceTokenRef {
        DataZeroTrustAccessGroups_Result_IsDefault_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_ServiceTokenRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScore {}

impl DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScore { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScore {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault_UserRiskScore {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault_UserRiskScore {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScore {
        DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScore {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScoreRef {
        DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScoreRef {
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
pub struct DataZeroTrustAccessGroups_Result_IsDefault {}

impl DataZeroTrustAccessGroups_Result_IsDefault { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_IsDefault {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_IsDefault>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_IsDefault {}

impl BuildDataZeroTrustAccessGroups_Result_IsDefault {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_IsDefault {
        DataZeroTrustAccessGroups_Result_IsDefault {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_IsDefaultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_IsDefaultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessGroups_Result_IsDefaultRef {
        DataZeroTrustAccessGroups_Result_IsDefaultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_IsDefaultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceTokenRef {
        DataZeroTrustAccessGroups_Result_IsDefault_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_AuthContextRef {
        DataZeroTrustAccessGroups_Result_IsDefault_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_AuthMethodRef {
        DataZeroTrustAccessGroups_Result_IsDefault_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_AzureAdRef {
        DataZeroTrustAccessGroups_Result_IsDefault_AzureAdRef::new(
            self.shared().clone(),
            format!("{}.azure_ad", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_CertificateRef {
        DataZeroTrustAccessGroups_Result_IsDefault_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMemberRef {
        DataZeroTrustAccessGroups_Result_IsDefault_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_CommonNameRef {
        DataZeroTrustAccessGroups_Result_IsDefault_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_DevicePostureRef {
        DataZeroTrustAccessGroups_Result_IsDefault_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_EmailRef {
        DataZeroTrustAccessGroups_Result_IsDefault_EmailRef::new(
            self.shared().clone(),
            format!("{}.email", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_EmailDomainRef {
        DataZeroTrustAccessGroups_Result_IsDefault_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_EmailListRef {
        DataZeroTrustAccessGroups_Result_IsDefault_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_EveryoneRef {
        DataZeroTrustAccessGroups_Result_IsDefault_EveryoneRef::new(
            self.shared().clone(),
            format!("{}.everyone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluationRef {
        DataZeroTrustAccessGroups_Result_IsDefault_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_GeoRef {
        DataZeroTrustAccessGroups_Result_IsDefault_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganizationRef {
        DataZeroTrustAccessGroups_Result_IsDefault_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_GroupRef {
        DataZeroTrustAccessGroups_Result_IsDefault_GroupRef::new(
            self.shared().clone(),
            format!("{}.group", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_GsuiteRef {
        DataZeroTrustAccessGroups_Result_IsDefault_GsuiteRef::new(
            self.shared().clone(),
            format!("{}.gsuite", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_IpRef {
        DataZeroTrustAccessGroups_Result_IsDefault_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_IpListRef {
        DataZeroTrustAccessGroups_Result_IsDefault_IpListRef::new(
            self.shared().clone(),
            format!("{}.ip_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppTokenRef {
        DataZeroTrustAccessGroups_Result_IsDefault_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_LoginMethodRef {
        DataZeroTrustAccessGroups_Result_IsDefault_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_OidcRef {
        DataZeroTrustAccessGroups_Result_IsDefault_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_OktaRef {
        DataZeroTrustAccessGroups_Result_IsDefault_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_SamlRef {
        DataZeroTrustAccessGroups_Result_IsDefault_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_ServiceTokenRef {
        DataZeroTrustAccessGroups_Result_IsDefault_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScoreRef {
        DataZeroTrustAccessGroups_Result_IsDefault_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Require_AnyValidServiceToken {}

impl DataZeroTrustAccessGroups_Result_Require_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessGroups_Result_Require_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_AnyValidServiceToken {
        DataZeroTrustAccessGroups_Result_Require_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_AnyValidServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Require_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Require_AuthContext {}

impl DataZeroTrustAccessGroups_Result_Require_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_AuthContext {}

impl BuildDataZeroTrustAccessGroups_Result_Require_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_AuthContext {
        DataZeroTrustAccessGroups_Result_Require_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_AuthContextRef {
        DataZeroTrustAccessGroups_Result_Require_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_AuthContextRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_AuthMethod {}

impl DataZeroTrustAccessGroups_Result_Require_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_AuthMethod {}

impl BuildDataZeroTrustAccessGroups_Result_Require_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_AuthMethod {
        DataZeroTrustAccessGroups_Result_Require_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_AuthMethodRef {
        DataZeroTrustAccessGroups_Result_Require_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_AuthMethodRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_AzureAd {}

impl DataZeroTrustAccessGroups_Result_Require_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_AzureAd {}

impl BuildDataZeroTrustAccessGroups_Result_Require_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_AzureAd {
        DataZeroTrustAccessGroups_Result_Require_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_AzureAdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_AzureAdRef {
        DataZeroTrustAccessGroups_Result_Require_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_AzureAdRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_Certificate {}

impl DataZeroTrustAccessGroups_Result_Require_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_Certificate {}

impl BuildDataZeroTrustAccessGroups_Result_Require_Certificate {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_Certificate {
        DataZeroTrustAccessGroups_Result_Require_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_CertificateRef {
        DataZeroTrustAccessGroups_Result_Require_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMember {}

impl DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMember { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMember {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_CloudflareAccountMember {}

impl BuildDataZeroTrustAccessGroups_Result_Require_CloudflareAccountMember {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMember {
        DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMember {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMemberRef {
        DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMemberRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_CommonName {}

impl DataZeroTrustAccessGroups_Result_Require_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_CommonName {}

impl BuildDataZeroTrustAccessGroups_Result_Require_CommonName {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_CommonName {
        DataZeroTrustAccessGroups_Result_Require_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_CommonNameRef {
        DataZeroTrustAccessGroups_Result_Require_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_CommonNameRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_DevicePosture {}

impl DataZeroTrustAccessGroups_Result_Require_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_DevicePosture {}

impl BuildDataZeroTrustAccessGroups_Result_Require_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_DevicePosture {
        DataZeroTrustAccessGroups_Result_Require_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_DevicePostureRef {
        DataZeroTrustAccessGroups_Result_Require_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_DevicePostureRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_Email {}

impl DataZeroTrustAccessGroups_Result_Require_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_Email {}

impl BuildDataZeroTrustAccessGroups_Result_Require_Email {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_Email {
        DataZeroTrustAccessGroups_Result_Require_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_EmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_EmailRef {
        DataZeroTrustAccessGroups_Result_Require_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_EmailRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_EmailDomain {}

impl DataZeroTrustAccessGroups_Result_Require_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_EmailDomain {}

impl BuildDataZeroTrustAccessGroups_Result_Require_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_EmailDomain {
        DataZeroTrustAccessGroups_Result_Require_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_EmailDomainRef {
        DataZeroTrustAccessGroups_Result_Require_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_EmailDomainRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_EmailList {}

impl DataZeroTrustAccessGroups_Result_Require_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_EmailList {}

impl BuildDataZeroTrustAccessGroups_Result_Require_EmailList {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_EmailList {
        DataZeroTrustAccessGroups_Result_Require_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_EmailListRef {
        DataZeroTrustAccessGroups_Result_Require_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_EmailListRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_Everyone {}

impl DataZeroTrustAccessGroups_Result_Require_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_Everyone {}

impl BuildDataZeroTrustAccessGroups_Result_Require_Everyone {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_Everyone {
        DataZeroTrustAccessGroups_Result_Require_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_EveryoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_EveryoneRef {
        DataZeroTrustAccessGroups_Result_Require_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result_Require_ExternalEvaluation {}

impl DataZeroTrustAccessGroups_Result_Require_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_ExternalEvaluation {}

impl BuildDataZeroTrustAccessGroups_Result_Require_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_ExternalEvaluation {
        DataZeroTrustAccessGroups_Result_Require_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_ExternalEvaluationRef {
        DataZeroTrustAccessGroups_Result_Require_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_ExternalEvaluationRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_Geo {}

impl DataZeroTrustAccessGroups_Result_Require_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_Geo {}

impl BuildDataZeroTrustAccessGroups_Result_Require_Geo {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_Geo {
        DataZeroTrustAccessGroups_Result_Require_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_GeoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_GeoRef {
        DataZeroTrustAccessGroups_Result_Require_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_GeoRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_GithubOrganization {}

impl DataZeroTrustAccessGroups_Result_Require_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_GithubOrganization {}

impl BuildDataZeroTrustAccessGroups_Result_Require_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_GithubOrganization {
        DataZeroTrustAccessGroups_Result_Require_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_GithubOrganizationRef {
        DataZeroTrustAccessGroups_Result_Require_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_GithubOrganizationRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_Group {}

impl DataZeroTrustAccessGroups_Result_Require_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_Group {}

impl BuildDataZeroTrustAccessGroups_Result_Require_Group {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_Group {
        DataZeroTrustAccessGroups_Result_Require_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_GroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_GroupRef {
        DataZeroTrustAccessGroups_Result_Require_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_GroupRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_Gsuite {}

impl DataZeroTrustAccessGroups_Result_Require_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_Gsuite {}

impl BuildDataZeroTrustAccessGroups_Result_Require_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_Gsuite {
        DataZeroTrustAccessGroups_Result_Require_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_GsuiteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_GsuiteRef {
        DataZeroTrustAccessGroups_Result_Require_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_GsuiteRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_Ip {}

impl DataZeroTrustAccessGroups_Result_Require_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_Ip {}

impl BuildDataZeroTrustAccessGroups_Result_Require_Ip {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_Ip {
        DataZeroTrustAccessGroups_Result_Require_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_IpRef {
        DataZeroTrustAccessGroups_Result_Require_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_IpRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_IpList {}

impl DataZeroTrustAccessGroups_Result_Require_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_IpList {}

impl BuildDataZeroTrustAccessGroups_Result_Require_IpList {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_IpList {
        DataZeroTrustAccessGroups_Result_Require_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_IpListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_IpListRef {
        DataZeroTrustAccessGroups_Result_Require_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_IpListRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_LinkedAppToken {}

impl DataZeroTrustAccessGroups_Result_Require_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_LinkedAppToken {}

impl BuildDataZeroTrustAccessGroups_Result_Require_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_LinkedAppToken {
        DataZeroTrustAccessGroups_Result_Require_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_LinkedAppTokenRef {
        DataZeroTrustAccessGroups_Result_Require_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_LinkedAppTokenRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_LoginMethod {}

impl DataZeroTrustAccessGroups_Result_Require_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_LoginMethod {}

impl BuildDataZeroTrustAccessGroups_Result_Require_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_LoginMethod {
        DataZeroTrustAccessGroups_Result_Require_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_LoginMethodRef {
        DataZeroTrustAccessGroups_Result_Require_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_LoginMethodRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_Oidc {}

impl DataZeroTrustAccessGroups_Result_Require_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_Oidc {}

impl BuildDataZeroTrustAccessGroups_Result_Require_Oidc {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_Oidc {
        DataZeroTrustAccessGroups_Result_Require_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_OidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_OidcRef {
        DataZeroTrustAccessGroups_Result_Require_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_OidcRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_Okta {}

impl DataZeroTrustAccessGroups_Result_Require_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_Okta {}

impl BuildDataZeroTrustAccessGroups_Result_Require_Okta {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_Okta {
        DataZeroTrustAccessGroups_Result_Require_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_OktaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_OktaRef {
        DataZeroTrustAccessGroups_Result_Require_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_OktaRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_Saml {}

impl DataZeroTrustAccessGroups_Result_Require_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_Saml {}

impl BuildDataZeroTrustAccessGroups_Result_Require_Saml {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_Saml {
        DataZeroTrustAccessGroups_Result_Require_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_SamlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_SamlRef {
        DataZeroTrustAccessGroups_Result_Require_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_SamlRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_ServiceToken {}

impl DataZeroTrustAccessGroups_Result_Require_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_ServiceToken {}

impl BuildDataZeroTrustAccessGroups_Result_Require_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_ServiceToken {
        DataZeroTrustAccessGroups_Result_Require_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_ServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Require_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_ServiceTokenRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require_UserRiskScore {}

impl DataZeroTrustAccessGroups_Result_Require_UserRiskScore { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require_UserRiskScore {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require_UserRiskScore {}

impl BuildDataZeroTrustAccessGroups_Result_Require_UserRiskScore {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require_UserRiskScore {
        DataZeroTrustAccessGroups_Result_Require_UserRiskScore {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_Require_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_Require_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessGroups_Result_Require_UserRiskScoreRef {
        DataZeroTrustAccessGroups_Result_Require_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_Require_UserRiskScoreRef {
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
pub struct DataZeroTrustAccessGroups_Result_Require {}

impl DataZeroTrustAccessGroups_Result_Require { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result_Require {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result_Require>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result_Require {}

impl BuildDataZeroTrustAccessGroups_Result_Require {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result_Require {
        DataZeroTrustAccessGroups_Result_Require {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_Result_RequireRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_Result_RequireRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessGroups_Result_RequireRef {
        DataZeroTrustAccessGroups_Result_RequireRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_Result_RequireRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessGroups_Result_Require_AnyValidServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Require_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessGroups_Result_Require_AuthContextRef {
        DataZeroTrustAccessGroups_Result_Require_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessGroups_Result_Require_AuthMethodRef {
        DataZeroTrustAccessGroups_Result_Require_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessGroups_Result_Require_AzureAdRef {
        DataZeroTrustAccessGroups_Result_Require_AzureAdRef::new(
            self.shared().clone(),
            format!("{}.azure_ad", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessGroups_Result_Require_CertificateRef {
        DataZeroTrustAccessGroups_Result_Require_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMemberRef {
        DataZeroTrustAccessGroups_Result_Require_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessGroups_Result_Require_CommonNameRef {
        DataZeroTrustAccessGroups_Result_Require_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessGroups_Result_Require_DevicePostureRef {
        DataZeroTrustAccessGroups_Result_Require_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessGroups_Result_Require_EmailRef {
        DataZeroTrustAccessGroups_Result_Require_EmailRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessGroups_Result_Require_EmailDomainRef {
        DataZeroTrustAccessGroups_Result_Require_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessGroups_Result_Require_EmailListRef {
        DataZeroTrustAccessGroups_Result_Require_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessGroups_Result_Require_EveryoneRef {
        DataZeroTrustAccessGroups_Result_Require_EveryoneRef::new(
            self.shared().clone(),
            format!("{}.everyone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessGroups_Result_Require_ExternalEvaluationRef {
        DataZeroTrustAccessGroups_Result_Require_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessGroups_Result_Require_GeoRef {
        DataZeroTrustAccessGroups_Result_Require_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessGroups_Result_Require_GithubOrganizationRef {
        DataZeroTrustAccessGroups_Result_Require_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessGroups_Result_Require_GroupRef {
        DataZeroTrustAccessGroups_Result_Require_GroupRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessGroups_Result_Require_GsuiteRef {
        DataZeroTrustAccessGroups_Result_Require_GsuiteRef::new(
            self.shared().clone(),
            format!("{}.gsuite", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessGroups_Result_Require_IpRef {
        DataZeroTrustAccessGroups_Result_Require_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessGroups_Result_Require_IpListRef {
        DataZeroTrustAccessGroups_Result_Require_IpListRef::new(
            self.shared().clone(),
            format!("{}.ip_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessGroups_Result_Require_LinkedAppTokenRef {
        DataZeroTrustAccessGroups_Result_Require_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessGroups_Result_Require_LoginMethodRef {
        DataZeroTrustAccessGroups_Result_Require_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessGroups_Result_Require_OidcRef {
        DataZeroTrustAccessGroups_Result_Require_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessGroups_Result_Require_OktaRef {
        DataZeroTrustAccessGroups_Result_Require_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessGroups_Result_Require_SamlRef {
        DataZeroTrustAccessGroups_Result_Require_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessGroups_Result_Require_ServiceTokenRef {
        DataZeroTrustAccessGroups_Result_Require_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> DataZeroTrustAccessGroups_Result_Require_UserRiskScoreRef {
        DataZeroTrustAccessGroups_Result_Require_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessGroups_Result {}

impl DataZeroTrustAccessGroups_Result { }

impl terrars::ToListMappable for DataZeroTrustAccessGroups_Result {
    type O = terrars::BlockAssignable<DataZeroTrustAccessGroups_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessGroups_Result {}

impl BuildDataZeroTrustAccessGroups_Result {
    pub fn build(self) -> DataZeroTrustAccessGroups_Result {
        DataZeroTrustAccessGroups_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessGroups_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessGroups_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessGroups_ResultRef {
        DataZeroTrustAccessGroups_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessGroups_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nRules evaluated with a NOT logical operator. To match a policy, a user cannot meet any of the Exclude rules."]
    pub fn exclude(&self) -> terrars::ListRef<DataZeroTrustAccessGroups_Result_ExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nRules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub fn include(&self) -> terrars::ListRef<DataZeroTrustAccessGroups_Result_IncludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_default` after provisioning.\nRules evaluated with an AND logical operator. To match a policy, a user must meet all of the Require rules."]
    pub fn is_default(&self) -> terrars::ListRef<DataZeroTrustAccessGroups_Result_IsDefaultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.is_default", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Access group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require` after provisioning.\nRules evaluated with an AND logical operator. To match a policy, a user must meet all of the Require rules."]
    pub fn require(&self) -> terrars::ListRef<DataZeroTrustAccessGroups_Result_RequireRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.require", self.base))
    }
}
