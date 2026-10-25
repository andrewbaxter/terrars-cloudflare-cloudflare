#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataWaitingRoomEventData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    event_id: terrars::PrimField<std::string::String>,
    waiting_room_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataWaitingRoomEvent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWaitingRoomEventData_>,
}

#[derive(Clone)]
pub struct DataWaitingRoomEvent(std::rc::Rc<DataWaitingRoomEvent_>);

impl DataWaitingRoomEvent {
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

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_page_html` after provisioning.\nIf set, the event will override the waiting room's `custom_page_html` property while it is active. If null, the event will inherit it."]
    pub fn custom_page_html(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_page_html", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA note that you can use to add more details about the event."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_session_renewal` after provisioning.\nIf set, the event will override the waiting room's `disable_session_renewal` property while it is active. If null, the event will inherit it."]
    pub fn disable_session_renewal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_session_renewal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_end_time` after provisioning.\nAn ISO 8601 timestamp that marks the end of the event."]
    pub fn event_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_id` after provisioning.\n"]
    pub fn event_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_start_time` after provisioning.\nAn ISO 8601 timestamp that marks the start of the event. At this time, queued users will be processed with the event's configuration. The start time must be at least one minute before `event_end_time`."]
    pub fn event_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique name to identify the event. Only alphanumeric characters, hyphens and underscores are allowed."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `new_users_per_minute` after provisioning.\nIf set, the event will override the waiting room's `new_users_per_minute` property while it is active. If null, the event will inherit it. This can only be set if the event's `total_active_users` property is also set."]
    pub fn new_users_per_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.new_users_per_minute", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `prequeue_start_time` after provisioning.\nAn ISO 8601 timestamp that marks when to begin queueing all users before the event starts. The prequeue must start at least five minutes before `event_start_time`."]
    pub fn prequeue_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prequeue_start_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `queueing_method` after provisioning.\nIf set, the event will override the waiting room's `queueing_method` property while it is active. If null, the event will inherit it."]
    pub fn queueing_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queueing_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nIf set, the event will override the waiting room's `session_duration` property while it is active. If null, the event will inherit it."]
    pub fn session_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shuffle_at_event_start` after provisioning.\nIf enabled, users in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. This is useful for situations when many users will join the event prequeue at the same time and you want to shuffle them to ensure fairness. Naturally, it makes the most sense to enable this feature when the `queueing_method` during the event respects ordering such as **fifo**, or else the shuffling may be unnecessary."]
    pub fn shuffle_at_event_start(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shuffle_at_event_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nSuspends or allows an event. If set to `true`, the event is ignored and traffic will be handled based on the waiting room configuration."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_active_users` after provisioning.\nIf set, the event will override the waiting room's `total_active_users` property while it is active. If null, the event will inherit it. This can only be set if the event's `new_users_per_minute` property is also set."]
    pub fn total_active_users(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_active_users", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `turnstile_action` after provisioning.\nIf set, the event will override the waiting room's `turnstile_action` property while it is active. If null, the event will inherit it.\nAvailable values: \"log\", \"infinite_queue\"."]
    pub fn turnstile_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.turnstile_action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `turnstile_mode` after provisioning.\nIf set, the event will override the waiting room's `turnstile_mode` property while it is active. If null, the event will inherit it.\nAvailable values: \"off\", \"invisible\", \"visible_non_interactive\", \"visible_managed\"."]
    pub fn turnstile_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.turnstile_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `waiting_room_id` after provisioning.\n"]
    pub fn waiting_room_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waiting_room_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataWaitingRoomEvent {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWaitingRoomEvent { }

impl terrars::ToListMappable for DataWaitingRoomEvent {
    type O = terrars::ListRef<DataWaitingRoomEventRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWaitingRoomEvent_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_waiting_room_event".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWaitingRoomEvent {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub event_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub waiting_room_id: terrars::PrimField<std::string::String>,
}

impl BuildDataWaitingRoomEvent {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWaitingRoomEvent {
        let out = DataWaitingRoomEvent(std::rc::Rc::new(DataWaitingRoomEvent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWaitingRoomEventData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                event_id: self.event_id,
                waiting_room_id: self.waiting_room_id,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWaitingRoomEventRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWaitingRoomEventRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWaitingRoomEventRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_page_html` after provisioning.\nIf set, the event will override the waiting room's `custom_page_html` property while it is active. If null, the event will inherit it."]
    pub fn custom_page_html(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_page_html", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA note that you can use to add more details about the event."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_session_renewal` after provisioning.\nIf set, the event will override the waiting room's `disable_session_renewal` property while it is active. If null, the event will inherit it."]
    pub fn disable_session_renewal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_session_renewal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_end_time` after provisioning.\nAn ISO 8601 timestamp that marks the end of the event."]
    pub fn event_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_id` after provisioning.\n"]
    pub fn event_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_start_time` after provisioning.\nAn ISO 8601 timestamp that marks the start of the event. At this time, queued users will be processed with the event's configuration. The start time must be at least one minute before `event_end_time`."]
    pub fn event_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique name to identify the event. Only alphanumeric characters, hyphens and underscores are allowed."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `new_users_per_minute` after provisioning.\nIf set, the event will override the waiting room's `new_users_per_minute` property while it is active. If null, the event will inherit it. This can only be set if the event's `total_active_users` property is also set."]
    pub fn new_users_per_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.new_users_per_minute", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `prequeue_start_time` after provisioning.\nAn ISO 8601 timestamp that marks when to begin queueing all users before the event starts. The prequeue must start at least five minutes before `event_start_time`."]
    pub fn prequeue_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prequeue_start_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `queueing_method` after provisioning.\nIf set, the event will override the waiting room's `queueing_method` property while it is active. If null, the event will inherit it."]
    pub fn queueing_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queueing_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nIf set, the event will override the waiting room's `session_duration` property while it is active. If null, the event will inherit it."]
    pub fn session_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shuffle_at_event_start` after provisioning.\nIf enabled, users in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. This is useful for situations when many users will join the event prequeue at the same time and you want to shuffle them to ensure fairness. Naturally, it makes the most sense to enable this feature when the `queueing_method` during the event respects ordering such as **fifo**, or else the shuffling may be unnecessary."]
    pub fn shuffle_at_event_start(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shuffle_at_event_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nSuspends or allows an event. If set to `true`, the event is ignored and traffic will be handled based on the waiting room configuration."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_active_users` after provisioning.\nIf set, the event will override the waiting room's `total_active_users` property while it is active. If null, the event will inherit it. This can only be set if the event's `new_users_per_minute` property is also set."]
    pub fn total_active_users(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_active_users", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `turnstile_action` after provisioning.\nIf set, the event will override the waiting room's `turnstile_action` property while it is active. If null, the event will inherit it.\nAvailable values: \"log\", \"infinite_queue\"."]
    pub fn turnstile_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.turnstile_action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `turnstile_mode` after provisioning.\nIf set, the event will override the waiting room's `turnstile_mode` property while it is active. If null, the event will inherit it.\nAvailable values: \"off\", \"invisible\", \"visible_non_interactive\", \"visible_managed\"."]
    pub fn turnstile_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.turnstile_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `waiting_room_id` after provisioning.\n"]
    pub fn waiting_room_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waiting_room_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
