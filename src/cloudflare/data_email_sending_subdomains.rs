#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataEmailSendingSubdomainsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct DataEmailSendingSubdomains_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmailSendingSubdomainsData_>,
}

#[derive(Clone)]
pub struct DataEmailSendingSubdomains(std::rc::Rc<DataEmailSendingSubdomains_>);

impl DataEmailSendingSubdomains {
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

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataEmailSendingSubdomains_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataEmailSendingSubdomains {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmailSendingSubdomains { }

impl terrars::ToListMappable for DataEmailSendingSubdomains {
    type O = terrars::ListRef<DataEmailSendingSubdomainsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmailSendingSubdomains_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_email_sending_subdomains".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmailSendingSubdomains {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildDataEmailSendingSubdomains {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmailSendingSubdomains {
        let out = DataEmailSendingSubdomains(std::rc::Rc::new(DataEmailSendingSubdomains_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEmailSendingSubdomainsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                max_items: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmailSendingSubdomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSendingSubdomainsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmailSendingSubdomainsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataEmailSendingSubdomains_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmailSendingSubdomains_Result {}

impl DataEmailSendingSubdomains_Result { }

impl terrars::ToListMappable for DataEmailSendingSubdomains_Result {
    type O = terrars::BlockAssignable<DataEmailSendingSubdomains_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailSendingSubdomains_Result {}

impl BuildDataEmailSendingSubdomains_Result {
    pub fn build(self) -> DataEmailSendingSubdomains_Result {
        DataEmailSendingSubdomains_Result {}
    }
}

#[allow(dead_code)]
pub struct DataEmailSendingSubdomains_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSendingSubdomains_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailSendingSubdomains_ResultRef {
        DataEmailSendingSubdomains_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailSendingSubdomains_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nThe date and time the destination address has been created."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dkim_selector` after provisioning.\nThe DKIM selector used for email signing. Wildcard rows publish the selector and sign with `d=<base>`."]
    pub fn dkim_selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dkim_selector", self.base))
    }

    #[doc =
        "Get a reference to the value of field `drop_suppressed_recipients` after provisioning.\nWhether a send request that includes a recipient suppressed on\nthis subdomain drops that recipient and still delivers to the\nrest, instead of failing the entire request."]
    pub fn drop_suppressed_recipients(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drop_suppressed_recipients", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether Email Sending is enabled on this subdomain."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSending subdomain identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified` after provisioning.\nThe date and time the destination address was last modified."]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe exact domain name or a leftmost wildcard such as `*.example.com`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview_enabled` after provisioning.\nWhether sent messages from this subdomain can be previewed in the activity log."]
    pub fn preview_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `return_path_domain` after provisioning.\nThe return-path domain used for bounce handling. Wildcard rows use `cf-bounce.<base>`."]
    pub fn return_path_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_path_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\nSending subdomain identifier."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
