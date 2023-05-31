use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct CertificatePackData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    certificate_authority: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudflare_branding: Option<PrimField<bool>>,
    hosts: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    validation_method: PrimField<String>,
    validity_days: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_active_status: Option<PrimField<bool>>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_errors: Option<Vec<CertificatePackValidationErrorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_records: Option<Vec<CertificatePackValidationRecordsEl>>,
    dynamic: CertificatePackDynamic,
}

struct CertificatePack_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CertificatePackData>,
}

#[derive(Clone)]
pub struct CertificatePack(Rc<CertificatePack_>);

impl CertificatePack {
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

    #[doc= "Set the field `cloudflare_branding`.\nWhether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_cloudflare_branding(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cloudflare_branding = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `wait_for_active_status`.\nWhether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_wait_for_active_status(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_active_status = Some(v.into());
        self
    }

    #[doc= "Set the field `validation_errors`.\n"]
    pub fn set_validation_errors(self, v: impl Into<BlockAssignable<CertificatePackValidationErrorsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_errors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_errors = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `validation_records`.\n"]
    pub fn set_validation_records(self, v: impl Into<BlockAssignable<CertificatePackValidationRecordsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_records = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_records = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `certificate_authority` after provisioning.\nWhich certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn certificate_authority(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloudflare_branding` after provisioning.\nWhether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn cloudflare_branding(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_branding", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hosts` after provisioning.\nList of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**"]
    pub fn hosts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nCertificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `validation_method` after provisioning.\nWhich validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn validation_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.validation_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `validity_days` after provisioning.\nHow long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn validity_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.validity_days", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_for_active_status` after provisioning.\nWhether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn wait_for_active_status(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_active_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `validation_errors` after provisioning.\n"]
    pub fn validation_errors(&self) -> ListRef<CertificatePackValidationErrorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `validation_records` after provisioning.\n"]
    pub fn validation_records(&self) -> ListRef<CertificatePackValidationRecordsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation_records", self.extract_ref()))
    }
}

