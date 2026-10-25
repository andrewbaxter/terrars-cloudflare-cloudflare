#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataClientCertificateData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataClientCertificate_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataClientCertificate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataClientCertificateData_>,
}

#[derive(Clone)]
pub struct DataClientCertificate(std::rc::Rc<DataClientCertificate_>);

impl DataClientCertificate {
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

    #[doc = "Set the field `client_certificate_id`.\nClient Certificate Tag"]
    pub fn set_client_certificate_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_certificate_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataClientCertificate_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\nThe Client Certificate PEM."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nCertificate Authority used to issue the Client Certificate."]
    pub fn certificate_authority(&self) -> DataClientCertificate_CertificateAuthorityRef {
        DataClientCertificate_CertificateAuthorityRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_certificate_id` after provisioning.\nClient Certificate Tag"]
    pub fn client_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `common_name` after provisioning.\nCommon Name of the Client Certificate."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\nCountry, provided by the CSR."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `csr` after provisioning.\nThe Certificate Signing Request (CSR). Must be newline-encoded."]
    pub fn csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nDate that the Client Certificate expires."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataClientCertificate_FilterRef {
        DataClientCertificate_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint_sha256` after provisioning.\nUnique identifier of the Client Certificate."]
    pub fn fingerprint_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint_sha256", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nClient Certificate Tag"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issued_on` after provisioning.\nDate that the Client Certificate was issued by the Certificate Authority."]
    pub fn issued_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issued_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation, provided by the CSR."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nOrganization, provided by the CSR."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organizational_unit` after provisioning.\nOrganizational Unit, provided by the CSR."]
    pub fn organizational_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `serial_number` after provisioning.\nThe serial number on the created Client Certificate."]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `signature` after provisioning.\nThe type of hash used for the Client Certificate.."]
    pub fn signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ski` after provisioning.\nSubject Key Identifier."]
    pub fn ski(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ski", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState, provided by the CSR."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nClient Certificates may be active or revoked, and the pending_reactivation or pending_revocation represent in-progress asynchronous transitions.\nAvailable values: \"active\", \"pending_reactivation\", \"pending_revocation\", \"revoked\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validity_days` after provisioning.\nThe number of days the Client Certificate will be valid after the issued_on date."]
    pub fn validity_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validity_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataClientCertificate {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataClientCertificate { }

impl terrars::ToListMappable for DataClientCertificate {
    type O = terrars::ListRef<DataClientCertificateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataClientCertificate_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_client_certificate".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataClientCertificate {
    pub tf_id: std::string::String,
}

impl BuildDataClientCertificate {
    pub fn build(self, stack: &mut terrars::Stack) -> DataClientCertificate {
        let out = DataClientCertificate(std::rc::Rc::new(DataClientCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataClientCertificateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                client_certificate_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataClientCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataClientCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataClientCertificateRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\nThe Client Certificate PEM."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nCertificate Authority used to issue the Client Certificate."]
    pub fn certificate_authority(&self) -> DataClientCertificate_CertificateAuthorityRef {
        DataClientCertificate_CertificateAuthorityRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_certificate_id` after provisioning.\nClient Certificate Tag"]
    pub fn client_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `common_name` after provisioning.\nCommon Name of the Client Certificate."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\nCountry, provided by the CSR."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `csr` after provisioning.\nThe Certificate Signing Request (CSR). Must be newline-encoded."]
    pub fn csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nDate that the Client Certificate expires."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataClientCertificate_FilterRef {
        DataClientCertificate_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint_sha256` after provisioning.\nUnique identifier of the Client Certificate."]
    pub fn fingerprint_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint_sha256", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nClient Certificate Tag"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issued_on` after provisioning.\nDate that the Client Certificate was issued by the Certificate Authority."]
    pub fn issued_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issued_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation, provided by the CSR."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nOrganization, provided by the CSR."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organizational_unit` after provisioning.\nOrganizational Unit, provided by the CSR."]
    pub fn organizational_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `serial_number` after provisioning.\nThe serial number on the created Client Certificate."]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `signature` after provisioning.\nThe type of hash used for the Client Certificate.."]
    pub fn signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ski` after provisioning.\nSubject Key Identifier."]
    pub fn ski(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ski", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState, provided by the CSR."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nClient Certificates may be active or revoked, and the pending_reactivation or pending_revocation represent in-progress asynchronous transitions.\nAvailable values: \"active\", \"pending_reactivation\", \"pending_revocation\", \"revoked\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validity_days` after provisioning.\nThe number of days the Client Certificate will be valid after the issued_on date."]
    pub fn validity_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validity_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataClientCertificate_CertificateAuthority {}

impl DataClientCertificate_CertificateAuthority { }

impl terrars::ToListMappable for DataClientCertificate_CertificateAuthority {
    type O = terrars::BlockAssignable<DataClientCertificate_CertificateAuthority>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataClientCertificate_CertificateAuthority {}

impl BuildDataClientCertificate_CertificateAuthority {
    pub fn build(self) -> DataClientCertificate_CertificateAuthority {
        DataClientCertificate_CertificateAuthority {}
    }
}

#[allow(dead_code)]
pub struct DataClientCertificate_CertificateAuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataClientCertificate_CertificateAuthorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataClientCertificate_CertificateAuthorityRef {
        DataClientCertificate_CertificateAuthorityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataClientCertificate_CertificateAuthorityRef {
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
pub struct DataClientCertificate_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataClientCertificate_Filter {
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

    #[doc =
        "Set the field `status`.\nClient Certitifcate Status to filter results by.\nAvailable values: \"all\", \"active\", \"pending_reactivation\", \"pending_revocation\", \"revoked\"."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataClientCertificate_Filter {
    type O = terrars::BlockAssignable<DataClientCertificate_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataClientCertificate_Filter {}

impl BuildDataClientCertificate_Filter {
    pub fn build(self) -> DataClientCertificate_Filter {
        DataClientCertificate_Filter {
            limit: core::default::Default::default(),
            offset: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataClientCertificate_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataClientCertificate_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataClientCertificate_FilterRef {
        DataClientCertificate_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataClientCertificate_FilterRef {
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

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nClient Certitifcate Status to filter results by.\nAvailable values: \"all\", \"active\", \"pending_reactivation\", \"pending_revocation\", \"revoked\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
