#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicWanStaticRouteData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    route_id: terrars::PrimField<std::string::String>,
}

struct DataMagicWanStaticRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicWanStaticRouteData_>,
}

#[derive(Clone)]
pub struct DataMagicWanStaticRoute(std::rc::Rc<DataMagicWanStaticRoute_>);

impl DataMagicWanStaticRoute {
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

    #[doc = "Set the field `account_id`.\nIdentifier"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route` after provisioning.\n"]
    pub fn route(&self) -> DataMagicWanStaticRoute_RouteRef {
        DataMagicWanStaticRoute_RouteRef::new(self.shared().clone(), format!("{}.route", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_id` after provisioning.\nIdentifier"]
    pub fn route_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataMagicWanStaticRoute {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicWanStaticRoute { }

impl terrars::ToListMappable for DataMagicWanStaticRoute {
    type O = terrars::ListRef<DataMagicWanStaticRouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicWanStaticRoute_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_wan_static_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicWanStaticRoute {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub route_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMagicWanStaticRoute {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicWanStaticRoute {
        let out = DataMagicWanStaticRoute(std::rc::Rc::new(DataMagicWanStaticRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicWanStaticRouteData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                route_id: self.route_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMagicWanStaticRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanStaticRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicWanStaticRouteRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route` after provisioning.\n"]
    pub fn route(&self) -> DataMagicWanStaticRoute_RouteRef {
        DataMagicWanStaticRoute_RouteRef::new(self.shared().clone(), format!("{}.route", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_id` after provisioning.\nIdentifier"]
    pub fn route_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMagicWanStaticRoute_Route_Scope {}

impl DataMagicWanStaticRoute_Route_Scope { }

impl terrars::ToListMappable for DataMagicWanStaticRoute_Route_Scope {
    type O = terrars::BlockAssignable<DataMagicWanStaticRoute_Route_Scope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanStaticRoute_Route_Scope {}

impl BuildDataMagicWanStaticRoute_Route_Scope {
    pub fn build(self) -> DataMagicWanStaticRoute_Route_Scope {
        DataMagicWanStaticRoute_Route_Scope {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanStaticRoute_Route_ScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanStaticRoute_Route_ScopeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicWanStaticRoute_Route_ScopeRef {
        DataMagicWanStaticRoute_Route_ScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanStaticRoute_Route_ScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `colo_names` after provisioning.\nList of colo names for the ECMP scope."]
    pub fn colo_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.colo_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `colo_regions` after provisioning.\nList of colo regions for the ECMP scope."]
    pub fn colo_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.colo_regions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicWanStaticRoute_Route {}

impl DataMagicWanStaticRoute_Route { }

impl terrars::ToListMappable for DataMagicWanStaticRoute_Route {
    type O = terrars::BlockAssignable<DataMagicWanStaticRoute_Route>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicWanStaticRoute_Route {}

impl BuildDataMagicWanStaticRoute_Route {
    pub fn build(self) -> DataMagicWanStaticRoute_Route {
        DataMagicWanStaticRoute_Route {}
    }
}

#[allow(dead_code)]
pub struct DataMagicWanStaticRoute_RouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicWanStaticRoute_RouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicWanStaticRoute_RouteRef {
        DataMagicWanStaticRoute_RouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicWanStaticRoute_RouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the route was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional human provided description of the static route."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the route was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nexthop` after provisioning.\nThe next-hop IP Address for the static route."]
    pub fn nexthop(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nexthop", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nIP Prefix in Classless Inter-Domain Routing format."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority of the static route."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\nUsed only for ECMP routes."]
    pub fn scope(&self) -> DataMagicWanStaticRoute_Route_ScopeRef {
        DataMagicWanStaticRoute_Route_ScopeRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nOptional weight of the ECMP scope - if provided."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
