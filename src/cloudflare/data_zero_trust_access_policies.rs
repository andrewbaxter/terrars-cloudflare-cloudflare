#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustAccessPoliciesData_ {
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
}

struct DataZeroTrustAccessPolicies_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustAccessPoliciesData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustAccessPolicies(std::rc::Rc<DataZeroTrustAccessPolicies_>);

impl DataZeroTrustAccessPolicies {
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

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustAccessPolicies_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustAccessPolicies {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustAccessPolicies { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies {
    type O = terrars::ListRef<DataZeroTrustAccessPoliciesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustAccessPolicies_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_policies".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustAccessPolicies {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustAccessPolicies {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustAccessPolicies {
        let out = DataZeroTrustAccessPolicies(std::rc::Rc::new(DataZeroTrustAccessPolicies_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustAccessPoliciesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustAccessPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustAccessPoliciesRef {
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

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustAccessPolicies_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_ApprovalGroups {}

impl DataZeroTrustAccessPolicies_Result_ApprovalGroups { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_ApprovalGroups {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_ApprovalGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_ApprovalGroups {}

impl BuildDataZeroTrustAccessPolicies_Result_ApprovalGroups {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_ApprovalGroups {
        DataZeroTrustAccessPolicies_Result_ApprovalGroups {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_ApprovalGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_ApprovalGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_ApprovalGroupsRef {
        DataZeroTrustAccessPolicies_Result_ApprovalGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_ApprovalGroupsRef {
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
pub struct DataZeroTrustAccessPolicies_Result_ConnectionRules_Rdp {}

impl DataZeroTrustAccessPolicies_Result_ConnectionRules_Rdp { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_ConnectionRules_Rdp {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_ConnectionRules_Rdp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_ConnectionRules_Rdp {}

impl BuildDataZeroTrustAccessPolicies_Result_ConnectionRules_Rdp {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_ConnectionRules_Rdp {
        DataZeroTrustAccessPolicies_Result_ConnectionRules_Rdp {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_ConnectionRules_RdpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_ConnectionRules_RdpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_ConnectionRules_RdpRef {
        DataZeroTrustAccessPolicies_Result_ConnectionRules_RdpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_ConnectionRules_RdpRef {
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
pub struct DataZeroTrustAccessPolicies_Result_ConnectionRules {}

impl DataZeroTrustAccessPolicies_Result_ConnectionRules { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_ConnectionRules {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_ConnectionRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_ConnectionRules {}

impl BuildDataZeroTrustAccessPolicies_Result_ConnectionRules {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_ConnectionRules {
        DataZeroTrustAccessPolicies_Result_ConnectionRules {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_ConnectionRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_ConnectionRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_ConnectionRulesRef {
        DataZeroTrustAccessPolicies_Result_ConnectionRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_ConnectionRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rdp` after provisioning.\nThe RDP-specific rules that define clipboard behavior for RDP connections."]
    pub fn rdp(&self) -> DataZeroTrustAccessPolicies_Result_ConnectionRules_RdpRef {
        DataZeroTrustAccessPolicies_Result_ConnectionRules_RdpRef::new(
            self.shared().clone(),
            format!("{}.rdp", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceToken {}

impl DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceToken {
        DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_AuthContext {}

impl DataZeroTrustAccessPolicies_Result_Exclude_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_AuthContext {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_AuthContext {
        DataZeroTrustAccessPolicies_Result_Exclude_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_AuthContextRef {
        DataZeroTrustAccessPolicies_Result_Exclude_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_AuthContextRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_AuthMethod {}

impl DataZeroTrustAccessPolicies_Result_Exclude_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_AuthMethod {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_AuthMethod {
        DataZeroTrustAccessPolicies_Result_Exclude_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_AuthMethodRef {
        DataZeroTrustAccessPolicies_Result_Exclude_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_AuthMethodRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_AzureAd {}

impl DataZeroTrustAccessPolicies_Result_Exclude_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_AzureAd {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_AzureAd {
        DataZeroTrustAccessPolicies_Result_Exclude_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_AzureAdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_AzureAdRef {
        DataZeroTrustAccessPolicies_Result_Exclude_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_AzureAdRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_Certificate {}

impl DataZeroTrustAccessPolicies_Result_Exclude_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_Certificate {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_Certificate {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_Certificate {
        DataZeroTrustAccessPolicies_Result_Exclude_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_CertificateRef {
        DataZeroTrustAccessPolicies_Result_Exclude_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMember {}

impl DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMember { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMember {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMember {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMember {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMember {
        DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMember {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMemberRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_CommonName {}

impl DataZeroTrustAccessPolicies_Result_Exclude_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_CommonName {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_CommonName {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_CommonName {
        DataZeroTrustAccessPolicies_Result_Exclude_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_CommonNameRef {
        DataZeroTrustAccessPolicies_Result_Exclude_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_CommonNameRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_DevicePosture {}

impl DataZeroTrustAccessPolicies_Result_Exclude_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_DevicePosture {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_DevicePosture {
        DataZeroTrustAccessPolicies_Result_Exclude_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_DevicePostureRef {
        DataZeroTrustAccessPolicies_Result_Exclude_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_DevicePostureRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_Email {}

impl DataZeroTrustAccessPolicies_Result_Exclude_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_Email {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_Email {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_Email {
        DataZeroTrustAccessPolicies_Result_Exclude_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_EmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_EmailRef {
        DataZeroTrustAccessPolicies_Result_Exclude_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_EmailRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_EmailDomain {}

impl DataZeroTrustAccessPolicies_Result_Exclude_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_EmailDomain {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_EmailDomain {
        DataZeroTrustAccessPolicies_Result_Exclude_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_EmailDomainRef {
        DataZeroTrustAccessPolicies_Result_Exclude_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_EmailDomainRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_EmailList {}

impl DataZeroTrustAccessPolicies_Result_Exclude_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_EmailList {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_EmailList {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_EmailList {
        DataZeroTrustAccessPolicies_Result_Exclude_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_EmailListRef {
        DataZeroTrustAccessPolicies_Result_Exclude_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_EmailListRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_Everyone {}

impl DataZeroTrustAccessPolicies_Result_Exclude_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_Everyone {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_Everyone {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_Everyone {
        DataZeroTrustAccessPolicies_Result_Exclude_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_EveryoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_EveryoneRef {
        DataZeroTrustAccessPolicies_Result_Exclude_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluation {}

impl DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluation {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluation {
        DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluationRef {
        DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluationRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_Geo {}

impl DataZeroTrustAccessPolicies_Result_Exclude_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_Geo {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_Geo {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_Geo {
        DataZeroTrustAccessPolicies_Result_Exclude_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_GeoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_GeoRef {
        DataZeroTrustAccessPolicies_Result_Exclude_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_GeoRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganization {}

impl DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_GithubOrganization {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganization {
        DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganizationRef {
        DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganizationRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_Group {}

impl DataZeroTrustAccessPolicies_Result_Exclude_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_Group {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_Group {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_Group {
        DataZeroTrustAccessPolicies_Result_Exclude_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_GroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_GroupRef {
        DataZeroTrustAccessPolicies_Result_Exclude_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_GroupRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_Gsuite {}

impl DataZeroTrustAccessPolicies_Result_Exclude_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_Gsuite {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_Gsuite {
        DataZeroTrustAccessPolicies_Result_Exclude_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_GsuiteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_GsuiteRef {
        DataZeroTrustAccessPolicies_Result_Exclude_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_GsuiteRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_Ip {}

impl DataZeroTrustAccessPolicies_Result_Exclude_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_Ip {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_Ip {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_Ip {
        DataZeroTrustAccessPolicies_Result_Exclude_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_IpRef {
        DataZeroTrustAccessPolicies_Result_Exclude_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_IpRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_IpList {}

impl DataZeroTrustAccessPolicies_Result_Exclude_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_IpList {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_IpList {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_IpList {
        DataZeroTrustAccessPolicies_Result_Exclude_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_IpListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_IpListRef {
        DataZeroTrustAccessPolicies_Result_Exclude_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_IpListRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppToken {}

impl DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_LinkedAppToken {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppToken {
        DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppTokenRef {
        DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppTokenRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_LoginMethod {}

impl DataZeroTrustAccessPolicies_Result_Exclude_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_LoginMethod {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_LoginMethod {
        DataZeroTrustAccessPolicies_Result_Exclude_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_LoginMethodRef {
        DataZeroTrustAccessPolicies_Result_Exclude_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_LoginMethodRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_Oidc {}

impl DataZeroTrustAccessPolicies_Result_Exclude_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_Oidc {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_Oidc {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_Oidc {
        DataZeroTrustAccessPolicies_Result_Exclude_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_OidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_OidcRef {
        DataZeroTrustAccessPolicies_Result_Exclude_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_OidcRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_Okta {}

impl DataZeroTrustAccessPolicies_Result_Exclude_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_Okta {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_Okta {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_Okta {
        DataZeroTrustAccessPolicies_Result_Exclude_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_OktaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_OktaRef {
        DataZeroTrustAccessPolicies_Result_Exclude_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_OktaRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_Saml {}

impl DataZeroTrustAccessPolicies_Result_Exclude_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_Saml {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_Saml {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_Saml {
        DataZeroTrustAccessPolicies_Result_Exclude_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_SamlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_SamlRef {
        DataZeroTrustAccessPolicies_Result_Exclude_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_SamlRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_ServiceToken {}

impl DataZeroTrustAccessPolicies_Result_Exclude_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_ServiceToken {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_ServiceToken {
        DataZeroTrustAccessPolicies_Result_Exclude_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_ServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Exclude_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_ServiceTokenRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScore {}

impl DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScore { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScore {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude_UserRiskScore {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude_UserRiskScore {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScore {
        DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScore {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScoreRef {
        DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScoreRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Exclude {}

impl DataZeroTrustAccessPolicies_Result_Exclude { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Exclude {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Exclude {}

impl BuildDataZeroTrustAccessPolicies_Result_Exclude {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Exclude {
        DataZeroTrustAccessPolicies_Result_Exclude {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_ExcludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicies_Result_ExcludeRef {
        DataZeroTrustAccessPolicies_Result_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Exclude_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_AuthContextRef {
        DataZeroTrustAccessPolicies_Result_Exclude_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_AuthMethodRef {
        DataZeroTrustAccessPolicies_Result_Exclude_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_AzureAdRef {
        DataZeroTrustAccessPolicies_Result_Exclude_AzureAdRef::new(
            self.shared().clone(),
            format!("{}.azure_ad", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_CertificateRef {
        DataZeroTrustAccessPolicies_Result_Exclude_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicies_Result_Exclude_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_CommonNameRef {
        DataZeroTrustAccessPolicies_Result_Exclude_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_DevicePostureRef {
        DataZeroTrustAccessPolicies_Result_Exclude_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_EmailRef {
        DataZeroTrustAccessPolicies_Result_Exclude_EmailRef::new(
            self.shared().clone(),
            format!("{}.email", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_EmailDomainRef {
        DataZeroTrustAccessPolicies_Result_Exclude_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_EmailListRef {
        DataZeroTrustAccessPolicies_Result_Exclude_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_EveryoneRef {
        DataZeroTrustAccessPolicies_Result_Exclude_EveryoneRef::new(
            self.shared().clone(),
            format!("{}.everyone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluationRef {
        DataZeroTrustAccessPolicies_Result_Exclude_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_GeoRef {
        DataZeroTrustAccessPolicies_Result_Exclude_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganizationRef {
        DataZeroTrustAccessPolicies_Result_Exclude_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_GroupRef {
        DataZeroTrustAccessPolicies_Result_Exclude_GroupRef::new(
            self.shared().clone(),
            format!("{}.group", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_GsuiteRef {
        DataZeroTrustAccessPolicies_Result_Exclude_GsuiteRef::new(
            self.shared().clone(),
            format!("{}.gsuite", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_IpRef {
        DataZeroTrustAccessPolicies_Result_Exclude_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_IpListRef {
        DataZeroTrustAccessPolicies_Result_Exclude_IpListRef::new(
            self.shared().clone(),
            format!("{}.ip_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppTokenRef {
        DataZeroTrustAccessPolicies_Result_Exclude_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_LoginMethodRef {
        DataZeroTrustAccessPolicies_Result_Exclude_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_OidcRef {
        DataZeroTrustAccessPolicies_Result_Exclude_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_OktaRef {
        DataZeroTrustAccessPolicies_Result_Exclude_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_SamlRef {
        DataZeroTrustAccessPolicies_Result_Exclude_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_ServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Exclude_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScoreRef {
        DataZeroTrustAccessPolicies_Result_Exclude_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceToken {}

impl DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceToken {
        DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Include_AuthContext {}

impl DataZeroTrustAccessPolicies_Result_Include_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_AuthContext {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_AuthContext {
        DataZeroTrustAccessPolicies_Result_Include_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_AuthContextRef {
        DataZeroTrustAccessPolicies_Result_Include_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_AuthContextRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_AuthMethod {}

impl DataZeroTrustAccessPolicies_Result_Include_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_AuthMethod {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_AuthMethod {
        DataZeroTrustAccessPolicies_Result_Include_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_AuthMethodRef {
        DataZeroTrustAccessPolicies_Result_Include_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_AuthMethodRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_AzureAd {}

impl DataZeroTrustAccessPolicies_Result_Include_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_AzureAd {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_AzureAd {
        DataZeroTrustAccessPolicies_Result_Include_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_AzureAdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_AzureAdRef {
        DataZeroTrustAccessPolicies_Result_Include_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_AzureAdRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_Certificate {}

impl DataZeroTrustAccessPolicies_Result_Include_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_Certificate {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_Certificate {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_Certificate {
        DataZeroTrustAccessPolicies_Result_Include_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_CertificateRef {
        DataZeroTrustAccessPolicies_Result_Include_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMember {}

impl DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMember { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMember {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMember {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMember {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMember {
        DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMember {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMemberRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_CommonName {}

impl DataZeroTrustAccessPolicies_Result_Include_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_CommonName {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_CommonName {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_CommonName {
        DataZeroTrustAccessPolicies_Result_Include_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_CommonNameRef {
        DataZeroTrustAccessPolicies_Result_Include_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_CommonNameRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_DevicePosture {}

impl DataZeroTrustAccessPolicies_Result_Include_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_DevicePosture {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_DevicePosture {
        DataZeroTrustAccessPolicies_Result_Include_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_DevicePostureRef {
        DataZeroTrustAccessPolicies_Result_Include_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_DevicePostureRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_Email {}

impl DataZeroTrustAccessPolicies_Result_Include_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_Email {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_Email {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_Email {
        DataZeroTrustAccessPolicies_Result_Include_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_EmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_EmailRef {
        DataZeroTrustAccessPolicies_Result_Include_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_EmailRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_EmailDomain {}

impl DataZeroTrustAccessPolicies_Result_Include_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_EmailDomain {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_EmailDomain {
        DataZeroTrustAccessPolicies_Result_Include_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_EmailDomainRef {
        DataZeroTrustAccessPolicies_Result_Include_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_EmailDomainRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_EmailList {}

impl DataZeroTrustAccessPolicies_Result_Include_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_EmailList {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_EmailList {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_EmailList {
        DataZeroTrustAccessPolicies_Result_Include_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_EmailListRef {
        DataZeroTrustAccessPolicies_Result_Include_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_EmailListRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_Everyone {}

impl DataZeroTrustAccessPolicies_Result_Include_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_Everyone {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_Everyone {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_Everyone {
        DataZeroTrustAccessPolicies_Result_Include_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_EveryoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_EveryoneRef {
        DataZeroTrustAccessPolicies_Result_Include_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluation {}

impl DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_ExternalEvaluation {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluation {
        DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluationRef {
        DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluationRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_Geo {}

impl DataZeroTrustAccessPolicies_Result_Include_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_Geo {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_Geo {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_Geo {
        DataZeroTrustAccessPolicies_Result_Include_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_GeoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_GeoRef {
        DataZeroTrustAccessPolicies_Result_Include_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_GeoRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_GithubOrganization {}

impl DataZeroTrustAccessPolicies_Result_Include_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_GithubOrganization {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_GithubOrganization {
        DataZeroTrustAccessPolicies_Result_Include_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_GithubOrganizationRef {
        DataZeroTrustAccessPolicies_Result_Include_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_GithubOrganizationRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_Group {}

impl DataZeroTrustAccessPolicies_Result_Include_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_Group {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_Group {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_Group {
        DataZeroTrustAccessPolicies_Result_Include_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_GroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_GroupRef {
        DataZeroTrustAccessPolicies_Result_Include_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_GroupRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_Gsuite {}

impl DataZeroTrustAccessPolicies_Result_Include_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_Gsuite {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_Gsuite {
        DataZeroTrustAccessPolicies_Result_Include_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_GsuiteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_GsuiteRef {
        DataZeroTrustAccessPolicies_Result_Include_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_GsuiteRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_Ip {}

impl DataZeroTrustAccessPolicies_Result_Include_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_Ip {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_Ip {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_Ip {
        DataZeroTrustAccessPolicies_Result_Include_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_IpRef {
        DataZeroTrustAccessPolicies_Result_Include_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_IpRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_IpList {}

impl DataZeroTrustAccessPolicies_Result_Include_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_IpList {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_IpList {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_IpList {
        DataZeroTrustAccessPolicies_Result_Include_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_IpListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_IpListRef {
        DataZeroTrustAccessPolicies_Result_Include_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_IpListRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_LinkedAppToken {}

impl DataZeroTrustAccessPolicies_Result_Include_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_LinkedAppToken {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_LinkedAppToken {
        DataZeroTrustAccessPolicies_Result_Include_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_LinkedAppTokenRef {
        DataZeroTrustAccessPolicies_Result_Include_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_LinkedAppTokenRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_LoginMethod {}

impl DataZeroTrustAccessPolicies_Result_Include_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_LoginMethod {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_LoginMethod {
        DataZeroTrustAccessPolicies_Result_Include_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_LoginMethodRef {
        DataZeroTrustAccessPolicies_Result_Include_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_LoginMethodRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_Oidc {}

impl DataZeroTrustAccessPolicies_Result_Include_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_Oidc {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_Oidc {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_Oidc {
        DataZeroTrustAccessPolicies_Result_Include_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_OidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_OidcRef {
        DataZeroTrustAccessPolicies_Result_Include_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_OidcRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_Okta {}

impl DataZeroTrustAccessPolicies_Result_Include_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_Okta {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_Okta {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_Okta {
        DataZeroTrustAccessPolicies_Result_Include_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_OktaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_OktaRef {
        DataZeroTrustAccessPolicies_Result_Include_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_OktaRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_Saml {}

impl DataZeroTrustAccessPolicies_Result_Include_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_Saml {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_Saml {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_Saml {
        DataZeroTrustAccessPolicies_Result_Include_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_SamlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_SamlRef {
        DataZeroTrustAccessPolicies_Result_Include_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_SamlRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_ServiceToken {}

impl DataZeroTrustAccessPolicies_Result_Include_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_ServiceToken {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_ServiceToken {
        DataZeroTrustAccessPolicies_Result_Include_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_ServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Include_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_ServiceTokenRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include_UserRiskScore {}

impl DataZeroTrustAccessPolicies_Result_Include_UserRiskScore { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include_UserRiskScore {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include_UserRiskScore {}

impl BuildDataZeroTrustAccessPolicies_Result_Include_UserRiskScore {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include_UserRiskScore {
        DataZeroTrustAccessPolicies_Result_Include_UserRiskScore {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Include_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Include_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Include_UserRiskScoreRef {
        DataZeroTrustAccessPolicies_Result_Include_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Include_UserRiskScoreRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Include {}

impl DataZeroTrustAccessPolicies_Result_Include { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Include {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Include {}

impl BuildDataZeroTrustAccessPolicies_Result_Include {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Include {
        DataZeroTrustAccessPolicies_Result_Include {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_IncludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicies_Result_IncludeRef {
        DataZeroTrustAccessPolicies_Result_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Include_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessPolicies_Result_Include_AuthContextRef {
        DataZeroTrustAccessPolicies_Result_Include_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessPolicies_Result_Include_AuthMethodRef {
        DataZeroTrustAccessPolicies_Result_Include_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessPolicies_Result_Include_AzureAdRef {
        DataZeroTrustAccessPolicies_Result_Include_AzureAdRef::new(
            self.shared().clone(),
            format!("{}.azure_ad", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessPolicies_Result_Include_CertificateRef {
        DataZeroTrustAccessPolicies_Result_Include_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicies_Result_Include_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessPolicies_Result_Include_CommonNameRef {
        DataZeroTrustAccessPolicies_Result_Include_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessPolicies_Result_Include_DevicePostureRef {
        DataZeroTrustAccessPolicies_Result_Include_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessPolicies_Result_Include_EmailRef {
        DataZeroTrustAccessPolicies_Result_Include_EmailRef::new(
            self.shared().clone(),
            format!("{}.email", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessPolicies_Result_Include_EmailDomainRef {
        DataZeroTrustAccessPolicies_Result_Include_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessPolicies_Result_Include_EmailListRef {
        DataZeroTrustAccessPolicies_Result_Include_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessPolicies_Result_Include_EveryoneRef {
        DataZeroTrustAccessPolicies_Result_Include_EveryoneRef::new(
            self.shared().clone(),
            format!("{}.everyone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluationRef {
        DataZeroTrustAccessPolicies_Result_Include_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessPolicies_Result_Include_GeoRef {
        DataZeroTrustAccessPolicies_Result_Include_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessPolicies_Result_Include_GithubOrganizationRef {
        DataZeroTrustAccessPolicies_Result_Include_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessPolicies_Result_Include_GroupRef {
        DataZeroTrustAccessPolicies_Result_Include_GroupRef::new(
            self.shared().clone(),
            format!("{}.group", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessPolicies_Result_Include_GsuiteRef {
        DataZeroTrustAccessPolicies_Result_Include_GsuiteRef::new(
            self.shared().clone(),
            format!("{}.gsuite", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessPolicies_Result_Include_IpRef {
        DataZeroTrustAccessPolicies_Result_Include_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessPolicies_Result_Include_IpListRef {
        DataZeroTrustAccessPolicies_Result_Include_IpListRef::new(
            self.shared().clone(),
            format!("{}.ip_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessPolicies_Result_Include_LinkedAppTokenRef {
        DataZeroTrustAccessPolicies_Result_Include_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessPolicies_Result_Include_LoginMethodRef {
        DataZeroTrustAccessPolicies_Result_Include_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessPolicies_Result_Include_OidcRef {
        DataZeroTrustAccessPolicies_Result_Include_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessPolicies_Result_Include_OktaRef {
        DataZeroTrustAccessPolicies_Result_Include_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessPolicies_Result_Include_SamlRef {
        DataZeroTrustAccessPolicies_Result_Include_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessPolicies_Result_Include_ServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Include_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> DataZeroTrustAccessPolicies_Result_Include_UserRiskScoreRef {
        DataZeroTrustAccessPolicies_Result_Include_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_MfaConfig {}

impl DataZeroTrustAccessPolicies_Result_MfaConfig { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_MfaConfig {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_MfaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_MfaConfig {}

impl BuildDataZeroTrustAccessPolicies_Result_MfaConfig {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_MfaConfig {
        DataZeroTrustAccessPolicies_Result_MfaConfig {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_MfaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_MfaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_MfaConfigRef {
        DataZeroTrustAccessPolicies_Result_MfaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_MfaConfigRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceToken {}

impl DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_AnyValidServiceToken {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_AnyValidServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceToken {
        DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Require_AuthContext {}

impl DataZeroTrustAccessPolicies_Result_Require_AuthContext { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_AuthContext {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_AuthContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_AuthContext {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_AuthContext {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_AuthContext {
        DataZeroTrustAccessPolicies_Result_Require_AuthContext {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_AuthContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_AuthContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_AuthContextRef {
        DataZeroTrustAccessPolicies_Result_Require_AuthContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_AuthContextRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_AuthMethod {}

impl DataZeroTrustAccessPolicies_Result_Require_AuthMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_AuthMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_AuthMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_AuthMethod {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_AuthMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_AuthMethod {
        DataZeroTrustAccessPolicies_Result_Require_AuthMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_AuthMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_AuthMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_AuthMethodRef {
        DataZeroTrustAccessPolicies_Result_Require_AuthMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_AuthMethodRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_AzureAd {}

impl DataZeroTrustAccessPolicies_Result_Require_AzureAd { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_AzureAd {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_AzureAd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_AzureAd {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_AzureAd {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_AzureAd {
        DataZeroTrustAccessPolicies_Result_Require_AzureAd {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_AzureAdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_AzureAdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_AzureAdRef {
        DataZeroTrustAccessPolicies_Result_Require_AzureAdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_AzureAdRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_Certificate {}

impl DataZeroTrustAccessPolicies_Result_Require_Certificate { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_Certificate {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_Certificate {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_Certificate {
        DataZeroTrustAccessPolicies_Result_Require_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_CertificateRef {
        DataZeroTrustAccessPolicies_Result_Require_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMember {}

impl DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMember { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMember {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMember>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMember {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMember {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMember {
        DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMember {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMemberRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_CommonName {}

impl DataZeroTrustAccessPolicies_Result_Require_CommonName { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_CommonName {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_CommonName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_CommonName {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_CommonName {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_CommonName {
        DataZeroTrustAccessPolicies_Result_Require_CommonName {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_CommonNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_CommonNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_CommonNameRef {
        DataZeroTrustAccessPolicies_Result_Require_CommonNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_CommonNameRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_DevicePosture {}

impl DataZeroTrustAccessPolicies_Result_Require_DevicePosture { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_DevicePosture {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_DevicePosture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_DevicePosture {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_DevicePosture {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_DevicePosture {
        DataZeroTrustAccessPolicies_Result_Require_DevicePosture {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_DevicePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_DevicePostureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_DevicePostureRef {
        DataZeroTrustAccessPolicies_Result_Require_DevicePostureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_DevicePostureRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_Email {}

impl DataZeroTrustAccessPolicies_Result_Require_Email { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_Email {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_Email {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_Email {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_Email {
        DataZeroTrustAccessPolicies_Result_Require_Email {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_EmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_EmailRef {
        DataZeroTrustAccessPolicies_Result_Require_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_EmailRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_EmailDomain {}

impl DataZeroTrustAccessPolicies_Result_Require_EmailDomain { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_EmailDomain {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_EmailDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_EmailDomain {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_EmailDomain {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_EmailDomain {
        DataZeroTrustAccessPolicies_Result_Require_EmailDomain {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_EmailDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_EmailDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_EmailDomainRef {
        DataZeroTrustAccessPolicies_Result_Require_EmailDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_EmailDomainRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_EmailList {}

impl DataZeroTrustAccessPolicies_Result_Require_EmailList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_EmailList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_EmailList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_EmailList {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_EmailList {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_EmailList {
        DataZeroTrustAccessPolicies_Result_Require_EmailList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_EmailListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_EmailListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_EmailListRef {
        DataZeroTrustAccessPolicies_Result_Require_EmailListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_EmailListRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_Everyone {}

impl DataZeroTrustAccessPolicies_Result_Require_Everyone { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_Everyone {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_Everyone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_Everyone {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_Everyone {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_Everyone {
        DataZeroTrustAccessPolicies_Result_Require_Everyone {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_EveryoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_EveryoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_EveryoneRef {
        DataZeroTrustAccessPolicies_Result_Require_EveryoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_EveryoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluation {}

impl DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluation { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluation {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_ExternalEvaluation {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_ExternalEvaluation {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluation {
        DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluationRef {
        DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluationRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_Geo {}

impl DataZeroTrustAccessPolicies_Result_Require_Geo { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_Geo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_Geo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_Geo {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_Geo {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_Geo {
        DataZeroTrustAccessPolicies_Result_Require_Geo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_GeoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_GeoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_GeoRef {
        DataZeroTrustAccessPolicies_Result_Require_GeoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_GeoRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_GithubOrganization {}

impl DataZeroTrustAccessPolicies_Result_Require_GithubOrganization { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_GithubOrganization {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_GithubOrganization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_GithubOrganization {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_GithubOrganization {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_GithubOrganization {
        DataZeroTrustAccessPolicies_Result_Require_GithubOrganization {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_GithubOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_GithubOrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_GithubOrganizationRef {
        DataZeroTrustAccessPolicies_Result_Require_GithubOrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_GithubOrganizationRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_Group {}

impl DataZeroTrustAccessPolicies_Result_Require_Group { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_Group {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_Group {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_Group {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_Group {
        DataZeroTrustAccessPolicies_Result_Require_Group {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_GroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_GroupRef {
        DataZeroTrustAccessPolicies_Result_Require_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_GroupRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_Gsuite {}

impl DataZeroTrustAccessPolicies_Result_Require_Gsuite { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_Gsuite {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_Gsuite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_Gsuite {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_Gsuite {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_Gsuite {
        DataZeroTrustAccessPolicies_Result_Require_Gsuite {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_GsuiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_GsuiteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_GsuiteRef {
        DataZeroTrustAccessPolicies_Result_Require_GsuiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_GsuiteRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_Ip {}

impl DataZeroTrustAccessPolicies_Result_Require_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_Ip {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_Ip {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_Ip {
        DataZeroTrustAccessPolicies_Result_Require_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_IpRef {
        DataZeroTrustAccessPolicies_Result_Require_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_IpRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_IpList {}

impl DataZeroTrustAccessPolicies_Result_Require_IpList { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_IpList {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_IpList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_IpList {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_IpList {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_IpList {
        DataZeroTrustAccessPolicies_Result_Require_IpList {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_IpListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_IpListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_IpListRef {
        DataZeroTrustAccessPolicies_Result_Require_IpListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_IpListRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_LinkedAppToken {}

impl DataZeroTrustAccessPolicies_Result_Require_LinkedAppToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_LinkedAppToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_LinkedAppToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_LinkedAppToken {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_LinkedAppToken {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_LinkedAppToken {
        DataZeroTrustAccessPolicies_Result_Require_LinkedAppToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_LinkedAppTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_LinkedAppTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_LinkedAppTokenRef {
        DataZeroTrustAccessPolicies_Result_Require_LinkedAppTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_LinkedAppTokenRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_LoginMethod {}

impl DataZeroTrustAccessPolicies_Result_Require_LoginMethod { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_LoginMethod {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_LoginMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_LoginMethod {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_LoginMethod {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_LoginMethod {
        DataZeroTrustAccessPolicies_Result_Require_LoginMethod {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_LoginMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_LoginMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_LoginMethodRef {
        DataZeroTrustAccessPolicies_Result_Require_LoginMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_LoginMethodRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_Oidc {}

impl DataZeroTrustAccessPolicies_Result_Require_Oidc { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_Oidc {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_Oidc {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_Oidc {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_Oidc {
        DataZeroTrustAccessPolicies_Result_Require_Oidc {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_OidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_OidcRef {
        DataZeroTrustAccessPolicies_Result_Require_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_OidcRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_Okta {}

impl DataZeroTrustAccessPolicies_Result_Require_Okta { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_Okta {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_Okta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_Okta {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_Okta {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_Okta {
        DataZeroTrustAccessPolicies_Result_Require_Okta {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_OktaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_OktaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_OktaRef {
        DataZeroTrustAccessPolicies_Result_Require_OktaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_OktaRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_Saml {}

impl DataZeroTrustAccessPolicies_Result_Require_Saml { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_Saml {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_Saml {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_Saml {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_Saml {
        DataZeroTrustAccessPolicies_Result_Require_Saml {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_SamlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_SamlRef {
        DataZeroTrustAccessPolicies_Result_Require_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_SamlRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_ServiceToken {}

impl DataZeroTrustAccessPolicies_Result_Require_ServiceToken { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_ServiceToken {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_ServiceToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_ServiceToken {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_ServiceToken {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_ServiceToken {
        DataZeroTrustAccessPolicies_Result_Require_ServiceToken {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_ServiceTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_ServiceTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_ServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Require_ServiceTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_ServiceTokenRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require_UserRiskScore {}

impl DataZeroTrustAccessPolicies_Result_Require_UserRiskScore { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require_UserRiskScore {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require_UserRiskScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require_UserRiskScore {}

impl BuildDataZeroTrustAccessPolicies_Result_Require_UserRiskScore {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require_UserRiskScore {
        DataZeroTrustAccessPolicies_Result_Require_UserRiskScore {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_Require_UserRiskScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_Require_UserRiskScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessPolicies_Result_Require_UserRiskScoreRef {
        DataZeroTrustAccessPolicies_Result_Require_UserRiskScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_Require_UserRiskScoreRef {
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
pub struct DataZeroTrustAccessPolicies_Result_Require {}

impl DataZeroTrustAccessPolicies_Result_Require { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result_Require {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result_Require>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result_Require {}

impl BuildDataZeroTrustAccessPolicies_Result_Require {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result_Require {
        DataZeroTrustAccessPolicies_Result_Require {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_Result_RequireRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_Result_RequireRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicies_Result_RequireRef {
        DataZeroTrustAccessPolicies_Result_RequireRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_Result_RequireRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `any_valid_service_token` after provisioning.\nAn empty object which matches on all service tokens."]
    pub fn any_valid_service_token(&self) -> DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Require_AnyValidServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.any_valid_service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_context` after provisioning.\n"]
    pub fn auth_context(&self) -> DataZeroTrustAccessPolicies_Result_Require_AuthContextRef {
        DataZeroTrustAccessPolicies_Result_Require_AuthContextRef::new(
            self.shared().clone(),
            format!("{}.auth_context", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> DataZeroTrustAccessPolicies_Result_Require_AuthMethodRef {
        DataZeroTrustAccessPolicies_Result_Require_AuthMethodRef::new(
            self.shared().clone(),
            format!("{}.auth_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `azure_ad` after provisioning.\n"]
    pub fn azure_ad(&self) -> DataZeroTrustAccessPolicies_Result_Require_AzureAdRef {
        DataZeroTrustAccessPolicies_Result_Require_AzureAdRef::new(
            self.shared().clone(),
            format!("{}.azure_ad", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> DataZeroTrustAccessPolicies_Result_Require_CertificateRef {
        DataZeroTrustAccessPolicies_Result_Require_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloudflare_account_member` after provisioning.\n"]
    pub fn cloudflare_account_member(&self) -> DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMemberRef {
        DataZeroTrustAccessPolicies_Result_Require_CloudflareAccountMemberRef::new(
            self.shared().clone(),
            format!("{}.cloudflare_account_member", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> DataZeroTrustAccessPolicies_Result_Require_CommonNameRef {
        DataZeroTrustAccessPolicies_Result_Require_CommonNameRef::new(
            self.shared().clone(),
            format!("{}.common_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> DataZeroTrustAccessPolicies_Result_Require_DevicePostureRef {
        DataZeroTrustAccessPolicies_Result_Require_DevicePostureRef::new(
            self.shared().clone(),
            format!("{}.device_posture", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> DataZeroTrustAccessPolicies_Result_Require_EmailRef {
        DataZeroTrustAccessPolicies_Result_Require_EmailRef::new(
            self.shared().clone(),
            format!("{}.email", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> DataZeroTrustAccessPolicies_Result_Require_EmailDomainRef {
        DataZeroTrustAccessPolicies_Result_Require_EmailDomainRef::new(
            self.shared().clone(),
            format!("{}.email_domain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_list` after provisioning.\n"]
    pub fn email_list(&self) -> DataZeroTrustAccessPolicies_Result_Require_EmailListRef {
        DataZeroTrustAccessPolicies_Result_Require_EmailListRef::new(
            self.shared().clone(),
            format!("{}.email_list", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `everyone` after provisioning.\nAn empty object which matches on all users."]
    pub fn everyone(&self) -> DataZeroTrustAccessPolicies_Result_Require_EveryoneRef {
        DataZeroTrustAccessPolicies_Result_Require_EveryoneRef::new(
            self.shared().clone(),
            format!("{}.everyone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluationRef {
        DataZeroTrustAccessPolicies_Result_Require_ExternalEvaluationRef::new(
            self.shared().clone(),
            format!("{}.external_evaluation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> DataZeroTrustAccessPolicies_Result_Require_GeoRef {
        DataZeroTrustAccessPolicies_Result_Require_GeoRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `github_organization` after provisioning.\n"]
    pub fn github_organization(&self) -> DataZeroTrustAccessPolicies_Result_Require_GithubOrganizationRef {
        DataZeroTrustAccessPolicies_Result_Require_GithubOrganizationRef::new(
            self.shared().clone(),
            format!("{}.github_organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> DataZeroTrustAccessPolicies_Result_Require_GroupRef {
        DataZeroTrustAccessPolicies_Result_Require_GroupRef::new(
            self.shared().clone(),
            format!("{}.group", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> DataZeroTrustAccessPolicies_Result_Require_GsuiteRef {
        DataZeroTrustAccessPolicies_Result_Require_GsuiteRef::new(
            self.shared().clone(),
            format!("{}.gsuite", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> DataZeroTrustAccessPolicies_Result_Require_IpRef {
        DataZeroTrustAccessPolicies_Result_Require_IpRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> DataZeroTrustAccessPolicies_Result_Require_IpListRef {
        DataZeroTrustAccessPolicies_Result_Require_IpListRef::new(
            self.shared().clone(),
            format!("{}.ip_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `linked_app_token` after provisioning.\n"]
    pub fn linked_app_token(&self) -> DataZeroTrustAccessPolicies_Result_Require_LinkedAppTokenRef {
        DataZeroTrustAccessPolicies_Result_Require_LinkedAppTokenRef::new(
            self.shared().clone(),
            format!("{}.linked_app_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> DataZeroTrustAccessPolicies_Result_Require_LoginMethodRef {
        DataZeroTrustAccessPolicies_Result_Require_LoginMethodRef::new(
            self.shared().clone(),
            format!("{}.login_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> DataZeroTrustAccessPolicies_Result_Require_OidcRef {
        DataZeroTrustAccessPolicies_Result_Require_OidcRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> DataZeroTrustAccessPolicies_Result_Require_OktaRef {
        DataZeroTrustAccessPolicies_Result_Require_OktaRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> DataZeroTrustAccessPolicies_Result_Require_SamlRef {
        DataZeroTrustAccessPolicies_Result_Require_SamlRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }

    #[doc = "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> DataZeroTrustAccessPolicies_Result_Require_ServiceTokenRef {
        DataZeroTrustAccessPolicies_Result_Require_ServiceTokenRef::new(
            self.shared().clone(),
            format!("{}.service_token", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_risk_score` after provisioning.\n"]
    pub fn user_risk_score(&self) -> DataZeroTrustAccessPolicies_Result_Require_UserRiskScoreRef {
        DataZeroTrustAccessPolicies_Result_Require_UserRiskScoreRef::new(
            self.shared().clone(),
            format!("{}.user_risk_score", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessPolicies_Result {}

impl DataZeroTrustAccessPolicies_Result { }

impl terrars::ToListMappable for DataZeroTrustAccessPolicies_Result {
    type O = terrars::BlockAssignable<DataZeroTrustAccessPolicies_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessPolicies_Result {}

impl BuildDataZeroTrustAccessPolicies_Result {
    pub fn build(self) -> DataZeroTrustAccessPolicies_Result {
        DataZeroTrustAccessPolicies_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessPolicies_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessPolicies_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustAccessPolicies_ResultRef {
        DataZeroTrustAccessPolicies_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessPolicies_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `app_count` after provisioning.\nNumber of access applications currently using this policy."]
    pub fn app_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `approval_groups` after provisioning.\nAdministrators who can approve a temporary authentication request."]
    pub fn approval_groups(&self) -> terrars::SetRef<DataZeroTrustAccessPolicies_Result_ApprovalGroupsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approval_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `approval_required` after provisioning.\nRequires the user to request access from an administrator at the start of each session."]
    pub fn approval_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_rules` after provisioning.\nThe rules that define how users may connect to targets secured by your application."]
    pub fn connection_rules(&self) -> DataZeroTrustAccessPolicies_Result_ConnectionRulesRef {
        DataZeroTrustAccessPolicies_Result_ConnectionRulesRef::new(
            self.shared().clone(),
            format!("{}.connection_rules", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `decision` after provisioning.\nThe action Access will take if a user matches this policy. Infrastructure application policies can only use the Allow action.\nAvailable values: \"allow\", \"deny\", \"non_identity\", \"bypass\"."]
    pub fn decision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nRules evaluated with a NOT logical operator. To match the policy, a user cannot meet any of the Exclude rules."]
    pub fn exclude(&self) -> terrars::SetRef<DataZeroTrustAccessPolicies_Result_ExcludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe UUID of the policy"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nRules evaluated with an OR logical operator. A user needs to meet only one of the Include rules."]
    pub fn include(&self) -> terrars::SetRef<DataZeroTrustAccessPolicies_Result_IncludeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }

    #[doc =
        "Get a reference to the value of field `isolation_required` after provisioning.\nRequire this application to be served in an isolated browser for users matching this policy. 'Client Web Isolation' must be on for the account in order to use this feature."]
    pub fn isolation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolation_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mfa_config` after provisioning.\nConfigures multi-factor authentication (MFA) settings."]
    pub fn mfa_config(&self) -> DataZeroTrustAccessPolicies_Result_MfaConfigRef {
        DataZeroTrustAccessPolicies_Result_MfaConfigRef::new(
            self.shared().clone(),
            format!("{}.mfa_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Access policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_prompt` after provisioning.\nA custom message that will appear on the purpose justification screen."]
    pub fn purpose_justification_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_prompt", self.base))
    }

    #[doc =
        "Get a reference to the value of field `purpose_justification_required` after provisioning.\nRequire users to enter a justification when they log in to the application."]
    pub fn purpose_justification_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purpose_justification_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require` after provisioning.\nRules evaluated with an AND logical operator. To match the policy, a user must meet all of the Require rules."]
    pub fn require(&self) -> terrars::SetRef<DataZeroTrustAccessPolicies_Result_RequireRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.require", self.base))
    }

    #[doc = "Get a reference to the value of field `reusable` after provisioning.\n"]
    pub fn reusable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reusable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nThe amount of time that tokens issued for the application will be valid. Must be in the format `300ms` or `2h45m`. Valid time units are: ns, us (or µs), ms, s, m, h."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
