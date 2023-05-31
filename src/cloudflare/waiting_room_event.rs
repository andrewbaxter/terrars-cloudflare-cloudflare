use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WaitingRoomEventData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_page_html: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_session_renewal: Option<PrimField<bool>>,
    event_end_time: PrimField<String>,
    event_start_time: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_users_per_minute: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prequeue_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queueing_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shuffle_at_event_start: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspended: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_active_users: Option<PrimField<f64>>,
    waiting_room_id: PrimField<String>,
    zone_id: PrimField<String>,
}

struct WaitingRoomEvent_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WaitingRoomEventData>,
}

#[derive(Clone)]
pub struct WaitingRoomEvent(Rc<WaitingRoomEvent_>);

impl WaitingRoomEvent {
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

    #[doc= "Set the field `custom_page_html`.\nThis is a templated html file that will be rendered at the edge."]
    pub fn set_custom_page_html(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_page_html = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA description to let users add more details about the event."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_session_renewal`.\nDisables automatic renewal of session cookies."]
    pub fn set_disable_session_renewal(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_session_renewal = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `new_users_per_minute`.\nThe number of new users that will be let into the route every minute."]
    pub fn set_new_users_per_minute(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().new_users_per_minute = Some(v.into());
        self
    }

    #[doc= "Set the field `prequeue_start_time`.\nISO 8601 timestamp that marks when to begin queueing all users before the event starts. Must occur at least 5 minutes before `event_start_time`."]
    pub fn set_prequeue_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().prequeue_start_time = Some(v.into());
        self
    }

