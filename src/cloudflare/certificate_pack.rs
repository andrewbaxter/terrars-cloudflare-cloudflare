use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct CertificatePackData {
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
    hosts: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    validation_method: terrars::PrimField<std::string::String>,
    validity_days: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_active_status: std::option::Option<terrars::PrimField<bool>>,
    zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_errors: std::option::Option<std::vec::Vec<CertificatePackValidationErrorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_records: std::option::Option<std::vec::Vec<CertificatePackValidationRecordsEl>>,
    dynamic: CertificatePackDynamic,
}

struct CertificatePack_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CertificatePackData>,
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
        "Set the field `cloudflare_branding`.\nWhether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_cloudflare_branding(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cloudflare_branding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait_for_active_status`.\nWhether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_wait_for_active_status(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_active_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validation_errors`.\n"]
    pub fn set_validation_errors(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CertificatePackValidationErrorsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_errors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_errors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation_records`.\n"]
    pub fn set_validation_records(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CertificatePackValidationRecordsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_records = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_records = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nWhich certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_branding` after provisioning.\nWhether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn cloudflare_branding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_branding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nList of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**"]
    pub fn hosts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nCertificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_method` after provisioning.\nWhich validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn validation_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validity_days` after provisioning.\nHow long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn validity_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validity_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_active_status` after provisioning.\nWhether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn wait_for_active_status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_active_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_errors` after provisioning.\n"]
    pub fn validation_errors(&self) -> terrars::ListRef<CertificatePackValidationErrorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_records` after provisioning.\n"]
    pub fn validation_records(&self) -> terrars::ListRef<CertificatePackValidationRecordsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_records", self.extract_ref()))
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
        "Which certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`. **Modifying this attribute will force creation of a new resource.**"]
    pub certificate_authority: terrars::PrimField<std::string::String>,
    #[doc =
        "List of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**"]
    pub hosts: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc =
        "Certificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**"]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc =
        "Which validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**"]
    pub validation_method: terrars::PrimField<std::string::String>,
    #[doc =
        "How long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**"]
    pub validity_days: terrars::PrimField<f64>,
    #[doc =
        "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildCertificatePack {
    pub fn build(self, stack: &mut terrars::Stack) -> CertificatePack {
        let out = CertificatePack(std::rc::Rc::new(CertificatePack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CertificatePackData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
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
        "Get a reference to the value of field `certificate_authority` after provisioning.\nWhich certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_branding` after provisioning.\nWhether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn cloudflare_branding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_branding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nList of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**"]
    pub fn hosts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nCertificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_method` after provisioning.\nWhich validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn validation_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validity_days` after provisioning.\nHow long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn validity_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validity_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_active_status` after provisioning.\nWhether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn wait_for_active_status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_active_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_errors` after provisioning.\n"]
    pub fn validation_errors(&self) -> terrars::ListRef<CertificatePackValidationErrorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_records` after provisioning.\n"]
    pub fn validation_records(&self) -> terrars::ListRef<CertificatePackValidationRecordsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_records", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CertificatePackValidationErrorsEl {}

impl CertificatePackValidationErrorsEl { }

impl terrars::ToListMappable for CertificatePackValidationErrorsEl {
    type O = terrars::BlockAssignable<CertificatePackValidationErrorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificatePackValidationErrorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificatePackValidationErrorsElRef {
        CertificatePackValidationErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificatePackValidationErrorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct CertificatePackValidationRecordsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cname_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cname_target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    txt_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    txt_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificatePackValidationRecordsEl {
    #[doc = "Set the field `cname_name`.\n"]
    pub fn set_cname_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cname_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cname_target`.\n"]
    pub fn set_cname_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cname_target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `emails`.\n"]
    pub fn set_emails(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.emails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_body`.\n"]
    pub fn set_http_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_body = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_url`.\n"]
    pub fn set_http_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `txt_name`.\n"]
    pub fn set_txt_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.txt_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `txt_value`.\n"]
    pub fn set_txt_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.txt_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CertificatePackValidationRecordsEl {
    type O = terrars::BlockAssignable<CertificatePackValidationRecordsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificatePackValidationRecordsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CertificatePackValidationRecordsElRef {
        CertificatePackValidationRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificatePackValidationRecordsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cname_name` after provisioning.\n"]
    pub fn cname_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname_name", self.base))
    }

    #[doc = "Get a reference to the value of field `cname_target` after provisioning.\n"]
    pub fn cname_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname_target", self.base))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc = "Get a reference to the value of field `http_body` after provisioning.\n"]
    pub fn http_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_body", self.base))
    }

    #[doc = "Get a reference to the value of field `http_url` after provisioning.\n"]
    pub fn http_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_url", self.base))
    }

    #[doc = "Get a reference to the value of field `txt_name` after provisioning.\n"]
    pub fn txt_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.txt_name", self.base))
    }

    #[doc = "Get a reference to the value of field `txt_value` after provisioning.\n"]
    pub fn txt_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.txt_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CertificatePackDynamic {
    validation_errors: std::option::Option<terrars::DynamicBlock<CertificatePackValidationErrorsEl>>,
    validation_records: std::option::Option<terrars::DynamicBlock<CertificatePackValidationRecordsEl>>,
}
