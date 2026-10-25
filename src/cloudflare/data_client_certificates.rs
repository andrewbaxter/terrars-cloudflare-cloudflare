#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataClientCertificatesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataClientCertificates_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataClientCertificatesData_>,
}

#[derive(Clone)]
pub struct DataClientCertificates(std::rc::Rc<DataClientCertificates_>);

impl DataClientCertificates {
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

    #[doc = "Set the field `limit`.\nLimit to the number of records returned."]
    pub fn set_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `offset`.\nOffset the results."]
    pub fn set_offset(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().offset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nClient Certitifcate Status to filter results by.\nAvailable values: \"all\", \"active\", \"pending_reactivation\", \"pending_revocation\", \"revoked\"."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nLimit to the number of records returned."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offset` after provisioning.\nOffset the results."]
    pub fn offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataClientCertificates_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nClient Certitifcate Status to filter results by.\nAvailable values: \"all\", \"active\", \"pending_reactivation\", \"pending_revocation\", \"revoked\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataClientCertificates {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataClientCertificates { }

impl terrars::ToListMappable for DataClientCertificates {
    type O = terrars::ListRef<DataClientCertificatesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataClientCertificates_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_client_certificates".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataClientCertificates {
    pub tf_id: std::string::String,
}

impl BuildDataClientCertificates {
    pub fn build(self, stack: &mut terrars::Stack) -> DataClientCertificates {
        let out = DataClientCertificates(std::rc::Rc::new(DataClientCertificates_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataClientCertificatesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                limit: core::default::Default::default(),
                max_items: core::default::Default::default(),
                offset: core::default::Default::default(),
                status: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataClientCertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataClientCertificatesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataClientCertificatesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nLimit to the number of records returned."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offset` after provisioning.\nOffset the results."]
    pub fn offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataClientCertificates_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nClient Certitifcate Status to filter results by.\nAvailable values: \"all\", \"active\", \"pending_reactivation\", \"pending_revocation\", \"revoked\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataClientCertificates_Result_CertificateAuthority {}

impl DataClientCertificates_Result_CertificateAuthority { }

impl terrars::ToListMappable for DataClientCertificates_Result_CertificateAuthority {
    type O = terrars::BlockAssignable<DataClientCertificates_Result_CertificateAuthority>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataClientCertificates_Result_CertificateAuthority {}

impl BuildDataClientCertificates_Result_CertificateAuthority {
    pub fn build(self) -> DataClientCertificates_Result_CertificateAuthority {
        DataClientCertificates_Result_CertificateAuthority {}
    }
}

#[allow(dead_code)]
pub struct DataClientCertificates_Result_CertificateAuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataClientCertificates_Result_CertificateAuthorityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataClientCertificates_Result_CertificateAuthorityRef {
        DataClientCertificates_Result_CertificateAuthorityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataClientCertificates_Result_CertificateAuthorityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataClientCertificates_Result {}

impl DataClientCertificates_Result { }

impl terrars::ToListMappable for DataClientCertificates_Result {
    type O = terrars::BlockAssignable<DataClientCertificates_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataClientCertificates_Result {}

impl BuildDataClientCertificates_Result {
    pub fn build(self) -> DataClientCertificates_Result {
        DataClientCertificates_Result {}
    }
}

#[allow(dead_code)]
pub struct DataClientCertificates_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataClientCertificates_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataClientCertificates_ResultRef {
        DataClientCertificates_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataClientCertificates_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\nThe Client Certificate PEM."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nCertificate Authority used to issue the Client Certificate."]
    pub fn certificate_authority(&self) -> DataClientCertificates_Result_CertificateAuthorityRef {
        DataClientCertificates_Result_CertificateAuthorityRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `common_name` after provisioning.\nCommon Name of the Client Certificate."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\nCountry, provided by the CSR."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc =
        "Get a reference to the value of field `csr` after provisioning.\nThe Certificate Signing Request (CSR). Must be newline-encoded."]
    pub fn csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csr", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nDate that the Client Certificate expires."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint_sha256` after provisioning.\nUnique identifier of the Client Certificate."]
    pub fn fingerprint_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint_sha256", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nClient Certificate Tag"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issued_on` after provisioning.\nDate that the Client Certificate was issued by the Certificate Authority."]
    pub fn issued_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issued_on", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation, provided by the CSR."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nOrganization, provided by the CSR."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `organizational_unit` after provisioning.\nOrganizational Unit, provided by the CSR."]
    pub fn organizational_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serial_number` after provisioning.\nThe serial number on the created Client Certificate."]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.base))
    }

    #[doc =
        "Get a reference to the value of field `signature` after provisioning.\nThe type of hash used for the Client Certificate.."]
    pub fn signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature", self.base))
    }

    #[doc = "Get a reference to the value of field `ski` after provisioning.\nSubject Key Identifier."]
    pub fn ski(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ski", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState, provided by the CSR."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nClient Certificates may be active or revoked, and the pending_reactivation or pending_revocation represent in-progress asynchronous transitions.\nAvailable values: \"active\", \"pending_reactivation\", \"pending_revocation\", \"revoked\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `validity_days` after provisioning.\nThe number of days the Client Certificate will be valid after the issued_on date."]
    pub fn validity_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validity_days", self.base))
    }
}
