use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct CustomHostnameData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_origin_server: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_origin_sni: Option<PrimField<String>>,
    hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_ssl_pending_validation: Option<PrimField<bool>>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl: Option<Vec<CustomHostnameSslEl>>,
    dynamic: CustomHostnameDynamic,
}

struct CustomHostname_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CustomHostnameData>,
}

#[derive(Clone)]
pub struct CustomHostname(Rc<CustomHostname_>);

impl CustomHostname {
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

    #[doc= "Set the field `custom_metadata`.\nCustom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly."]
    pub fn set_custom_metadata(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_metadata = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_origin_server`.\nThe custom origin server used for certificates."]
    pub fn set_custom_origin_server(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_origin_server = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_origin_sni`.\nThe [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates."]
    pub fn set_custom_origin_sni(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_origin_sni = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `wait_for_ssl_pending_validation`.\nWhether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`."]
    pub fn set_wait_for_ssl_pending_validation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_ssl_pending_validation = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl`.\n"]
    pub fn set_ssl(self, v: impl Into<BlockAssignable<CustomHostnameSslEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `custom_metadata` after provisioning.\nCustom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly."]
    pub fn custom_metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.custom_metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_origin_server` after provisioning.\nThe custom origin server used for certificates."]
    pub fn custom_origin_server(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_server", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_origin_sni` after provisioning.\nThe [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates."]
    pub fn custom_origin_sni(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_sni", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nHostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ownership_verification` after provisioning.\n"]
    pub fn ownership_verification(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ownership_verification", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ownership_verification_http` after provisioning.\n"]
    pub fn ownership_verification_http(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ownership_verification_http", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nStatus of the certificate."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_for_ssl_pending_validation` after provisioning.\nWhether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`."]
    pub fn wait_for_ssl_pending_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_ssl_pending_validation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> ListRef<CustomHostnameSslElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }
}

impl Referable for CustomHostname {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CustomHostname { }

impl ToListMappable for CustomHostname {
    type O = ListRef<CustomHostnameRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CustomHostname_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_custom_hostname".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCustomHostname {
    pub tf_id: String,
    #[doc= "Hostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**"]
    pub hostname: PrimField<String>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildCustomHostname {
    pub fn build(self, stack: &mut Stack) -> CustomHostname {
        let out = CustomHostname(Rc::new(CustomHostname_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CustomHostnameData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                custom_metadata: core::default::Default::default(),
                custom_origin_server: core::default::Default::default(),
                custom_origin_sni: core::default::Default::default(),
                hostname: self.hostname,
                id: core::default::Default::default(),
                wait_for_ssl_pending_validation: core::default::Default::default(),
                zone_id: self.zone_id,
                ssl: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CustomHostnameRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomHostnameRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CustomHostnameRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `custom_metadata` after provisioning.\nCustom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly."]
    pub fn custom_metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.custom_metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_origin_server` after provisioning.\nThe custom origin server used for certificates."]
    pub fn custom_origin_server(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_server", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_origin_sni` after provisioning.\nThe [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates."]
    pub fn custom_origin_sni(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_sni", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nHostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ownership_verification` after provisioning.\n"]
    pub fn ownership_verification(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ownership_verification", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ownership_verification_http` after provisioning.\n"]
    pub fn ownership_verification_http(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ownership_verification_http", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nStatus of the certificate."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_for_ssl_pending_validation` after provisioning.\nWhether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`."]
    pub fn wait_for_ssl_pending_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_ssl_pending_validation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> ListRef<CustomHostnameSslElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CustomHostnameSslElValidationErrorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}

impl CustomHostnameSslElValidationErrorsEl {
    #[doc= "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}

impl ToListMappable for CustomHostnameSslElValidationErrorsEl {
    type O = BlockAssignable<CustomHostnameSslElValidationErrorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostnameSslElValidationErrorsEl {}

impl BuildCustomHostnameSslElValidationErrorsEl {
    pub fn build(self) -> CustomHostnameSslElValidationErrorsEl {
        CustomHostnameSslElValidationErrorsEl { message: core::default::Default::default() }
    }
}

pub struct CustomHostnameSslElValidationErrorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomHostnameSslElValidationErrorsElRef {
    fn new(shared: StackShared, base: String) -> CustomHostnameSslElValidationErrorsElRef {
        CustomHostnameSslElValidationErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostnameSslElValidationErrorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomHostnameSslElValidationRecordsEl {
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

impl CustomHostnameSslElValidationRecordsEl {
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

impl ToListMappable for CustomHostnameSslElValidationRecordsEl {
    type O = BlockAssignable<CustomHostnameSslElValidationRecordsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostnameSslElValidationRecordsEl {}

impl BuildCustomHostnameSslElValidationRecordsEl {
    pub fn build(self) -> CustomHostnameSslElValidationRecordsEl {
        CustomHostnameSslElValidationRecordsEl {
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

pub struct CustomHostnameSslElValidationRecordsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomHostnameSslElValidationRecordsElRef {
    fn new(shared: StackShared, base: String) -> CustomHostnameSslElValidationRecordsElRef {
        CustomHostnameSslElValidationRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostnameSslElValidationRecordsElRef {
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

#[derive(Serialize)]
pub struct CustomHostnameSslElSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ciphers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    early_hints: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_tls_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls13: Option<PrimField<String>>,
}

impl CustomHostnameSslElSettingsEl {
    #[doc= "Set the field `ciphers`.\nList of SSL/TLS ciphers to associate with this certificate."]
    pub fn set_ciphers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ciphers = Some(v.into());
        self
    }

    #[doc= "Set the field `early_hints`.\nWhether early hints should be supported. Available values: `on`, `off`."]
    pub fn set_early_hints(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.early_hints = Some(v.into());
        self
    }

    #[doc= "Set the field `http2`.\nWhether HTTP2 should be supported. Available values: `on`, `off`."]
    pub fn set_http2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http2 = Some(v.into());
        self
    }

    #[doc= "Set the field `min_tls_version`.\nLowest version of TLS this certificate should support. Available values: `1.0`, `1.1`, `1.2`, `1.3`."]
    pub fn set_min_tls_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_tls_version = Some(v.into());
        self
    }

    #[doc= "Set the field `tls13`.\nWhether TLSv1.3 should be supported. Available values: `on`, `off`."]
    pub fn set_tls13(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls13 = Some(v.into());
        self
    }
}

impl ToListMappable for CustomHostnameSslElSettingsEl {
    type O = BlockAssignable<CustomHostnameSslElSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostnameSslElSettingsEl {}

impl BuildCustomHostnameSslElSettingsEl {
    pub fn build(self) -> CustomHostnameSslElSettingsEl {
        CustomHostnameSslElSettingsEl {
            ciphers: core::default::Default::default(),
            early_hints: core::default::Default::default(),
            http2: core::default::Default::default(),
            min_tls_version: core::default::Default::default(),
            tls13: core::default::Default::default(),
        }
    }
}

pub struct CustomHostnameSslElSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomHostnameSslElSettingsElRef {
    fn new(shared: StackShared, base: String) -> CustomHostnameSslElSettingsElRef {
        CustomHostnameSslElSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostnameSslElSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ciphers` after provisioning.\nList of SSL/TLS ciphers to associate with this certificate."]
    pub fn ciphers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ciphers", self.base))
    }

    #[doc= "Get a reference to the value of field `early_hints` after provisioning.\nWhether early hints should be supported. Available values: `on`, `off`."]
    pub fn early_hints(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.early_hints", self.base))
    }

    #[doc= "Get a reference to the value of field `http2` after provisioning.\nWhether HTTP2 should be supported. Available values: `on`, `off`."]
    pub fn http2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc= "Get a reference to the value of field `min_tls_version` after provisioning.\nLowest version of TLS this certificate should support. Available values: `1.0`, `1.1`, `1.2`, `1.3`."]
    pub fn min_tls_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_tls_version", self.base))
    }

    #[doc= "Get a reference to the value of field `tls13` after provisioning.\nWhether TLSv1.3 should be supported. Available values: `on`, `off`."]
    pub fn tls13(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls13", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomHostnameSslElDynamic {
    settings: Option<DynamicBlock<CustomHostnameSslElSettingsEl>>,
}

#[derive(Serialize)]
pub struct CustomHostnameSslEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wildcard: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<Vec<CustomHostnameSslElSettingsEl>>,
    dynamic: CustomHostnameSslElDynamic,
}

impl CustomHostnameSslEl {
    #[doc= "Set the field `certificate_authority`.\n"]
    pub fn set_certificate_authority(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_authority = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_certificate`.\nIf a custom uploaded certificate is used."]
    pub fn set_custom_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_key`.\nThe key for a custom uploaded certificate."]
    pub fn set_custom_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_key = Some(v.into());
        self
    }

    #[doc= "Set the field `method`.\nDomain control validation (DCV) method used for this hostname. Available values: `http`, `txt`, `email`."]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nLevel of validation to be used for this hostname. Available values: `dv`. Defaults to `dv`."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `wildcard`.\nIndicates whether the certificate covers a wildcard."]
    pub fn set_wildcard(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.wildcard = Some(v.into());
        self
    }

    #[doc= "Set the field `settings`.\n"]
    pub fn set_settings(mut self, v: impl Into<BlockAssignable<CustomHostnameSslElSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CustomHostnameSslEl {
    type O = BlockAssignable<CustomHostnameSslEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostnameSslEl {}

impl BuildCustomHostnameSslEl {
    pub fn build(self) -> CustomHostnameSslEl {
        CustomHostnameSslEl {
            certificate_authority: core::default::Default::default(),
            custom_certificate: core::default::Default::default(),
            custom_key: core::default::Default::default(),
            method: core::default::Default::default(),
            type_: core::default::Default::default(),
            wildcard: core::default::Default::default(),
            settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CustomHostnameSslElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomHostnameSslElRef {
    fn new(shared: StackShared, base: String) -> CustomHostnameSslElRef {
        CustomHostnameSslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostnameSslElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_certificate` after provisioning.\nIf a custom uploaded certificate is used."]
    pub fn custom_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_key` after provisioning.\nThe key for a custom uploaded certificate."]
    pub fn custom_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_key", self.base))
    }

    #[doc= "Get a reference to the value of field `method` after provisioning.\nDomain control validation (DCV) method used for this hostname. Available values: `http`, `txt`, `email`."]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nLevel of validation to be used for this hostname. Available values: `dv`. Defaults to `dv`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `validation_errors` after provisioning.\n"]
    pub fn validation_errors(&self) -> ListRef<CustomHostnameSslElValidationErrorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.base))
    }

    #[doc= "Get a reference to the value of field `validation_records` after provisioning.\n"]
    pub fn validation_records(&self) -> ListRef<CustomHostnameSslElValidationRecordsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation_records", self.base))
    }

    #[doc= "Get a reference to the value of field `wildcard` after provisioning.\nIndicates whether the certificate covers a wildcard."]
    pub fn wildcard(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }

    #[doc= "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<CustomHostnameSslElSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomHostnameDynamic {
    ssl: Option<DynamicBlock<CustomHostnameSslEl>>,
}
