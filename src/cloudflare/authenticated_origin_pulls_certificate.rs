use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AuthenticatedOriginPullsCertificateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    certificate: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    private_key: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AuthenticatedOriginPullsCertificateTimeoutsEl>,
}

struct AuthenticatedOriginPullsCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AuthenticatedOriginPullsCertificateData>,
}

#[derive(Clone)]
pub struct AuthenticatedOriginPullsCertificate(Rc<AuthenticatedOriginPullsCertificate_>);

impl AuthenticatedOriginPullsCertificate {
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

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AuthenticatedOriginPullsCertificateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\nThe public client certificate. **Modifying this attribute will force creation of a new resource.**"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_on` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn expires_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issuer` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_key` after provisioning.\nThe private key of the client certificate. **Modifying this attribute will force creation of a new resource.**"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `serial_number` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signature` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn signature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uploaded_on` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn uploaded_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uploaded_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AuthenticatedOriginPullsCertificateTimeoutsElRef {
        AuthenticatedOriginPullsCertificateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for AuthenticatedOriginPullsCertificate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AuthenticatedOriginPullsCertificate { }

impl ToListMappable for AuthenticatedOriginPullsCertificate {
    type O = ListRef<AuthenticatedOriginPullsCertificateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AuthenticatedOriginPullsCertificate_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_authenticated_origin_pulls_certificate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAuthenticatedOriginPullsCertificate {
    pub tf_id: String,
    #[doc= "The public client certificate. **Modifying this attribute will force creation of a new resource.**"]
    pub certificate: PrimField<String>,
    #[doc= "The private key of the client certificate. **Modifying this attribute will force creation of a new resource.**"]
    pub private_key: PrimField<String>,
    #[doc= "The form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**"]
    pub type_: PrimField<String>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildAuthenticatedOriginPullsCertificate {
    pub fn build(self, stack: &mut Stack) -> AuthenticatedOriginPullsCertificate {
        let out = AuthenticatedOriginPullsCertificate(Rc::new(AuthenticatedOriginPullsCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AuthenticatedOriginPullsCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                certificate: self.certificate,
                id: core::default::Default::default(),
                private_key: self.private_key,
                type_: self.type_,
                zone_id: self.zone_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AuthenticatedOriginPullsCertificateRef {
    shared: StackShared,
    base: String,
}

impl Ref for AuthenticatedOriginPullsCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AuthenticatedOriginPullsCertificateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\nThe public client certificate. **Modifying this attribute will force creation of a new resource.**"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_on` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn expires_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issuer` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_key` after provisioning.\nThe private key of the client certificate. **Modifying this attribute will force creation of a new resource.**"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `serial_number` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signature` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn signature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uploaded_on` after provisioning.\n**Modifying this attribute will force creation of a new resource.**"]
    pub fn uploaded_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uploaded_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AuthenticatedOriginPullsCertificateTimeoutsElRef {
        AuthenticatedOriginPullsCertificateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AuthenticatedOriginPullsCertificateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl AuthenticatedOriginPullsCertificateTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for AuthenticatedOriginPullsCertificateTimeoutsEl {
    type O = BlockAssignable<AuthenticatedOriginPullsCertificateTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuthenticatedOriginPullsCertificateTimeoutsEl {}

impl BuildAuthenticatedOriginPullsCertificateTimeoutsEl {
    pub fn build(self) -> AuthenticatedOriginPullsCertificateTimeoutsEl {
        AuthenticatedOriginPullsCertificateTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct AuthenticatedOriginPullsCertificateTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AuthenticatedOriginPullsCertificateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AuthenticatedOriginPullsCertificateTimeoutsElRef {
        AuthenticatedOriginPullsCertificateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuthenticatedOriginPullsCertificateTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