impl Referable for CertificatePack {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CertificatePack { }

impl ToListMappable for CertificatePack {
    type O = ListRef<CertificatePackRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CertificatePack_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_certificate_pack".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCertificatePack {
    pub tf_id: String,
    #[doc= "Which certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`. **Modifying this attribute will force creation of a new resource.**"]
    pub certificate_authority: PrimField<String>,
    #[doc= "List of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**"]
    pub hosts: SetField<PrimField<String>>,
    #[doc= "Certificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**"]
    pub type_: PrimField<String>,
    #[doc= "Which validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**"]
    pub validation_method: PrimField<String>,
    #[doc= "How long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**"]
    pub validity_days: PrimField<f64>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildCertificatePack {
    pub fn build(self, stack: &mut Stack) -> CertificatePack {
        let out = CertificatePack(Rc::new(CertificatePack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CertificatePackData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                certificate_authority: self.certificate_authority,
                cloudflare_branding: core::default::Default::default(),
                hosts: self.hosts,
                id: core::default::Default::default(),
                type_: self.type_,
                validation_method: self.validation_method,
                validity_days: self.validity_days,
                wait_for_active_status: core::default::Default::default(),
                zone_id: self.zone_id,
                validation_errors: core::default::Default::default(),
                validation_records: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CertificatePackRef {
    shared: StackShared,
    base: String,
}

impl Ref for CertificatePackRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CertificatePackRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `certificate_authority` after provisioning.\nWhich certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn certificate_authority(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloudflare_branding` after provisioning.\nWhether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn cloudflare_branding(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_branding", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hosts` after provisioning.\nList of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**"]
    pub fn hosts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nCertificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `validation_method` after provisioning.\nWhich validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn validation_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.validation_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `validity_days` after provisioning.\nHow long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn validity_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.validity_days", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_for_active_status` after provisioning.\nWhether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn wait_for_active_status(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_active_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `validation_errors` after provisioning.\n"]
    pub fn validation_errors(&self) -> ListRef<CertificatePackValidationErrorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `validation_records` after provisioning.\n"]
    pub fn validation_records(&self) -> ListRef<CertificatePackValidationRecordsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation_records", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CertificatePackValidationErrorsEl {}

impl CertificatePackValidationErrorsEl { }

impl ToListMappable for CertificatePackValidationErrorsEl {
    type O = BlockAssignable<CertificatePackValidationErrorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificatePackValidationErrorsEl {}

impl BuildCertificatePackValidationErrorsEl {
    pub fn build(self) -> CertificatePackValidationErrorsEl {
        CertificatePackValidationErrorsEl {}
    }
}

pub struct CertificatePackValidationErrorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CertificatePackValidationErrorsElRef {
    fn new(shared: StackShared, base: String) -> CertificatePackValidationErrorsElRef {
        CertificatePackValidationErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificatePackValidationErrorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct CertificatePackValidationRecordsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cname_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cname_target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emails: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    txt_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    txt_value: Option<PrimField<String>>,
}

impl CertificatePackValidationRecordsEl {
    #[doc= "Set the field `cname_name`.\n"]
    pub fn set_cname_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cname_name = Some(v.into());
        self
    }

    #[doc= "Set the field `cname_target`.\n"]
    pub fn set_cname_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cname_target = Some(v.into());
        self
    }

    #[doc= "Set the field `emails`.\n"]
    pub fn set_emails(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.emails = Some(v.into());
        self
    }

    #[doc= "Set the field `http_body`.\n"]
    pub fn set_http_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_body = Some(v.into());
        self
    }

    #[doc= "Set the field `http_url`.\n"]
    pub fn set_http_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_url = Some(v.into());
        self
    }

    #[doc= "Set the field `txt_name`.\n"]
    pub fn set_txt_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.txt_name = Some(v.into());
        self
    }

    #[doc= "Set the field `txt_value`.\n"]
    pub fn set_txt_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.txt_value = Some(v.into());
        self
    }
}

impl ToListMappable for CertificatePackValidationRecordsEl {
    type O = BlockAssignable<CertificatePackValidationRecordsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificatePackValidationRecordsEl {}

impl BuildCertificatePackValidationRecordsEl {
    pub fn build(self) -> CertificatePackValidationRecordsEl {
        CertificatePackValidationRecordsEl {
            cname_name: core::default::Default::default(),
            cname_target: core::default::Default::default(),
            emails: core::default::Default::default(),
            http_body: core::default::Default::default(),
            http_url: core::default::Default::default(),
            txt_name: core::default::Default::default(),
            txt_value: core::default::Default::default(),
        }
    }
}

pub struct CertificatePackValidationRecordsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CertificatePackValidationRecordsElRef {
    fn new(shared: StackShared, base: String) -> CertificatePackValidationRecordsElRef {
        CertificatePackValidationRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificatePackValidationRecordsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cname_name` after provisioning.\n"]
    pub fn cname_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cname_name", self.base))
    }

    #[doc= "Get a reference to the value of field `cname_target` after provisioning.\n"]
    pub fn cname_target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cname_target", self.base))
    }

    #[doc= "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc= "Get a reference to the value of field `http_body` after provisioning.\n"]
    pub fn http_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_body", self.base))
    }

    #[doc= "Get a reference to the value of field `http_url` after provisioning.\n"]
    pub fn http_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_url", self.base))
    }

    #[doc= "Get a reference to the value of field `txt_name` after provisioning.\n"]
    pub fn txt_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.txt_name", self.base))
    }

    #[doc= "Get a reference to the value of field `txt_value` after provisioning.\n"]
    pub fn txt_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.txt_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CertificatePackDynamic {
    validation_errors: Option<DynamicBlock<CertificatePackValidationErrorsEl>>,
    validation_records: Option<DynamicBlock<CertificatePackValidationRecordsEl>>,
}
