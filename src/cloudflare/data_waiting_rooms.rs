#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataWaitingRoomsData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataWaitingRooms_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWaitingRoomsData_>,
}

#[derive(Clone)]
pub struct DataWaitingRooms(std::rc::Rc<DataWaitingRooms_>);

impl DataWaitingRooms {
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

    #[doc =
        "Set the field `account_id`.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataWaitingRooms_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataWaitingRooms {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWaitingRooms { }

impl terrars::ToListMappable for DataWaitingRooms {
    type O = terrars::ListRef<DataWaitingRoomsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWaitingRooms_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_waiting_rooms".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWaitingRooms {
    pub tf_id: std::string::String,
}

impl BuildDataWaitingRooms {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWaitingRooms {
        let out = DataWaitingRooms(std::rc::Rc::new(DataWaitingRooms_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWaitingRoomsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWaitingRoomsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWaitingRoomsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWaitingRoomsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataWaitingRooms_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWaitingRooms_Result_AdditionalRoutes {}

impl DataWaitingRooms_Result_AdditionalRoutes { }

impl terrars::ToListMappable for DataWaitingRooms_Result_AdditionalRoutes {
    type O = terrars::BlockAssignable<DataWaitingRooms_Result_AdditionalRoutes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWaitingRooms_Result_AdditionalRoutes {}

impl BuildDataWaitingRooms_Result_AdditionalRoutes {
    pub fn build(self) -> DataWaitingRooms_Result_AdditionalRoutes {
        DataWaitingRooms_Result_AdditionalRoutes {}
    }
}

#[allow(dead_code)]
pub struct DataWaitingRooms_Result_AdditionalRoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWaitingRooms_Result_AdditionalRoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWaitingRooms_Result_AdditionalRoutesRef {
        DataWaitingRooms_Result_AdditionalRoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWaitingRooms_Result_AdditionalRoutesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe hostname to which this waiting room will be applied (no wildcards). The hostname must be the primary domain, subdomain, or custom hostname (if using SSL for SaaS) of this zone. Please do not include the scheme (http:// or https://)."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nSets the path within the host to enable the waiting room on. The waiting room will be enabled for all subpaths as well. If there are two waiting rooms on the same subpath, the waiting room for the most specific path will be chosen. Wildcards and query parameters are not supported."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWaitingRooms_Result_CookieAttributes {}

impl DataWaitingRooms_Result_CookieAttributes { }

impl terrars::ToListMappable for DataWaitingRooms_Result_CookieAttributes {
    type O = terrars::BlockAssignable<DataWaitingRooms_Result_CookieAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWaitingRooms_Result_CookieAttributes {}

impl BuildDataWaitingRooms_Result_CookieAttributes {
    pub fn build(self) -> DataWaitingRooms_Result_CookieAttributes {
        DataWaitingRooms_Result_CookieAttributes {}
    }
}

#[allow(dead_code)]
pub struct DataWaitingRooms_Result_CookieAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWaitingRooms_Result_CookieAttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWaitingRooms_Result_CookieAttributesRef {
        DataWaitingRooms_Result_CookieAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWaitingRooms_Result_CookieAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `samesite` after provisioning.\nConfigures the SameSite attribute on the waiting room cookie. Value `auto` will be translated to `lax` or `none` depending if **Always Use HTTPS** is enabled. Note that when using value `none`, the secure attribute cannot be set to `never`.\nAvailable values: \"auto\", \"lax\", \"none\", \"strict\"."]
    pub fn samesite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.samesite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secure` after provisioning.\nConfigures the Secure attribute on the waiting room cookie. Value `always` indicates that the Secure attribute will be set in the Set-Cookie header, `never` indicates that the Secure attribute will not be set, and `auto` will set the Secure attribute depending if **Always Use HTTPS** is enabled.\nAvailable values: \"auto\", \"always\", \"never\"."]
    pub fn secure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWaitingRooms_Result {}

impl DataWaitingRooms_Result { }

impl terrars::ToListMappable for DataWaitingRooms_Result {
    type O = terrars::BlockAssignable<DataWaitingRooms_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWaitingRooms_Result {}

impl BuildDataWaitingRooms_Result {
    pub fn build(self) -> DataWaitingRooms_Result {
        DataWaitingRooms_Result {}
    }
}

#[allow(dead_code)]
pub struct DataWaitingRooms_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWaitingRooms_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWaitingRooms_ResultRef {
        DataWaitingRooms_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWaitingRooms_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_routes` after provisioning.\nOnly available for the Waiting Room Advanced subscription. Additional hostname and path combinations to which this waiting room will be applied. There is an implied wildcard at the end of the path. The hostname and path combination must be unique to this and all other waiting rooms."]
    pub fn additional_routes(&self) -> terrars::ListRef<DataWaitingRooms_Result_AdditionalRoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_routes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cookie_attributes` after provisioning.\nConfigures cookie attributes for the waiting room cookie. This encrypted cookie stores a user's status in the waiting room, such as queue position."]
    pub fn cookie_attributes(&self) -> DataWaitingRooms_Result_CookieAttributesRef {
        DataWaitingRooms_Result_CookieAttributesRef::new(
            self.shared().clone(),
            format!("{}.cookie_attributes", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `cookie_suffix` after provisioning.\nAppends a '_' + a custom suffix to the end of Cloudflare Waiting Room's cookie name(__cf_waitingroom). If `cookie_suffix` is \"abcd\", the cookie name will be `__cf_waitingroom_abcd`. This field is required if using `additional_routes`."]
    pub fn cookie_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_page_html` after provisioning.\nOnly available for the Waiting Room Advanced subscription. This is a template html file that will be rendered at the edge. If no custom_page_html is provided, the default waiting room will be used. The template is based on mustache ( https://mustache.github.io/ ). There are several variables that are evaluated by the Cloudflare edge:\n1. {{`waitTimeKnown`}} Acts like a boolean value that indicates the behavior to take when wait time is not available, for instance when queue_all is **true**.\n2. {{`waitTimeFormatted`}} Estimated wait time for the user. For example, five minutes. Alternatively, you can use:\n3. {{`waitTime`}} Number of minutes of estimated wait for a user.\n4. {{`waitTimeHours`}} Number of hours of estimated wait for a user (`Math.floor(waitTime/60)`).\n5. {{`waitTimeHourMinutes`}} Number of minutes above the `waitTimeHours` value (`waitTime%60`).\n6. {{`queueIsFull`}} Changes to **true** when no more people can be added to the queue.\n\nTo view the full list of variables, look at the `cfWaitingRoom` object described under the `json_response_enabled` property in other Waiting Room API calls."]
    pub fn custom_page_html(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_page_html", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_template_language` after provisioning.\nThe language of the default page template. If no default_template_language is provided, then `en-US` (English) will be used.\nAvailable values: \"en-US\", \"es-ES\", \"de-DE\", \"fr-FR\", \"it-IT\", \"ja-JP\", \"ko-KR\", \"pt-BR\", \"zh-CN\", \"zh-TW\", \"nl-NL\", \"pl-PL\", \"id-ID\", \"tr-TR\", \"ar-EG\", \"ru-RU\", \"fa-IR\", \"bg-BG\", \"hr-HR\", \"cs-CZ\", \"da-DK\", \"fi-FI\", \"lt-LT\", \"lv-LV\", \"ms-MY\", \"nb-NO\", \"ro-RO\", \"el-GR\", \"he-IL\", \"hi-IN\", \"hu-HU\", \"sr-BA\", \"sk-SK\", \"sl-SI\", \"sv-SE\", \"tl-PH\", \"th-TH\", \"uk-UA\", \"vi-VN\"."]
    pub fn default_template_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_template_language", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA note that you can use to add more details about the waiting room."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_session_renewal` after provisioning.\nOnly available for the Waiting Room Advanced subscription. Disables automatic renewal of session cookies. If `true`, an accepted user will have session_duration minutes to browse the site. After that, they will have to go through the waiting room again. If `false`, a user's session cookie will be automatically renewed on every request."]
    pub fn disable_session_renewal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_session_renewal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled_origin_commands` after provisioning.\nA list of enabled origin commands."]
    pub fn enabled_origin_commands(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_origin_commands", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe host name to which the waiting room will be applied (no wildcards). Please do not include the scheme (http:// or https://). The host and path combination must be unique."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `json_response_enabled` after provisioning.\nOnly available for the Waiting Room Advanced subscription. If `true`, requests to the waiting room with the header `Accept: application/json` will receive a JSON response object with information on the user's status in the waiting room as opposed to the configured static HTML page. This JSON response object has one property `cfWaitingRoom` which is an object containing the following fields:\n1. `inWaitingRoom`: Boolean indicating if the user is in the waiting room (always **true**).\n2. `waitTimeKnown`: Boolean indicating if the current estimated wait times are accurate. If **false**, they are not available.\n3. `waitTime`: Valid only when `waitTimeKnown` is **true**. Integer indicating the current estimated time in minutes the user will wait in the waiting room. When `queueingMethod` is **random**, this is set to `waitTime50Percentile`.\n4. `waitTime25Percentile`: Valid only when `queueingMethod` is **random** and `waitTimeKnown` is **true**. Integer indicating the current estimated maximum wait time for the 25% of users that gain entry the fastest (25th percentile).\n5. `waitTime50Percentile`: Valid only when `queueingMethod` is **random** and `waitTimeKnown` is **true**. Integer indicating the current estimated maximum wait time for the 50% of users that gain entry the fastest (50th percentile). In other words, half of the queued users are expected to let into the origin website before `waitTime50Percentile` and half are expected to be let in after it.\n6. `waitTime75Percentile`: Valid only when `queueingMethod` is **random** and `waitTimeKnown` is **true**. Integer indicating the current estimated maximum wait time for the 75% of users that gain entry the fastest (75th percentile).\n7. `waitTimeFormatted`: String displaying the `waitTime` formatted in English for users. If `waitTimeKnown` is **false**, `waitTimeFormatted` will display **unavailable**.\n8. `queueIsFull`: Boolean indicating if the waiting room's queue is currently full and not accepting new users at the moment.\n9. `queueAll`: Boolean indicating if all users will be queued in the waiting room and no one will be let into the origin website.\n10. `lastUpdated`: String displaying the timestamp as an ISO 8601 string of the user's last attempt to leave the waiting room and be let into the origin website. The user is able to make another attempt after `refreshIntervalSeconds` past this time. If the user makes a request too soon, it will be ignored and `lastUpdated` will not change.\n11. `refreshIntervalSeconds`: Integer indicating the number of seconds after `lastUpdated` until the user is able to make another attempt to leave the waiting room and be let into the origin website. When the `queueingMethod` is `reject`, there is no specified refresh time —\\_it will always be **zero**.\n12. `queueingMethod`: The queueing method currently used by the waiting room. It is either **fifo**, **random**, **passthrough**, or **reject**.\n13. `isFIFOQueue`: Boolean indicating if the waiting room uses a FIFO (First-In-First-Out) queue.\n14. `isRandomQueue`: Boolean indicating if the waiting room uses a Random queue where users gain access randomly.\n15. `isPassthroughQueue`: Boolean indicating if the waiting room uses a passthrough queue. Keep in mind that when passthrough is enabled, this JSON response will only exist when `queueAll` is **true** or `isEventPrequeueing` is **true** because in all other cases requests will go directly to the origin.\n16. `isRejectQueue`: Boolean indicating if the waiting room uses a reject queue.\n17. `isEventActive`: Boolean indicating if an event is currently occurring. Events are able to change a waiting room's behavior during a specified period of time. For additional information, look at the event properties `prequeue_start_time`, `event_start_time`, and `event_end_time` in the documentation for creating waiting room events. Events are considered active between these start and end times, as well as during the prequeueing period if it exists.\n18. `isEventPrequeueing`: Valid only when `isEventActive` is **true**. Boolean indicating if an event is currently prequeueing users before it starts.\n19. `timeUntilEventStart`: Valid only when `isEventPrequeueing` is **true**. Integer indicating the number of minutes until the event starts.\n20. `timeUntilEventStartFormatted`: String displaying the `timeUntilEventStart` formatted in English for users. If `isEventPrequeueing` is **false**, `timeUntilEventStartFormatted` will display **unavailable**.\n21. `timeUntilEventEnd`: Valid only when `isEventActive` is **true**. Integer indicating the number of minutes until the event ends.\n22. `timeUntilEventEndFormatted`: String displaying the `timeUntilEventEnd` formatted in English for users. If `isEventActive` is **false**, `timeUntilEventEndFormatted` will display **unavailable**.\n23. `shuffleAtEventStart`: Valid only when `isEventActive` is **true**. Boolean indicating if the users in the prequeue are shuffled randomly when the event starts.\n24. `turnstile`: Empty when turnstile isn't enabled. String displaying an html tag to display the Turnstile widget. Please add the `{{{turnstile}}}` tag to the `custom_html` template to ensure the Turnstile widget appears.\n25. `infiniteQueue`: Boolean indicating whether the response is for a user in the infinite queue.\n\nAn example cURL to a waiting room could be:\n\n\tcurl -X GET \"https://example.com/waitingroom\" \\\n\t\t-H \"Accept: application/json\"\n\nIf `json_response_enabled` is **true** and the request hits the waiting room, an example JSON response when `queueingMethod` is **fifo** and no event is active could be:\n\n\t{\n\t\t\"cfWaitingRoom\": {\n\t\t\t\"inWaitingRoom\": true,\n\t\t\t\"waitTimeKnown\": true,\n\t\t\t\"waitTime\": 10,\n\t\t\t\"waitTime25Percentile\": 0,\n\t\t\t\"waitTime50Percentile\": 0,\n\t\t\t\"waitTime75Percentile\": 0,\n\t\t\t\"waitTimeFormatted\": \"10 minutes\",\n\t\t\t\"queueIsFull\": false,\n\t\t\t\"queueAll\": false,\n\t\t\t\"lastUpdated\": \"2020-08-03T23:46:00.000Z\",\n\t\t\t\"refreshIntervalSeconds\": 20,\n\t\t\t\"queueingMethod\": \"fifo\",\n\t\t\t\"isFIFOQueue\": true,\n\t\t\t\"isRandomQueue\": false,\n\t\t\t\"isPassthroughQueue\": false,\n\t\t\t\"isRejectQueue\": false,\n\t\t\t\"isEventActive\": false,\n\t\t\t\"isEventPrequeueing\": false,\n\t\t\t\"timeUntilEventStart\": 0,\n\t\t\t\"timeUntilEventStartFormatted\": \"unavailable\",\n\t\t\t\"timeUntilEventEnd\": 0,\n\t\t\t\"timeUntilEventEndFormatted\": \"unavailable\",\n\t\t\t\"shuffleAtEventStart\": false\n\t\t}\n\t}\n\nIf `json_response_enabled` is **true** and the request hits the waiting room, an example JSON response when `queueingMethod` is **random** and an event is active could be:\n\n\t{\n\t\t\"cfWaitingRoom\": {\n\t\t\t\"inWaitingRoom\": true,\n\t\t\t\"waitTimeKnown\": true,\n\t\t\t\"waitTime\": 10,\n\t\t\t\"waitTime25Percentile\": 5,\n\t\t\t\"waitTime50Percentile\": 10,\n\t\t\t\"waitTime75Percentile\": 15,\n\t\t\t\"waitTimeFormatted\": \"5 minutes to 15 minutes\",\n\t\t\t\"queueIsFull\": false,\n\t\t\t\"queueAll\": false,\n\t\t\t\"lastUpdated\": \"2020-08-03T23:46:00.000Z\",\n\t\t\t\"refreshIntervalSeconds\": 20,\n\t\t\t\"queueingMethod\": \"random\",\n\t\t\t\"isFIFOQueue\": false,\n\t\t\t\"isRandomQueue\": true,\n\t\t\t\"isPassthroughQueue\": false,\n\t\t\t\"isRejectQueue\": false,\n\t\t\t\"isEventActive\": true,\n\t\t\t\"isEventPrequeueing\": false,\n\t\t\t\"timeUntilEventStart\": 0,\n\t\t\t\"timeUntilEventStartFormatted\": \"unavailable\",\n\t\t\t\"timeUntilEventEnd\": 15,\n\t\t\t\"timeUntilEventEndFormatted\": \"15 minutes\",\n\t\t\t\"shuffleAtEventStart\": true\n\t\t}\n\t}"]
    pub fn json_response_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_response_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique name to identify the waiting room. Only alphanumeric characters, hyphens and underscores are allowed."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `new_users_per_minute` after provisioning.\nSets the number of new users that will be let into the route every minute. This value is used as baseline for the number of users that are let in per minute. So it is possible that there is a little more or little less traffic coming to the route based on the traffic patterns at that time around the world."]
    pub fn new_users_per_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.new_users_per_minute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_event_prequeue_start_time` after provisioning.\nAn ISO 8601 timestamp that marks when the next event will begin queueing."]
    pub fn next_event_prequeue_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_event_prequeue_start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_event_start_time` after provisioning.\nAn ISO 8601 timestamp that marks when the next event will start."]
    pub fn next_event_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_event_start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nSets the path within the host to enable the waiting room on. The waiting room will be enabled for all subpaths as well. If there are two waiting rooms on the same subpath, the waiting room for the most specific path will be chosen. Wildcards and query parameters are not supported."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `queue_all` after provisioning.\nIf queue_all is `true`, all the traffic that is coming to a route will be sent to the waiting room. No new traffic can get to the route once this field is set and estimated time will become unavailable."]
    pub fn queue_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `queueing_method` after provisioning.\nSets the queueing method used by the waiting room. Changing this parameter from the **default** queueing method is only available for the Waiting Room Advanced subscription. Regardless of the queueing method, if `queue_all` is enabled or an event is prequeueing, users in the waiting room will not be accepted to the origin. These users will always see a waiting room page that refreshes automatically. The valid queueing methods are:\n1. `fifo` **(default)**: First-In-First-Out queue where customers gain access in the order they arrived.\n2. `random`: Random queue where customers gain access randomly, regardless of arrival time.\n3. `passthrough`: Users will pass directly through the waiting room and into the origin website. As a result, any configured limits will not be respected while this is enabled. This method can be used as an alternative to disabling a waiting room (with `suspended`) so that analytics are still reported. This can be used if you wish to allow all traffic normally, but want to restrict traffic during a waiting room event, or vice versa.\n4. `reject`: Users will be immediately rejected from the waiting room. As a result, no users will reach the origin website while this is enabled. This can be used if you wish to reject all traffic while performing maintenance, block traffic during a specified period of time (an event), or block traffic while events are not occurring. Consider a waiting room used for vaccine distribution that only allows traffic during sign-up events, and otherwise blocks all traffic. For this case, the waiting room uses `reject`, and its events override this with `fifo`, `random`, or `passthrough`. When this queueing method is enabled and neither `queueAll` is enabled nor an event is prequeueing, the waiting room page **will not refresh automatically**.\nAvailable values: \"fifo\", \"random\", \"passthrough\", \"reject\"."]
    pub fn queueing_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queueing_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `queueing_status_code` after provisioning.\nHTTP status code returned to a user while in the queue.\nAvailable values: 200, 202, 429."]
    pub fn queueing_status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queueing_status_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nLifetime of a cookie (in minutes) set by Cloudflare for users who get access to the route. If a user is not seen by Cloudflare again in that time period, they will be treated as a new user that visits the route."]
    pub fn session_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nSuspends or allows traffic going to the waiting room. If set to `true`, the traffic will not go to the waiting room."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_active_users` after provisioning.\nSets the total number of active user sessions on the route at a point in time. A route is a combination of host and path on which a waiting room is available. This value is used as a baseline for the total number of active user sessions on the route. It is possible to have a situation where there are more or less active users sessions on the route based on the traffic patterns at that time around the world."]
    pub fn total_active_users(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_active_users", self.base))
    }

    #[doc =
        "Get a reference to the value of field `turnstile_action` after provisioning.\nWhich action to take when a bot is detected using Turnstile. `log` will\nhave no impact on queueing behavior, simply keeping track of how many\nbots are detected in Waiting Room Analytics. `infinite_queue` will send\nbots to a false queueing state, where they will never reach your\norigin. `infinite_queue` requires Advanced Waiting Room.\nAvailable values: \"log\", \"infinite_queue\"."]
    pub fn turnstile_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.turnstile_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `turnstile_mode` after provisioning.\nWhich Turnstile widget type to use for detecting bot traffic. See\n[the Turnstile documentation](https://developers.cloudflare.com/turnstile/concepts/widget/#widget-types)\nfor the definitions of these widget types. Set to `off` to disable the\nTurnstile integration entirely. Setting this to anything other than\n`off` or `invisible` requires Advanced Waiting Room.\nAvailable values: \"off\", \"invisible\", \"visible_non_interactive\", \"visible_managed\"."]
    pub fn turnstile_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.turnstile_mode", self.base))
    }
}
