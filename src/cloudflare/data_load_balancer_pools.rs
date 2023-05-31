use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataLoadBalancerPoolsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataLoadBalancerPoolsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pools: Option<Vec<DataLoadBalancerPoolsPoolsEl>>,
    dynamic: DataLoadBalancerPoolsDynamic,
}

struct DataLoadBalancerPools_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLoadBalancerPoolsData>,
}

#[derive(Clone)]
pub struct DataLoadBalancerPools(Rc<DataLoadBalancerPools_>);

impl DataLoadBalancerPools {
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
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataLoadBalancerPoolsFilterEl>>) -> Self {
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

    #[doc= "Set the field `pools`.\n"]
    pub fn set_pools(self, v: impl Into<BlockAssignable<DataLoadBalancerPoolsPoolsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pools = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pools = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the datasource lookups."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataLoadBalancerPoolsFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pools` after provisioning.\n"]
    pub fn pools(&self) -> ListRef<DataLoadBalancerPoolsPoolsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pools", self.extract_ref()))
    }
}

impl Referable for DataLoadBalancerPools {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataLoadBalancerPools { }

impl ToListMappable for DataLoadBalancerPools {
    type O = ListRef<DataLoadBalancerPoolsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataLoadBalancerPools_ {
    fn extract_datasource_type(&self) -> String {
        "cloudflare_load_balancer_pools".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLoadBalancerPools {
    pub tf_id: String,
    #[doc= "The account identifier to target for the datasource lookups."]
    pub account_id: PrimField<String>,
}

impl BuildDataLoadBalancerPools {
    pub fn build(self, stack: &mut Stack) -> DataLoadBalancerPools {
        let out = DataLoadBalancerPools(Rc::new(DataLoadBalancerPools_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLoadBalancerPoolsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
                filter: core::default::Default::default(),
                pools: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLoadBalancerPoolsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLoadBalancerPoolsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLoadBalancerPoolsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the datasource lookups."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataLoadBalancerPoolsFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pools` after provisioning.\n"]
    pub fn pools(&self) -> ListRef<DataLoadBalancerPoolsPoolsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pools", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPoolsFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataLoadBalancerPoolsFilterEl {
    #[doc= "Set the field `name`.\nA regular expression matching the name of the Load Balancer pool to lookup."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataLoadBalancerPoolsFilterEl {
    type O = BlockAssignable<DataLoadBalancerPoolsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPoolsFilterEl {}

impl BuildDataLoadBalancerPoolsFilterEl {
    pub fn build(self) -> DataLoadBalancerPoolsFilterEl {
        DataLoadBalancerPoolsFilterEl { name: core::default::Default::default() }
    }
}

pub struct DataLoadBalancerPoolsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLoadBalancerPoolsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataLoadBalancerPoolsFilterElRef {
        DataLoadBalancerPoolsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPoolsFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA regular expression matching the name of the Load Balancer pool to lookup."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPoolsPoolsElLoadSheddingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_policy: Option<PrimField<String>>,
}

impl DataLoadBalancerPoolsPoolsElLoadSheddingEl {
    #[doc= "Set the field `default_percent`.\n"]
    pub fn set_default_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_percent = Some(v.into());
        self
    }

    #[doc= "Set the field `default_policy`.\n"]
    pub fn set_default_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `session_percent`.\n"]
    pub fn set_session_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_percent = Some(v.into());
        self
    }

    #[doc= "Set the field `session_policy`.\n"]
    pub fn set_session_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_policy = Some(v.into());
        self
    }
}

impl ToListMappable for DataLoadBalancerPoolsPoolsElLoadSheddingEl {
    type O = BlockAssignable<DataLoadBalancerPoolsPoolsElLoadSheddingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPoolsPoolsElLoadSheddingEl {}

impl BuildDataLoadBalancerPoolsPoolsElLoadSheddingEl {
    pub fn build(self) -> DataLoadBalancerPoolsPoolsElLoadSheddingEl {
        DataLoadBalancerPoolsPoolsElLoadSheddingEl {
            default_percent: core::default::Default::default(),
            default_policy: core::default::Default::default(),
            session_percent: core::default::Default::default(),
            session_policy: core::default::Default::default(),
        }
    }
}

pub struct DataLoadBalancerPoolsPoolsElLoadSheddingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLoadBalancerPoolsPoolsElLoadSheddingElRef {
    fn new(shared: StackShared, base: String) -> DataLoadBalancerPoolsPoolsElLoadSheddingElRef {
        DataLoadBalancerPoolsPoolsElLoadSheddingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPoolsPoolsElLoadSheddingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_percent` after provisioning.\n"]
    pub fn default_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_percent", self.base))
    }

    #[doc= "Get a reference to the value of field `default_policy` after provisioning.\n"]
    pub fn default_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `session_percent` after provisioning.\n"]
    pub fn session_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_percent", self.base))
    }

    #[doc= "Get a reference to the value of field `session_policy` after provisioning.\n"]
    pub fn session_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPoolsPoolsElOriginsElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataLoadBalancerPoolsPoolsElOriginsElHeaderEl {
    #[doc= "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataLoadBalancerPoolsPoolsElOriginsElHeaderEl {
    type O = BlockAssignable<DataLoadBalancerPoolsPoolsElOriginsElHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPoolsPoolsElOriginsElHeaderEl {}

impl BuildDataLoadBalancerPoolsPoolsElOriginsElHeaderEl {
    pub fn build(self) -> DataLoadBalancerPoolsPoolsElOriginsElHeaderEl {
        DataLoadBalancerPoolsPoolsElOriginsElHeaderEl {
            header: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef {
    fn new(shared: StackShared, base: String) -> DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef {
        DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPoolsPoolsElOriginsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<SetField<DataLoadBalancerPoolsPoolsElOriginsElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl DataLoadBalancerPoolsPoolsElOriginsEl {
    #[doc= "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<SetField<DataLoadBalancerPoolsPoolsElOriginsElHeaderEl>>) -> Self {
        self.header = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for DataLoadBalancerPoolsPoolsElOriginsEl {
    type O = BlockAssignable<DataLoadBalancerPoolsPoolsElOriginsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPoolsPoolsElOriginsEl {}

impl BuildDataLoadBalancerPoolsPoolsElOriginsEl {
    pub fn build(self) -> DataLoadBalancerPoolsPoolsElOriginsEl {
        DataLoadBalancerPoolsPoolsElOriginsEl {
            address: core::default::Default::default(),
            enabled: core::default::Default::default(),
            header: core::default::Default::default(),
            name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

pub struct DataLoadBalancerPoolsPoolsElOriginsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLoadBalancerPoolsPoolsElOriginsElRef {
    fn new(shared: StackShared, base: String) -> DataLoadBalancerPoolsPoolsElOriginsElRef {
        DataLoadBalancerPoolsPoolsElOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPoolsPoolsElOriginsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> SetRef<DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef> {
        SetRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPoolsPoolsEl {}

impl DataLoadBalancerPoolsPoolsEl { }

impl ToListMappable for DataLoadBalancerPoolsPoolsEl {
    type O = BlockAssignable<DataLoadBalancerPoolsPoolsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPoolsPoolsEl {}

impl BuildDataLoadBalancerPoolsPoolsEl {
    pub fn build(self) -> DataLoadBalancerPoolsPoolsEl {
        DataLoadBalancerPoolsPoolsEl {}
    }
}

pub struct DataLoadBalancerPoolsPoolsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLoadBalancerPoolsPoolsElRef {
    fn new(shared: StackShared, base: String) -> DataLoadBalancerPoolsPoolsElRef {
        DataLoadBalancerPoolsPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPoolsPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `check_regions` after provisioning.\nList of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://support.cloudflare.com/hc/en-us/articles/115000540888-Load-Balancing-Geographic-Regions)."]
    pub fn check_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.check_regions", self.base))
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nBrief description of the Load Balancer Pool intention."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether this pool is enabled. Disabled pools will not receive traffic and are excluded from health checks."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nID for this load balancer pool."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `latitude` after provisioning.\nLatitude this pool is physically located at; used for proximity steering."]
    pub fn latitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.base))
    }

    #[doc= "Get a reference to the value of field `load_shedding` after provisioning.\nSetting for controlling load shedding for this pool."]
    pub fn load_shedding(&self) -> SetRef<DataLoadBalancerPoolsPoolsElLoadSheddingElRef> {
        SetRef::new(self.shared().clone(), format!("{}.load_shedding", self.base))
    }

    #[doc= "Get a reference to the value of field `longitude` after provisioning.\nLongitude this pool is physically located at; used for proximity steering."]
    pub fn longitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.base))
    }

    #[doc= "Get a reference to the value of field `minimum_origins` after provisioning.\nMinimum number of origins that must be healthy for this pool to serve traffic."]
    pub fn minimum_origins(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc= "Get a reference to the value of field `monitor` after provisioning.\nID of the Monitor to use for health checking origins within this pool."]
    pub fn monitor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nShort name (tag) for the pool."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `notification_email` after provisioning.\nEmail address to send health status notifications to. Multiple emails are set as a comma delimited list."]
    pub fn notification_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notification_email", self.base))
    }

    #[doc= "Get a reference to the value of field `origins` after provisioning.\nThe list of origins within this pool."]
    pub fn origins(&self) -> SetRef<DataLoadBalancerPoolsPoolsElOriginsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.origins", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLoadBalancerPoolsDynamic {
    filter: Option<DynamicBlock<DataLoadBalancerPoolsFilterEl>>,
    pools: Option<DynamicBlock<DataLoadBalancerPoolsPoolsEl>>,
}
