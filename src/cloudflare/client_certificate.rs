#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ClientCertificateData_ {
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
    csr: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reactivate: std::option::Option<terrars::PrimField<bool>>,
    validity_days: terrars::PrimField<f64>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct ClientCertificate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ClientCertificateData_>,
}

#[derive(Clone)]
pub struct ClientCertificate(std::rc::Rc<ClientCertificate_>);

impl ClientCertificate {
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

    #[doc = "Set the field `reactivate`.\n"]
    pub fn set_reactivate(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reactivate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\nThe Client Certificate PEM."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nCertificate Authority used to issue the Client Certificate."]
    pub fn certificate_authority(&self) -> ClientCertificate_CertificateAuthorityRef {
        ClientCertificate_CertificateAuthorityRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `reactivate` after provisioning.\n"]
    pub fn reactivate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reactivate", self.extract_ref()))
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

impl terrars::Referable for ClientCertificate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ClientCertificate { }

impl terrars::ToListMappable for ClientCertificate {
    type O = terrars::ListRef<ClientCertificateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ClientCertificate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_client_certificate".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildClientCertificate {
    pub tf_id: std::string::String,
    #[doc = "The Certificate Signing Request (CSR). Must be newline-encoded."]
    pub csr: terrars::PrimField<std::string::String>,
    #[doc = "The number of days the Client Certificate will be valid after the issued_on date."]
    pub validity_days: terrars::PrimField<f64>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildClientCertificate {
    pub fn build(self, stack: &mut terrars::Stack) -> ClientCertificate {
        let out = ClientCertificate(std::rc::Rc::new(ClientCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ClientCertificateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                csr: self.csr,
                reactivate: core::default::Default::default(),
                validity_days: self.validity_days,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ClientCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClientCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ClientCertificateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\nThe Client Certificate PEM."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nCertificate Authority used to issue the Client Certificate."]
    pub fn certificate_authority(&self) -> ClientCertificate_CertificateAuthorityRef {
        ClientCertificate_CertificateAuthorityRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `reactivate` after provisioning.\n"]
    pub fn reactivate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reactivate", self.extract_ref()))
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
pub struct ClientCertificate_CertificateAuthority {}

impl ClientCertificate_CertificateAuthority { }

impl terrars::ToListMappable for ClientCertificate_CertificateAuthority {
    type O = terrars::BlockAssignable<ClientCertificate_CertificateAuthority>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClientCertificate_CertificateAuthority {}

impl BuildClientCertificate_CertificateAuthority {
    pub fn build(self) -> ClientCertificate_CertificateAuthority {
        ClientCertificate_CertificateAuthority {}
    }
}

#[allow(dead_code)]
pub struct ClientCertificate_CertificateAuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClientCertificate_CertificateAuthorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClientCertificate_CertificateAuthorityRef {
        ClientCertificate_CertificateAuthorityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClientCertificate_CertificateAuthorityRef {
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
