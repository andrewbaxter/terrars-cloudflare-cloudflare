use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZonesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataZonesFilterEl>>,
    dynamic: DataZonesDynamic,
}

struct DataZones_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataZonesData>,
}

#[derive(Clone)]
pub struct DataZones(Rc<DataZones_>);

impl DataZones {
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

    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataZonesFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zones` after provisioning.\nA list of zone objects."]
    pub fn zones(&self) -> ListRef<DataZonesZonesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataZonesFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

impl Referable for DataZones {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataZones { }

impl ToListMappable for DataZones {
    type O = ListRef<DataZonesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataZones_ {
    fn extract_datasource_type(&self) -> String {
        "cloudflare_zones".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZones {
    pub tf_id: String,
}

impl BuildDataZones {
    pub fn build(self, stack: &mut Stack) -> DataZones {
        let out = DataZones(Rc::new(DataZones_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataZonesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZonesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataZonesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZonesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zones` after provisioning.\nA list of zone objects."]
    pub fn zones(&self) -> ListRef<DataZonesZonesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataZonesFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZonesZonesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataZonesZonesEl {
    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataZonesZonesEl {
    type O = BlockAssignable<DataZonesZonesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZonesZonesEl {}

impl BuildDataZonesZonesEl {
    pub fn build(self) -> DataZonesZonesEl {
        DataZonesZonesEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataZonesZonesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataZonesZonesElRef {
    fn new(shared: StackShared, base: String) -> DataZonesZonesElRef {
        DataZonesZonesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZonesZonesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZonesFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_type: Option<PrimField<String>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paused: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataZonesFilterEl {
    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource."]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `lookup_type`.\nThe type of search to perform for the `name` value when querying the zone API. Available values: `contains`, `exact`. Defaults to `exact`."]
    pub fn set_lookup_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lookup_type = Some(v.into());
        self
    }

    #[doc= "Set the field `match_`.\nA RE2 compatible regular expression to filter the\tresults. This is performed client side whereas the `name` and `lookup_type`\tare performed on the Cloudflare server side."]
    pub fn set_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_ = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nA string value to search for."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `paused`.\nPaused status of the zone to lookup. Defaults to `false`."]
    pub fn set_paused(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.paused = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\nStatus of the zone to lookup."]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataZonesFilterEl {
    type O = BlockAssignable<DataZonesFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZonesFilterEl {}

impl BuildDataZonesFilterEl {
    pub fn build(self) -> DataZonesFilterEl {
        DataZonesFilterEl {
            account_id: core::default::Default::default(),
            lookup_type: core::default::Default::default(),
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
            paused: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataZonesFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataZonesFilterElRef {
    fn new(shared: StackShared, base: String) -> DataZonesFilterElRef {
        DataZonesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZonesFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc= "Get a reference to the value of field `lookup_type` after provisioning.\nThe type of search to perform for the `name` value when querying the zone API. Available values: `contains`, `exact`. Defaults to `exact`."]
    pub fn lookup_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lookup_type", self.base))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\nA RE2 compatible regular expression to filter the\tresults. This is performed client side whereas the `name` and `lookup_type`\tare performed on the Cloudflare server side."]
    pub fn match_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA string value to search for."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nPaused status of the zone to lookup. Defaults to `false`."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nStatus of the zone to lookup."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataZonesDynamic {
    filter: Option<DynamicBlock<DataZonesFilterEl>>,
}
