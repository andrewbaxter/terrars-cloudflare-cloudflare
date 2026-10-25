#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ObservatoryScheduledTestData_ {
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
    frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct ObservatoryScheduledTest_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ObservatoryScheduledTestData_>,
}

#[derive(Clone)]
pub struct ObservatoryScheduledTest(std::rc::Rc<ObservatoryScheduledTest_>);

impl ObservatoryScheduledTest {
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
        "Set the field `frequency`.\nThe frequency of the scheduled test. Defaults to WEEKLY for free plans, DAILY for paid plans.\nAvailable values: \"DAILY\", \"WEEKLY\"."]
    pub fn set_frequency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nA test region.\nAvailable values: \"asia-east1\", \"asia-northeast1\", \"asia-northeast2\", \"asia-south1\", \"asia-southeast1\", \"australia-southeast1\", \"europe-north1\", \"europe-southwest1\", \"europe-west1\", \"europe-west2\", \"europe-west3\", \"europe-west4\", \"europe-west8\", \"europe-west9\", \"me-west1\", \"southamerica-east1\", \"us-central1\", \"us-east1\", \"us-east4\", \"us-south1\", \"us-west1\"."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nThe frequency of the scheduled test. Defaults to WEEKLY for free plans, DAILY for paid plans.\nAvailable values: \"DAILY\", \"WEEKLY\"."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nA URL."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA test region.\nAvailable values: \"asia-east1\", \"asia-northeast1\", \"asia-northeast2\", \"asia-south1\", \"asia-southeast1\", \"australia-southeast1\", \"europe-north1\", \"europe-southwest1\", \"europe-west1\", \"europe-west2\", \"europe-west3\", \"europe-west4\", \"europe-west8\", \"europe-west9\", \"me-west1\", \"southamerica-east1\", \"us-central1\", \"us-east1\", \"us-east4\", \"us-south1\", \"us-west1\"."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nThe test schedule."]
    pub fn schedule(&self) -> ObservatoryScheduledTest_ScheduleRef {
        ObservatoryScheduledTest_ScheduleRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> ObservatoryScheduledTest_TestRef {
        ObservatoryScheduledTest_TestRef::new(self.shared().clone(), format!("{}.test", self.extract_ref()))
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

impl terrars::Referable for ObservatoryScheduledTest {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ObservatoryScheduledTest { }

impl terrars::ToListMappable for ObservatoryScheduledTest {
    type O = terrars::ListRef<ObservatoryScheduledTestRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ObservatoryScheduledTest_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_observatory_scheduled_test".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildObservatoryScheduledTest {
    pub tf_id: std::string::String,
    #[doc = "A URL."]
    pub url: terrars::PrimField<std::string::String>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildObservatoryScheduledTest {
    pub fn build(self, stack: &mut terrars::Stack) -> ObservatoryScheduledTest {
        let out = ObservatoryScheduledTest(std::rc::Rc::new(ObservatoryScheduledTest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ObservatoryScheduledTestData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                frequency: core::default::Default::default(),
                region: core::default::Default::default(),
                url: self.url,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ObservatoryScheduledTestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservatoryScheduledTestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ObservatoryScheduledTestRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nThe frequency of the scheduled test. Defaults to WEEKLY for free plans, DAILY for paid plans.\nAvailable values: \"DAILY\", \"WEEKLY\"."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nA URL."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA test region.\nAvailable values: \"asia-east1\", \"asia-northeast1\", \"asia-northeast2\", \"asia-south1\", \"asia-southeast1\", \"australia-southeast1\", \"europe-north1\", \"europe-southwest1\", \"europe-west1\", \"europe-west2\", \"europe-west3\", \"europe-west4\", \"europe-west8\", \"europe-west9\", \"me-west1\", \"southamerica-east1\", \"us-central1\", \"us-east1\", \"us-east4\", \"us-south1\", \"us-west1\"."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nThe test schedule."]
    pub fn schedule(&self) -> ObservatoryScheduledTest_ScheduleRef {
        ObservatoryScheduledTest_ScheduleRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> ObservatoryScheduledTest_TestRef {
        ObservatoryScheduledTest_TestRef::new(self.shared().clone(), format!("{}.test", self.extract_ref()))
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

#[derive(Serialize)]
pub struct ObservatoryScheduledTest_Schedule {}

impl ObservatoryScheduledTest_Schedule { }

impl terrars::ToListMappable for ObservatoryScheduledTest_Schedule {
    type O = terrars::BlockAssignable<ObservatoryScheduledTest_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservatoryScheduledTest_Schedule {}

impl BuildObservatoryScheduledTest_Schedule {
    pub fn build(self) -> ObservatoryScheduledTest_Schedule {
        ObservatoryScheduledTest_Schedule {}
    }
}

#[allow(dead_code)]
pub struct ObservatoryScheduledTest_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservatoryScheduledTest_ScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ObservatoryScheduledTest_ScheduleRef {
        ObservatoryScheduledTest_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservatoryScheduledTest_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nThe frequency of the test.\nAvailable values: \"DAILY\", \"WEEKLY\"."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA test region.\nAvailable values: \"asia-east1\", \"asia-northeast1\", \"asia-northeast2\", \"asia-south1\", \"asia-southeast1\", \"australia-southeast1\", \"europe-north1\", \"europe-southwest1\", \"europe-west1\", \"europe-west2\", \"europe-west3\", \"europe-west4\", \"europe-west8\", \"europe-west9\", \"me-west1\", \"southamerica-east1\", \"us-central1\", \"us-east1\", \"us-east4\", \"us-south1\", \"us-west1\"."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nA URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservatoryScheduledTest_Test_DesktopReport_Error {}

impl ObservatoryScheduledTest_Test_DesktopReport_Error { }

impl terrars::ToListMappable for ObservatoryScheduledTest_Test_DesktopReport_Error {
    type O = terrars::BlockAssignable<ObservatoryScheduledTest_Test_DesktopReport_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservatoryScheduledTest_Test_DesktopReport_Error {}

impl BuildObservatoryScheduledTest_Test_DesktopReport_Error {
    pub fn build(self) -> ObservatoryScheduledTest_Test_DesktopReport_Error {
        ObservatoryScheduledTest_Test_DesktopReport_Error {}
    }
}

#[allow(dead_code)]
pub struct ObservatoryScheduledTest_Test_DesktopReport_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservatoryScheduledTest_Test_DesktopReport_ErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservatoryScheduledTest_Test_DesktopReport_ErrorRef {
        ObservatoryScheduledTest_Test_DesktopReport_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservatoryScheduledTest_Test_DesktopReport_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nThe error code of the Lighthouse result.\nAvailable values: \"NOT_REACHABLE\", \"DNS_FAILURE\", \"NOT_HTML\", \"LIGHTHOUSE_TIMEOUT\", \"UNKNOWN\"."]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `detail` after provisioning.\nDetailed error message."]
    pub fn detail(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detail", self.base))
    }

    #[doc =
        "Get a reference to the value of field `final_displayed_url` after provisioning.\nThe final URL displayed to the user."]
    pub fn final_displayed_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_displayed_url", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservatoryScheduledTest_Test_DesktopReport {}

impl ObservatoryScheduledTest_Test_DesktopReport { }

impl terrars::ToListMappable for ObservatoryScheduledTest_Test_DesktopReport {
    type O = terrars::BlockAssignable<ObservatoryScheduledTest_Test_DesktopReport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservatoryScheduledTest_Test_DesktopReport {}

impl BuildObservatoryScheduledTest_Test_DesktopReport {
    pub fn build(self) -> ObservatoryScheduledTest_Test_DesktopReport {
        ObservatoryScheduledTest_Test_DesktopReport {}
    }
}

#[allow(dead_code)]
pub struct ObservatoryScheduledTest_Test_DesktopReportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservatoryScheduledTest_Test_DesktopReportRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservatoryScheduledTest_Test_DesktopReportRef {
        ObservatoryScheduledTest_Test_DesktopReportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservatoryScheduledTest_Test_DesktopReportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cls` after provisioning.\nCumulative Layout Shift."]
    pub fn cls(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `device_type` after provisioning.\nThe type of device.\nAvailable values: \"DESKTOP\", \"MOBILE\"."]
    pub fn device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> ObservatoryScheduledTest_Test_DesktopReport_ErrorRef {
        ObservatoryScheduledTest_Test_DesktopReport_ErrorRef::new(
            self.shared().clone(),
            format!("{}.error", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `fcp` after provisioning.\nFirst Contentful Paint."]
    pub fn fcp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fcp", self.base))
    }

    #[doc =
        "Get a reference to the value of field `json_report_url` after provisioning.\nThe URL to the full Lighthouse JSON report."]
    pub fn json_report_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_report_url", self.base))
    }

    #[doc = "Get a reference to the value of field `lcp` after provisioning.\nLargest Contentful Paint."]
    pub fn lcp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lcp", self.base))
    }

    #[doc =
        "Get a reference to the value of field `performance_score` after provisioning.\nThe Lighthouse performance score."]
    pub fn performance_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance_score", self.base))
    }

    #[doc = "Get a reference to the value of field `si` after provisioning.\nSpeed Index."]
    pub fn si(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.si", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the Lighthouse report.\nAvailable values: \"RUNNING\", \"COMPLETE\", \"FAILED\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `tbt` after provisioning.\nTotal Blocking Time."]
    pub fn tbt(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tbt", self.base))
    }

    #[doc = "Get a reference to the value of field `ttfb` after provisioning.\nTime To First Byte."]
    pub fn ttfb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttfb", self.base))
    }

    #[doc = "Get a reference to the value of field `tti` after provisioning.\nTime To Interactive."]
    pub fn tti(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tti", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservatoryScheduledTest_Test_MobileReport_Error {}

impl ObservatoryScheduledTest_Test_MobileReport_Error { }

impl terrars::ToListMappable for ObservatoryScheduledTest_Test_MobileReport_Error {
    type O = terrars::BlockAssignable<ObservatoryScheduledTest_Test_MobileReport_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservatoryScheduledTest_Test_MobileReport_Error {}

impl BuildObservatoryScheduledTest_Test_MobileReport_Error {
    pub fn build(self) -> ObservatoryScheduledTest_Test_MobileReport_Error {
        ObservatoryScheduledTest_Test_MobileReport_Error {}
    }
}

#[allow(dead_code)]
pub struct ObservatoryScheduledTest_Test_MobileReport_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservatoryScheduledTest_Test_MobileReport_ErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservatoryScheduledTest_Test_MobileReport_ErrorRef {
        ObservatoryScheduledTest_Test_MobileReport_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservatoryScheduledTest_Test_MobileReport_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nThe error code of the Lighthouse result.\nAvailable values: \"NOT_REACHABLE\", \"DNS_FAILURE\", \"NOT_HTML\", \"LIGHTHOUSE_TIMEOUT\", \"UNKNOWN\"."]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `detail` after provisioning.\nDetailed error message."]
    pub fn detail(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detail", self.base))
    }

    #[doc =
        "Get a reference to the value of field `final_displayed_url` after provisioning.\nThe final URL displayed to the user."]
    pub fn final_displayed_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_displayed_url", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservatoryScheduledTest_Test_MobileReport {}

impl ObservatoryScheduledTest_Test_MobileReport { }

impl terrars::ToListMappable for ObservatoryScheduledTest_Test_MobileReport {
    type O = terrars::BlockAssignable<ObservatoryScheduledTest_Test_MobileReport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservatoryScheduledTest_Test_MobileReport {}

impl BuildObservatoryScheduledTest_Test_MobileReport {
    pub fn build(self) -> ObservatoryScheduledTest_Test_MobileReport {
        ObservatoryScheduledTest_Test_MobileReport {}
    }
}

#[allow(dead_code)]
pub struct ObservatoryScheduledTest_Test_MobileReportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservatoryScheduledTest_Test_MobileReportRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ObservatoryScheduledTest_Test_MobileReportRef {
        ObservatoryScheduledTest_Test_MobileReportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservatoryScheduledTest_Test_MobileReportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cls` after provisioning.\nCumulative Layout Shift."]
    pub fn cls(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `device_type` after provisioning.\nThe type of device.\nAvailable values: \"DESKTOP\", \"MOBILE\"."]
    pub fn device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> ObservatoryScheduledTest_Test_MobileReport_ErrorRef {
        ObservatoryScheduledTest_Test_MobileReport_ErrorRef::new(
            self.shared().clone(),
            format!("{}.error", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `fcp` after provisioning.\nFirst Contentful Paint."]
    pub fn fcp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fcp", self.base))
    }

    #[doc =
        "Get a reference to the value of field `json_report_url` after provisioning.\nThe URL to the full Lighthouse JSON report."]
    pub fn json_report_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_report_url", self.base))
    }

    #[doc = "Get a reference to the value of field `lcp` after provisioning.\nLargest Contentful Paint."]
    pub fn lcp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lcp", self.base))
    }

    #[doc =
        "Get a reference to the value of field `performance_score` after provisioning.\nThe Lighthouse performance score."]
    pub fn performance_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance_score", self.base))
    }

    #[doc = "Get a reference to the value of field `si` after provisioning.\nSpeed Index."]
    pub fn si(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.si", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the Lighthouse report.\nAvailable values: \"RUNNING\", \"COMPLETE\", \"FAILED\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `tbt` after provisioning.\nTotal Blocking Time."]
    pub fn tbt(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tbt", self.base))
    }

    #[doc = "Get a reference to the value of field `ttfb` after provisioning.\nTime To First Byte."]
    pub fn ttfb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttfb", self.base))
    }

    #[doc = "Get a reference to the value of field `tti` after provisioning.\nTime To Interactive."]
    pub fn tti(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tti", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservatoryScheduledTest_Test_Region {}

impl ObservatoryScheduledTest_Test_Region { }

impl terrars::ToListMappable for ObservatoryScheduledTest_Test_Region {
    type O = terrars::BlockAssignable<ObservatoryScheduledTest_Test_Region>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservatoryScheduledTest_Test_Region {}

impl BuildObservatoryScheduledTest_Test_Region {
    pub fn build(self) -> ObservatoryScheduledTest_Test_Region {
        ObservatoryScheduledTest_Test_Region {}
    }
}

#[allow(dead_code)]
pub struct ObservatoryScheduledTest_Test_RegionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservatoryScheduledTest_Test_RegionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ObservatoryScheduledTest_Test_RegionRef {
        ObservatoryScheduledTest_Test_RegionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservatoryScheduledTest_Test_RegionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA test region.\nAvailable values: \"asia-east1\", \"asia-northeast1\", \"asia-northeast2\", \"asia-south1\", \"asia-southeast1\", \"australia-southeast1\", \"europe-north1\", \"europe-southwest1\", \"europe-west1\", \"europe-west2\", \"europe-west3\", \"europe-west4\", \"europe-west8\", \"europe-west9\", \"me-west1\", \"southamerica-east1\", \"us-central1\", \"us-east1\", \"us-east4\", \"us-south1\", \"us-west1\"."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservatoryScheduledTest_Test {}

impl ObservatoryScheduledTest_Test { }

impl terrars::ToListMappable for ObservatoryScheduledTest_Test {
    type O = terrars::BlockAssignable<ObservatoryScheduledTest_Test>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservatoryScheduledTest_Test {}

impl BuildObservatoryScheduledTest_Test {
    pub fn build(self) -> ObservatoryScheduledTest_Test {
        ObservatoryScheduledTest_Test {}
    }
}

#[allow(dead_code)]
pub struct ObservatoryScheduledTest_TestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservatoryScheduledTest_TestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ObservatoryScheduledTest_TestRef {
        ObservatoryScheduledTest_TestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservatoryScheduledTest_TestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `date` after provisioning.\n"]
    pub fn date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date", self.base))
    }

    #[doc = "Get a reference to the value of field `desktop_report` after provisioning.\nThe Lighthouse report."]
    pub fn desktop_report(&self) -> ObservatoryScheduledTest_Test_DesktopReportRef {
        ObservatoryScheduledTest_Test_DesktopReportRef::new(
            self.shared().clone(),
            format!("{}.desktop_report", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `mobile_report` after provisioning.\nThe Lighthouse report."]
    pub fn mobile_report(&self) -> ObservatoryScheduledTest_Test_MobileReportRef {
        ObservatoryScheduledTest_Test_MobileReportRef::new(
            self.shared().clone(),
            format!("{}.mobile_report", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nA test region with a label."]
    pub fn region(&self) -> ObservatoryScheduledTest_Test_RegionRef {
        ObservatoryScheduledTest_Test_RegionRef::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedule_frequency` after provisioning.\nThe frequency of the test.\nAvailable values: \"DAILY\", \"WEEKLY\"."]
    pub fn schedule_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nA URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
