use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccessOrganizationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    auth_domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_redirect_to_identity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_ui_read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_read_only_toggle_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_seat_expiration_inactive_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_design: Option<Vec<AccessOrganizationLoginDesignEl>>,
    dynamic: AccessOrganizationDynamic,
}

struct AccessOrganization_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessOrganizationData>,
}

#[derive(Clone)]
pub struct AccessOrganization(Rc<AccessOrganization_>);

impl AccessOrganization {
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

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_redirect_to_identity`.\nWhen set to true, users skip the identity provider selection step during login."]
    pub fn set_auto_redirect_to_identity(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_redirect_to_identity = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `is_ui_read_only`.\nWhen set to true, this will disable all editing of Access resources via the Zero Trust Dashboard."]
    pub fn set_is_ui_read_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_ui_read_only = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nThe name of your Zero Trust organization."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `ui_read_only_toggle_reason`.\nA description of the reason why the UI read only field is being toggled."]
    pub fn set_ui_read_only_toggle_reason(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ui_read_only_toggle_reason = Some(v.into());
        self
    }

    #[doc= "Set the field `user_seat_expiration_inactive_time`.\nThe amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`."]
    pub fn set_user_seat_expiration_inactive_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_seat_expiration_inactive_time = Some(v.into());
        self
    }

    #[doc= "Set the field `zone_id`.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn set_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `login_design`.\n"]
    pub fn set_login_design(self, v: impl Into<BlockAssignable<AccessOrganizationLoginDesignEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().login_design = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.login_design = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auth_domain` after provisioning.\nThe unique subdomain assigned to your Zero Trust organization."]
    pub fn auth_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_redirect_to_identity` after provisioning.\nWhen set to true, users skip the identity provider selection step during login."]
    pub fn auto_redirect_to_identity(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_redirect_to_identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_ui_read_only` after provisioning.\nWhen set to true, this will disable all editing of Access resources via the Zero Trust Dashboard."]
    pub fn is_ui_read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_ui_read_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of your Zero Trust organization."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ui_read_only_toggle_reason` after provisioning.\nA description of the reason why the UI read only field is being toggled."]
    pub fn ui_read_only_toggle_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ui_read_only_toggle_reason", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_seat_expiration_inactive_time` after provisioning.\nThe amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`."]
    pub fn user_seat_expiration_inactive_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_seat_expiration_inactive_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `login_design` after provisioning.\n"]
    pub fn login_design(&self) -> ListRef<AccessOrganizationLoginDesignElRef> {
        ListRef::new(self.shared().clone(), format!("{}.login_design", self.extract_ref()))
    }
}

impl Referable for AccessOrganization {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AccessOrganization { }

impl ToListMappable for AccessOrganization {
    type O = ListRef<AccessOrganizationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AccessOrganization_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_access_organization".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessOrganization {
    pub tf_id: String,
    #[doc= "The unique subdomain assigned to your Zero Trust organization."]
    pub auth_domain: PrimField<String>,
}

impl BuildAccessOrganization {
    pub fn build(self, stack: &mut Stack) -> AccessOrganization {
        let out = AccessOrganization(Rc::new(AccessOrganization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccessOrganizationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessOrganizationRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessOrganizationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessOrganizationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auth_domain` after provisioning.\nThe unique subdomain assigned to your Zero Trust organization."]
    pub fn auth_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_redirect_to_identity` after provisioning.\nWhen set to true, users skip the identity provider selection step during login."]
    pub fn auto_redirect_to_identity(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_redirect_to_identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_ui_read_only` after provisioning.\nWhen set to true, this will disable all editing of Access resources via the Zero Trust Dashboard."]
    pub fn is_ui_read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_ui_read_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of your Zero Trust organization."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ui_read_only_toggle_reason` after provisioning.\nA description of the reason why the UI read only field is being toggled."]
    pub fn ui_read_only_toggle_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ui_read_only_toggle_reason", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_seat_expiration_inactive_time` after provisioning.\nThe amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`."]
    pub fn user_seat_expiration_inactive_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_seat_expiration_inactive_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `login_design` after provisioning.\n"]
    pub fn login_design(&self) -> ListRef<AccessOrganizationLoginDesignElRef> {
        ListRef::new(self.shared().clone(), format!("{}.login_design", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccessOrganizationLoginDesignEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logo_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_color: Option<PrimField<String>>,
}

impl AccessOrganizationLoginDesignEl {
    #[doc= "Set the field `background_color`.\nThe background color on the login page."]
    pub fn set_background_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.background_color = Some(v.into());
        self
    }

    #[doc= "Set the field `footer_text`.\nThe text at the bottom of the login page."]
    pub fn set_footer_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.footer_text = Some(v.into());
        self
    }

    #[doc= "Set the field `header_text`.\nThe text at the top of the login page."]
    pub fn set_header_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_text = Some(v.into());
        self
    }

    #[doc= "Set the field `logo_path`.\nThe URL of the logo on the login page."]
    pub fn set_logo_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logo_path = Some(v.into());
        self
    }

    #[doc= "Set the field `text_color`.\nThe text color on the login page."]
    pub fn set_text_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text_color = Some(v.into());
        self
    }
}

impl ToListMappable for AccessOrganizationLoginDesignEl {
    type O = BlockAssignable<AccessOrganizationLoginDesignEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessOrganizationLoginDesignElRef {
    fn new(shared: StackShared, base: String) -> AccessOrganizationLoginDesignElRef {
        AccessOrganizationLoginDesignElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessOrganizationLoginDesignElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `background_color` after provisioning.\nThe background color on the login page."]
    pub fn background_color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.background_color", self.base))
    }

    #[doc= "Get a reference to the value of field `footer_text` after provisioning.\nThe text at the bottom of the login page."]
    pub fn footer_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.footer_text", self.base))
    }

    #[doc= "Get a reference to the value of field `header_text` after provisioning.\nThe text at the top of the login page."]
    pub fn header_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_text", self.base))
    }

    #[doc= "Get a reference to the value of field `logo_path` after provisioning.\nThe URL of the logo on the login page."]
    pub fn logo_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logo_path", self.base))
    }

    #[doc= "Get a reference to the value of field `text_color` after provisioning.\nThe text color on the login page."]
    pub fn text_color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_color", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessOrganizationDynamic {
    login_design: Option<DynamicBlock<AccessOrganizationLoginDesignEl>>,
}
