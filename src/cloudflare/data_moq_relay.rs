#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMoqRelayData_ {
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
    filter: std::option::Option<DataMoqRelay_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relay_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataMoqRelay_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMoqRelayData_>,
}

#[derive(Clone)]
pub struct DataMoqRelay(std::rc::Rc<DataMoqRelay_>);

impl DataMoqRelay {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataMoqRelay_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `relay_id`.\n"]
    pub fn set_relay_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().relay_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nupstreams and lingering_subscribe are mutually exclusive."]
    pub fn config(&self) -> DataMoqRelay_ConfigRef {
        DataMoqRelay_ConfigRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataMoqRelay_FilterRef {
        DataMoqRelay_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\n"]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `relay_id` after provisioning.\n"]
    pub fn relay_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relay_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\n\"connected\" when active, omitted otherwise.\nAvailable values: \"connected\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}

impl terrars::Referable for DataMoqRelay {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMoqRelay { }

impl terrars::ToListMappable for DataMoqRelay {
    type O = terrars::ListRef<DataMoqRelayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMoqRelay_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_moq_relay".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMoqRelay {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMoqRelay {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMoqRelay {
        let out = DataMoqRelay(std::rc::Rc::new(DataMoqRelay_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMoqRelayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                filter: core::default::Default::default(),
                relay_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMoqRelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMoqRelayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMoqRelayRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nupstreams and lingering_subscribe are mutually exclusive."]
    pub fn config(&self) -> DataMoqRelay_ConfigRef {
        DataMoqRelay_ConfigRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataMoqRelay_FilterRef {
        DataMoqRelay_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\n"]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `relay_id` after provisioning.\n"]
    pub fn relay_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relay_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\n\"connected\" when active, omitted otherwise.\nAvailable values: \"connected\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMoqRelay_Config_LingeringSubscribe {}

impl DataMoqRelay_Config_LingeringSubscribe { }

impl terrars::ToListMappable for DataMoqRelay_Config_LingeringSubscribe {
    type O = terrars::BlockAssignable<DataMoqRelay_Config_LingeringSubscribe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMoqRelay_Config_LingeringSubscribe {}

impl BuildDataMoqRelay_Config_LingeringSubscribe {
    pub fn build(self) -> DataMoqRelay_Config_LingeringSubscribe {
        DataMoqRelay_Config_LingeringSubscribe {}
    }
}

#[allow(dead_code)]
pub struct DataMoqRelay_Config_LingeringSubscribeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMoqRelay_Config_LingeringSubscribeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMoqRelay_Config_LingeringSubscribeRef {
        DataMoqRelay_Config_LingeringSubscribeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMoqRelay_Config_LingeringSubscribeRef {
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
pub struct DataMoqRelay_Config_Upstreams_Upstreams {}

impl DataMoqRelay_Config_Upstreams_Upstreams { }

impl terrars::ToListMappable for DataMoqRelay_Config_Upstreams_Upstreams {
    type O = terrars::BlockAssignable<DataMoqRelay_Config_Upstreams_Upstreams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMoqRelay_Config_Upstreams_Upstreams {}

impl BuildDataMoqRelay_Config_Upstreams_Upstreams {
    pub fn build(self) -> DataMoqRelay_Config_Upstreams_Upstreams {
        DataMoqRelay_Config_Upstreams_Upstreams {}
    }
}

#[allow(dead_code)]
pub struct DataMoqRelay_Config_Upstreams_UpstreamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMoqRelay_Config_Upstreams_UpstreamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMoqRelay_Config_Upstreams_UpstreamsRef {
        DataMoqRelay_Config_Upstreams_UpstreamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMoqRelay_Config_Upstreams_UpstreamsRef {
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
pub struct DataMoqRelay_Config_Upstreams {}

impl DataMoqRelay_Config_Upstreams { }

impl terrars::ToListMappable for DataMoqRelay_Config_Upstreams {
    type O = terrars::BlockAssignable<DataMoqRelay_Config_Upstreams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMoqRelay_Config_Upstreams {}

impl BuildDataMoqRelay_Config_Upstreams {
    pub fn build(self) -> DataMoqRelay_Config_Upstreams {
        DataMoqRelay_Config_Upstreams {}
    }
}

#[allow(dead_code)]
pub struct DataMoqRelay_Config_UpstreamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMoqRelay_Config_UpstreamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMoqRelay_Config_UpstreamsRef {
        DataMoqRelay_Config_UpstreamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMoqRelay_Config_UpstreamsRef {
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
    pub fn upstreams(&self) -> terrars::ListRef<DataMoqRelay_Config_Upstreams_UpstreamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstreams", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMoqRelay_Config {}

impl DataMoqRelay_Config { }

impl terrars::ToListMappable for DataMoqRelay_Config {
    type O = terrars::BlockAssignable<DataMoqRelay_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMoqRelay_Config {}

impl BuildDataMoqRelay_Config {
    pub fn build(self) -> DataMoqRelay_Config {
        DataMoqRelay_Config {}
    }
}

#[allow(dead_code)]
pub struct DataMoqRelay_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMoqRelay_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMoqRelay_ConfigRef {
        DataMoqRelay_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMoqRelay_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lingering_subscribe` after provisioning.\n"]
    pub fn lingering_subscribe(&self) -> DataMoqRelay_Config_LingeringSubscribeRef {
        DataMoqRelay_Config_LingeringSubscribeRef::new(
            self.shared().clone(),
            format!("{}.lingering_subscribe", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `upstreams` after provisioning.\nUpstreams are external MOQT server publishers that a relay falls back\nto when it has no local publisher for a requested namespace/track."]
    pub fn upstreams(&self) -> DataMoqRelay_Config_UpstreamsRef {
        DataMoqRelay_Config_UpstreamsRef::new(self.shared().clone(), format!("{}.upstreams", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMoqRelay_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asc: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_page: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMoqRelay_Filter {
    #[doc =
        "Set the field `asc`.\nSort order by `created`. When true, results are returned oldest-first\n(ascending); otherwise newest-first (descending, the default)."]
    pub fn set_asc(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.asc = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `created_after`.\nCursor for pagination. Returns relays created strictly after this\nRFC 3339 timestamp (typically the `created` value of the last item\non the current page, to fetch the next page)."]
    pub fn set_created_after(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_after = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `created_before`.\nCursor for pagination. Returns relays created strictly before this\nRFC 3339 timestamp (typically the `created` value of the first item\non the current page, to fetch the previous page)."]
    pub fn set_created_before(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_before = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `per_page`.\nMaximum number of relays to return per page. Values above the maximum are\nclamped to it rather than rejected."]
    pub fn set_per_page(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.per_page = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMoqRelay_Filter {
    type O = terrars::BlockAssignable<DataMoqRelay_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMoqRelay_Filter {}

impl BuildDataMoqRelay_Filter {
    pub fn build(self) -> DataMoqRelay_Filter {
        DataMoqRelay_Filter {
            asc: core::default::Default::default(),
            created_after: core::default::Default::default(),
            created_before: core::default::Default::default(),
            per_page: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMoqRelay_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMoqRelay_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMoqRelay_FilterRef {
        DataMoqRelay_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMoqRelay_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asc` after provisioning.\nSort order by `created`. When true, results are returned oldest-first\n(ascending); otherwise newest-first (descending, the default)."]
    pub fn asc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asc", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_after` after provisioning.\nCursor for pagination. Returns relays created strictly after this\nRFC 3339 timestamp (typically the `created` value of the last item\non the current page, to fetch the next page)."]
    pub fn created_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_before` after provisioning.\nCursor for pagination. Returns relays created strictly before this\nRFC 3339 timestamp (typically the `created` value of the first item\non the current page, to fetch the previous page)."]
    pub fn created_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.base))
    }

    #[doc =
        "Get a reference to the value of field `per_page` after provisioning.\nMaximum number of relays to return per page. Values above the maximum are\nclamped to it rather than rejected."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.base))
    }
}
