#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataOriginCaCertificateData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataOriginCaCertificate_Filter>,
}

struct DataOriginCaCertificate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOriginCaCertificateData_>,
}

#[derive(Clone)]
pub struct DataOriginCaCertificate(std::rc::Rc<DataOriginCaCertificate_>);

impl DataOriginCaCertificate {
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

    #[doc = "Set the field `certificate_id`.\nThe x509 serial number of the Origin CA certificate."]
    pub fn set_certificate_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataOriginCaCertificate_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `certificate` after provisioning.\nThe Origin CA certificate. Will be newline-encoded."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_id` after provisioning.\nThe x509 serial number of the Origin CA certificate."]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `csr` after provisioning.\nThe Certificate Signing Request (CSR). Must be newline-encoded."]
    pub fn csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expires_on` after provisioning.\nWhen the certificate will expire."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataOriginCaCertificate_FilterRef {
        DataOriginCaCertificate_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nArray of hostnames or wildcard names bound to the certificate.\nHostnames must be fully qualified domain names (FQDNs) belonging to zones on your account (e.g., `example.com` or `sub.example.com`). Wildcards are supported only as a `*.` prefix for a single level (e.g., `*.example.com`). Double wildcards (`*.*.example.com`) and interior wildcards (`foo.*.example.com`) are not allowed. The wildcard suffix must be a multi-label domain (`*.example.com` is valid, but `*.com` is not). Unicode/IDN hostnames are accepted and automatically converted to punycode."]
    pub fn hostnames(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe x509 serial number of the Origin CA certificate."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `request_type` after provisioning.\nSignature type desired on certificate (\"origin-rsa\" (rsa), \"origin-ecc\" (ecdsa), or \"keyless-certificate\" (for Keyless SSL servers).\nAvailable values: \"origin-rsa\", \"origin-ecc\", \"keyless-certificate\"."]
    pub fn request_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requested_validity` after provisioning.\nThe number of days for which the certificate should be valid.\nAvailable values: 7, 30, 90, 365, 730, 1095, 5475."]
    pub fn requested_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requested_validity", self.extract_ref()))
    }
}

impl terrars::Referable for DataOriginCaCertificate {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOriginCaCertificate { }

impl terrars::ToListMappable for DataOriginCaCertificate {
    type O = terrars::ListRef<DataOriginCaCertificateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOriginCaCertificate_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_origin_ca_certificate".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOriginCaCertificate {
    pub tf_id: std::string::String,
}

impl BuildDataOriginCaCertificate {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOriginCaCertificate {
        let out = DataOriginCaCertificate(std::rc::Rc::new(DataOriginCaCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOriginCaCertificateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                certificate_id: core::default::Default::default(),
                filter: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOriginCaCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOriginCaCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOriginCaCertificateRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `certificate` after provisioning.\nThe Origin CA certificate. Will be newline-encoded."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_id` after provisioning.\nThe x509 serial number of the Origin CA certificate."]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `csr` after provisioning.\nThe Certificate Signing Request (CSR). Must be newline-encoded."]
    pub fn csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expires_on` after provisioning.\nWhen the certificate will expire."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataOriginCaCertificate_FilterRef {
        DataOriginCaCertificate_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nArray of hostnames or wildcard names bound to the certificate.\nHostnames must be fully qualified domain names (FQDNs) belonging to zones on your account (e.g., `example.com` or `sub.example.com`). Wildcards are supported only as a `*.` prefix for a single level (e.g., `*.example.com`). Double wildcards (`*.*.example.com`) and interior wildcards (`foo.*.example.com`) are not allowed. The wildcard suffix must be a multi-label domain (`*.example.com` is valid, but `*.com` is not). Unicode/IDN hostnames are accepted and automatically converted to punycode."]
    pub fn hostnames(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe x509 serial number of the Origin CA certificate."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `request_type` after provisioning.\nSignature type desired on certificate (\"origin-rsa\" (rsa), \"origin-ecc\" (ecdsa), or \"keyless-certificate\" (for Keyless SSL servers).\nAvailable values: \"origin-rsa\", \"origin-ecc\", \"keyless-certificate\"."]
    pub fn request_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requested_validity` after provisioning.\nThe number of days for which the certificate should be valid.\nAvailable values: 7, 30, 90, 365, 730, 1095, 5475."]
    pub fn requested_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requested_validity", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOriginCaCertificate_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offset: std::option::Option<terrars::PrimField<f64>>,
    zone_id: terrars::PrimField<std::string::String>,
}

impl DataOriginCaCertificate_Filter {
    #[doc = "Set the field `limit`.\nLimit to the number of records returned."]
    pub fn set_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `offset`.\nOffset the results."]
    pub fn set_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.offset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOriginCaCertificate_Filter {
    type O = terrars::BlockAssignable<DataOriginCaCertificate_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOriginCaCertificate_Filter {
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildDataOriginCaCertificate_Filter {
    pub fn build(self) -> DataOriginCaCertificate_Filter {
        DataOriginCaCertificate_Filter {
            limit: core::default::Default::default(),
            offset: core::default::Default::default(),
            zone_id: self.zone_id,
        }
    }
}

#[allow(dead_code)]
pub struct DataOriginCaCertificate_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOriginCaCertificate_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOriginCaCertificate_FilterRef {
        DataOriginCaCertificate_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOriginCaCertificate_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nLimit to the number of records returned."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc = "Get a reference to the value of field `offset` after provisioning.\nOffset the results."]
    pub fn offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offset", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.base))
    }
}
