#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataCustomHostnamesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_origin_server: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<DataCustomHostnames_Hostname>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCustomHostnames_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCustomHostnamesData_>,
}

#[derive(Clone)]
pub struct DataCustomHostnames(std::rc::Rc<DataCustomHostnames_>);

impl DataCustomHostnames {
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
        "Set the field `certificate_authority`.\nFilter by the certificate authority that issued the SSL certificate.\nAvailable values: \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub fn set_certificate_authority(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_authority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_origin_server`.\nFilter by custom origin server name."]
    pub fn set_custom_origin_server(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_origin_server = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nDirection to order hostnames.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(self, v: impl std::convert::Into<DataCustomHostnames_Hostname>) -> Self {
        self.0.data.borrow_mut().hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hostname_status`.\nFilter by the hostname's activation status.\nAvailable values: \"active\", \"pending\", \"active_redeploying\", \"moved\", \"pending_deletion\", \"deleted\", \"pending_blocked\", \"pending_migration\", \"pending_provisioned\", \"test_pending\", \"test_active\", \"test_active_apex\", \"test_blocked\", \"test_failed\", \"provisioned\", \"blocked\"."]
    pub fn set_hostname_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hostname_status = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `id`.\nHostname ID to match against. This ID was generated and returned during the initial custom_hostname creation. This parameter cannot be used with the 'hostname', 'hostname.exact', 'hostname.contain', or 'hostname.startsWith' parameters."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\nField to order hostnames by.\nAvailable values: \"ssl\", \"ssl_status\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl`.\nWhether to filter hostnames based on if they have SSL enabled.\nAvailable values: 0, 1."]
    pub fn set_ssl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_status`.\nFilter by SSL certificate status.\nAvailable values: \"initializing\", \"pending_validation\", \"deleted\", \"pending_issuance\", \"pending_deployment\", \"pending_deletion\", \"pending_expiration\", \"expired\", \"active\", \"initializing_timed_out\", \"validation_timed_out\", \"issuance_timed_out\", \"deployment_timed_out\", \"deletion_timed_out\", \"pending_cleanup\", \"staging_deployment\", \"staging_active\", \"deactivating\", \"inactive\", \"backup_issued\", \"holding_deployment\"."]
    pub fn set_ssl_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ssl_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wildcard`.\nFilter by whether the custom hostname is a wildcard hostname."]
    pub fn set_wildcard(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wildcard = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nFilter by the certificate authority that issued the SSL certificate.\nAvailable values: \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_server` after provisioning.\nFilter by custom origin server name."]
    pub fn custom_origin_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_server", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order hostnames.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> DataCustomHostnames_HostnameRef {
        DataCustomHostnames_HostnameRef::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_status` after provisioning.\nFilter by the hostname's activation status.\nAvailable values: \"active\", \"pending\", \"active_redeploying\", \"moved\", \"pending_deletion\", \"deleted\", \"pending_blocked\", \"pending_migration\", \"pending_provisioned\", \"test_pending\", \"test_active\", \"test_active_apex\", \"test_blocked\", \"test_failed\", \"provisioned\", \"blocked\"."]
    pub fn hostname_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nHostname ID to match against. This ID was generated and returned during the initial custom_hostname creation. This parameter cannot be used with the 'hostname', 'hostname.exact', 'hostname.contain', or 'hostname.startsWith' parameters."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order hostnames by.\nAvailable values: \"ssl\", \"ssl_status\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataCustomHostnames_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssl` after provisioning.\nWhether to filter hostnames based on if they have SSL enabled.\nAvailable values: 0, 1."]
    pub fn ssl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssl_status` after provisioning.\nFilter by SSL certificate status.\nAvailable values: \"initializing\", \"pending_validation\", \"deleted\", \"pending_issuance\", \"pending_deployment\", \"pending_deletion\", \"pending_expiration\", \"expired\", \"active\", \"initializing_timed_out\", \"validation_timed_out\", \"issuance_timed_out\", \"deployment_timed_out\", \"deletion_timed_out\", \"pending_cleanup\", \"staging_deployment\", \"staging_active\", \"deactivating\", \"inactive\", \"backup_issued\", \"holding_deployment\"."]
    pub fn ssl_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wildcard` after provisioning.\nFilter by whether the custom hostname is a wildcard hostname."]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataCustomHostnames {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCustomHostnames { }

impl terrars::ToListMappable for DataCustomHostnames {
    type O = terrars::ListRef<DataCustomHostnamesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCustomHostnames_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_custom_hostnames".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCustomHostnames {
    pub tf_id: std::string::String,
}

impl BuildDataCustomHostnames {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCustomHostnames {
        let out = DataCustomHostnames(std::rc::Rc::new(DataCustomHostnames_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCustomHostnamesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                certificate_authority: core::default::Default::default(),
                custom_origin_server: core::default::Default::default(),
                direction: core::default::Default::default(),
                hostname: core::default::Default::default(),
                hostname_status: core::default::Default::default(),
                id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                order: core::default::Default::default(),
                ssl: core::default::Default::default(),
                ssl_status: core::default::Default::default(),
                wildcard: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCustomHostnamesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomHostnamesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCustomHostnamesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nFilter by the certificate authority that issued the SSL certificate.\nAvailable values: \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_server` after provisioning.\nFilter by custom origin server name."]
    pub fn custom_origin_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_server", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order hostnames.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> DataCustomHostnames_HostnameRef {
        DataCustomHostnames_HostnameRef::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_status` after provisioning.\nFilter by the hostname's activation status.\nAvailable values: \"active\", \"pending\", \"active_redeploying\", \"moved\", \"pending_deletion\", \"deleted\", \"pending_blocked\", \"pending_migration\", \"pending_provisioned\", \"test_pending\", \"test_active\", \"test_active_apex\", \"test_blocked\", \"test_failed\", \"provisioned\", \"blocked\"."]
    pub fn hostname_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nHostname ID to match against. This ID was generated and returned during the initial custom_hostname creation. This parameter cannot be used with the 'hostname', 'hostname.exact', 'hostname.contain', or 'hostname.startsWith' parameters."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order hostnames by.\nAvailable values: \"ssl\", \"ssl_status\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataCustomHostnames_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssl` after provisioning.\nWhether to filter hostnames based on if they have SSL enabled.\nAvailable values: 0, 1."]
    pub fn ssl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssl_status` after provisioning.\nFilter by SSL certificate status.\nAvailable values: \"initializing\", \"pending_validation\", \"deleted\", \"pending_issuance\", \"pending_deployment\", \"pending_deletion\", \"pending_expiration\", \"expired\", \"active\", \"initializing_timed_out\", \"validation_timed_out\", \"issuance_timed_out\", \"deployment_timed_out\", \"deletion_timed_out\", \"pending_cleanup\", \"staging_deployment\", \"staging_active\", \"deactivating\", \"inactive\", \"backup_issued\", \"holding_deployment\"."]
    pub fn ssl_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wildcard` after provisioning.\nFilter by whether the custom hostname is a wildcard hostname."]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCustomHostnames_Hostname {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starts_with: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCustomHostnames_Hostname {
    #[doc =
        "Set the field `contain`.\nFilters hostnames by a substring match on the hostname value. This parameter cannot be used with the 'id', 'hostname', 'hostname.exact', or 'hostname.startsWith' parameters."]
    pub fn set_contain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contain = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exact`.\nFully qualified domain name to match against. This parameter cannot be used with the 'id', 'hostname', 'hostname.contain', or 'hostname.startsWith' parameters."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `starts_with`.\nFilters hostnames by a prefix match on the hostname value. This parameter cannot be used with the 'id', 'hostname', 'hostname.exact', or 'hostname.contain' parameters."]
    pub fn set_starts_with(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.starts_with = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCustomHostnames_Hostname {
    type O = terrars::BlockAssignable<DataCustomHostnames_Hostname>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomHostnames_Hostname {}

impl BuildDataCustomHostnames_Hostname {
    pub fn build(self) -> DataCustomHostnames_Hostname {
        DataCustomHostnames_Hostname {
            contain: core::default::Default::default(),
            exact: core::default::Default::default(),
            starts_with: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCustomHostnames_HostnameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomHostnames_HostnameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCustomHostnames_HostnameRef {
        DataCustomHostnames_HostnameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomHostnames_HostnameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contain` after provisioning.\nFilters hostnames by a substring match on the hostname value. This parameter cannot be used with the 'id', 'hostname', 'hostname.exact', or 'hostname.startsWith' parameters."]
    pub fn contain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nFully qualified domain name to match against. This parameter cannot be used with the 'id', 'hostname', 'hostname.contain', or 'hostname.startsWith' parameters."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `starts_with` after provisioning.\nFilters hostnames by a prefix match on the hostname value. This parameter cannot be used with the 'id', 'hostname', 'hostname.exact', or 'hostname.contain' parameters."]
    pub fn starts_with(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starts_with", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomHostnames_Result_OwnershipVerification {}

impl DataCustomHostnames_Result_OwnershipVerification { }

impl terrars::ToListMappable for DataCustomHostnames_Result_OwnershipVerification {
    type O = terrars::BlockAssignable<DataCustomHostnames_Result_OwnershipVerification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomHostnames_Result_OwnershipVerification {}

impl BuildDataCustomHostnames_Result_OwnershipVerification {
    pub fn build(self) -> DataCustomHostnames_Result_OwnershipVerification {
        DataCustomHostnames_Result_OwnershipVerification {}
    }
}

#[allow(dead_code)]
pub struct DataCustomHostnames_Result_OwnershipVerificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomHostnames_Result_OwnershipVerificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCustomHostnames_Result_OwnershipVerificationRef {
        DataCustomHostnames_Result_OwnershipVerificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomHostnames_Result_OwnershipVerificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDNS Name for record."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nDNS Record type.\nAvailable values: \"txt\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nContent for the record."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomHostnames_Result_OwnershipVerificationHttp {}

impl DataCustomHostnames_Result_OwnershipVerificationHttp { }

impl terrars::ToListMappable for DataCustomHostnames_Result_OwnershipVerificationHttp {
    type O = terrars::BlockAssignable<DataCustomHostnames_Result_OwnershipVerificationHttp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomHostnames_Result_OwnershipVerificationHttp {}

impl BuildDataCustomHostnames_Result_OwnershipVerificationHttp {
    pub fn build(self) -> DataCustomHostnames_Result_OwnershipVerificationHttp {
        DataCustomHostnames_Result_OwnershipVerificationHttp {}
    }
}

#[allow(dead_code)]
pub struct DataCustomHostnames_Result_OwnershipVerificationHttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomHostnames_Result_OwnershipVerificationHttpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCustomHostnames_Result_OwnershipVerificationHttpRef {
        DataCustomHostnames_Result_OwnershipVerificationHttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomHostnames_Result_OwnershipVerificationHttpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_body` after provisioning.\nToken to be served."]
    pub fn http_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_url` after provisioning.\nThe HTTP URL that will be checked during custom hostname verification and where the customer should host the token."]
    pub fn http_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomHostnames_Result_Ssl_DcvDelegationRecords {}

impl DataCustomHostnames_Result_Ssl_DcvDelegationRecords { }

impl terrars::ToListMappable for DataCustomHostnames_Result_Ssl_DcvDelegationRecords {
    type O = terrars::BlockAssignable<DataCustomHostnames_Result_Ssl_DcvDelegationRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomHostnames_Result_Ssl_DcvDelegationRecords {}

impl BuildDataCustomHostnames_Result_Ssl_DcvDelegationRecords {
    pub fn build(self) -> DataCustomHostnames_Result_Ssl_DcvDelegationRecords {
        DataCustomHostnames_Result_Ssl_DcvDelegationRecords {}
    }
}

#[allow(dead_code)]
pub struct DataCustomHostnames_Result_Ssl_DcvDelegationRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomHostnames_Result_Ssl_DcvDelegationRecordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCustomHostnames_Result_Ssl_DcvDelegationRecordsRef {
        DataCustomHostnames_Result_Ssl_DcvDelegationRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomHostnames_Result_Ssl_DcvDelegationRecordsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cname` after provisioning.\nThe CNAME record hostname for DCV delegation."]
    pub fn cname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cname_target` after provisioning.\nThe CNAME record target value for DCV delegation."]
    pub fn cname_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname_target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `emails` after provisioning.\nThe set of email addresses that the certificate authority (CA) will use to complete domain validation."]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_body` after provisioning.\nThe content that the certificate authority (CA) will expect to find at the http_url during the domain validation."]
    pub fn http_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_url` after provisioning.\nThe url that will be checked during domain validation."]
    pub fn http_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_url", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the validation record."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `txt_name` after provisioning.\nThe hostname that the certificate authority (CA) will check for a TXT record during domain validation ."]
    pub fn txt_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.txt_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `txt_value` after provisioning.\nThe TXT record that the certificate authority (CA) will check during domain validation."]
    pub fn txt_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.txt_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomHostnames_Result_Ssl_Settings {}

impl DataCustomHostnames_Result_Ssl_Settings { }

impl terrars::ToListMappable for DataCustomHostnames_Result_Ssl_Settings {
    type O = terrars::BlockAssignable<DataCustomHostnames_Result_Ssl_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomHostnames_Result_Ssl_Settings {}

impl BuildDataCustomHostnames_Result_Ssl_Settings {
    pub fn build(self) -> DataCustomHostnames_Result_Ssl_Settings {
        DataCustomHostnames_Result_Ssl_Settings {}
    }
}

#[allow(dead_code)]
pub struct DataCustomHostnames_Result_Ssl_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomHostnames_Result_Ssl_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCustomHostnames_Result_Ssl_SettingsRef {
        DataCustomHostnames_Result_Ssl_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomHostnames_Result_Ssl_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ciphers` after provisioning.\nAn allowlist of ciphers for TLS termination. These ciphers must be in the BoringSSL format."]
    pub fn ciphers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ciphers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `early_hints` after provisioning.\nWhether or not Early Hints is enabled.\nAvailable values: \"on\", \"off\"."]
    pub fn early_hints(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.early_hints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http2` after provisioning.\nWhether or not HTTP2 is enabled.\nAvailable values: \"on\", \"off\"."]
    pub fn http2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_tls_version` after provisioning.\nThe minimum TLS version supported.\nAvailable values: \"1.0\", \"1.1\", \"1.2\", \"1.3\"."]
    pub fn min_tls_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_tls_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_1_3` after provisioning.\nWhether or not TLS 1.3 is enabled.\nAvailable values: \"on\", \"off\"."]
    pub fn tls_1_3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_1_3", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomHostnames_Result_Ssl_ValidationErrors {}

impl DataCustomHostnames_Result_Ssl_ValidationErrors { }

impl terrars::ToListMappable for DataCustomHostnames_Result_Ssl_ValidationErrors {
    type O = terrars::BlockAssignable<DataCustomHostnames_Result_Ssl_ValidationErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomHostnames_Result_Ssl_ValidationErrors {}

impl BuildDataCustomHostnames_Result_Ssl_ValidationErrors {
    pub fn build(self) -> DataCustomHostnames_Result_Ssl_ValidationErrors {
        DataCustomHostnames_Result_Ssl_ValidationErrors {}
    }
}

#[allow(dead_code)]
pub struct DataCustomHostnames_Result_Ssl_ValidationErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomHostnames_Result_Ssl_ValidationErrorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCustomHostnames_Result_Ssl_ValidationErrorsRef {
        DataCustomHostnames_Result_Ssl_ValidationErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomHostnames_Result_Ssl_ValidationErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\nA domain validation error."]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomHostnames_Result_Ssl_ValidationRecords {}

impl DataCustomHostnames_Result_Ssl_ValidationRecords { }

impl terrars::ToListMappable for DataCustomHostnames_Result_Ssl_ValidationRecords {
    type O = terrars::BlockAssignable<DataCustomHostnames_Result_Ssl_ValidationRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomHostnames_Result_Ssl_ValidationRecords {}

impl BuildDataCustomHostnames_Result_Ssl_ValidationRecords {
    pub fn build(self) -> DataCustomHostnames_Result_Ssl_ValidationRecords {
        DataCustomHostnames_Result_Ssl_ValidationRecords {}
    }
}

#[allow(dead_code)]
pub struct DataCustomHostnames_Result_Ssl_ValidationRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomHostnames_Result_Ssl_ValidationRecordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCustomHostnames_Result_Ssl_ValidationRecordsRef {
        DataCustomHostnames_Result_Ssl_ValidationRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomHostnames_Result_Ssl_ValidationRecordsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cname` after provisioning.\nThe CNAME record hostname for DCV delegation."]
    pub fn cname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cname_target` after provisioning.\nThe CNAME record target value for DCV delegation."]
    pub fn cname_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname_target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `emails` after provisioning.\nThe set of email addresses that the certificate authority (CA) will use to complete domain validation."]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_body` after provisioning.\nThe content that the certificate authority (CA) will expect to find at the http_url during the domain validation."]
    pub fn http_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_url` after provisioning.\nThe url that will be checked during domain validation."]
    pub fn http_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_url", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the validation record."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `txt_name` after provisioning.\nThe hostname that the certificate authority (CA) will check for a TXT record during domain validation ."]
    pub fn txt_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.txt_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `txt_value` after provisioning.\nThe TXT record that the certificate authority (CA) will check during domain validation."]
    pub fn txt_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.txt_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomHostnames_Result_Ssl {}

impl DataCustomHostnames_Result_Ssl { }

impl terrars::ToListMappable for DataCustomHostnames_Result_Ssl {
    type O = terrars::BlockAssignable<DataCustomHostnames_Result_Ssl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomHostnames_Result_Ssl {}

impl BuildDataCustomHostnames_Result_Ssl {
    pub fn build(self) -> DataCustomHostnames_Result_Ssl {
        DataCustomHostnames_Result_Ssl {}
    }
}

#[allow(dead_code)]
pub struct DataCustomHostnames_Result_SslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomHostnames_Result_SslRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCustomHostnames_Result_SslRef {
        DataCustomHostnames_Result_SslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomHostnames_Result_SslRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bundle_method` after provisioning.\nA ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it.\nAvailable values: \"ubiquitous\", \"optimal\", \"force\"."]
    pub fn bundle_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nThe Certificate Authority that will issue the certificate.\nAvailable values: \"digicert\", \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_certificate` after provisioning.\nIf a custom uploaded certificate is used."]
    pub fn custom_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_csr_id` after provisioning.\nThe identifier for the Custom CSR that was used."]
    pub fn custom_csr_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_csr_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_key` after provisioning.\nThe key for a custom uploaded certificate."]
    pub fn custom_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dcv_delegation_records` after provisioning.\nDCV Delegation records for domain validation."]
    pub fn dcv_delegation_records(&self) -> terrars::ListRef<DataCustomHostnames_Result_Ssl_DcvDelegationRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dcv_delegation_records", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nThe time the custom certificate expires on."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nA list of Hostnames on a custom uploaded certificate."]
    pub fn hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nCustom hostname SSL identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issuer` after provisioning.\nThe issuer on a custom uploaded certificate."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nDomain control validation (DCV) method used for this hostname.\nAvailable values: \"http\", \"txt\", \"email\"."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serial_number` after provisioning.\nThe serial number on a custom uploaded certificate."]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> DataCustomHostnames_Result_Ssl_SettingsRef {
        DataCustomHostnames_Result_Ssl_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `signature` after provisioning.\nThe signature on a custom uploaded certificate."]
    pub fn signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the hostname's SSL certificates.\nAvailable values: \"initializing\", \"pending_validation\", \"deleted\", \"pending_issuance\", \"pending_deployment\", \"pending_deletion\", \"pending_expiration\", \"expired\", \"active\", \"initializing_timed_out\", \"validation_timed_out\", \"issuance_timed_out\", \"deployment_timed_out\", \"deletion_timed_out\", \"pending_cleanup\", \"staging_deployment\", \"staging_active\", \"deactivating\", \"inactive\", \"backup_issued\", \"holding_deployment\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nLevel of validation to be used for this hostname. Domain validation (dv) must be used.\nAvailable values: \"dv\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uploaded_on` after provisioning.\nThe time the custom certificate was uploaded."]
    pub fn uploaded_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uploaded_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `validation_errors` after provisioning.\nDomain validation errors that have been received by the certificate authority (CA)."]
    pub fn validation_errors(&self) -> terrars::ListRef<DataCustomHostnames_Result_Ssl_ValidationErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `validation_records` after provisioning.\n"]
    pub fn validation_records(&self) -> terrars::ListRef<DataCustomHostnames_Result_Ssl_ValidationRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_records", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wildcard` after provisioning.\nIndicates whether the certificate covers a wildcard."]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCustomHostnames_Result {}

impl DataCustomHostnames_Result { }

impl terrars::ToListMappable for DataCustomHostnames_Result {
    type O = terrars::BlockAssignable<DataCustomHostnames_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCustomHostnames_Result {}

impl BuildDataCustomHostnames_Result {
    pub fn build(self) -> DataCustomHostnames_Result {
        DataCustomHostnames_Result {}
    }
}

#[allow(dead_code)]
pub struct DataCustomHostnames_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCustomHostnames_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCustomHostnames_ResultRef {
        DataCustomHostnames_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCustomHostnames_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThis is the time the hostname was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_metadata` after provisioning.\nUnique key/value metadata for this hostname. These are per-hostname (customer) settings."]
    pub fn custom_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_server` after provisioning.\na valid hostname that’s been added to your DNS zone as an A, AAAA, or CNAME record."]
    pub fn custom_origin_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_server", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_sni` after provisioning.\nA hostname that will be sent to your custom origin server as SNI for TLS handshake. This can be a valid subdomain of the zone or custom origin server name or the string ':request_host_header:' which will cause the host header in the request to be used as SNI. Not configurable with default/fallback origin server."]
    pub fn custom_origin_sni(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_sni", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe custom hostname that will point to your hostname via CNAME."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nCustom hostname identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ownership_verification` after provisioning.\nThis is a record which can be placed to activate a hostname."]
    pub fn ownership_verification(&self) -> DataCustomHostnames_Result_OwnershipVerificationRef {
        DataCustomHostnames_Result_OwnershipVerificationRef::new(
            self.shared().clone(),
            format!("{}.ownership_verification", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `ownership_verification_http` after provisioning.\nThis presents the token to be served by the given http url to activate a hostname."]
    pub fn ownership_verification_http(&self) -> DataCustomHostnames_Result_OwnershipVerificationHttpRef {
        DataCustomHostnames_Result_OwnershipVerificationHttpRef::new(
            self.shared().clone(),
            format!("{}.ownership_verification_http", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> DataCustomHostnames_Result_SslRef {
        DataCustomHostnames_Result_SslRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the hostname's activation.\nAvailable values: \"active\", \"pending\", \"active_redeploying\", \"moved\", \"pending_deletion\", \"deleted\", \"pending_blocked\", \"pending_migration\", \"pending_provisioned\", \"test_pending\", \"test_active\", \"test_active_apex\", \"test_blocked\", \"test_failed\", \"provisioned\", \"blocked\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `verification_errors` after provisioning.\nThese are errors that were encountered while trying to activate a hostname."]
    pub fn verification_errors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verification_errors", self.base))
    }
}
