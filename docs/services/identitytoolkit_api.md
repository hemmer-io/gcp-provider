# Identitytoolkit_api Service



**Resources**: 12

---

## Overview

The identitytoolkit_api service provides access to 12 resource types:

- [Identity_platform](#identity_platform) [C]
- [Oauth_idp_config](#oauth_idp_config) [CRUD]
- [Project](#project) [RU]
- [Identitytoolkit](#identitytoolkit) [R]
- [Mfa_enrollment](#mfa_enrollment) [C]
- [Mfa_sign_in](#mfa_sign_in) [C]
- [Default_supported_idp_config](#default_supported_idp_config) [CRUD]
- [Default_supported_idp](#default_supported_idp) [R]
- [Tenant](#tenant) [CRUD]
- [Account](#account) [C]
- [Inbound_saml_config](#inbound_saml_config) [CRUD]
- [Relyingparty](#relyingparty) [CR]

---

## Resources


### Identity_platform

Initialize Identity Platform for a Cloud project. Identity Platform is an end-to-end authentication system for third-party users to access your apps and services. These could include mobile/web apps, games, APIs and beyond. This is the publicly available variant of EnableIdentityPlatform that is only available to billing-enabled projects.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project` | String | ✅ | The resource name of the target project the developer wants to enable Identity Platform for. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create identity_platform
identity_platform = provider.identitytoolkit_api.Identity_platform {
    project = "value"  # The resource name of the target project the developer wants to enable Identity Platform for.
}

```

---


### Oauth_idp_config

Create an Oidc Idp configuration for an Identity Toolkit project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the OAuthIdpConfig resource, for example: 'projects/my-awesome-project/oauthIdpConfigs/oauth-config-id'. Ignored during create requests. |
| `response_type` | String |  | The response type to request for in the OAuth authorization flow. You can set either `id_token` or `code` to true, but not both. Setting both types to be simultaneously true (`{code: true, id_token: true}`) is not yet supported. |
| `client_secret` | String |  | The client secret of the OAuth client, to enable OIDC code flow. |
| `client_id` | String |  | The client id of an OAuth client. |
| `enabled` | bool |  | True if allows the user to sign in with the provider. |
| `issuer` | String |  | For OIDC Idps, the issuer identifier. |
| `display_name` | String |  | The config's display name set by developers. |
| `parent` | String | ✅ | The parent resource name where the config to be created, for example: "projects/my-awesome-project" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the OAuthIdpConfig resource, for example: 'projects/my-awesome-project/oauthIdpConfigs/oauth-config-id'. Ignored during create requests. |
| `response_type` | String | The response type to request for in the OAuth authorization flow. You can set either `id_token` or `code` to true, but not both. Setting both types to be simultaneously true (`{code: true, id_token: true}`) is not yet supported. |
| `client_secret` | String | The client secret of the OAuth client, to enable OIDC code flow. |
| `client_id` | String | The client id of an OAuth client. |
| `enabled` | bool | True if allows the user to sign in with the provider. |
| `issuer` | String | For OIDC Idps, the issuer identifier. |
| `display_name` | String | The config's display name set by developers. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create oauth_idp_config
oauth_idp_config = provider.identitytoolkit_api.Oauth_idp_config {
    parent = "value"  # The parent resource name where the config to be created, for example: "projects/my-awesome-project"
}

# Access oauth_idp_config outputs
oauth_idp_config_id = oauth_idp_config.id
oauth_idp_config_name = oauth_idp_config.name
oauth_idp_config_response_type = oauth_idp_config.response_type
oauth_idp_config_client_secret = oauth_idp_config.client_secret
oauth_idp_config_client_id = oauth_idp_config.client_id
oauth_idp_config_enabled = oauth_idp_config.enabled
oauth_idp_config_issuer = oauth_idp_config.issuer
oauth_idp_config_display_name = oauth_idp_config.display_name
```

---


### Project

Retrieve an Identity Toolkit project configuration.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of the Config resource. Example: "projects/my-awesome-project/config" |
| `mfa` | String |  | Configuration for this project's multi-factor authentication, including whether it is active and what factors can be used for the second factor |
| `default_hosting_site` | String |  | Output only. Default Firebase hosting site name |
| `recaptcha_config` | String |  | The project-level reCAPTCHA config. |
| `sign_in` | String |  | Configuration related to local sign in methods. |
| `subtype` | String |  | Output only. The subtype of this config. |
| `email_privacy_config` | String |  | Configuration for settings related to email privacy and public visibility. |
| `monitoring` | String |  | Configuration related to monitoring project activity. |
| `sms_region_config` | String |  | Configures which regions are enabled for SMS verification code sending. |
| `password_policy_config` | String |  | The project level password policy configuration. |
| `blocking_functions` | String |  | Configuration related to blocking functions. |
| `client` | String |  | Options related to how clients making requests on behalf of a project should be configured. |
| `autodelete_anonymous_users` | bool |  | Whether anonymous users will be auto-deleted after a period of 30 days. |
| `multi_tenant` | String |  | Configuration related to multi-tenant functionality. |
| `authorized_domains` | Vec<String> |  | List of domains authorized for OAuth redirects |
| `notification` | String |  | Configuration related to sending notifications to users. |
| `quota` | String |  | Configuration related to quotas. |
| `mobile_links_config` | String |  | Configuration for settings related to univeral links (iOS) and app links (Android). |
| `name` | String | ✅ | Output only. The name of the Config resource. Example: "projects/my-awesome-project/config" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The name of the Config resource. Example: "projects/my-awesome-project/config" |
| `mfa` | String | Configuration for this project's multi-factor authentication, including whether it is active and what factors can be used for the second factor |
| `default_hosting_site` | String | Output only. Default Firebase hosting site name |
| `recaptcha_config` | String | The project-level reCAPTCHA config. |
| `sign_in` | String | Configuration related to local sign in methods. |
| `subtype` | String | Output only. The subtype of this config. |
| `email_privacy_config` | String | Configuration for settings related to email privacy and public visibility. |
| `monitoring` | String | Configuration related to monitoring project activity. |
| `sms_region_config` | String | Configures which regions are enabled for SMS verification code sending. |
| `password_policy_config` | String | The project level password policy configuration. |
| `blocking_functions` | String | Configuration related to blocking functions. |
| `client` | String | Options related to how clients making requests on behalf of a project should be configured. |
| `autodelete_anonymous_users` | bool | Whether anonymous users will be auto-deleted after a period of 30 days. |
| `multi_tenant` | String | Configuration related to multi-tenant functionality. |
| `authorized_domains` | Vec<String> | List of domains authorized for OAuth redirects |
| `notification` | String | Configuration related to sending notifications to users. |
| `quota` | String | Configuration related to quotas. |
| `mobile_links_config` | String | Configuration for settings related to univeral links (iOS) and app links (Android). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project outputs
project_id = project.id
project_name = project.name
project_mfa = project.mfa
project_default_hosting_site = project.default_hosting_site
project_recaptcha_config = project.recaptcha_config
project_sign_in = project.sign_in
project_subtype = project.subtype
project_email_privacy_config = project.email_privacy_config
project_monitoring = project.monitoring
project_sms_region_config = project.sms_region_config
project_password_policy_config = project.password_policy_config
project_blocking_functions = project.blocking_functions
project_client = project.client
project_autodelete_anonymous_users = project.autodelete_anonymous_users
project_multi_tenant = project.multi_tenant
project_authorized_domains = project.authorized_domains
project_notification = project.notification
project_quota = project.quota
project_mobile_links_config = project.mobile_links_config
```

---


### Identitytoolkit

Gets password policy config set on the project or tenant.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `force_upgrade_on_signin` | bool | Users must have a password compliant with the password policy to sign-in. |
| `allowed_non_alphanumeric_characters` | Vec<String> | Output only. Allowed characters which satisfy the non_alphanumeric requirement. |
| `custom_strength_options` | String | The custom strength options enforced by the password policy. |
| `enforcement_state` | String | Output only. Which enforcement mode to use for the password policy. |
| `schema_version` | i64 | Output only. schema version number for the password policy |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access identitytoolkit outputs
identitytoolkit_id = identitytoolkit.id
identitytoolkit_force_upgrade_on_signin = identitytoolkit.force_upgrade_on_signin
identitytoolkit_allowed_non_alphanumeric_characters = identitytoolkit.allowed_non_alphanumeric_characters
identitytoolkit_custom_strength_options = identitytoolkit.custom_strength_options
identitytoolkit_enforcement_state = identitytoolkit.enforcement_state
identitytoolkit_schema_version = identitytoolkit.schema_version
```

---


### Mfa_enrollment

Step one of the MFA enrollment process. In SMS case, this sends an SMS verification code to the user.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id_token` | String |  | Required. User's ID token. |
| `tenant_id` | String |  | The ID of the Identity Platform tenant that the user enrolling MFA belongs to. If not set, the user belongs to the default Identity Platform project. |
| `phone_enrollment_info` | String |  | Verification info to authorize sending an SMS for phone verification. |
| `totp_enrollment_info` | String |  | Sign-in info specific to TOTP auth. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mfa_enrollment
mfa_enrollment = provider.identitytoolkit_api.Mfa_enrollment {
}

```

---


### Mfa_sign_in

Sends the MFA challenge

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mfa_pending_credential` | String |  | Required. Pending credential from first factor sign-in. |
| `phone_sign_in_info` | String |  | Verification info to authorize sending an SMS for phone verification. |
| `mfa_enrollment_id` | String |  | Required. MFA enrollment id from the user's list of current MFA enrollments. |
| `tenant_id` | String |  | The ID of the Identity Platform tenant the user is signing in to. If not set, the user will sign in to the default Identity Platform project. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mfa_sign_in
mfa_sign_in = provider.identitytoolkit_api.Mfa_sign_in {
}

```

---


### Default_supported_idp_config

Create a default supported Idp configuration for an Identity Toolkit project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the DefaultSupportedIdpConfig resource, for example: "projects/my-awesome-project/defaultSupportedIdpConfigs/google.com" |
| `apple_sign_in_config` | String |  | Additional config for Apple-based projects. |
| `client_id` | String |  | OAuth client ID. |
| `client_secret` | String |  | OAuth client secret. |
| `enabled` | bool |  | True if allows the user to sign in with the provider. |
| `parent` | String | ✅ | The parent resource name where the config to be created, for example: "projects/my-awesome-project" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the DefaultSupportedIdpConfig resource, for example: "projects/my-awesome-project/defaultSupportedIdpConfigs/google.com" |
| `apple_sign_in_config` | String | Additional config for Apple-based projects. |
| `client_id` | String | OAuth client ID. |
| `client_secret` | String | OAuth client secret. |
| `enabled` | bool | True if allows the user to sign in with the provider. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create default_supported_idp_config
default_supported_idp_config = provider.identitytoolkit_api.Default_supported_idp_config {
    parent = "value"  # The parent resource name where the config to be created, for example: "projects/my-awesome-project"
}

# Access default_supported_idp_config outputs
default_supported_idp_config_id = default_supported_idp_config.id
default_supported_idp_config_name = default_supported_idp_config.name
default_supported_idp_config_apple_sign_in_config = default_supported_idp_config.apple_sign_in_config
default_supported_idp_config_client_id = default_supported_idp_config.client_id
default_supported_idp_config_client_secret = default_supported_idp_config.client_secret
default_supported_idp_config_enabled = default_supported_idp_config.enabled
```

---


### Default_supported_idp

List all default supported Idps.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_supported_idps` | Vec<String> | The set of configs. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access default_supported_idp outputs
default_supported_idp_id = default_supported_idp.id
default_supported_idp_default_supported_idps = default_supported_idp.default_supported_idps
default_supported_idp_next_page_token = default_supported_idp.next_page_token
```

---


### Tenant

Create a tenant. Requires write permission on the Agent project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `password_policy_config` | String |  | The tenant-level password policy config |
| `disable_auth` | bool |  | Whether authentication is disabled for the tenant. If true, the users under the disabled tenant are not allowed to sign-in. Admins of the disabled tenant are not able to manage its users. |
| `enable_anonymous_user` | bool |  | Whether to enable anonymous user authentication. |
| `allow_password_signup` | bool |  | Whether to allow email/password user authentication. |
| `sms_region_config` | String |  | Configures which regions are enabled for SMS verification code sending. |
| `email_privacy_config` | String |  | Configuration for settings related to email privacy and public visibility. |
| `mobile_links_config` | String |  | Optional. Deprecated. Never launched. Configuration for settings related to univeral links (iOS) and app links (Android). |
| `monitoring` | String |  | Configuration related to monitoring project activity. |
| `test_phone_numbers` | HashMap<String, String> |  | A map of pairs that can be used for MFA. The phone number should be in E.164 format (https://www.itu.int/rec/T-REC-E.164/) and a maximum of 10 pairs can be added (error will be thrown once exceeded). |
| `mfa_config` | String |  | The tenant-level configuration of MFA options. |
| `hash_config` | String |  | Output only. Hash config information of a tenant for display on Pantheon. This can only be displayed on Pantheon to avoid the sensitive information to get accidentally leaked. Only returned in GetTenant response to restrict reading of this information. Requires firebaseauth.configs.getHashConfig permission on the agent project for returning this field. |
| `name` | String |  | Output only. Resource name of a tenant. For example: "projects/{project-id}/tenants/{tenant-id}" |
| `inheritance` | String |  | Specify the settings that the tenant could inherit. |
| `display_name` | String |  | Display name of the tenant. |
| `recaptcha_config` | String |  | The tenant-level reCAPTCHA config. |
| `autodelete_anonymous_users` | bool |  | Whether anonymous users will be auto-deleted after a period of 30 days. |
| `enable_email_link_signin` | bool |  | Whether to enable email link user authentication. |
| `client` | String |  | Options related to how clients making requests on behalf of a project should be configured. |
| `parent` | String | ✅ | The parent resource name where the tenant will be created. For example, "projects/project1". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `password_policy_config` | String | The tenant-level password policy config |
| `disable_auth` | bool | Whether authentication is disabled for the tenant. If true, the users under the disabled tenant are not allowed to sign-in. Admins of the disabled tenant are not able to manage its users. |
| `enable_anonymous_user` | bool | Whether to enable anonymous user authentication. |
| `allow_password_signup` | bool | Whether to allow email/password user authentication. |
| `sms_region_config` | String | Configures which regions are enabled for SMS verification code sending. |
| `email_privacy_config` | String | Configuration for settings related to email privacy and public visibility. |
| `mobile_links_config` | String | Optional. Deprecated. Never launched. Configuration for settings related to univeral links (iOS) and app links (Android). |
| `monitoring` | String | Configuration related to monitoring project activity. |
| `test_phone_numbers` | HashMap<String, String> | A map of pairs that can be used for MFA. The phone number should be in E.164 format (https://www.itu.int/rec/T-REC-E.164/) and a maximum of 10 pairs can be added (error will be thrown once exceeded). |
| `mfa_config` | String | The tenant-level configuration of MFA options. |
| `hash_config` | String | Output only. Hash config information of a tenant for display on Pantheon. This can only be displayed on Pantheon to avoid the sensitive information to get accidentally leaked. Only returned in GetTenant response to restrict reading of this information. Requires firebaseauth.configs.getHashConfig permission on the agent project for returning this field. |
| `name` | String | Output only. Resource name of a tenant. For example: "projects/{project-id}/tenants/{tenant-id}" |
| `inheritance` | String | Specify the settings that the tenant could inherit. |
| `display_name` | String | Display name of the tenant. |
| `recaptcha_config` | String | The tenant-level reCAPTCHA config. |
| `autodelete_anonymous_users` | bool | Whether anonymous users will be auto-deleted after a period of 30 days. |
| `enable_email_link_signin` | bool | Whether to enable email link user authentication. |
| `client` | String | Options related to how clients making requests on behalf of a project should be configured. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tenant
tenant = provider.identitytoolkit_api.Tenant {
    parent = "value"  # The parent resource name where the tenant will be created. For example, "projects/project1".
}

# Access tenant outputs
tenant_id = tenant.id
tenant_password_policy_config = tenant.password_policy_config
tenant_disable_auth = tenant.disable_auth
tenant_enable_anonymous_user = tenant.enable_anonymous_user
tenant_allow_password_signup = tenant.allow_password_signup
tenant_sms_region_config = tenant.sms_region_config
tenant_email_privacy_config = tenant.email_privacy_config
tenant_mobile_links_config = tenant.mobile_links_config
tenant_monitoring = tenant.monitoring
tenant_test_phone_numbers = tenant.test_phone_numbers
tenant_mfa_config = tenant.mfa_config
tenant_hash_config = tenant.hash_config
tenant_name = tenant.name
tenant_inheritance = tenant.inheritance
tenant_display_name = tenant.display_name
tenant_recaptcha_config = tenant.recaptcha_config
tenant_autodelete_anonymous_users = tenant.autodelete_anonymous_users
tenant_enable_email_link_signin = tenant.enable_email_link_signin
tenant_client = tenant.client
```

---


### Account

Revokes a user's token from an Identity Provider (IdP). This is done by manually providing an IdP credential, and the token types for revocation. An [API key](https://cloud.google.com/docs/authentication/api-keys) is required in the request in order to identify the Google Cloud project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tenant_id` | String |  | The ID of the Identity Platform tenant the user is signing in to. If not set, the user will sign in to the default Identity Platform project. |
| `token` | String |  | Required. The token to be revoked. If an authorization_code is passed in, the API will first exchange the code for access token and then revoke the token exchanged. |
| `redirect_uri` | String |  | The redirect URI provided in the initial authorization request made by the client to the IDP. The URI must use the HTTPS protocol, include a domain name, and can't contain an IP address or localhost. Required if token_type is CODE. |
| `token_type` | String |  | Required. The type of the token to be revoked. |
| `id_token` | String |  | Required. A valid Identity Platform ID token to link the account. If there was a successful token revocation request on the account and no tokens are generated after the revocation, the duplicate requests will be ignored and returned immediately. |
| `provider_id` | String |  | Required. The idp provider for the token. Currently only supports Apple Idp. The format should be "apple.com". |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.identitytoolkit_api.Account {
}

```

---


### Inbound_saml_config

Create an inbound SAML configuration for an Identity Toolkit project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the InboundSamlConfig resource, for example: 'projects/my-awesome-project/inboundSamlConfigs/my-config-id'. Ignored during create requests. |
| `idp_config` | String |  | The SAML IdP (Identity Provider) configuration when the project acts as the relying party. |
| `enabled` | bool |  | True if allows the user to sign in with the provider. |
| `display_name` | String |  | The config's display name set by developers. |
| `sp_config` | String |  | The SAML SP (Service Provider) configuration when the project acts as the relying party to receive and accept an authentication assertion issued by a SAML identity provider. |
| `parent` | String | ✅ | The parent resource name where the config to be created, for example: "projects/my-awesome-project" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the InboundSamlConfig resource, for example: 'projects/my-awesome-project/inboundSamlConfigs/my-config-id'. Ignored during create requests. |
| `idp_config` | String | The SAML IdP (Identity Provider) configuration when the project acts as the relying party. |
| `enabled` | bool | True if allows the user to sign in with the provider. |
| `display_name` | String | The config's display name set by developers. |
| `sp_config` | String | The SAML SP (Service Provider) configuration when the project acts as the relying party to receive and accept an authentication assertion issued by a SAML identity provider. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create inbound_saml_config
inbound_saml_config = provider.identitytoolkit_api.Inbound_saml_config {
    parent = "value"  # The parent resource name where the config to be created, for example: "projects/my-awesome-project"
}

# Access inbound_saml_config outputs
inbound_saml_config_id = inbound_saml_config.id
inbound_saml_config_name = inbound_saml_config.name
inbound_saml_config_idp_config = inbound_saml_config.idp_config
inbound_saml_config_enabled = inbound_saml_config.enabled
inbound_saml_config_display_name = inbound_saml_config.display_name
inbound_saml_config_sp_config = inbound_saml_config.sp_config
```

---


### Relyingparty

Returns the account info.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `local_id` | Vec<String> |  | The list of local ID's of the users to inquiry. |
| `id_token` | String |  | The GITKit token of the authenticated user. |
| `email` | Vec<String> |  | The list of emails of the users to inquiry. |
| `delegated_project_number` | String |  | GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration. |
| `phone_number` | Vec<String> |  | Privileged caller can query users by specified phone number. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `allow_password_user` | bool | Whether to allow password user sign in or sign up. |
| `api_key` | String | Browser API key, needed when making http request to Apiary. |
| `change_email_template` | String | Change email template. |
| `enable_anonymous_user` | bool | Whether anonymous user is enabled. |
| `dynamic_links_domain` | String |  |
| `project_id` | String | Project ID of the relying party. |
| `reset_password_template` | String | Reset password email template. |
| `use_email_sending` | bool | Whether to use email sending provided by Firebear. |
| `legacy_reset_password_template` | String | Legacy reset password email template. |
| `authorized_domains` | Vec<String> | Authorized domains. |
| `idp_config` | Vec<String> | OAuth2 provider configuration. |
| `verify_email_template` | String | Verify email template. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create relyingparty
relyingparty = provider.identitytoolkit_api.Relyingparty {
}

# Access relyingparty outputs
relyingparty_id = relyingparty.id
relyingparty_allow_password_user = relyingparty.allow_password_user
relyingparty_api_key = relyingparty.api_key
relyingparty_change_email_template = relyingparty.change_email_template
relyingparty_enable_anonymous_user = relyingparty.enable_anonymous_user
relyingparty_dynamic_links_domain = relyingparty.dynamic_links_domain
relyingparty_project_id = relyingparty.project_id
relyingparty_reset_password_template = relyingparty.reset_password_template
relyingparty_use_email_sending = relyingparty.use_email_sending
relyingparty_legacy_reset_password_template = relyingparty.legacy_reset_password_template
relyingparty_authorized_domains = relyingparty.authorized_domains
relyingparty_idp_config = relyingparty.idp_config
relyingparty_verify_email_template = relyingparty.verify_email_template
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple identity_platform resources
identity_platform_0 = provider.identitytoolkit_api.Identity_platform {
    project = "value-0"
}
identity_platform_1 = provider.identitytoolkit_api.Identity_platform {
    project = "value-1"
}
identity_platform_2 = provider.identitytoolkit_api.Identity_platform {
    project = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    identity_platform = provider.identitytoolkit_api.Identity_platform {
        project = "production-value"
    }
```

---

## Related Documentation

- [GCP Identitytoolkit_api Documentation](https://cloud.google.com/identitytoolkit_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
