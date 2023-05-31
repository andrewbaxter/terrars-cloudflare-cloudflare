use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct OriginCaCertificateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csr: Option<PrimField<String>>,
    hostnames: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_days_for_renewal: Option<PrimField<f64>>,
    request_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_validity: Option<PrimField<f64>>,
}

struct OriginCaCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OriginCaCertificateData>,
}

#[derive(Clone)]
pub struct OriginCaCertificate(Rc<OriginCaCertificate_>);

impl OriginCaCertificate {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `csr`.\nThe Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_csr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().csr = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `min_days_for_renewal`.\nNumber of days prior to the expiry to trigger a renewal of the certificate if a Terraform operation is run."]
    pub fn set_min_days_for_renewal(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_days_for_renewal = Some(v.into());
        self
    }

    #[doc= "Set the field `requested_validity`.\nThe number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_requested_validity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().requested_validity = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\nThe Origin CA certificate."]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `csr` after provisioning.\nThe Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**"]
    pub fn csr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_on` after provisioning.\nThe datetime when the certificate will expire."]
    pub fn expires_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostnames` after provisioning.\nA list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**"]
    pub fn hostnames(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_days_for_renewal` after provisioning.\nNumber of days prior to the expiry to trigger a renewal of the certificate if a Terraform operation is run."]
    pub fn min_days_for_renewal(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_days_for_renewal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_type` after provisioning.\nThe signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn request_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `requested_validity` after provisioning.\nThe number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn requested_validity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.requested_validity", self.extract_ref()))
    }
}

impl Referable for OriginCaCertificate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for OriginCaCertificate { }

impl ToListMappable for OriginCaCertificate {
    type O = ListRef<OriginCaCertificateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OriginCaCertificate_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_origin_ca_certificate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOriginCaCertificate {
    pub tf_id: String,
    #[doc= "A list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**"]
    pub hostnames: SetField<PrimField<String>>,
    #[doc= "The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**"]
    pub request_type: PrimField<String>,
}

impl BuildOriginCaCertificate {
    pub fn build(self, stack: &mut Stack) -> OriginCaCertificate {
        let out = OriginCaCertificate(Rc::new(OriginCaCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OriginCaCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                csr: core::default::Default::default(),
                hostnames: self.hostnames,
                id: core::default::Default::default(),
                min_days_for_renewal: core::default::Default::default(),
                request_type: self.request_type,
                requested_validity: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OriginCaCertificateRef {
    shared: StackShared,
    base: String,
}

impl Ref for OriginCaCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OriginCaCertificateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\nThe Origin CA certificate."]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `csr` after provisioning.\nThe Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**"]
    pub fn csr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_on` after provisioning.\nThe datetime when the certificate will expire."]
    pub fn expires_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostnames` after provisioning.\nA list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**"]
    pub fn hostnames(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_days_for_renewal` after provisioning.\nNumber of days prior to the expiry to trigger a renewal of the certificate if a Terraform operation is run."]
    pub fn min_days_for_renewal(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_days_for_renewal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_type` after provisioning.\nThe signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn request_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `requested_validity` after provisioning.\nThe number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn requested_validity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.requested_validity", self.extract_ref()))
    }
}
