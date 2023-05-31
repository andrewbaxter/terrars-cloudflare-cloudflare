use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct CustomSslData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_ssl_options: Option<Vec<CustomSslCustomSslOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_ssl_priority: Option<Vec<CustomSslCustomSslPriorityEl>>,
    dynamic: CustomSslDynamic,
}

struct CustomSsl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CustomSslData>,
}

#[derive(Clone)]
pub struct CustomSsl(Rc<CustomSsl_>);

impl CustomSsl {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_ssl_options`.\n"]
    pub fn set_custom_ssl_options(self, v: impl Into<BlockAssignable<CustomSslCustomSslOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_ssl_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_ssl_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `custom_ssl_priority`.\n"]
    pub fn set_custom_ssl_priority(self, v: impl Into<BlockAssignable<CustomSslCustomSslPriorityEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_ssl_priority = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_ssl_priority = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `expires_on` after provisioning.\n"]
    pub fn expires_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signature` after provisioning.\n"]
    pub fn signature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uploaded_on` after provisioning.\n"]
    pub fn uploaded_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uploaded_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_ssl_options` after provisioning.\n"]
    pub fn custom_ssl_options(&self) -> ListRef<CustomSslCustomSslOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_ssl_options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_ssl_priority` after provisioning.\n"]
    pub fn custom_ssl_priority(&self) -> ListRef<CustomSslCustomSslPriorityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_ssl_priority", self.extract_ref()))
    }
}

impl Referable for CustomSsl {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CustomSsl { }

impl ToListMappable for CustomSsl {
    type O = ListRef<CustomSslRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CustomSsl_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_custom_ssl".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCustomSsl {
    pub tf_id: String,
    #[doc= "The zone identifier to target for the resource."]
    pub zone_id: PrimField<String>,
}

impl BuildCustomSsl {
    pub fn build(self, stack: &mut Stack) -> CustomSsl {
        let out = CustomSsl(Rc::new(CustomSsl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CustomSslData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                zone_id: self.zone_id,
                custom_ssl_options: core::default::Default::default(),
                custom_ssl_priority: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CustomSslRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomSslRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CustomSslRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expires_on` after provisioning.\n"]
    pub fn expires_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signature` after provisioning.\n"]
    pub fn signature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uploaded_on` after provisioning.\n"]
    pub fn uploaded_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uploaded_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_ssl_options` after provisioning.\n"]
    pub fn custom_ssl_options(&self) -> ListRef<CustomSslCustomSslOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_ssl_options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_ssl_priority` after provisioning.\n"]
    pub fn custom_ssl_priority(&self) -> ListRef<CustomSslCustomSslPriorityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_ssl_priority", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CustomSslCustomSslOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bundle_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo_restrictions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl CustomSslCustomSslOptionsEl {
    #[doc= "Set the field `bundle_method`.\nMethod of building intermediate certificate chain. A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`."]
    pub fn set_bundle_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bundle_method = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate`.\nCertificate certificate and the intermediate(s)."]
    pub fn set_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `geo_restrictions`.\nSpecifies the region where your private key can be held locally. Available values: `us`, `eu`, `highest_security`."]
    pub fn set_geo_restrictions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.geo_restrictions = Some(v.into());
        self
    }

    #[doc= "Set the field `private_key`.\nCertificate's private key."]
    pub fn set_private_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_key = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nWhether to enable support for legacy clients which do not include SNI in the TLS handshake. Available values: `legacy_custom`, `sni_custom`."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for CustomSslCustomSslOptionsEl {
    type O = BlockAssignable<CustomSslCustomSslOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomSslCustomSslOptionsEl {}

impl BuildCustomSslCustomSslOptionsEl {
    pub fn build(self) -> CustomSslCustomSslOptionsEl {
        CustomSslCustomSslOptionsEl {
            bundle_method: core::default::Default::default(),
            certificate: core::default::Default::default(),
            geo_restrictions: core::default::Default::default(),
            private_key: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct CustomSslCustomSslOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomSslCustomSslOptionsElRef {
    fn new(shared: StackShared, base: String) -> CustomSslCustomSslOptionsElRef {
        CustomSslCustomSslOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomSslCustomSslOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bundle_method` after provisioning.\nMethod of building intermediate certificate chain. A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`."]
    pub fn bundle_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bundle_method", self.base))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\nCertificate certificate and the intermediate(s)."]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `geo_restrictions` after provisioning.\nSpecifies the region where your private key can be held locally. Available values: `us`, `eu`, `highest_security`."]
    pub fn geo_restrictions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.geo_restrictions", self.base))
    }

    #[doc= "Get a reference to the value of field `private_key` after provisioning.\nCertificate's private key."]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nWhether to enable support for legacy clients which do not include SNI in the TLS handshake. Available values: `legacy_custom`, `sni_custom`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomSslCustomSslPriorityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
}

impl CustomSslCustomSslPriorityEl {
    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
}

impl ToListMappable for CustomSslCustomSslPriorityEl {
    type O = BlockAssignable<CustomSslCustomSslPriorityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomSslCustomSslPriorityEl {}

impl BuildCustomSslCustomSslPriorityEl {
    pub fn build(self) -> CustomSslCustomSslPriorityEl {
        CustomSslCustomSslPriorityEl {
            id: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}

pub struct CustomSslCustomSslPriorityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomSslCustomSslPriorityElRef {
    fn new(shared: StackShared, base: String) -> CustomSslCustomSslPriorityElRef {
        CustomSslCustomSslPriorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomSslCustomSslPriorityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomSslDynamic {
    custom_ssl_options: Option<DynamicBlock<CustomSslCustomSslOptionsEl>>,
    custom_ssl_priority: Option<DynamicBlock<CustomSslCustomSslPriorityEl>>,
}
