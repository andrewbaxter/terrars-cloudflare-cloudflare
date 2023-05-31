use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WaitingRoomData {
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
    default_template_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_session_renewal: Option<PrimField<bool>>,
    host: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_response_enabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    new_users_per_minute: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queueing_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspended: Option<PrimField<bool>>,
    total_active_users: PrimField<f64>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<WaitingRoomTimeoutsEl>,
}

struct WaitingRoom_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WaitingRoomData>,
}

#[derive(Clone)]
pub struct WaitingRoom(Rc<WaitingRoom_>);

impl WaitingRoom {
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

    #[doc= "Set the field `default_template_language`.\nThe language to use for the default waiting room page. Available values: `de-DE`, `es-ES`, `en-US`, `fr-FR`, `id-ID`, `it-IT`, `ja-JP`, `ko-KR`, `nl-NL`, `pl-PL`, `pt-BR`, `tr-TR`, `zh-CN`, `zh-TW`, `ru-RU`, `fa-IR`. Defaults to `en-US`."]
    pub fn set_default_template_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_template_language = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA description to add more details about the waiting room."]
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

    #[doc= "Set the field `json_response_enabled`.\nIf true, requests to the waiting room with the header `Accept: application/json` will receive a JSON response object."]
    pub fn set_json_response_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().json_response_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\nThe path within the host to enable the waiting room on. Defaults to `/`."]
    pub fn set_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().path = Some(v.into());
        self
    }

    #[doc= "Set the field `queue_all`.\nIf queue_all is true, then all traffic will be sent to the waiting room."]
    pub fn set_queue_all(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().queue_all = Some(v.into());
        self
    }

    #[doc= "Set the field `queueing_method`.\nThe queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`. Defaults to `fifo`."]
    pub fn set_queueing_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().queueing_method = Some(v.into());
        self
    }

    #[doc= "Set the field `session_duration`.\nLifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin. Defaults to `5`."]
    pub fn set_session_duration(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().session_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `suspended`.\nSuspends the waiting room."]
    pub fn set_suspended(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<WaitingRoomTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `custom_page_html` after provisioning.\nThis is a templated html file that will be rendered at the edge."]
    pub fn custom_page_html(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_page_html", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_template_language` after provisioning.\nThe language to use for the default waiting room page. Available values: `de-DE`, `es-ES`, `en-US`, `fr-FR`, `id-ID`, `it-IT`, `ja-JP`, `ko-KR`, `nl-NL`, `pl-PL`, `pt-BR`, `tr-TR`, `zh-CN`, `zh-TW`, `ru-RU`, `fa-IR`. Defaults to `en-US`."]
    pub fn default_template_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_template_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description to add more details about the waiting room."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_session_renewal` after provisioning.\nDisables automatic renewal of session cookies."]
    pub fn disable_session_renewal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_session_renewal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\nHost name for which the waiting room will be applied (no wildcards)."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `json_response_enabled` after provisioning.\nIf true, requests to the waiting room with the header `Accept: application/json` will receive a JSON response object."]
    pub fn json_response_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_response_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA unique name to identify the waiting room. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_users_per_minute` after provisioning.\nThe number of new users that will be let into the route every minute."]
    pub fn new_users_per_minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_users_per_minute", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path within the host to enable the waiting room on. Defaults to `/`."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `queue_all` after provisioning.\nIf queue_all is true, then all traffic will be sent to the waiting room."]
    pub fn queue_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `queueing_method` after provisioning.\nThe queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`. Defaults to `fifo`."]
    pub fn queueing_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queueing_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_duration` after provisioning.\nLifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin. Defaults to `5`."]
    pub fn session_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suspended` after provisioning.\nSuspends the waiting room."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_active_users` after provisioning.\nThe total number of active user sessions on the route at a point in time."]
    pub fn total_active_users(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_active_users", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WaitingRoomTimeoutsElRef {
        WaitingRoomTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for WaitingRoom {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WaitingRoom { }

impl ToListMappable for WaitingRoom {
    type O = ListRef<WaitingRoomRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WaitingRoom_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_waiting_room".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWaitingRoom {
    pub tf_id: String,
    #[doc= "Host name for which the waiting room will be applied (no wildcards)."]
    pub host: PrimField<String>,
    #[doc= "A unique name to identify the waiting room. **Modifying this attribute will force creation of a new resource.**"]
    pub name: PrimField<String>,
    #[doc= "The number of new users that will be let into the route every minute."]
    pub new_users_per_minute: PrimField<f64>,
    #[doc= "The total number of active user sessions on the route at a point in time."]
    pub total_active_users: PrimField<f64>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildWaitingRoom {
    pub fn build(self, stack: &mut Stack) -> WaitingRoom {
        let out = WaitingRoom(Rc::new(WaitingRoom_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WaitingRoomData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                custom_page_html: core::default::Default::default(),
                default_template_language: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_session_renewal: core::default::Default::default(),
                host: self.host,
                id: core::default::Default::default(),
                json_response_enabled: core::default::Default::default(),
                name: self.name,
                new_users_per_minute: self.new_users_per_minute,
                path: core::default::Default::default(),
                queue_all: core::default::Default::default(),
                queueing_method: core::default::Default::default(),
                session_duration: core::default::Default::default(),
                suspended: core::default::Default::default(),
                total_active_users: self.total_active_users,
                zone_id: self.zone_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WaitingRoomRef {
    shared: StackShared,
    base: String,
}

impl Ref for WaitingRoomRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WaitingRoomRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `custom_page_html` after provisioning.\nThis is a templated html file that will be rendered at the edge."]
    pub fn custom_page_html(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_page_html", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_template_language` after provisioning.\nThe language to use for the default waiting room page. Available values: `de-DE`, `es-ES`, `en-US`, `fr-FR`, `id-ID`, `it-IT`, `ja-JP`, `ko-KR`, `nl-NL`, `pl-PL`, `pt-BR`, `tr-TR`, `zh-CN`, `zh-TW`, `ru-RU`, `fa-IR`. Defaults to `en-US`."]
    pub fn default_template_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_template_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description to add more details about the waiting room."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_session_renewal` after provisioning.\nDisables automatic renewal of session cookies."]
    pub fn disable_session_renewal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_session_renewal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\nHost name for which the waiting room will be applied (no wildcards)."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `json_response_enabled` after provisioning.\nIf true, requests to the waiting room with the header `Accept: application/json` will receive a JSON response object."]
    pub fn json_response_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_response_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA unique name to identify the waiting room. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_users_per_minute` after provisioning.\nThe number of new users that will be let into the route every minute."]
    pub fn new_users_per_minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_users_per_minute", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path within the host to enable the waiting room on. Defaults to `/`."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `queue_all` after provisioning.\nIf queue_all is true, then all traffic will be sent to the waiting room."]
    pub fn queue_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `queueing_method` after provisioning.\nThe queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`. Defaults to `fifo`."]
    pub fn queueing_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queueing_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_duration` after provisioning.\nLifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin. Defaults to `5`."]
    pub fn session_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suspended` after provisioning.\nSuspends the waiting room."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_active_users` after provisioning.\nThe total number of active user sessions on the route at a point in time."]
    pub fn total_active_users(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_active_users", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WaitingRoomTimeoutsElRef {
        WaitingRoomTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WaitingRoomTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl WaitingRoomTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for WaitingRoomTimeoutsEl {
    type O = BlockAssignable<WaitingRoomTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWaitingRoomTimeoutsEl {}

impl BuildWaitingRoomTimeoutsEl {
    pub fn build(self) -> WaitingRoomTimeoutsEl {
        WaitingRoomTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct WaitingRoomTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WaitingRoomTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> WaitingRoomTimeoutsElRef {
        WaitingRoomTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WaitingRoomTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
