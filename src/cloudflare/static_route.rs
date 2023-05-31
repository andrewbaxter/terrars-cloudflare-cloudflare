use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct StaticRouteData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colo_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colo_regions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    nexthop: PrimField<String>,
    prefix: PrimField<String>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

struct StaticRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StaticRouteData>,
}

#[derive(Clone)]
pub struct StaticRoute(Rc<StaticRoute_>);

impl StaticRoute {
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

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `colo_names`.\nList of Cloudflare colocation regions for this static route."]
    pub fn set_colo_names(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().colo_names = Some(v.into());
        self
    }

    #[doc= "Set the field `colo_regions`.\nList of Cloudflare colocation names for this static route."]
    pub fn set_colo_regions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().colo_regions = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nDescription of the static route."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\nThe optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_weight(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().weight = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `colo_names` after provisioning.\nList of Cloudflare colocation regions for this static route."]
    pub fn colo_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.colo_names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `colo_regions` after provisioning.\nList of Cloudflare colocation names for this static route."]
    pub fn colo_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.colo_regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the static route."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `nexthop` after provisioning.\nThe nexthop IP address where traffic will be routed to."]
    pub fn nexthop(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nexthop", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\nYour network prefix using CIDR notation."]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nThe priority for the static route."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nThe optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }
}

impl Referable for StaticRoute {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for StaticRoute { }

impl ToListMappable for StaticRoute {
    type O = ListRef<StaticRouteRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for StaticRoute_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_static_route".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStaticRoute {
    pub tf_id: String,
    #[doc= "The nexthop IP address where traffic will be routed to."]
    pub nexthop: PrimField<String>,
    #[doc= "Your network prefix using CIDR notation."]
    pub prefix: PrimField<String>,
    #[doc= "The priority for the static route."]
    pub priority: PrimField<f64>,
}

impl BuildStaticRoute {
    pub fn build(self, stack: &mut Stack) -> StaticRoute {
        let out = StaticRoute(Rc::new(StaticRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StaticRouteData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                colo_names: core::default::Default::default(),
                colo_regions: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                nexthop: self.nexthop,
                prefix: self.prefix,
                priority: self.priority,
                weight: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StaticRouteRef {
    shared: StackShared,
    base: String,
}

impl Ref for StaticRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StaticRouteRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `colo_names` after provisioning.\nList of Cloudflare colocation regions for this static route."]
    pub fn colo_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.colo_names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `colo_regions` after provisioning.\nList of Cloudflare colocation names for this static route."]
    pub fn colo_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.colo_regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the static route."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `nexthop` after provisioning.\nThe nexthop IP address where traffic will be routed to."]
    pub fn nexthop(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nexthop", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\nYour network prefix using CIDR notation."]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nThe priority for the static route."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nThe optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }
}
