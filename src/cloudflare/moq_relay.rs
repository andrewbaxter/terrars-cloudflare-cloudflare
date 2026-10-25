#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct MoqRelayData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<MoqRelay_Config>,
    name: terrars::PrimField<std::string::String>,
}

struct MoqRelay_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MoqRelayData_>,
}

#[derive(Clone)]
pub struct MoqRelay(std::rc::Rc<MoqRelay_>);

impl MoqRelay {
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

    #[doc = "Set the field `config`.\nupstreams and lingering_subscribe are mutually exclusive."]
    pub fn set_config(self, v: impl std::convert::Into<MoqRelay_Config>) -> Self {
        self.0.data.borrow_mut().config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nupstreams and lingering_subscribe are mutually exclusive."]
    pub fn config(&self) -> MoqRelay_ConfigRef {
        MoqRelay_ConfigRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nServer-generated unique identifier (32 hex chars)."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\n"]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nHuman-readable name for the relay."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\n\"connected\" when active, omitted otherwise.\nAvailable values: \"connected\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_publish_subscribe` after provisioning.\nFull access token (publish + subscribe). Treat as sensitive."]
    pub fn token_publish_subscribe(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_publish_subscribe", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_subscribe` after provisioning.\nSubscribe-only token. Treat as sensitive."]
    pub fn token_subscribe(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_subscribe", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer-generated unique identifier (32 hex chars)."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}

impl terrars::Referable for MoqRelay {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MoqRelay { }

impl terrars::ToListMappable for MoqRelay {
    type O = terrars::ListRef<MoqRelayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MoqRelay_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_moq_relay".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMoqRelay {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Human-readable name for the relay."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMoqRelay {
    pub fn build(self, stack: &mut terrars::Stack) -> MoqRelay {
        let out = MoqRelay(std::rc::Rc::new(MoqRelay_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MoqRelayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                config: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MoqRelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MoqRelayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MoqRelayRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nupstreams and lingering_subscribe are mutually exclusive."]
    pub fn config(&self) -> MoqRelay_ConfigRef {
        MoqRelay_ConfigRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nServer-generated unique identifier (32 hex chars)."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\n"]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nHuman-readable name for the relay."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\n\"connected\" when active, omitted otherwise.\nAvailable values: \"connected\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_publish_subscribe` after provisioning.\nFull access token (publish + subscribe). Treat as sensitive."]
    pub fn token_publish_subscribe(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_publish_subscribe", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_subscribe` after provisioning.\nSubscribe-only token. Treat as sensitive."]
    pub fn token_subscribe(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_subscribe", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer-generated unique identifier (32 hex chars)."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MoqRelay_Config_LingeringSubscribe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_timeout_ms: std::option::Option<terrars::PrimField<f64>>,
}

impl MoqRelay_Config_LingeringSubscribe {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_timeout_ms`.\nRelay-level ceiling on lingering subscribe timeout (ms). Default 30000."]
    pub fn set_max_timeout_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_timeout_ms = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MoqRelay_Config_LingeringSubscribe {
    type O = terrars::BlockAssignable<MoqRelay_Config_LingeringSubscribe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMoqRelay_Config_LingeringSubscribe {}

impl BuildMoqRelay_Config_LingeringSubscribe {
    pub fn build(self) -> MoqRelay_Config_LingeringSubscribe {
        MoqRelay_Config_LingeringSubscribe {
            enabled: core::default::Default::default(),
            max_timeout_ms: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MoqRelay_Config_LingeringSubscribeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MoqRelay_Config_LingeringSubscribeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MoqRelay_Config_LingeringSubscribeRef {
        MoqRelay_Config_LingeringSubscribeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MoqRelay_Config_LingeringSubscribeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_timeout_ms` after provisioning.\nRelay-level ceiling on lingering subscribe timeout (ms). Default 30000."]
    pub fn max_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_timeout_ms", self.base))
    }
}

#[derive(Serialize)]
pub struct MoqRelay_Config_Upstreams_Upstreams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MoqRelay_Config_Upstreams_Upstreams {
    #[doc =
        "Set the field `url`.\nUpstream MOQT server publisher URL. Must be an absolute URL with a\nhost and a scheme the relay can dial: moqt:// (raw QUIC) or https://\n(WebTransport). Validated on update (PUT); rejected with 21013."]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MoqRelay_Config_Upstreams_Upstreams {
    type O = terrars::BlockAssignable<MoqRelay_Config_Upstreams_Upstreams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMoqRelay_Config_Upstreams_Upstreams {}

impl BuildMoqRelay_Config_Upstreams_Upstreams {
    pub fn build(self) -> MoqRelay_Config_Upstreams_Upstreams {
        MoqRelay_Config_Upstreams_Upstreams { url: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MoqRelay_Config_Upstreams_UpstreamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MoqRelay_Config_Upstreams_UpstreamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MoqRelay_Config_Upstreams_UpstreamsRef {
        MoqRelay_Config_Upstreams_UpstreamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MoqRelay_Config_Upstreams_UpstreamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nUpstream MOQT server publisher URL. Must be an absolute URL with a\nhost and a scheme the relay can dial: moqt:// (raw QUIC) or https://\n(WebTransport). Validated on update (PUT); rejected with 21013."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct MoqRelay_Config_Upstreams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstreams: std::option::Option<std::vec::Vec<MoqRelay_Config_Upstreams_Upstreams>>,
}

impl MoqRelay_Config_Upstreams {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `upstreams`.\nOrdered list of upstream MOQT server publishers. Each entry is an\nobject (not a bare string) so per-upstream configuration can be\nadded in the future without another breaking change."]
    pub fn set_upstreams(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<MoqRelay_Config_Upstreams_Upstreams>>,
    ) -> Self {
        self.upstreams = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MoqRelay_Config_Upstreams {
    type O = terrars::BlockAssignable<MoqRelay_Config_Upstreams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMoqRelay_Config_Upstreams {}

impl BuildMoqRelay_Config_Upstreams {
    pub fn build(self) -> MoqRelay_Config_Upstreams {
        MoqRelay_Config_Upstreams {
            enabled: core::default::Default::default(),
            upstreams: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MoqRelay_Config_UpstreamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MoqRelay_Config_UpstreamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MoqRelay_Config_UpstreamsRef {
        MoqRelay_Config_UpstreamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MoqRelay_Config_UpstreamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `upstreams` after provisioning.\nOrdered list of upstream MOQT server publishers. Each entry is an\nobject (not a bare string) so per-upstream configuration can be\nadded in the future without another breaking change."]
    pub fn upstreams(&self) -> terrars::ListRef<MoqRelay_Config_Upstreams_UpstreamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstreams", self.base))
    }
}

#[derive(Serialize)]
pub struct MoqRelay_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lingering_subscribe: std::option::Option<MoqRelay_Config_LingeringSubscribe>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstreams: std::option::Option<MoqRelay_Config_Upstreams>,
}

impl MoqRelay_Config {
    #[doc = "Set the field `lingering_subscribe`.\n"]
    pub fn set_lingering_subscribe(mut self, v: impl std::convert::Into<MoqRelay_Config_LingeringSubscribe>) -> Self {
        self.lingering_subscribe = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `upstreams`.\nUpstreams are external MOQT server publishers that a relay falls back\nto when it has no local publisher for a requested namespace/track."]
    pub fn set_upstreams(mut self, v: impl std::convert::Into<MoqRelay_Config_Upstreams>) -> Self {
        self.upstreams = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MoqRelay_Config {
    type O = terrars::BlockAssignable<MoqRelay_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMoqRelay_Config {}

impl BuildMoqRelay_Config {
    pub fn build(self) -> MoqRelay_Config {
        MoqRelay_Config {
            lingering_subscribe: core::default::Default::default(),
            upstreams: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MoqRelay_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MoqRelay_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MoqRelay_ConfigRef {
        MoqRelay_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MoqRelay_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lingering_subscribe` after provisioning.\n"]
    pub fn lingering_subscribe(&self) -> MoqRelay_Config_LingeringSubscribeRef {
        MoqRelay_Config_LingeringSubscribeRef::new(
            self.shared().clone(),
            format!("{}.lingering_subscribe", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `upstreams` after provisioning.\nUpstreams are external MOQT server publishers that a relay falls back\nto when it has no local publisher for a requested namespace/track."]
    pub fn upstreams(&self) -> MoqRelay_Config_UpstreamsRef {
        MoqRelay_Config_UpstreamsRef::new(self.shared().clone(), format!("{}.upstreams", self.base))
    }
}
