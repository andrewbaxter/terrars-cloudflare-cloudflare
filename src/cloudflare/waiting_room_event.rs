#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WaitingRoomEventData_ {
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
    custom_page_html: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_session_renewal: std::option::Option<terrars::PrimField<bool>>,
    event_end_time: terrars::PrimField<std::string::String>,
    event_start_time: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_users_per_minute: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prequeue_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queueing_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shuffle_at_event_start: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspended: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_active_users: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    turnstile_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    turnstile_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    waiting_room_id: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct WaitingRoomEvent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WaitingRoomEventData_>,
}

#[derive(Clone)]
pub struct WaitingRoomEvent(std::rc::Rc<WaitingRoomEvent_>);

impl WaitingRoomEvent {
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
        "Set the field `custom_page_html`.\nIf set, the event will override the waiting room's `custom_page_html` property while it is active. If null, the event will inherit it."]
    pub fn set_custom_page_html(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_page_html = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA note that you can use to add more details about the event."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_session_renewal`.\nIf set, the event will override the waiting room's `disable_session_renewal` property while it is active. If null, the event will inherit it."]
    pub fn set_disable_session_renewal(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_session_renewal = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `new_users_per_minute`.\nIf set, the event will override the waiting room's `new_users_per_minute` property while it is active. If null, the event will inherit it. This can only be set if the event's `total_active_users` property is also set."]
    pub fn set_new_users_per_minute(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().new_users_per_minute = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prequeue_start_time`.\nAn ISO 8601 timestamp that marks when to begin queueing all users before the event starts. The prequeue must start at least five minutes before `event_start_time`."]
    pub fn set_prequeue_start_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().prequeue_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `queueing_method`.\nIf set, the event will override the waiting room's `queueing_method` property while it is active. If null, the event will inherit it."]
    pub fn set_queueing_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().queueing_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_duration`.\nIf set, the event will override the waiting room's `session_duration` property while it is active. If null, the event will inherit it."]
    pub fn set_session_duration(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().session_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shuffle_at_event_start`.\nIf enabled, users in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. This is useful for situations when many users will join the event prequeue at the same time and you want to shuffle them to ensure fairness. Naturally, it makes the most sense to enable this feature when the `queueing_method` during the event respects ordering such as **fifo**, or else the shuffling may be unnecessary."]
    pub fn set_shuffle_at_event_start(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().shuffle_at_event_start = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suspended`.\nSuspends or allows an event. If set to `true`, the event is ignored and traffic will be handled based on the waiting room configuration."]
    pub fn set_suspended(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `total_active_users`.\nIf set, the event will override the waiting room's `total_active_users` property while it is active. If null, the event will inherit it. This can only be set if the event's `new_users_per_minute` property is also set."]
    pub fn set_total_active_users(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().total_active_users = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `turnstile_action`.\nIf set, the event will override the waiting room's `turnstile_action` property while it is active. If null, the event will inherit it.\nAvailable values: \"log\", \"infinite_queue\"."]
    pub fn set_turnstile_action(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().turnstile_action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `turnstile_mode`.\nIf set, the event will override the waiting room's `turnstile_mode` property while it is active. If null, the event will inherit it.\nAvailable values: \"off\", \"invisible\", \"visible_non_interactive\", \"visible_managed\"."]
    pub fn set_turnstile_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().turnstile_mode = std::option::Option::Some(v.into());
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

impl terrars::Referable for WaitingRoomEvent {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WaitingRoomEvent { }

impl terrars::ToListMappable for WaitingRoomEvent {
    type O = terrars::ListRef<WaitingRoomEventRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WaitingRoomEvent_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_waiting_room_event".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWaitingRoomEvent {
    pub tf_id: std::string::String,
    #[doc = "An ISO 8601 timestamp that marks the end of the event."]
    pub event_end_time: terrars::PrimField<std::string::String>,
    #[doc =
        "An ISO 8601 timestamp that marks the start of the event. At this time, queued users will be processed with the event's configuration. The start time must be at least one minute before `event_end_time`."]
    pub event_start_time: terrars::PrimField<std::string::String>,
    #[doc = "A unique name to identify the event. Only alphanumeric characters, hyphens and underscores are allowed."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub waiting_room_id: terrars::PrimField<std::string::String>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildWaitingRoomEvent {
    pub fn build(self, stack: &mut terrars::Stack) -> WaitingRoomEvent {
        let out = WaitingRoomEvent(std::rc::Rc::new(WaitingRoomEvent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WaitingRoomEventData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                custom_page_html: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_session_renewal: core::default::Default::default(),
                event_end_time: self.event_end_time,
                event_start_time: self.event_start_time,
                name: self.name,
                new_users_per_minute: core::default::Default::default(),
                prequeue_start_time: core::default::Default::default(),
                queueing_method: core::default::Default::default(),
                session_duration: core::default::Default::default(),
                shuffle_at_event_start: core::default::Default::default(),
                suspended: core::default::Default::default(),
                total_active_users: core::default::Default::default(),
                turnstile_action: core::default::Default::default(),
                turnstile_mode: core::default::Default::default(),
                waiting_room_id: self.waiting_room_id,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WaitingRoomEventRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WaitingRoomEventRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WaitingRoomEventRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
