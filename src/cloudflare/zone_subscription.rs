#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneSubscriptionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_plan: std::option::Option<ZoneSubscription_RatePlan>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct ZoneSubscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZoneSubscriptionData_>,
}

#[derive(Clone)]
pub struct ZoneSubscription(std::rc::Rc<ZoneSubscription_>);

impl ZoneSubscription {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc =
        "Set the field `frequency`.\nHow often the subscription is renewed automatically.\nAvailable values: \"weekly\", \"monthly\", \"quarterly\", \"yearly\".\nNote: Some plans may not support frequency configuration and will return \"not-applicable\"."]
    pub fn set_frequency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_plan`.\nThe rate plan applied to the subscription."]
    pub fn set_rate_plan(self, v: impl std::convert::Into<ZoneSubscription_RatePlan>) -> Self {
        self.0.data.borrow_mut().rate_plan = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `frequency` after provisioning.\nHow often the subscription is renewed automatically.\nAvailable values: \"weekly\", \"monthly\", \"quarterly\", \"yearly\".\nNote: Some plans may not support frequency configuration and will return \"not-applicable\"."]
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
    pub fn rate_plan(&self) -> ZoneSubscription_RatePlanRef {
        ZoneSubscription_RatePlanRef::new(self.shared().clone(), format!("{}.rate_plan", self.extract_ref()))
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

impl terrars::Referable for ZoneSubscription {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZoneSubscription { }

impl terrars::ToListMappable for ZoneSubscription {
    type O = terrars::ListRef<ZoneSubscriptionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZoneSubscription_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zone_subscription".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneSubscription {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildZoneSubscription {
    pub fn build(self, stack: &mut terrars::Stack) -> ZoneSubscription {
        let out = ZoneSubscription(std::rc::Rc::new(ZoneSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZoneSubscriptionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                frequency: core::default::Default::default(),
                rate_plan: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneSubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSubscriptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneSubscriptionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `frequency` after provisioning.\nHow often the subscription is renewed automatically.\nAvailable values: \"weekly\", \"monthly\", \"quarterly\", \"yearly\".\nNote: Some plans may not support frequency configuration and will return \"not-applicable\"."]
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
    pub fn rate_plan(&self) -> ZoneSubscription_RatePlanRef {
        ZoneSubscription_RatePlanRef::new(self.shared().clone(), format!("{}.rate_plan", self.extract_ref()))
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
pub struct ZoneSubscription_RatePlan {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZoneSubscription_RatePlan {
    #[doc =
        "Set the field `id`.\nThe ID of the rate plan.\nAvailable values: \"free\", \"lite\", \"pro\", \"pro_plus\", \"business\", \"enterprise\", \"partners_free\", \"partners_pro\", \"partners_business\", \"partners_enterprise\", \"partners_ent\"."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nThe scope that this rate plan applies to."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZoneSubscription_RatePlan {
    type O = terrars::BlockAssignable<ZoneSubscription_RatePlan>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSubscription_RatePlan {}

impl BuildZoneSubscription_RatePlan {
    pub fn build(self) -> ZoneSubscription_RatePlan {
        ZoneSubscription_RatePlan {
            id: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZoneSubscription_RatePlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSubscription_RatePlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZoneSubscription_RatePlanRef {
        ZoneSubscription_RatePlanRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSubscription_RatePlanRef {
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
