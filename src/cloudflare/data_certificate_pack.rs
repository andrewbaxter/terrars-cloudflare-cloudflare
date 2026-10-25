#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataCertificatePackData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_pack_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataCertificatePack_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCertificatePack_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCertificatePackData_>,
}

#[derive(Clone)]
pub struct DataCertificatePack(std::rc::Rc<DataCertificatePack_>);

impl DataCertificatePack {
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

    #[doc = "Set the field `certificate_pack_id`.\nThe unique identifier for a certificate_pack."]
    pub fn set_certificate_pack_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_pack_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataCertificatePack_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nCertificate Authority selected for the order.  For information on any certificate authority specific details or restrictions [see this page for more details](https://developers.cloudflare.com/ssl/reference/certificate-authorities).\nAvailable values: \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_pack_id` after provisioning.\nThe unique identifier for a certificate_pack."]
    pub fn certificate_pack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_pack_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificates` after provisioning.\nArray of certificates in this pack."]
    pub fn certificates(&self) -> terrars::ListRef<DataCertificatePack_CertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_branding` after provisioning.\nWhether or not to add Cloudflare Branding for the order.  This will add a subdomain of sni.cloudflaressl.com as the Common Name if set to true."]
    pub fn cloudflare_branding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_branding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dcv_delegation_records` after provisioning.\nDCV Delegation records for domain validation."]
    pub fn dcv_delegation_records(&self) -> terrars::ListRef<DataCertificatePack_DcvDelegationRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dcv_delegation_records", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataCertificatePack_FilterRef {
        DataCertificatePack_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nComma separated list of valid host names for the certificate packs. Must contain the zone apex, may not contain more than 50 hosts, and may not be empty."]
    pub fn hosts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for a certificate_pack."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `primary_certificate` after provisioning.\nIdentifier of the primary certificate in a pack."]
    pub fn primary_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of certificate pack.\nAvailable values: \"initializing\", \"pending_validation\", \"deleted\", \"pending_issuance\", \"pending_deployment\", \"pending_deletion\", \"pending_expiration\", \"expired\", \"active\", \"initializing_timed_out\", \"validation_timed_out\", \"issuance_timed_out\", \"deployment_timed_out\", \"deletion_timed_out\", \"pending_cleanup\", \"staging_deployment\", \"staging_active\", \"deactivating\", \"inactive\", \"backup_issued\", \"holding_deployment\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of certificate pack.\nAvailable values: \"mh_custom\", \"managed_hostname\", \"sni_custom\", \"universal\", \"advanced\", \"total_tls\", \"keyless\", \"legacy_custom\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_errors` after provisioning.\nDomain validation errors that have been received by the certificate authority (CA)."]
    pub fn validation_errors(&self) -> terrars::ListRef<DataCertificatePack_ValidationErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_method` after provisioning.\nValidation Method selected for the order.\nAvailable values: \"txt\", \"http\", \"email\"."]
    pub fn validation_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_records` after provisioning.\nCertificates' validation records."]
    pub fn validation_records(&self) -> terrars::ListRef<DataCertificatePack_ValidationRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_records", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validity_days` after provisioning.\nValidity Days selected for the order.\nAvailable values: 14, 30, 90, 365."]
    pub fn validity_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validity_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataCertificatePack {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCertificatePack { }

impl terrars::ToListMappable for DataCertificatePack {
    type O = terrars::ListRef<DataCertificatePackRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCertificatePack_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_certificate_pack".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCertificatePack {
    pub tf_id: std::string::String,
}

impl BuildDataCertificatePack {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCertificatePack {
        let out = DataCertificatePack(std::rc::Rc::new(DataCertificatePack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCertificatePackData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                certificate_pack_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCertificatePackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificatePackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCertificatePackRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nCertificate Authority selected for the order.  For information on any certificate authority specific details or restrictions [see this page for more details](https://developers.cloudflare.com/ssl/reference/certificate-authorities).\nAvailable values: \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_pack_id` after provisioning.\nThe unique identifier for a certificate_pack."]
    pub fn certificate_pack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_pack_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificates` after provisioning.\nArray of certificates in this pack."]
    pub fn certificates(&self) -> terrars::ListRef<DataCertificatePack_CertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_branding` after provisioning.\nWhether or not to add Cloudflare Branding for the order.  This will add a subdomain of sni.cloudflaressl.com as the Common Name if set to true."]
    pub fn cloudflare_branding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_branding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dcv_delegation_records` after provisioning.\nDCV Delegation records for domain validation."]
    pub fn dcv_delegation_records(&self) -> terrars::ListRef<DataCertificatePack_DcvDelegationRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dcv_delegation_records", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataCertificatePack_FilterRef {
        DataCertificatePack_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nComma separated list of valid host names for the certificate packs. Must contain the zone apex, may not contain more than 50 hosts, and may not be empty."]
    pub fn hosts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for a certificate_pack."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `primary_certificate` after provisioning.\nIdentifier of the primary certificate in a pack."]
    pub fn primary_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of certificate pack.\nAvailable values: \"initializing\", \"pending_validation\", \"deleted\", \"pending_issuance\", \"pending_deployment\", \"pending_deletion\", \"pending_expiration\", \"expired\", \"active\", \"initializing_timed_out\", \"validation_timed_out\", \"issuance_timed_out\", \"deployment_timed_out\", \"deletion_timed_out\", \"pending_cleanup\", \"staging_deployment\", \"staging_active\", \"deactivating\", \"inactive\", \"backup_issued\", \"holding_deployment\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of certificate pack.\nAvailable values: \"mh_custom\", \"managed_hostname\", \"sni_custom\", \"universal\", \"advanced\", \"total_tls\", \"keyless\", \"legacy_custom\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_errors` after provisioning.\nDomain validation errors that have been received by the certificate authority (CA)."]
    pub fn validation_errors(&self) -> terrars::ListRef<DataCertificatePack_ValidationErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_method` after provisioning.\nValidation Method selected for the order.\nAvailable values: \"txt\", \"http\", \"email\"."]
    pub fn validation_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_records` after provisioning.\nCertificates' validation records."]
    pub fn validation_records(&self) -> terrars::ListRef<DataCertificatePack_ValidationRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_records", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validity_days` after provisioning.\nValidity Days selected for the order.\nAvailable values: 14, 30, 90, 365."]
    pub fn validity_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validity_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCertificatePack_Certificates_GeoRestrictions {}

impl DataCertificatePack_Certificates_GeoRestrictions { }

impl terrars::ToListMappable for DataCertificatePack_Certificates_GeoRestrictions {
    type O = terrars::BlockAssignable<DataCertificatePack_Certificates_GeoRestrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCertificatePack_Certificates_GeoRestrictions {}

impl BuildDataCertificatePack_Certificates_GeoRestrictions {
    pub fn build(self) -> DataCertificatePack_Certificates_GeoRestrictions {
        DataCertificatePack_Certificates_GeoRestrictions {}
    }
}

#[allow(dead_code)]
pub struct DataCertificatePack_Certificates_GeoRestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificatePack_Certificates_GeoRestrictionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCertificatePack_Certificates_GeoRestrictionsRef {
        DataCertificatePack_Certificates_GeoRestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCertificatePack_Certificates_GeoRestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nAvailable values: \"us\", \"eu\", \"highest_security\"."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCertificatePack_Certificates {}

impl DataCertificatePack_Certificates { }

impl terrars::ToListMappable for DataCertificatePack_Certificates {
    type O = terrars::BlockAssignable<DataCertificatePack_Certificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCertificatePack_Certificates {}

impl BuildDataCertificatePack_Certificates {
    pub fn build(self) -> DataCertificatePack_Certificates {
        DataCertificatePack_Certificates {}
    }
}

#[allow(dead_code)]
pub struct DataCertificatePack_CertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificatePack_CertificatesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCertificatePack_CertificatesRef {
        DataCertificatePack_CertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCertificatePack_CertificatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bundle_method` after provisioning.\nCertificate bundle method."]
    pub fn bundle_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nWhen the certificate from the authority expires."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `geo_restrictions` after provisioning.\nSpecify the region where your private key can be held locally."]
    pub fn geo_restrictions(&self) -> DataCertificatePack_Certificates_GeoRestrictionsRef {
        DataCertificatePack_Certificates_GeoRestrictionsRef::new(
            self.shared().clone(),
            format!("{}.geo_restrictions", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `hosts` after provisioning.\nHostnames covered by this certificate."]
    pub fn hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nCertificate identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issuer` after provisioning.\nThe certificate authority that issued the certificate."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the certificate was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nThe order/priority in which the certificate will be used."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `signature` after provisioning.\nThe type of hash used for the certificate."]
    pub fn signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nCertificate status."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uploaded_on` after provisioning.\nWhen the certificate was uploaded to Cloudflare."]
    pub fn uploaded_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uploaded_on", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCertificatePack_DcvDelegationRecords {}

impl DataCertificatePack_DcvDelegationRecords { }

impl terrars::ToListMappable for DataCertificatePack_DcvDelegationRecords {
    type O = terrars::BlockAssignable<DataCertificatePack_DcvDelegationRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCertificatePack_DcvDelegationRecords {}

impl BuildDataCertificatePack_DcvDelegationRecords {
    pub fn build(self) -> DataCertificatePack_DcvDelegationRecords {
        DataCertificatePack_DcvDelegationRecords {}
    }
}

#[allow(dead_code)]
pub struct DataCertificatePack_DcvDelegationRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificatePack_DcvDelegationRecordsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCertificatePack_DcvDelegationRecordsRef {
        DataCertificatePack_DcvDelegationRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCertificatePack_DcvDelegationRecordsRef {
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
pub struct DataCertificatePack_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCertificatePack_Filter {
    #[doc =
        "Set the field `deploy`.\nSpecify the deployment environment for the certificate packs.\nAvailable values: \"staging\", \"production\"."]
    pub fn set_deploy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deploy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nInclude Certificate Packs of all statuses, not just active ones.\nAvailable values: \"all\"."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCertificatePack_Filter {
    type O = terrars::BlockAssignable<DataCertificatePack_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCertificatePack_Filter {}

impl BuildDataCertificatePack_Filter {
    pub fn build(self) -> DataCertificatePack_Filter {
        DataCertificatePack_Filter {
            deploy: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCertificatePack_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificatePack_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCertificatePack_FilterRef {
        DataCertificatePack_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCertificatePack_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deploy` after provisioning.\nSpecify the deployment environment for the certificate packs.\nAvailable values: \"staging\", \"production\"."]
    pub fn deploy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nInclude Certificate Packs of all statuses, not just active ones.\nAvailable values: \"all\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCertificatePack_ValidationErrors {}

impl DataCertificatePack_ValidationErrors { }

impl terrars::ToListMappable for DataCertificatePack_ValidationErrors {
    type O = terrars::BlockAssignable<DataCertificatePack_ValidationErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCertificatePack_ValidationErrors {}

impl BuildDataCertificatePack_ValidationErrors {
    pub fn build(self) -> DataCertificatePack_ValidationErrors {
        DataCertificatePack_ValidationErrors {}
    }
}

#[allow(dead_code)]
pub struct DataCertificatePack_ValidationErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificatePack_ValidationErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCertificatePack_ValidationErrorsRef {
        DataCertificatePack_ValidationErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCertificatePack_ValidationErrorsRef {
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
pub struct DataCertificatePack_ValidationRecords {}

impl DataCertificatePack_ValidationRecords { }

impl terrars::ToListMappable for DataCertificatePack_ValidationRecords {
    type O = terrars::BlockAssignable<DataCertificatePack_ValidationRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCertificatePack_ValidationRecords {}

impl BuildDataCertificatePack_ValidationRecords {
    pub fn build(self) -> DataCertificatePack_ValidationRecords {
        DataCertificatePack_ValidationRecords {}
    }
}

#[allow(dead_code)]
pub struct DataCertificatePack_ValidationRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCertificatePack_ValidationRecordsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCertificatePack_ValidationRecordsRef {
        DataCertificatePack_ValidationRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCertificatePack_ValidationRecordsRef {
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
