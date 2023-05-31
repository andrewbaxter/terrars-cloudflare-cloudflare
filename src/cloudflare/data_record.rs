use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataRecordData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    zone_id: PrimField<String>,
}

struct DataRecord_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRecordData>,
}

#[derive(Clone)]
pub struct DataRecord(Rc<DataRecord_>);

impl DataRecord {
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

    #[doc= "Set the field `priority`.\nDNS priority to filter record results on."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nDNS record type to filter record results on. Defaults to `A`."]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nHostname to filter DNS record results on."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\nLocked status of the found DNS record."]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nDNS priority to filter record results on."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxiable` after provisioning.\nProxiable status of the found DNS record."]
    pub fn proxiable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxiable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxied` after provisioning.\nProxied status of the found DNS record."]
    pub fn proxied(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nTTL of the found DNS record."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nDNS record type to filter record results on. Defaults to `A`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue of the found DNS record."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_name` after provisioning.\nZone name of the found DNS record."]
    pub fn zone_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_name", self.extract_ref()))
    }
}

impl Referable for DataRecord {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRecord { }

impl ToListMappable for DataRecord {
    type O = ListRef<DataRecordRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRecord_ {
    fn extract_datasource_type(&self) -> String {
        "cloudflare_record".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRecord {
    pub tf_id: String,
    #[doc= "Hostname to filter DNS record results on."]
    pub hostname: PrimField<String>,
    #[doc= "The zone identifier to target for the resource."]
    pub zone_id: PrimField<String>,
}

impl BuildDataRecord {
    pub fn build(self, stack: &mut Stack) -> DataRecord {
        let out = DataRecord(Rc::new(DataRecord_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRecordData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                hostname: self.hostname,
                id: core::default::Default::default(),
                priority: core::default::Default::default(),
                type_: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRecordRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRecordRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRecordRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nHostname to filter DNS record results on."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\nLocked status of the found DNS record."]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nDNS priority to filter record results on."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxiable` after provisioning.\nProxiable status of the found DNS record."]
    pub fn proxiable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxiable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxied` after provisioning.\nProxied status of the found DNS record."]
    pub fn proxied(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nTTL of the found DNS record."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nDNS record type to filter record results on. Defaults to `A`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue of the found DNS record."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_name` after provisioning.\nZone name of the found DNS record."]
    pub fn zone_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_name", self.extract_ref()))
    }
}
