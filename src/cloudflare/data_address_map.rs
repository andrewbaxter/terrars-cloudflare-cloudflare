#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAddressMapData_ {
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
    address_map_id: terrars::PrimField<std::string::String>,
}

struct DataAddressMap_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAddressMapData_>,
}

#[derive(Clone)]
pub struct DataAddressMap(std::rc::Rc<DataAddressMap_>);

impl DataAddressMap {
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

    #[doc = "Set the field `account_id`.\nIdentifier of a Cloudflare account."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nIdentifier of a Cloudflare account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address_map_id` after provisioning.\nIdentifier of an Address Map."]
    pub fn address_map_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_map_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `can_delete` after provisioning.\nIf set to false, then the Address Map cannot be deleted via API. This is true for Cloudflare-managed maps."]
    pub fn can_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_delete", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `can_modify_ips` after provisioning.\nIf set to false, then the IPs on the Address Map cannot be modified via the API. This is true for Cloudflare-managed maps."]
    pub fn can_modify_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_modify_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_sni` after provisioning.\nIf you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map. If Cloudflare receives a TLS handshake from a client without an SNI, it will respond with the default SNI on those IPs. The default SNI can be any valid zone or subdomain owned by the account."]
    pub fn default_sni(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_sni", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description field which may be used to describe the types of IPs or zones on the map."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the Address Map is enabled or not. Cloudflare's DNS will not respond with IP addresses on an Address Map until the map is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of an Address Map."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ips` after provisioning.\nThe set of IPs on the Address Map."]
    pub fn ips(&self) -> terrars::ListRef<DataAddressMap_IpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memberships` after provisioning.\nZones and Accounts which will be assigned IPs on this Address Map. A zone membership will take priority over an account membership."]
    pub fn memberships(&self) -> terrars::ListRef<DataAddressMap_MembershipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memberships", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataAddressMap {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAddressMap { }

impl terrars::ToListMappable for DataAddressMap {
    type O = terrars::ListRef<DataAddressMapRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAddressMap_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_address_map".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAddressMap {
    pub tf_id: std::string::String,
    #[doc = "Identifier of an Address Map."]
    pub address_map_id: terrars::PrimField<std::string::String>,
}

impl BuildDataAddressMap {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAddressMap {
        let out = DataAddressMap(std::rc::Rc::new(DataAddressMap_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAddressMapData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                address_map_id: self.address_map_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAddressMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAddressMapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAddressMapRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nIdentifier of a Cloudflare account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address_map_id` after provisioning.\nIdentifier of an Address Map."]
    pub fn address_map_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_map_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `can_delete` after provisioning.\nIf set to false, then the Address Map cannot be deleted via API. This is true for Cloudflare-managed maps."]
    pub fn can_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_delete", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `can_modify_ips` after provisioning.\nIf set to false, then the IPs on the Address Map cannot be modified via the API. This is true for Cloudflare-managed maps."]
    pub fn can_modify_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_modify_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_sni` after provisioning.\nIf you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map. If Cloudflare receives a TLS handshake from a client without an SNI, it will respond with the default SNI on those IPs. The default SNI can be any valid zone or subdomain owned by the account."]
    pub fn default_sni(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_sni", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description field which may be used to describe the types of IPs or zones on the map."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the Address Map is enabled or not. Cloudflare's DNS will not respond with IP addresses on an Address Map until the map is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of an Address Map."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ips` after provisioning.\nThe set of IPs on the Address Map."]
    pub fn ips(&self) -> terrars::ListRef<DataAddressMap_IpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memberships` after provisioning.\nZones and Accounts which will be assigned IPs on this Address Map. A zone membership will take priority over an account membership."]
    pub fn memberships(&self) -> terrars::ListRef<DataAddressMap_MembershipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memberships", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAddressMap_Ips {}

impl DataAddressMap_Ips { }

impl terrars::ToListMappable for DataAddressMap_Ips {
    type O = terrars::BlockAssignable<DataAddressMap_Ips>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAddressMap_Ips {}

impl BuildDataAddressMap_Ips {
    pub fn build(self) -> DataAddressMap_Ips {
        DataAddressMap_Ips {}
    }
}

#[allow(dead_code)]
pub struct DataAddressMap_IpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAddressMap_IpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAddressMap_IpsRef {
        DataAddressMap_IpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAddressMap_IpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 or IPv6 address."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAddressMap_Memberships {}

impl DataAddressMap_Memberships { }

impl terrars::ToListMappable for DataAddressMap_Memberships {
    type O = terrars::BlockAssignable<DataAddressMap_Memberships>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAddressMap_Memberships {}

impl BuildDataAddressMap_Memberships {
    pub fn build(self) -> DataAddressMap_Memberships {
        DataAddressMap_Memberships {}
    }
}

#[allow(dead_code)]
pub struct DataAddressMap_MembershipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAddressMap_MembershipsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAddressMap_MembershipsRef {
        DataAddressMap_MembershipsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAddressMap_MembershipsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `can_delete` after provisioning.\nControls whether the membership can be deleted via the API or not."]
    pub fn can_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_delete", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nThe identifier for the membership (eg. a zone or account tag)."]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe type of the membership.\nAvailable values: \"zone\", \"account\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }
}
