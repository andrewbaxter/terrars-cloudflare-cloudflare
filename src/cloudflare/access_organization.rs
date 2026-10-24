use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccessOrganizationData {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    auth_domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_redirect_to_identity: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_ui_read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ui_read_only_toggle_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_seat_expiration_inactive_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_design: std::option::Option<std::vec::Vec<AccessOrganizationLoginDesignEl>>,
    dynamic: AccessOrganizationDynamic,
}

struct AccessOrganization_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessOrganizationData>,
}

#[derive(Clone)]
pub struct AccessOrganization(std::rc::Rc<AccessOrganization_>);

impl AccessOrganization {
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

    #[doc = "Set the field `account_id`.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_redirect_to_identity`.\nWhen set to true, users skip the identity provider selection step during login."]
    pub fn set_auto_redirect_to_identity(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_redirect_to_identity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_ui_read_only`.\nWhen set to true, this will disable all editing of Access resources via the Zero Trust Dashboard."]
    pub fn set_is_ui_read_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_ui_read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of your Zero Trust organization."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ui_read_only_toggle_reason`.\nA description of the reason why the UI read only field is being toggled."]
    pub fn set_ui_read_only_toggle_reason(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().ui_read_only_toggle_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_seat_expiration_inactive_time`.\nThe amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`."]
    pub fn set_user_seat_expiration_inactive_time(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().user_seat_expiration_inactive_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_design`.\n"]
    pub fn set_login_design(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessOrganizationLoginDesignEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().login_design = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.login_design = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_domain` after provisioning.\nThe unique subdomain assigned to your Zero Trust organization."]
    pub fn auth_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_redirect_to_identity` after provisioning.\nWhen set to true, users skip the identity provider selection step during login."]
    pub fn auto_redirect_to_identity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_redirect_to_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_ui_read_only` after provisioning.\nWhen set to true, this will disable all editing of Access resources via the Zero Trust Dashboard."]
    pub fn is_ui_read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ui_read_only", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of your Zero Trust organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ui_read_only_toggle_reason` after provisioning.\nA description of the reason why the UI read only field is being toggled."]
    pub fn ui_read_only_toggle_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ui_read_only_toggle_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_seat_expiration_inactive_time` after provisioning.\nThe amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`."]
    pub fn user_seat_expiration_inactive_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_seat_expiration_inactive_time", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `login_design` after provisioning.\n"]
    pub fn login_design(&self) -> terrars::ListRef<AccessOrganizationLoginDesignElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.login_design", self.extract_ref()))
    }
}

impl terrars::Referable for AccessOrganization {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessOrganization { }

impl terrars::ToListMappable for AccessOrganization {
    type O = terrars::ListRef<AccessOrganizationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessOrganization_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_access_organization".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessOrganization {
    pub tf_id: std::string::String,
    #[doc = "The unique subdomain assigned to your Zero Trust organization."]
    pub auth_domain: terrars::PrimField<std::string::String>,
}

impl BuildAccessOrganization {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessOrganization {
        let out = AccessOrganization(std::rc::Rc::new(AccessOrganization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccessOrganizationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                auth_domain: self.auth_domain,
                auto_redirect_to_identity: core::default::Default::default(),
                id: core::default::Default::default(),
                is_ui_read_only: core::default::Default::default(),
                name: core::default::Default::default(),
                ui_read_only_toggle_reason: core::default::Default::default(),
                user_seat_expiration_inactive_time: core::default::Default::default(),
                zone_id: core::default::Default::default(),
                login_design: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessOrganizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessOrganizationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_domain` after provisioning.\nThe unique subdomain assigned to your Zero Trust organization."]
    pub fn auth_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_redirect_to_identity` after provisioning.\nWhen set to true, users skip the identity provider selection step during login."]
    pub fn auto_redirect_to_identity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_redirect_to_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_ui_read_only` after provisioning.\nWhen set to true, this will disable all editing of Access resources via the Zero Trust Dashboard."]
    pub fn is_ui_read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ui_read_only", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of your Zero Trust organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ui_read_only_toggle_reason` after provisioning.\nA description of the reason why the UI read only field is being toggled."]
    pub fn ui_read_only_toggle_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ui_read_only_toggle_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_seat_expiration_inactive_time` after provisioning.\nThe amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`."]
    pub fn user_seat_expiration_inactive_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_seat_expiration_inactive_time", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `login_design` after provisioning.\n"]
    pub fn login_design(&self) -> terrars::ListRef<AccessOrganizationLoginDesignElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.login_design", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccessOrganizationLoginDesignEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    background_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    footer_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logo_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_color: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessOrganizationLoginDesignEl {
    #[doc = "Set the field `background_color`.\nThe background color on the login page."]
    pub fn set_background_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.background_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `footer_text`.\nThe text at the bottom of the login page."]
    pub fn set_footer_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.footer_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_text`.\nThe text at the top of the login page."]
    pub fn set_header_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logo_path`.\nThe URL of the logo on the login page."]
    pub fn set_logo_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logo_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_color`.\nThe text color on the login page."]
    pub fn set_text_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text_color = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessOrganizationLoginDesignEl {
    type O = terrars::BlockAssignable<AccessOrganizationLoginDesignEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessOrganizationLoginDesignEl {}

impl BuildAccessOrganizationLoginDesignEl {
    pub fn build(self) -> AccessOrganizationLoginDesignEl {
        AccessOrganizationLoginDesignEl {
            background_color: core::default::Default::default(),
            footer_text: core::default::Default::default(),
            header_text: core::default::Default::default(),
            logo_path: core::default::Default::default(),
            text_color: core::default::Default::default(),
        }
    }
}

pub struct AccessOrganizationLoginDesignElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessOrganizationLoginDesignElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessOrganizationLoginDesignElRef {
        AccessOrganizationLoginDesignElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessOrganizationLoginDesignElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `background_color` after provisioning.\nThe background color on the login page."]
    pub fn background_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.background_color", self.base))
    }

    #[doc =
        "Get a reference to the value of field `footer_text` after provisioning.\nThe text at the bottom of the login page."]
    pub fn footer_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.footer_text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_text` after provisioning.\nThe text at the top of the login page."]
    pub fn header_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logo_path` after provisioning.\nThe URL of the logo on the login page."]
    pub fn logo_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo_path", self.base))
    }

    #[doc = "Get a reference to the value of field `text_color` after provisioning.\nThe text color on the login page."]
    pub fn text_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_color", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessOrganizationDynamic {
    login_design: std::option::Option<terrars::DynamicBlock<AccessOrganizationLoginDesignEl>>,
}
