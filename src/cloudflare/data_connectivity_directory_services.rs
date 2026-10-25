#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataConnectivityDirectoryServicesData_ {
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
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataConnectivityDirectoryServices_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataConnectivityDirectoryServicesData_>,
}

#[derive(Clone)]
pub struct DataConnectivityDirectoryServices(std::rc::Rc<DataConnectivityDirectoryServices_>);

impl DataConnectivityDirectoryServices {
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

    #[doc = "Set the field `account_id`.\nAccount identifier"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nAvailable values: \"tcp\", \"http\"."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataConnectivityDirectoryServices_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"tcp\", \"http\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataConnectivityDirectoryServices {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataConnectivityDirectoryServices { }

impl terrars::ToListMappable for DataConnectivityDirectoryServices {
    type O = terrars::ListRef<DataConnectivityDirectoryServicesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataConnectivityDirectoryServices_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_connectivity_directory_services".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectivityDirectoryServices {
    pub tf_id: std::string::String,
}

impl BuildDataConnectivityDirectoryServices {
    pub fn build(self, stack: &mut terrars::Stack) -> DataConnectivityDirectoryServices {
        let out = DataConnectivityDirectoryServices(std::rc::Rc::new(DataConnectivityDirectoryServices_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataConnectivityDirectoryServicesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                type_: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectivityDirectoryServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryServicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataConnectivityDirectoryServicesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataConnectivityDirectoryServices_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"tcp\", \"http\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataConnectivityDirectoryServices_Result_Host_Network {}

impl DataConnectivityDirectoryServices_Result_Host_Network { }

impl terrars::ToListMappable for DataConnectivityDirectoryServices_Result_Host_Network {
    type O = terrars::BlockAssignable<DataConnectivityDirectoryServices_Result_Host_Network>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectivityDirectoryServices_Result_Host_Network {}

impl BuildDataConnectivityDirectoryServices_Result_Host_Network {
    pub fn build(self) -> DataConnectivityDirectoryServices_Result_Host_Network {
        DataConnectivityDirectoryServices_Result_Host_Network {}
    }
}

#[allow(dead_code)]
pub struct DataConnectivityDirectoryServices_Result_Host_NetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryServices_Result_Host_NetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectivityDirectoryServices_Result_Host_NetworkRef {
        DataConnectivityDirectoryServices_Result_Host_NetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectivityDirectoryServices_Result_Host_NetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\n"]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectivityDirectoryServices_Result_Host_ResolverNetwork {}

impl DataConnectivityDirectoryServices_Result_Host_ResolverNetwork { }

impl terrars::ToListMappable for DataConnectivityDirectoryServices_Result_Host_ResolverNetwork {
    type O = terrars::BlockAssignable<DataConnectivityDirectoryServices_Result_Host_ResolverNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectivityDirectoryServices_Result_Host_ResolverNetwork {}

impl BuildDataConnectivityDirectoryServices_Result_Host_ResolverNetwork {
    pub fn build(self) -> DataConnectivityDirectoryServices_Result_Host_ResolverNetwork {
        DataConnectivityDirectoryServices_Result_Host_ResolverNetwork {}
    }
}

#[allow(dead_code)]
pub struct DataConnectivityDirectoryServices_Result_Host_ResolverNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryServices_Result_Host_ResolverNetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectivityDirectoryServices_Result_Host_ResolverNetworkRef {
        DataConnectivityDirectoryServices_Result_Host_ResolverNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectivityDirectoryServices_Result_Host_ResolverNetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolver_ips` after provisioning.\n"]
    pub fn resolver_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resolver_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\n"]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectivityDirectoryServices_Result_Host {}

impl DataConnectivityDirectoryServices_Result_Host { }

impl terrars::ToListMappable for DataConnectivityDirectoryServices_Result_Host {
    type O = terrars::BlockAssignable<DataConnectivityDirectoryServices_Result_Host>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectivityDirectoryServices_Result_Host {}

impl BuildDataConnectivityDirectoryServices_Result_Host {
    pub fn build(self) -> DataConnectivityDirectoryServices_Result_Host {
        DataConnectivityDirectoryServices_Result_Host {}
    }
}

#[allow(dead_code)]
pub struct DataConnectivityDirectoryServices_Result_HostRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryServices_Result_HostRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectivityDirectoryServices_Result_HostRef {
        DataConnectivityDirectoryServices_Result_HostRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectivityDirectoryServices_Result_HostRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4` after provisioning.\n"]
    pub fn ipv4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> DataConnectivityDirectoryServices_Result_Host_NetworkRef {
        DataConnectivityDirectoryServices_Result_Host_NetworkRef::new(
            self.shared().clone(),
            format!("{}.network", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resolver_network` after provisioning.\n"]
    pub fn resolver_network(&self) -> DataConnectivityDirectoryServices_Result_Host_ResolverNetworkRef {
        DataConnectivityDirectoryServices_Result_Host_ResolverNetworkRef::new(
            self.shared().clone(),
            format!("{}.resolver_network", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataConnectivityDirectoryServices_Result_TlsSettings {}

impl DataConnectivityDirectoryServices_Result_TlsSettings { }

impl terrars::ToListMappable for DataConnectivityDirectoryServices_Result_TlsSettings {
    type O = terrars::BlockAssignable<DataConnectivityDirectoryServices_Result_TlsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectivityDirectoryServices_Result_TlsSettings {}

impl BuildDataConnectivityDirectoryServices_Result_TlsSettings {
    pub fn build(self) -> DataConnectivityDirectoryServices_Result_TlsSettings {
        DataConnectivityDirectoryServices_Result_TlsSettings {}
    }
}

#[allow(dead_code)]
pub struct DataConnectivityDirectoryServices_Result_TlsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryServices_Result_TlsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectivityDirectoryServices_Result_TlsSettingsRef {
        DataConnectivityDirectoryServices_Result_TlsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectivityDirectoryServices_Result_TlsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cert_verification_mode` after provisioning.\nTLS certificate verification mode for the connection to the origin.\n\n- `\"verify_full\"` — verify certificate chain and hostname (default)\n- `\"verify_ca\"` — verify certificate chain only, skip hostname check\n- `\"disabled\"` — do not verify the server certificate at all"]
    pub fn cert_verification_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert_verification_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectivityDirectoryServices_Result {}

impl DataConnectivityDirectoryServices_Result { }

impl terrars::ToListMappable for DataConnectivityDirectoryServices_Result {
    type O = terrars::BlockAssignable<DataConnectivityDirectoryServices_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectivityDirectoryServices_Result {}

impl BuildDataConnectivityDirectoryServices_Result {
    pub fn build(self) -> DataConnectivityDirectoryServices_Result {
        DataConnectivityDirectoryServices_Result {}
    }
}

#[allow(dead_code)]
pub struct DataConnectivityDirectoryServices_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryServices_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataConnectivityDirectoryServices_ResultRef {
        DataConnectivityDirectoryServices_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectivityDirectoryServices_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_protocol` after provisioning.\nAvailable values: \"postgresql\", \"mysql\"."]
    pub fn app_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> DataConnectivityDirectoryServices_Result_HostRef {
        DataConnectivityDirectoryServices_Result_HostRef::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.base))
    }

    #[doc = "Get a reference to the value of field `https_port` after provisioning.\n"]
    pub fn https_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_port", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_port` after provisioning.\n"]
    pub fn tcp_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_settings` after provisioning.\nTLS settings for a connectivity service.\n\nIf omitted, the default mode (`verify_full`) is used."]
    pub fn tls_settings(&self) -> DataConnectivityDirectoryServices_Result_TlsSettingsRef {
        DataConnectivityDirectoryServices_Result_TlsSettingsRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"tcp\", \"http\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
