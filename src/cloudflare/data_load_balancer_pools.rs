use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataLoadBalancerPoolsData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataLoadBalancerPoolsFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pools: std::option::Option<std::vec::Vec<DataLoadBalancerPoolsPoolsEl>>,
    dynamic: DataLoadBalancerPoolsDynamic,
}

struct DataLoadBalancerPools_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLoadBalancerPoolsData>,
}

#[derive(Clone)]
pub struct DataLoadBalancerPools(std::rc::Rc<DataLoadBalancerPools_>);

impl DataLoadBalancerPools {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLoadBalancerPoolsFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pools`.\n"]
    pub fn set_pools(self, v: impl std::convert::Into<terrars::BlockAssignable<DataLoadBalancerPoolsPoolsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the datasource lookups."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataLoadBalancerPoolsFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pools` after provisioning.\n"]
    pub fn pools(&self) -> terrars::ListRef<DataLoadBalancerPoolsPoolsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pools", self.extract_ref()))
    }
}

impl terrars::Referable for DataLoadBalancerPools {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLoadBalancerPools { }

impl terrars::ToListMappable for DataLoadBalancerPools {
    type O = terrars::ListRef<DataLoadBalancerPoolsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLoadBalancerPools_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_load_balancer_pools".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLoadBalancerPools {
    pub tf_id: std::string::String,
    #[doc = "The account identifier to target for the datasource lookups."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLoadBalancerPools {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLoadBalancerPools {
        let out = DataLoadBalancerPools(std::rc::Rc::new(DataLoadBalancerPools_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLoadBalancerPoolsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                id: core::default::Default::default(),
                filter: core::default::Default::default(),
                pools: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLoadBalancerPoolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPoolsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLoadBalancerPoolsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the datasource lookups."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataLoadBalancerPoolsFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pools` after provisioning.\n"]
    pub fn pools(&self) -> terrars::ListRef<DataLoadBalancerPoolsPoolsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pools", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPoolsFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLoadBalancerPoolsFilterEl {
    #[doc = "Set the field `name`.\nA regular expression matching the name of the Load Balancer pool to lookup."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadBalancerPoolsFilterEl {
    type O = terrars::BlockAssignable<DataLoadBalancerPoolsFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPoolsFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancerPoolsFilterElRef {
        DataLoadBalancerPoolsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPoolsFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA regular expression matching the name of the Load Balancer pool to lookup."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPoolsPoolsElLoadSheddingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLoadBalancerPoolsPoolsElLoadSheddingEl {
    #[doc = "Set the field `default_percent`.\n"]
    pub fn set_default_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_policy`.\n"]
    pub fn set_default_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_percent`.\n"]
    pub fn set_session_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.session_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_policy`.\n"]
    pub fn set_session_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.session_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadBalancerPoolsPoolsElLoadSheddingEl {
    type O = terrars::BlockAssignable<DataLoadBalancerPoolsPoolsElLoadSheddingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPoolsPoolsElLoadSheddingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancerPoolsPoolsElLoadSheddingElRef {
        DataLoadBalancerPoolsPoolsElLoadSheddingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPoolsPoolsElLoadSheddingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_percent` after provisioning.\n"]
    pub fn default_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `default_policy` after provisioning.\n"]
    pub fn default_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `session_percent` after provisioning.\n"]
    pub fn session_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `session_policy` after provisioning.\n"]
    pub fn session_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPoolsPoolsElOriginsElHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataLoadBalancerPoolsPoolsElOriginsElHeaderEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadBalancerPoolsPoolsElOriginsElHeaderEl {
    type O = terrars::BlockAssignable<DataLoadBalancerPoolsPoolsElOriginsElHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef {
        DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPoolsPoolsElOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::SetField<DataLoadBalancerPoolsPoolsElOriginsElHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLoadBalancerPoolsPoolsElOriginsEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataLoadBalancerPoolsPoolsElOriginsElHeaderEl>>,
    ) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadBalancerPoolsPoolsElOriginsEl {
    type O = terrars::BlockAssignable<DataLoadBalancerPoolsPoolsElOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPoolsPoolsElOriginsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancerPoolsPoolsElOriginsElRef {
        DataLoadBalancerPoolsPoolsElOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPoolsPoolsElOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::SetRef<DataLoadBalancerPoolsPoolsElOriginsElHeaderElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPoolsPoolsEl {}

impl DataLoadBalancerPoolsPoolsEl { }

impl terrars::ToListMappable for DataLoadBalancerPoolsPoolsEl {
    type O = terrars::BlockAssignable<DataLoadBalancerPoolsPoolsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPoolsPoolsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancerPoolsPoolsElRef {
        DataLoadBalancerPoolsPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPoolsPoolsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_regions` after provisioning.\nList of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://support.cloudflare.com/hc/en-us/articles/115000540888-Load-Balancing-Geographic-Regions)."]
    pub fn check_regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.check_regions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nBrief description of the Load Balancer Pool intention."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether this pool is enabled. Disabled pools will not receive traffic and are excluded from health checks."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID for this load balancer pool."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `latitude` after provisioning.\nLatitude this pool is physically located at; used for proximity steering."]
    pub fn latitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.base))
    }

    #[doc =
        "Get a reference to the value of field `load_shedding` after provisioning.\nSetting for controlling load shedding for this pool."]
    pub fn load_shedding(&self) -> terrars::SetRef<DataLoadBalancerPoolsPoolsElLoadSheddingElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.load_shedding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `longitude` after provisioning.\nLongitude this pool is physically located at; used for proximity steering."]
    pub fn longitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_origins` after provisioning.\nMinimum number of origins that must be healthy for this pool to serve traffic."]
    pub fn minimum_origins(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monitor` after provisioning.\nID of the Monitor to use for health checking origins within this pool."]
    pub fn monitor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nShort name (tag) for the pool."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `notification_email` after provisioning.\nEmail address to send health status notifications to. Multiple emails are set as a comma delimited list."]
    pub fn notification_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origins` after provisioning.\nThe list of origins within this pool."]
    pub fn origins(&self) -> terrars::SetRef<DataLoadBalancerPoolsPoolsElOriginsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.origins", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLoadBalancerPoolsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataLoadBalancerPoolsFilterEl>>,
    pools: std::option::Option<terrars::DynamicBlock<DataLoadBalancerPoolsPoolsEl>>,
}
