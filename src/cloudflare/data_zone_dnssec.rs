use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZoneDnssecData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    zone_id: PrimField<String>,
}

struct DataZoneDnssec_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataZoneDnssecData>,
}

#[derive(Clone)]
pub struct DataZoneDnssec(Rc<DataZoneDnssec_>);

impl DataZoneDnssec {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\nPublic Key for the Zone DNSSEC."]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe status of the Zone DNSSEC."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for DataZoneDnssec {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataZoneDnssec { }

impl ToListMappable for DataZoneDnssec {
    type O = ListRef<DataZoneDnssecRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataZoneDnssec_ {
    fn extract_datasource_type(&self) -> String {
        "cloudflare_zone_dnssec".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZoneDnssec {
    pub tf_id: String,
    #[doc= "The zone identifier to target for the resource."]
    pub zone_id: PrimField<String>,
}

impl BuildDataZoneDnssec {
    pub fn build(self, stack: &mut Stack) -> DataZoneDnssec {
        let out = DataZoneDnssec(Rc::new(DataZoneDnssec_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataZoneDnssecData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZoneDnssecRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataZoneDnssecRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZoneDnssecRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\nPublic Key for the Zone DNSSEC."]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe status of the Zone DNSSEC."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
