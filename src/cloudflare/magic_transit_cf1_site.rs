#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct MagicTransitCf1SiteData_ {
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
    account_id: terrars::PrimField<std::string::String>,
    body: std::vec::Vec<MagicTransitCf1Site_Body>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<MagicTransitCf1Site_Location>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct MagicTransitCf1Site_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MagicTransitCf1SiteData_>,
}

#[derive(Clone)]
pub struct MagicTransitCf1Site(std::rc::Rc<MagicTransitCf1Site_>);

impl MagicTransitCf1Site {
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

    #[doc = "Set the field `description`.\nA human-provided description of the CF1 Site."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(self, v: impl std::convert::Into<MagicTransitCf1Site_Location>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nA human-provided name describing the CF1 Site that should be unique within the account."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::ListRef<MagicTransitCf1Site_BodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-provided description of the CF1 Site."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> MagicTransitCf1Site_LocationRef {
        MagicTransitCf1Site_LocationRef::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA human-provided name describing the CF1 Site that should be unique within the account."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl terrars::Referable for MagicTransitCf1Site {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MagicTransitCf1Site { }

impl terrars::ToListMappable for MagicTransitCf1Site {
    type O = terrars::ListRef<MagicTransitCf1SiteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MagicTransitCf1Site_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_cf1_site".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMagicTransitCf1Site {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub body: std::vec::Vec<MagicTransitCf1Site_Body>,
}

impl BuildMagicTransitCf1Site {
    pub fn build(self, stack: &mut terrars::Stack) -> MagicTransitCf1Site {
        let out = MagicTransitCf1Site(std::rc::Rc::new(MagicTransitCf1Site_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MagicTransitCf1SiteData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                body: self.body,
                description: core::default::Default::default(),
                location: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MagicTransitCf1SiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitCf1SiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MagicTransitCf1SiteRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::ListRef<MagicTransitCf1Site_BodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-provided description of the CF1 Site."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> MagicTransitCf1Site_LocationRef {
        MagicTransitCf1Site_LocationRef::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA human-provided name describing the CF1 Site that should be unique within the account."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MagicTransitCf1Site_Body_Location {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lat: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MagicTransitCf1Site_Body_Location {
    #[doc = "Set the field `lat`.\nLatitude of the CF1 Site."]
    pub fn set_lat(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lat = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `long`.\nLongitude of the CF1 Site."]
    pub fn set_long(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.long = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of nearest town, city, or village."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitCf1Site_Body_Location {
    type O = terrars::BlockAssignable<MagicTransitCf1Site_Body_Location>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitCf1Site_Body_Location {}

impl BuildMagicTransitCf1Site_Body_Location {
    pub fn build(self) -> MagicTransitCf1Site_Body_Location {
        MagicTransitCf1Site_Body_Location {
            lat: core::default::Default::default(),
            long: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitCf1Site_Body_LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitCf1Site_Body_LocationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicTransitCf1Site_Body_LocationRef {
        MagicTransitCf1Site_Body_LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitCf1Site_Body_LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lat` after provisioning.\nLatitude of the CF1 Site."]
    pub fn lat(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lat", self.base))
    }

    #[doc = "Get a reference to the value of field `long` after provisioning.\nLongitude of the CF1 Site."]
    pub fn long(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of nearest town, city, or village."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicTransitCf1Site_Body {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<MagicTransitCf1Site_Body_Location>,
    name: terrars::PrimField<std::string::String>,
}

impl MagicTransitCf1Site_Body {
    #[doc = "Set the field `description`.\nA human-provided description of the CF1 Site."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<MagicTransitCf1Site_Body_Location>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitCf1Site_Body {
    type O = terrars::BlockAssignable<MagicTransitCf1Site_Body>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitCf1Site_Body {
    #[doc = "A human-provided name describing the CF1 Site that should be unique within the account."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMagicTransitCf1Site_Body {
    pub fn build(self) -> MagicTransitCf1Site_Body {
        MagicTransitCf1Site_Body {
            description: core::default::Default::default(),
            location: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitCf1Site_BodyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitCf1Site_BodyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicTransitCf1Site_BodyRef {
        MagicTransitCf1Site_BodyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitCf1Site_BodyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-provided description of the CF1 Site."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> MagicTransitCf1Site_Body_LocationRef {
        MagicTransitCf1Site_Body_LocationRef::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA human-provided name describing the CF1 Site that should be unique within the account."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicTransitCf1Site_Location {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lat: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MagicTransitCf1Site_Location {
    #[doc = "Set the field `lat`.\nLatitude of the CF1 Site."]
    pub fn set_lat(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lat = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `long`.\nLongitude of the CF1 Site."]
    pub fn set_long(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.long = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of nearest town, city, or village."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitCf1Site_Location {
    type O = terrars::BlockAssignable<MagicTransitCf1Site_Location>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitCf1Site_Location {}

impl BuildMagicTransitCf1Site_Location {
    pub fn build(self) -> MagicTransitCf1Site_Location {
        MagicTransitCf1Site_Location {
            lat: core::default::Default::default(),
            long: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitCf1Site_LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitCf1Site_LocationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicTransitCf1Site_LocationRef {
        MagicTransitCf1Site_LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitCf1Site_LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lat` after provisioning.\nLatitude of the CF1 Site."]
    pub fn lat(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lat", self.base))
    }

    #[doc = "Get a reference to the value of field `long` after provisioning.\nLongitude of the CF1 Site."]
    pub fn long(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of nearest town, city, or village."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
