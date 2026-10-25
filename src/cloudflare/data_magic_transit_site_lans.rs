#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicTransitSiteLansData_ {
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
    site_id: terrars::PrimField<std::string::String>,
}

struct DataMagicTransitSiteLans_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicTransitSiteLansData_>,
}

#[derive(Clone)]
pub struct DataMagicTransitSiteLans(std::rc::Rc<DataMagicTransitSiteLans_>);

impl DataMagicTransitSiteLans {
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

    #[doc = "Set the field `account_id`.\nIdentifier"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataMagicTransitSiteLans_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataMagicTransitSiteLans {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicTransitSiteLans { }

impl terrars::ToListMappable for DataMagicTransitSiteLans {
    type O = terrars::ListRef<DataMagicTransitSiteLansRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicTransitSiteLans_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_site_lans".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicTransitSiteLans {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub site_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMagicTransitSiteLans {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicTransitSiteLans {
        let out = DataMagicTransitSiteLans(std::rc::Rc::new(DataMagicTransitSiteLans_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicTransitSiteLansData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                site_id: self.site_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMagicTransitSiteLansRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteLansRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicTransitSiteLansRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataMagicTransitSiteLans_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteLans_Result_Nat {}

impl DataMagicTransitSiteLans_Result_Nat { }

impl terrars::ToListMappable for DataMagicTransitSiteLans_Result_Nat {
    type O = terrars::BlockAssignable<DataMagicTransitSiteLans_Result_Nat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteLans_Result_Nat {}

impl BuildDataMagicTransitSiteLans_Result_Nat {
    pub fn build(self) -> DataMagicTransitSiteLans_Result_Nat {
        DataMagicTransitSiteLans_Result_Nat {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteLans_Result_NatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteLans_Result_NatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitSiteLans_Result_NatRef {
        DataMagicTransitSiteLans_Result_NatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteLans_Result_NatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `static_prefix` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn static_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteLans_Result_RoutedSubnets_Nat {}

impl DataMagicTransitSiteLans_Result_RoutedSubnets_Nat { }

impl terrars::ToListMappable for DataMagicTransitSiteLans_Result_RoutedSubnets_Nat {
    type O = terrars::BlockAssignable<DataMagicTransitSiteLans_Result_RoutedSubnets_Nat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteLans_Result_RoutedSubnets_Nat {}

impl BuildDataMagicTransitSiteLans_Result_RoutedSubnets_Nat {
    pub fn build(self) -> DataMagicTransitSiteLans_Result_RoutedSubnets_Nat {
        DataMagicTransitSiteLans_Result_RoutedSubnets_Nat {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteLans_Result_RoutedSubnets_NatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteLans_Result_RoutedSubnets_NatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicTransitSiteLans_Result_RoutedSubnets_NatRef {
        DataMagicTransitSiteLans_Result_RoutedSubnets_NatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteLans_Result_RoutedSubnets_NatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `static_prefix` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn static_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteLans_Result_RoutedSubnets {}

impl DataMagicTransitSiteLans_Result_RoutedSubnets { }

impl terrars::ToListMappable for DataMagicTransitSiteLans_Result_RoutedSubnets {
    type O = terrars::BlockAssignable<DataMagicTransitSiteLans_Result_RoutedSubnets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteLans_Result_RoutedSubnets {}

impl BuildDataMagicTransitSiteLans_Result_RoutedSubnets {
    pub fn build(self) -> DataMagicTransitSiteLans_Result_RoutedSubnets {
        DataMagicTransitSiteLans_Result_RoutedSubnets {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteLans_Result_RoutedSubnetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteLans_Result_RoutedSubnetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicTransitSiteLans_Result_RoutedSubnetsRef {
        DataMagicTransitSiteLans_Result_RoutedSubnetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteLans_Result_RoutedSubnetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nat` after provisioning.\n"]
    pub fn nat(&self) -> DataMagicTransitSiteLans_Result_RoutedSubnets_NatRef {
        DataMagicTransitSiteLans_Result_RoutedSubnets_NatRef::new(
            self.shared().clone(),
            format!("{}.nat", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `next_hop` after provisioning.\nA valid IPv4 address."]
    pub fn next_hop(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelay {}

impl DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelay { }

impl terrars::ToListMappable for DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelay {
    type O = terrars::BlockAssignable<DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelay {}

impl BuildDataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelay {
    pub fn build(self) -> DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelay {
        DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelay {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelayRef {
        DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `server_addresses` after provisioning.\nList of DHCP server IPs."]
    pub fn server_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptions {}

impl DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptions { }

impl terrars::ToListMappable for DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptions {
    type O = terrars::BlockAssignable<DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptions {}

impl BuildDataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptions {
    pub fn build(self) -> DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptions {
        DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptions {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptionsRef {
        DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nDHCP option number (1-254). Options 0 and 255 are reserved by RFC 2132. Options 3, 6, and 51 are not allowed because they conflict with connector-managed configuration."]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the option value. text: a string (max 255 bytes). hex: colon-separated hex bytes (e.g. \"01:04:aa:bb:cc\", max 255 bytes). ip: an IPv4 address (e.g. \"10.20.30.40\"). byte: an unsigned integer 0-255 (1 byte). short: an unsigned integer 0-65535 (2 bytes). integer: an unsigned integer 0-4294967295 (4 bytes).\nAvailable values: \"text\", \"hex\", \"ip\", \"byte\", \"short\", \"integer\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe option value, interpreted according to the type field."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer {}

impl DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer { }

impl terrars::ToListMappable for DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer {
    type O = terrars::BlockAssignable<DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer {}

impl BuildDataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer {
    pub fn build(self) -> DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer {
        DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServerRef {
        DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dhcp_options` after provisioning.\nOptional list of custom DHCP options to include in DHCP responses. Only valid when DHCP server is enabled."]
    pub fn dhcp_options(
        &self,
    ) -> terrars::ListRef<DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServer_DhcpOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dhcp_options", self.base))
    }

    #[doc = "Get a reference to the value of field `dhcp_pool_end` after provisioning.\nA valid IPv4 address."]
    pub fn dhcp_pool_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dhcp_pool_end", self.base))
    }

    #[doc = "Get a reference to the value of field `dhcp_pool_start` after provisioning.\nA valid IPv4 address."]
    pub fn dhcp_pool_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dhcp_pool_start", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_server` after provisioning.\nA valid IPv4 address."]
    pub fn dns_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_server", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reservations` after provisioning.\nMapping of MAC addresses to IP addresses"]
    pub fn reservations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.reservations", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteLans_Result_StaticAddressing {}

impl DataMagicTransitSiteLans_Result_StaticAddressing { }

impl terrars::ToListMappable for DataMagicTransitSiteLans_Result_StaticAddressing {
    type O = terrars::BlockAssignable<DataMagicTransitSiteLans_Result_StaticAddressing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteLans_Result_StaticAddressing {}

impl BuildDataMagicTransitSiteLans_Result_StaticAddressing {
    pub fn build(self) -> DataMagicTransitSiteLans_Result_StaticAddressing {
        DataMagicTransitSiteLans_Result_StaticAddressing {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteLans_Result_StaticAddressingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteLans_Result_StaticAddressingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicTransitSiteLans_Result_StaticAddressingRef {
        DataMagicTransitSiteLans_Result_StaticAddressingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteLans_Result_StaticAddressingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `dhcp_relay` after provisioning.\n"]
    pub fn dhcp_relay(&self) -> DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelayRef {
        DataMagicTransitSiteLans_Result_StaticAddressing_DhcpRelayRef::new(
            self.shared().clone(),
            format!("{}.dhcp_relay", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dhcp_server` after provisioning.\n"]
    pub fn dhcp_server(&self) -> DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServerRef {
        DataMagicTransitSiteLans_Result_StaticAddressing_DhcpServerRef::new(
            self.shared().clone(),
            format!("{}.dhcp_server", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `secondary_address` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn secondary_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `virtual_address` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn virtual_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_address", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteLans_Result {}

impl DataMagicTransitSiteLans_Result { }

impl terrars::ToListMappable for DataMagicTransitSiteLans_Result {
    type O = terrars::BlockAssignable<DataMagicTransitSiteLans_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteLans_Result {}

impl BuildDataMagicTransitSiteLans_Result {
    pub fn build(self) -> DataMagicTransitSiteLans_Result {
        DataMagicTransitSiteLans_Result {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteLans_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteLans_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitSiteLans_ResultRef {
        DataMagicTransitSiteLans_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteLans_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bond_id` after provisioning.\n"]
    pub fn bond_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bond_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ha_link` after provisioning.\nmark true to use this LAN for HA probing. only works for site with HA turned on. only one LAN can be set as the ha_link."]
    pub fn ha_link(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ha_link", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_breakout` after provisioning.\nmark true to use this LAN for source-based breakout traffic"]
    pub fn is_breakout(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_breakout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_prioritized` after provisioning.\nmark true to use this LAN for source-based prioritized traffic"]
    pub fn is_prioritized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_prioritized", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `nat` after provisioning.\n"]
    pub fn nat(&self) -> DataMagicTransitSiteLans_Result_NatRef {
        DataMagicTransitSiteLans_Result_NatRef::new(self.shared().clone(), format!("{}.nat", self.base))
    }

    #[doc = "Get a reference to the value of field `physport` after provisioning.\n"]
    pub fn physport(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physport", self.base))
    }

    #[doc = "Get a reference to the value of field `routed_subnets` after provisioning.\n"]
    pub fn routed_subnets(&self) -> terrars::ListRef<DataMagicTransitSiteLans_Result_RoutedSubnetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routed_subnets", self.base))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `static_addressing` after provisioning.\nIf the site is not configured in high availability mode, this configuration is optional (if omitted, use DHCP). However, if in high availability mode, static_address is required along with secondary and virtual address."]
    pub fn static_addressing(&self) -> DataMagicTransitSiteLans_Result_StaticAddressingRef {
        DataMagicTransitSiteLans_Result_StaticAddressingRef::new(
            self.shared().clone(),
            format!("{}.static_addressing", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vlan_tag` after provisioning.\nVLAN ID. Use zero for untagged."]
    pub fn vlan_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag", self.base))
    }
}
