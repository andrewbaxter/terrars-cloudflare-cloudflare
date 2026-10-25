#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustCasbIntegrationData_ {
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
    filter: std::option::Option<DataZeroTrustCasbIntegration_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustCasbIntegration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustCasbIntegrationData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustCasbIntegration(std::rc::Rc<DataZeroTrustCasbIntegration_>);

impl DataZeroTrustCasbIntegration {
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
    pub fn set_filter(self, v: impl std::convert::Into<DataZeroTrustCasbIntegration_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nIntegration ID to look up. Exactly one of `id` or `filter` must be configured."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application` after provisioning.\n"]
    pub fn application(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_method` after provisioning.\nThe integration's authentication method."]
    pub fn auth_method(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.auth_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorization_link` after provisioning.\nAuthorization link for the integration."]
    pub fn authorization_link(&self) -> DataZeroTrustCasbIntegration_AuthorizationLinkRef {
        DataZeroTrustCasbIntegration_AuthorizationLinkRef::new(
            self.shared().clone(),
            format!("{}.authorization_link", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\nWhen the integration was created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials_expiry` after provisioning.\nCredentials expiry time."]
    pub fn credentials_expiry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_expiry", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dlp_profiles` after provisioning.\nDLP Profiles enabled for the integration."]
    pub fn dlp_profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dlp_profiles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustCasbIntegration_FilterRef {
        DataZeroTrustCasbIntegration_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_details` after provisioning.\nHealth details with remediation hints."]
    pub fn health_details(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIntegration ID to look up. Exactly one of `id` or `filter` must be configured."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_paused` after provisioning.\nWhether the user paused the integration."]
    pub fn is_paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_paused", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_hydrated` after provisioning.\nLast time the integration was hydrated."]
    pub fn last_hydrated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_hydrated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the integration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nIntegration status."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nWhen the integration was last updated."]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `use_cases` after provisioning.\nUse cases enabled for the integration."]
    pub fn use_cases(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.use_cases", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustCasbIntegration {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustCasbIntegration { }

impl terrars::ToListMappable for DataZeroTrustCasbIntegration {
    type O = terrars::ListRef<DataZeroTrustCasbIntegrationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustCasbIntegration_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_casb_integration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustCasbIntegration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustCasbIntegration {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustCasbIntegration {
        let out = DataZeroTrustCasbIntegration(std::rc::Rc::new(DataZeroTrustCasbIntegration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustCasbIntegrationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustCasbIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbIntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustCasbIntegrationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application` after provisioning.\n"]
    pub fn application(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_method` after provisioning.\nThe integration's authentication method."]
    pub fn auth_method(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.auth_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorization_link` after provisioning.\nAuthorization link for the integration."]
    pub fn authorization_link(&self) -> DataZeroTrustCasbIntegration_AuthorizationLinkRef {
        DataZeroTrustCasbIntegration_AuthorizationLinkRef::new(
            self.shared().clone(),
            format!("{}.authorization_link", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\nWhen the integration was created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials_expiry` after provisioning.\nCredentials expiry time."]
    pub fn credentials_expiry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_expiry", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dlp_profiles` after provisioning.\nDLP Profiles enabled for the integration."]
    pub fn dlp_profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dlp_profiles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustCasbIntegration_FilterRef {
        DataZeroTrustCasbIntegration_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_details` after provisioning.\nHealth details with remediation hints."]
    pub fn health_details(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIntegration ID to look up. Exactly one of `id` or `filter` must be configured."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_paused` after provisioning.\nWhether the user paused the integration."]
    pub fn is_paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_paused", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_hydrated` after provisioning.\nLast time the integration was hydrated."]
    pub fn last_hydrated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_hydrated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the integration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nIntegration status."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nWhen the integration was last updated."]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `use_cases` after provisioning.\nUse cases enabled for the integration."]
    pub fn use_cases(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.use_cases", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustCasbIntegration_AuthorizationLink {}

impl DataZeroTrustCasbIntegration_AuthorizationLink { }

impl terrars::ToListMappable for DataZeroTrustCasbIntegration_AuthorizationLink {
    type O = terrars::BlockAssignable<DataZeroTrustCasbIntegration_AuthorizationLink>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustCasbIntegration_AuthorizationLink {}

impl BuildDataZeroTrustCasbIntegration_AuthorizationLink {
    pub fn build(self) -> DataZeroTrustCasbIntegration_AuthorizationLink {
        DataZeroTrustCasbIntegration_AuthorizationLink {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustCasbIntegration_AuthorizationLinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbIntegration_AuthorizationLinkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustCasbIntegration_AuthorizationLinkRef {
        DataZeroTrustCasbIntegration_AuthorizationLinkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustCasbIntegration_AuthorizationLinkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `components` after provisioning.\n"]
    pub fn components(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.components", self.base))
    }

    #[doc = "Get a reference to the value of field `link` after provisioning.\n"]
    pub fn link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.link", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustCasbIntegration_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dlp_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_cases: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustCasbIntegration_Filter {
    #[doc = "Set the field `application`.\nFilter by application/vendor (e.g., GOOGLE_WORKSPACE, MICROSOFT_INTERNAL)."]
    pub fn set_application(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dlp_enabled`.\nFilter by DLP enabled status (true/false)."]
    pub fn set_dlp_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dlp_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nField to order results by.\nAvailable values: \"application\", \"created\", \"name\", \"status\"."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `page`.\nPage number within the paginated result set."]
    pub fn set_page(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `page_size`.\nNumber of results per page."]
    pub fn set_page_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.page_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nSearch integrations by name or application."]
    pub fn set_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nFilter by integration status.\nAvailable values: \"Healthy\", \"Initializing\", \"Offline\", \"Unhealthy\"."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_cases`.\nFilter by one enabled use case (for example, casb or ces)."]
    pub fn set_use_cases(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.use_cases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustCasbIntegration_Filter {
    type O = terrars::BlockAssignable<DataZeroTrustCasbIntegration_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustCasbIntegration_Filter {}

impl BuildDataZeroTrustCasbIntegration_Filter {
    pub fn build(self) -> DataZeroTrustCasbIntegration_Filter {
        DataZeroTrustCasbIntegration_Filter {
            application: core::default::Default::default(),
            direction: core::default::Default::default(),
            dlp_enabled: core::default::Default::default(),
            order: core::default::Default::default(),
            page: core::default::Default::default(),
            page_size: core::default::Default::default(),
            search: core::default::Default::default(),
            status: core::default::Default::default(),
            use_cases: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustCasbIntegration_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbIntegration_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustCasbIntegration_FilterRef {
        DataZeroTrustCasbIntegration_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustCasbIntegration_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application` after provisioning.\nFilter by application/vendor (e.g., GOOGLE_WORKSPACE, MICROSOFT_INTERNAL)."]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.base))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dlp_enabled` after provisioning.\nFilter by DLP enabled status (true/false)."]
    pub fn dlp_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dlp_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order results by.\nAvailable values: \"application\", \"created\", \"name\", \"status\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc =
        "Get a reference to the value of field `page` after provisioning.\nPage number within the paginated result set."]
    pub fn page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.base))
    }

    #[doc = "Get a reference to the value of field `page_size` after provisioning.\nNumber of results per page."]
    pub fn page_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch integrations by name or application."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nFilter by integration status.\nAvailable values: \"Healthy\", \"Initializing\", \"Offline\", \"Unhealthy\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_cases` after provisioning.\nFilter by one enabled use case (for example, casb or ces)."]
    pub fn use_cases(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_cases", self.base))
    }
}