    #[doc= "Set the field `queueing_method`.\nThe queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`."]
    pub fn set_queueing_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().queueing_method = Some(v.into());
        self
    }

    #[doc= "Set the field `session_duration`.\nLifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin."]
    pub fn set_session_duration(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().session_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `shuffle_at_event_start`.\nUsers in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. Defaults to `false`."]
    pub fn set_shuffle_at_event_start(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().shuffle_at_event_start = Some(v.into());
        self
    }

    #[doc= "Set the field `suspended`.\nIf suspended, the event is ignored and traffic will be handled based on the waiting room configuration."]
    pub fn set_suspended(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = Some(v.into());
        self
    }

    #[doc= "Set the field `total_active_users`.\nThe total number of active user sessions on the route at a point in time."]
    pub fn set_total_active_users(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().total_active_users = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nCreation time."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_page_html` after provisioning.\nThis is a templated html file that will be rendered at the edge."]
    pub fn custom_page_html(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_page_html", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description to let users add more details about the event."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_session_renewal` after provisioning.\nDisables automatic renewal of session cookies."]
    pub fn disable_session_renewal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_session_renewal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_end_time` after provisioning.\nISO 8601 timestamp that marks the end of the event. **Modifying this attribute will force creation of a new resource.**"]
    pub fn event_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_end_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_start_time` after provisioning.\nISO 8601 timestamp that marks the start of the event. Must occur at least 1 minute before `event_end_time`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn event_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_start_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nLast modified time."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA unique name to identify the event. Only alphanumeric characters, hyphens, and underscores are allowed. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_users_per_minute` after provisioning.\nThe number of new users that will be let into the route every minute."]
    pub fn new_users_per_minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_users_per_minute", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prequeue_start_time` after provisioning.\nISO 8601 timestamp that marks when to begin queueing all users before the event starts. Must occur at least 5 minutes before `event_start_time`."]
    pub fn prequeue_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prequeue_start_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `queueing_method` after provisioning.\nThe queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`."]
    pub fn queueing_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queueing_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_duration` after provisioning.\nLifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin."]
    pub fn session_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shuffle_at_event_start` after provisioning.\nUsers in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. Defaults to `false`."]
    pub fn shuffle_at_event_start(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.shuffle_at_event_start", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suspended` after provisioning.\nIf suspended, the event is ignored and traffic will be handled based on the waiting room configuration."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_active_users` after provisioning.\nThe total number of active user sessions on the route at a point in time."]
    pub fn total_active_users(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_active_users", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `waiting_room_id` after provisioning.\nThe Waiting Room ID the event should apply to. **Modifying this attribute will force creation of a new resource.**"]
    pub fn waiting_room_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.waiting_room_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for WaitingRoomEvent {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WaitingRoomEvent { }

impl ToListMappable for WaitingRoomEvent {
    type O = ListRef<WaitingRoomEventRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WaitingRoomEvent_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_waiting_room_event".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWaitingRoomEvent {
    pub tf_id: String,
    #[doc= "ISO 8601 timestamp that marks the end of the event. **Modifying this attribute will force creation of a new resource.**"]
    pub event_end_time: PrimField<String>,
    #[doc= "ISO 8601 timestamp that marks the start of the event. Must occur at least 1 minute before `event_end_time`. **Modifying this attribute will force creation of a new resource.**"]
    pub event_start_time: PrimField<String>,
    #[doc= "A unique name to identify the event. Only alphanumeric characters, hyphens, and underscores are allowed. **Modifying this attribute will force creation of a new resource.**"]
    pub name: PrimField<String>,
    #[doc= "The Waiting Room ID the event should apply to. **Modifying this attribute will force creation of a new resource.**"]
    pub waiting_room_id: PrimField<String>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildWaitingRoomEvent {
    pub fn build(self, stack: &mut Stack) -> WaitingRoomEvent {
        let out = WaitingRoomEvent(Rc::new(WaitingRoomEvent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WaitingRoomEventData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                custom_page_html: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_session_renewal: core::default::Default::default(),
                event_end_time: self.event_end_time,
                event_start_time: self.event_start_time,
                id: core::default::Default::default(),
                name: self.name,
                new_users_per_minute: core::default::Default::default(),
                prequeue_start_time: core::default::Default::default(),
                queueing_method: core::default::Default::default(),
                session_duration: core::default::Default::default(),
                shuffle_at_event_start: core::default::Default::default(),
                suspended: core::default::Default::default(),
                total_active_users: core::default::Default::default(),
                waiting_room_id: self.waiting_room_id,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WaitingRoomEventRef {
    shared: StackShared,
    base: String,
}

impl Ref for WaitingRoomEventRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WaitingRoomEventRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nCreation time."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_page_html` after provisioning.\nThis is a templated html file that will be rendered at the edge."]
    pub fn custom_page_html(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_page_html", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description to let users add more details about the event."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_session_renewal` after provisioning.\nDisables automatic renewal of session cookies."]
    pub fn disable_session_renewal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_session_renewal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_end_time` after provisioning.\nISO 8601 timestamp that marks the end of the event. **Modifying this attribute will force creation of a new resource.**"]
    pub fn event_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_end_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_start_time` after provisioning.\nISO 8601 timestamp that marks the start of the event. Must occur at least 1 minute before `event_end_time`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn event_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_start_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nLast modified time."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA unique name to identify the event. Only alphanumeric characters, hyphens, and underscores are allowed. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_users_per_minute` after provisioning.\nThe number of new users that will be let into the route every minute."]
    pub fn new_users_per_minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_users_per_minute", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prequeue_start_time` after provisioning.\nISO 8601 timestamp that marks when to begin queueing all users before the event starts. Must occur at least 5 minutes before `event_start_time`."]
    pub fn prequeue_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prequeue_start_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `queueing_method` after provisioning.\nThe queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`."]
    pub fn queueing_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queueing_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_duration` after provisioning.\nLifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin."]
    pub fn session_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shuffle_at_event_start` after provisioning.\nUsers in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. Defaults to `false`."]
    pub fn shuffle_at_event_start(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.shuffle_at_event_start", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suspended` after provisioning.\nIf suspended, the event is ignored and traffic will be handled based on the waiting room configuration."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_active_users` after provisioning.\nThe total number of active user sessions on the route at a point in time."]
    pub fn total_active_users(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_active_users", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `waiting_room_id` after provisioning.\nThe Waiting Room ID the event should apply to. **Modifying this attribute will force creation of a new resource.**"]
    pub fn waiting_room_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.waiting_room_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
