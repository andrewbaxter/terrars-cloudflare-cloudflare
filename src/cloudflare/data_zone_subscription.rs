#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZoneSubscriptionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZoneSubscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZoneSubscriptionData_>,
}

#[derive(Clone)]
pub struct DataZoneSubscription(std::rc::Rc<DataZoneSubscription_>);

impl DataZoneSubscription {
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

    #[doc = "Set the field `zone_id`.\nIdentifier"]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThe monetary unit in which pricing information is displayed."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_period_end` after provisioning.\nThe end of the current period and also when the next billing is due."]
    pub fn current_period_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_period_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_period_start` after provisioning.\nWhen the current billing period started. May match initial_period_start if this is the first period."]
    pub fn current_period_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_period_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nHow often the subscription is renewed automatically.\nAvailable values: \"weekly\", \"monthly\", \"quarterly\", \"yearly\", \"not-applicable\"."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `price` after provisioning.\nThe price of the subscription that will be billed, in US dollars."]
    pub fn price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rate_plan` after provisioning.\nThe rate plan applied to the subscription."]
    pub fn rate_plan(&self) -> DataZoneSubscription_RatePlanRef {
        DataZoneSubscription_RatePlanRef::new(self.shared().clone(), format!("{}.rate_plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state that the subscription is in.\nAvailable values: \"Trial\", \"Provisioned\", \"Paid\", \"AwaitingPayment\", \"Cancelled\", \"Failed\", \"Expired\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZoneSubscription {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZoneSubscription { }

impl terrars::ToListMappable for DataZoneSubscription {
    type O = terrars::ListRef<DataZoneSubscriptionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZoneSubscription_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zone_subscription".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZoneSubscription {
    pub tf_id: std::string::String,
}

impl BuildDataZoneSubscription {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZoneSubscription {
        let out = DataZoneSubscription(std::rc::Rc::new(DataZoneSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZoneSubscriptionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZoneSubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneSubscriptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZoneSubscriptionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThe monetary unit in which pricing information is displayed."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_period_end` after provisioning.\nThe end of the current period and also when the next billing is due."]
    pub fn current_period_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_period_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_period_start` after provisioning.\nWhen the current billing period started. May match initial_period_start if this is the first period."]
    pub fn current_period_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_period_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nHow often the subscription is renewed automatically.\nAvailable values: \"weekly\", \"monthly\", \"quarterly\", \"yearly\", \"not-applicable\"."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `price` after provisioning.\nThe price of the subscription that will be billed, in US dollars."]
    pub fn price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rate_plan` after provisioning.\nThe rate plan applied to the subscription."]
    pub fn rate_plan(&self) -> DataZoneSubscription_RatePlanRef {
        DataZoneSubscription_RatePlanRef::new(self.shared().clone(), format!("{}.rate_plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state that the subscription is in.\nAvailable values: \"Trial\", \"Provisioned\", \"Paid\", \"AwaitingPayment\", \"Cancelled\", \"Failed\", \"Expired\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZoneSubscription_RatePlan {}

impl DataZoneSubscription_RatePlan { }

impl terrars::ToListMappable for DataZoneSubscription_RatePlan {
    type O = terrars::BlockAssignable<DataZoneSubscription_RatePlan>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZoneSubscription_RatePlan {}

impl BuildDataZoneSubscription_RatePlan {
    pub fn build(self) -> DataZoneSubscription_RatePlan {
        DataZoneSubscription_RatePlan {}
    }
}

#[allow(dead_code)]
pub struct DataZoneSubscription_RatePlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneSubscription_RatePlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZoneSubscription_RatePlanRef {
        DataZoneSubscription_RatePlanRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZoneSubscription_RatePlanRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThe currency applied to the rate plan subscription."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `externally_managed` after provisioning.\nWhether this rate plan is managed externally from Cloudflare."]
    pub fn externally_managed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.externally_managed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of the rate plan.\nAvailable values: \"free\", \"lite\", \"pro\", \"pro_plus\", \"business\", \"enterprise\", \"partners_free\", \"partners_pro\", \"partners_business\", \"partners_enterprise\", \"partners_ent\"."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_contract` after provisioning.\nWhether a rate plan is enterprise-based (or newly adopted term contract)."]
    pub fn is_contract(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_contract", self.base))
    }

    #[doc = "Get a reference to the value of field `public_name` after provisioning.\nThe full name of the rate plan."]
    pub fn public_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope that this rate plan applies to."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sets` after provisioning.\nThe list of sets this rate plan applies to. Returns array of strings."]
    pub fn sets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sets", self.base))
    }
}
