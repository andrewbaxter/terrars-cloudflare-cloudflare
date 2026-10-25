#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct CertificatePackData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    certificate_authority: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_branding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    validation_method: terrars::PrimField<std::string::String>,
    validity_days: terrars::PrimField<f64>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct CertificatePack_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CertificatePackData_>,
}

#[derive(Clone)]
pub struct CertificatePack(std::rc::Rc<CertificatePack_>);

impl CertificatePack {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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
        "Set the field `cloudflare_branding`.\nWhether or not to add Cloudflare Branding for the order.  This will add a subdomain of sni.cloudflaressl.com as the Common Name if set to true."]
    pub fn set_cloudflare_branding(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cloudflare_branding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hosts`.\nComma separated list of valid host names for the certificate packs. Must contain the zone apex, may not contain more than 50 hosts, and may not be empty."]
    pub fn set_hosts(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nCertificate Authority selected for the order.  For information on any certificate authority specific details or restrictions [see this page for more details](https://developers.cloudflare.com/ssl/reference/certificate-authorities).\nAvailable values: \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificates` after provisioning.\nArray of certificates in this pack."]
    pub fn certificates(&self) -> terrars::ListRef<CertificatePack_CertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_branding` after provisioning.\nWhether or not to add Cloudflare Branding for the order.  This will add a subdomain of sni.cloudflaressl.com as the Common Name if set to true."]
    pub fn cloudflare_branding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_branding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dcv_delegation_records` after provisioning.\nDCV Delegation records for domain validation."]
    pub fn dcv_delegation_records(&self) -> terrars::ListRef<CertificatePack_DcvDelegationRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dcv_delegation_records", self.extract_ref()))
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
        "Get a reference to the value of field `type_` after provisioning.\nType of certificate pack.\nAvailable values: \"advanced\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_errors` after provisioning.\nDomain validation errors that have been received by the certificate authority (CA)."]
    pub fn validation_errors(&self) -> terrars::ListRef<CertificatePack_ValidationErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_method` after provisioning.\nValidation Method selected for the order.\nAvailable values: \"txt\", \"http\", \"email\"."]
    pub fn validation_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_records` after provisioning.\nCertificates' validation records."]
    pub fn validation_records(&self) -> terrars::ListRef<CertificatePack_ValidationRecordsRef> {
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

impl terrars::Referable for CertificatePack {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CertificatePack { }

impl terrars::ToListMappable for CertificatePack {
    type O = terrars::ListRef<CertificatePackRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CertificatePack_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_certificate_pack".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCertificatePack {
    pub tf_id: std::string::String,
    #[doc =
        "Certificate Authority selected for the order.  For information on any certificate authority specific details or restrictions [see this page for more details](https://developers.cloudflare.com/ssl/reference/certificate-authorities).\nAvailable values: \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub certificate_authority: terrars::PrimField<std::string::String>,
    #[doc = "Type of certificate pack.\nAvailable values: \"advanced\"."]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = "Validation Method selected for the order.\nAvailable values: \"txt\", \"http\", \"email\"."]
    pub validation_method: terrars::PrimField<std::string::String>,
    #[doc = "Validity Days selected for the order.\nAvailable values: 14, 30, 90, 365."]
    pub validity_days: terrars::PrimField<f64>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildCertificatePack {
    pub fn build(self, stack: &mut terrars::Stack) -> CertificatePack {
        let out = CertificatePack(std::rc::Rc::new(CertificatePack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CertificatePackData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                certificate_authority: self.certificate_authority,
                cloudflare_branding: core::default::Default::default(),
                hosts: core::default::Default::default(),
                type_: self.type_,
                validation_method: self.validation_method,
                validity_days: self.validity_days,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CertificatePackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificatePackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CertificatePackRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nCertificate Authority selected for the order.  For information on any certificate authority specific details or restrictions [see this page for more details](https://developers.cloudflare.com/ssl/reference/certificate-authorities).\nAvailable values: \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificates` after provisioning.\nArray of certificates in this pack."]
    pub fn certificates(&self) -> terrars::ListRef<CertificatePack_CertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_branding` after provisioning.\nWhether or not to add Cloudflare Branding for the order.  This will add a subdomain of sni.cloudflaressl.com as the Common Name if set to true."]
    pub fn cloudflare_branding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_branding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dcv_delegation_records` after provisioning.\nDCV Delegation records for domain validation."]
    pub fn dcv_delegation_records(&self) -> terrars::ListRef<CertificatePack_DcvDelegationRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dcv_delegation_records", self.extract_ref()))
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
        "Get a reference to the value of field `type_` after provisioning.\nType of certificate pack.\nAvailable values: \"advanced\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_errors` after provisioning.\nDomain validation errors that have been received by the certificate authority (CA)."]
    pub fn validation_errors(&self) -> terrars::ListRef<CertificatePack_ValidationErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_method` after provisioning.\nValidation Method selected for the order.\nAvailable values: \"txt\", \"http\", \"email\"."]
    pub fn validation_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_records` after provisioning.\nCertificates' validation records."]
    pub fn validation_records(&self) -> terrars::ListRef<CertificatePack_ValidationRecordsRef> {
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
pub struct CertificatePack_Certificates_GeoRestrictions {}

impl CertificatePack_Certificates_GeoRestrictions { }

impl terrars::ToListMappable for CertificatePack_Certificates_GeoRestrictions {
    type O = terrars::BlockAssignable<CertificatePack_Certificates_GeoRestrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificatePack_Certificates_GeoRestrictions {}

impl BuildCertificatePack_Certificates_GeoRestrictions {
    pub fn build(self) -> CertificatePack_Certificates_GeoRestrictions {
        CertificatePack_Certificates_GeoRestrictions {}
    }
}

#[allow(dead_code)]
pub struct CertificatePack_Certificates_GeoRestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificatePack_Certificates_GeoRestrictionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificatePack_Certificates_GeoRestrictionsRef {
        CertificatePack_Certificates_GeoRestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificatePack_Certificates_GeoRestrictionsRef {
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
pub struct CertificatePack_Certificates {}

impl CertificatePack_Certificates { }

impl terrars::ToListMappable for CertificatePack_Certificates {
    type O = terrars::BlockAssignable<CertificatePack_Certificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificatePack_Certificates {}

impl BuildCertificatePack_Certificates {
    pub fn build(self) -> CertificatePack_Certificates {
        CertificatePack_Certificates {}
    }
}

#[allow(dead_code)]
pub struct CertificatePack_CertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificatePack_CertificatesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificatePack_CertificatesRef {
        CertificatePack_CertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificatePack_CertificatesRef {
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
    pub fn geo_restrictions(&self) -> CertificatePack_Certificates_GeoRestrictionsRef {
        CertificatePack_Certificates_GeoRestrictionsRef::new(
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
pub struct CertificatePack_DcvDelegationRecords {}

impl CertificatePack_DcvDelegationRecords { }

impl terrars::ToListMappable for CertificatePack_DcvDelegationRecords {
    type O = terrars::BlockAssignable<CertificatePack_DcvDelegationRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificatePack_DcvDelegationRecords {}

impl BuildCertificatePack_DcvDelegationRecords {
    pub fn build(self) -> CertificatePack_DcvDelegationRecords {
        CertificatePack_DcvDelegationRecords {}
    }
}

#[allow(dead_code)]
pub struct CertificatePack_DcvDelegationRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificatePack_DcvDelegationRecordsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificatePack_DcvDelegationRecordsRef {
        CertificatePack_DcvDelegationRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificatePack_DcvDelegationRecordsRef {
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
pub struct CertificatePack_ValidationErrors {}

impl CertificatePack_ValidationErrors { }

impl terrars::ToListMappable for CertificatePack_ValidationErrors {
    type O = terrars::BlockAssignable<CertificatePack_ValidationErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificatePack_ValidationErrors {}

impl BuildCertificatePack_ValidationErrors {
    pub fn build(self) -> CertificatePack_ValidationErrors {
        CertificatePack_ValidationErrors {}
    }
}

#[allow(dead_code)]
pub struct CertificatePack_ValidationErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificatePack_ValidationErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificatePack_ValidationErrorsRef {
        CertificatePack_ValidationErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificatePack_ValidationErrorsRef {
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
pub struct CertificatePack_ValidationRecords {}

impl CertificatePack_ValidationRecords { }

impl terrars::ToListMappable for CertificatePack_ValidationRecords {
    type O = terrars::BlockAssignable<CertificatePack_ValidationRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificatePack_ValidationRecords {}

impl BuildCertificatePack_ValidationRecords {
    pub fn build(self) -> CertificatePack_ValidationRecords {
        CertificatePack_ValidationRecords {}
    }
}

#[allow(dead_code)]
pub struct CertificatePack_ValidationRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificatePack_ValidationRecordsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificatePack_ValidationRecordsRef {
        CertificatePack_ValidationRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificatePack_ValidationRecordsRef {
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
