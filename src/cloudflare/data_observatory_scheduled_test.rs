#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataObservatoryScheduledTestData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataObservatoryScheduledTest_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataObservatoryScheduledTestData_>,
}

#[derive(Clone)]
pub struct DataObservatoryScheduledTest(std::rc::Rc<DataObservatoryScheduledTest_>);

impl DataObservatoryScheduledTest {
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
        "Set the field `region`.\nA test region.\nAvailable values: \"asia-east1\", \"asia-northeast1\", \"asia-northeast2\", \"asia-south1\", \"asia-southeast1\", \"australia-southeast1\", \"europe-north1\", \"europe-southwest1\", \"europe-west1\", \"europe-west2\", \"europe-west3\", \"europe-west4\", \"europe-west8\", \"europe-west9\", \"me-west1\", \"southamerica-east1\", \"us-central1\", \"us-east1\", \"us-east4\", \"us-south1\", \"us-west1\"."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nThe frequency of the test.\nAvailable values: \"DAILY\", \"WEEKLY\"."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA test region.\nAvailable values: \"asia-east1\", \"asia-northeast1\", \"asia-northeast2\", \"asia-south1\", \"asia-southeast1\", \"australia-southeast1\", \"europe-north1\", \"europe-southwest1\", \"europe-west1\", \"europe-west2\", \"europe-west3\", \"europe-west4\", \"europe-west8\", \"europe-west9\", \"me-west1\", \"southamerica-east1\", \"us-central1\", \"us-east1\", \"us-east4\", \"us-south1\", \"us-west1\"."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nA URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataObservatoryScheduledTest {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataObservatoryScheduledTest { }

impl terrars::ToListMappable for DataObservatoryScheduledTest {
    type O = terrars::ListRef<DataObservatoryScheduledTestRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataObservatoryScheduledTest_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_observatory_scheduled_test".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataObservatoryScheduledTest {
    pub tf_id: std::string::String,
    #[doc = "A URL."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildDataObservatoryScheduledTest {
    pub fn build(self, stack: &mut terrars::Stack) -> DataObservatoryScheduledTest {
        let out = DataObservatoryScheduledTest(std::rc::Rc::new(DataObservatoryScheduledTest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataObservatoryScheduledTestData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                url: self.url,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataObservatoryScheduledTestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataObservatoryScheduledTestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataObservatoryScheduledTestRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nThe frequency of the test.\nAvailable values: \"DAILY\", \"WEEKLY\"."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA test region.\nAvailable values: \"asia-east1\", \"asia-northeast1\", \"asia-northeast2\", \"asia-south1\", \"asia-southeast1\", \"australia-southeast1\", \"europe-north1\", \"europe-southwest1\", \"europe-west1\", \"europe-west2\", \"europe-west3\", \"europe-west4\", \"europe-west8\", \"europe-west9\", \"me-west1\", \"southamerica-east1\", \"us-central1\", \"us-east1\", \"us-east4\", \"us-south1\", \"us-west1\"."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nA URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
