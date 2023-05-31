use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneDnssecData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    modified_on: Option<PrimField<String>>,
    zone_id: PrimField<String>,
}

struct ZoneDnssec_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ZoneDnssecData>,
}

#[derive(Clone)]
pub struct ZoneDnssec(Rc<ZoneDnssec_>);

impl ZoneDnssec {
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

    #[doc= "Set the field `modified_on`.\nZone DNSSEC updated time."]
    pub fn set_modified_on(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().modified_on = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\nZone DNSSEC algorithm."]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest` after provisioning.\nZone DNSSEC digest."]
    pub fn digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest_algorithm` after provisioning.\nDigest algorithm use for Zone DNSSEC."]
    pub fn digest_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest_type` after provisioning.\nDigest Type for Zone DNSSEC."]
    pub fn digest_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ds` after provisioning.\nDS for the Zone DNSSEC."]
    pub fn ds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `flags` after provisioning.\nZone DNSSEC flags."]
    pub fn flags(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.flags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_tag` after provisioning.\nKey Tag for the Zone DNSSEC."]
    pub fn key_tag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_type` after provisioning.\nKey type used for Zone DNSSEC."]
    pub fn key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nZone DNSSEC updated time."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\nPublic Key for the Zone DNSSEC."]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe status of the Zone DNSSEC."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for ZoneDnssec {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ZoneDnssec { }

impl ToListMappable for ZoneDnssec {
    type O = ListRef<ZoneDnssecRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ZoneDnssec_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_zone_dnssec".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneDnssec {
    pub tf_id: String,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildZoneDnssec {
    pub fn build(self, stack: &mut Stack) -> ZoneDnssec {
        let out = ZoneDnssec(Rc::new(ZoneDnssec_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ZoneDnssecData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                modified_on: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneDnssecRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneDnssecRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneDnssecRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\nZone DNSSEC algorithm."]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest` after provisioning.\nZone DNSSEC digest."]
    pub fn digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest_algorithm` after provisioning.\nDigest algorithm use for Zone DNSSEC."]
    pub fn digest_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest_type` after provisioning.\nDigest Type for Zone DNSSEC."]
    pub fn digest_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ds` after provisioning.\nDS for the Zone DNSSEC."]
    pub fn ds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `flags` after provisioning.\nZone DNSSEC flags."]
    pub fn flags(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.flags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_tag` after provisioning.\nKey Tag for the Zone DNSSEC."]
    pub fn key_tag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_type` after provisioning.\nKey type used for Zone DNSSEC."]
    pub fn key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nZone DNSSEC updated time."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\nPublic Key for the Zone DNSSEC."]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe status of the Zone DNSSEC."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
