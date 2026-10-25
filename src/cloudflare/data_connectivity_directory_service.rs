#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataConnectivityDirectoryServiceData_ {
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
    filter: std::option::Option<DataConnectivityDirectoryService_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataConnectivityDirectoryService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataConnectivityDirectoryServiceData_>,
}

#[derive(Clone)]
pub struct DataConnectivityDirectoryService(std::rc::Rc<DataConnectivityDirectoryService_>);

impl DataConnectivityDirectoryService {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataConnectivityDirectoryService_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_id`.\n"]
    pub fn set_service_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_protocol` after provisioning.\nAvailable values: \"postgresql\", \"mysql\"."]
    pub fn app_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataConnectivityDirectoryService_FilterRef {
        DataConnectivityDirectoryService_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> DataConnectivityDirectoryService_HostRef {
        DataConnectivityDirectoryService_HostRef::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `https_port` after provisioning.\n"]
    pub fn https_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_port` after provisioning.\n"]
    pub fn tcp_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls_settings` after provisioning.\nTLS settings for a connectivity service.\n\nIf omitted, the default mode (`verify_full`) is used."]
    pub fn tls_settings(&self) -> DataConnectivityDirectoryService_TlsSettingsRef {
        DataConnectivityDirectoryService_TlsSettingsRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"tcp\", \"http\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataConnectivityDirectoryService {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataConnectivityDirectoryService { }

impl terrars::ToListMappable for DataConnectivityDirectoryService {
    type O = terrars::ListRef<DataConnectivityDirectoryServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataConnectivityDirectoryService_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_connectivity_directory_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectivityDirectoryService {
    pub tf_id: std::string::String,
}

impl BuildDataConnectivityDirectoryService {
    pub fn build(self, stack: &mut terrars::Stack) -> DataConnectivityDirectoryService {
        let out = DataConnectivityDirectoryService(std::rc::Rc::new(DataConnectivityDirectoryService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataConnectivityDirectoryServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                service_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectivityDirectoryServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataConnectivityDirectoryServiceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_protocol` after provisioning.\nAvailable values: \"postgresql\", \"mysql\"."]
    pub fn app_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataConnectivityDirectoryService_FilterRef {
        DataConnectivityDirectoryService_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> DataConnectivityDirectoryService_HostRef {
        DataConnectivityDirectoryService_HostRef::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `https_port` after provisioning.\n"]
    pub fn https_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_port` after provisioning.\n"]
    pub fn tcp_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls_settings` after provisioning.\nTLS settings for a connectivity service.\n\nIf omitted, the default mode (`verify_full`) is used."]
    pub fn tls_settings(&self) -> DataConnectivityDirectoryService_TlsSettingsRef {
        DataConnectivityDirectoryService_TlsSettingsRef::new(
            self.shared().clone(),
            format!("{}.tls_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"tcp\", \"http\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataConnectivityDirectoryService_Filter {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectivityDirectoryService_Filter {
    #[doc = "Set the field `type_`.\nAvailable values: \"tcp\", \"http\"."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectivityDirectoryService_Filter {
    type O = terrars::BlockAssignable<DataConnectivityDirectoryService_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectivityDirectoryService_Filter {}

impl BuildDataConnectivityDirectoryService_Filter {
    pub fn build(self) -> DataConnectivityDirectoryService_Filter {
        DataConnectivityDirectoryService_Filter { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataConnectivityDirectoryService_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryService_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataConnectivityDirectoryService_FilterRef {
        DataConnectivityDirectoryService_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectivityDirectoryService_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"tcp\", \"http\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectivityDirectoryService_Host_Network {}

impl DataConnectivityDirectoryService_Host_Network { }

impl terrars::ToListMappable for DataConnectivityDirectoryService_Host_Network {
    type O = terrars::BlockAssignable<DataConnectivityDirectoryService_Host_Network>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectivityDirectoryService_Host_Network {}

impl BuildDataConnectivityDirectoryService_Host_Network {
    pub fn build(self) -> DataConnectivityDirectoryService_Host_Network {
        DataConnectivityDirectoryService_Host_Network {}
    }
}

#[allow(dead_code)]
pub struct DataConnectivityDirectoryService_Host_NetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryService_Host_NetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectivityDirectoryService_Host_NetworkRef {
        DataConnectivityDirectoryService_Host_NetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectivityDirectoryService_Host_NetworkRef {
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
pub struct DataConnectivityDirectoryService_Host_ResolverNetwork {}

impl DataConnectivityDirectoryService_Host_ResolverNetwork { }

impl terrars::ToListMappable for DataConnectivityDirectoryService_Host_ResolverNetwork {
    type O = terrars::BlockAssignable<DataConnectivityDirectoryService_Host_ResolverNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectivityDirectoryService_Host_ResolverNetwork {}

impl BuildDataConnectivityDirectoryService_Host_ResolverNetwork {
    pub fn build(self) -> DataConnectivityDirectoryService_Host_ResolverNetwork {
        DataConnectivityDirectoryService_Host_ResolverNetwork {}
    }
}

#[allow(dead_code)]
pub struct DataConnectivityDirectoryService_Host_ResolverNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryService_Host_ResolverNetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectivityDirectoryService_Host_ResolverNetworkRef {
        DataConnectivityDirectoryService_Host_ResolverNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectivityDirectoryService_Host_ResolverNetworkRef {
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
pub struct DataConnectivityDirectoryService_Host {}

impl DataConnectivityDirectoryService_Host { }

impl terrars::ToListMappable for DataConnectivityDirectoryService_Host {
    type O = terrars::BlockAssignable<DataConnectivityDirectoryService_Host>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectivityDirectoryService_Host {}

impl BuildDataConnectivityDirectoryService_Host {
    pub fn build(self) -> DataConnectivityDirectoryService_Host {
        DataConnectivityDirectoryService_Host {}
    }
}

#[allow(dead_code)]
pub struct DataConnectivityDirectoryService_HostRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryService_HostRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataConnectivityDirectoryService_HostRef {
        DataConnectivityDirectoryService_HostRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectivityDirectoryService_HostRef {
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
    pub fn network(&self) -> DataConnectivityDirectoryService_Host_NetworkRef {
        DataConnectivityDirectoryService_Host_NetworkRef::new(
            self.shared().clone(),
            format!("{}.network", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resolver_network` after provisioning.\n"]
    pub fn resolver_network(&self) -> DataConnectivityDirectoryService_Host_ResolverNetworkRef {
        DataConnectivityDirectoryService_Host_ResolverNetworkRef::new(
            self.shared().clone(),
            format!("{}.resolver_network", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataConnectivityDirectoryService_TlsSettings {}

impl DataConnectivityDirectoryService_TlsSettings { }

impl terrars::ToListMappable for DataConnectivityDirectoryService_TlsSettings {
    type O = terrars::BlockAssignable<DataConnectivityDirectoryService_TlsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectivityDirectoryService_TlsSettings {}

impl BuildDataConnectivityDirectoryService_TlsSettings {
    pub fn build(self) -> DataConnectivityDirectoryService_TlsSettings {
        DataConnectivityDirectoryService_TlsSettings {}
    }
}

#[allow(dead_code)]
pub struct DataConnectivityDirectoryService_TlsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectivityDirectoryService_TlsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectivityDirectoryService_TlsSettingsRef {
        DataConnectivityDirectoryService_TlsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectivityDirectoryService_TlsSettingsRef {
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
