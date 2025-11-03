# Cloudidentity_api Service



**Resources**: 22

---

## Overview

The cloudidentity_api service provides access to 22 resource types:

- [Membership](#membership) [CRD]
- [Inbound_sso_assignment](#inbound_sso_assignment) [CRUD]
- [Userinvitation](#userinvitation) [CR]
- [Idp_credential](#idp_credential) [CRD]
- [Inbound_oidc_sso_profile](#inbound_oidc_sso_profile) [CRUD]
- [Inbound_saml_sso_profile](#inbound_saml_sso_profile) [CRUD]
- [Device](#device) [CRD]
- [Client_state](#client_state) [RU]
- [Policie](#policie) [R]
- [Group](#group) [CRUD]
- [Device_user](#device_user) [CRD]
- [Policie](#policie) [CRUD]
- [Userinvitation](#userinvitation) [CR]
- [Group](#group) [CRUD]
- [Idp_credential](#idp_credential) [CRD]
- [Device](#device) [CRD]
- [Client_state](#client_state) [RU]
- [Device_user](#device_user) [CRD]
- [Inbound_oidc_sso_profile](#inbound_oidc_sso_profile) [CRUD]
- [Inbound_saml_sso_profile](#inbound_saml_sso_profile) [CRUD]
- [Inbound_sso_assignment](#inbound_sso_assignment) [CRUD]
- [Membership](#membership) [CRD]

---

## Resources


### Membership

Creates a `Membership`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Output only. The type of the membership. |
| `preferred_member_key` | String |  | Required. Immutable. The `EntityKey` of the member. |
| `roles` | Vec<String> |  | The `MembershipRole`s that apply to the `Membership`. If unspecified, defaults to a single `MembershipRole` with `name` `MEMBER`. Must not contain duplicate `MembershipRole`s with the same `name`. |
| `create_time` | String |  | Output only. The time when the `Membership` was created. |
| `update_time` | String |  | Output only. The time when the `Membership` was last updated. |
| `delivery_setting` | String |  | Output only. Delivery setting associated with the membership. |
| `name` | String |  | Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Membership`. Shall be of the form `groups/{group}/memberships/{membership}`. |
| `parent` | String | ✅ | Required. The parent `Group` resource under which to create the `Membership`. Must be of the form `groups/{group}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Output only. The type of the membership. |
| `preferred_member_key` | String | Required. Immutable. The `EntityKey` of the member. |
| `roles` | Vec<String> | The `MembershipRole`s that apply to the `Membership`. If unspecified, defaults to a single `MembershipRole` with `name` `MEMBER`. Must not contain duplicate `MembershipRole`s with the same `name`. |
| `create_time` | String | Output only. The time when the `Membership` was created. |
| `update_time` | String | Output only. The time when the `Membership` was last updated. |
| `delivery_setting` | String | Output only. Delivery setting associated with the membership. |
| `name` | String | Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Membership`. Shall be of the form `groups/{group}/memberships/{membership}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create membership
membership = provider.cloudidentity_api.Membership {
    parent = "value"  # Required. The parent `Group` resource under which to create the `Membership`. Must be of the form `groups/{group}`.
}

# Access membership outputs
membership_id = membership.id
membership_type = membership.type
membership_preferred_member_key = membership.preferred_member_key
membership_roles = membership.roles
membership_create_time = membership.create_time
membership_update_time = membership.update_time
membership_delivery_setting = membership.delivery_setting
membership_name = membership.name
```

---


### Inbound_sso_assignment

Creates an InboundSsoAssignment for users and devices in a `Customer` under a given `Group` or `OrgUnit`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rank` | i64 |  | Must be zero (which is the default value so it can be omitted) for assignments with `target_org_unit` set and must be greater-than-or-equal-to one for assignments with `target_group` set. |
| `name` | String |  | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Inbound SSO Assignment. |
| `sso_mode` | String |  | Inbound SSO behavior. |
| `sign_in_behavior` | String |  | Assertions about users assigned to an IdP will always be accepted from that IdP. This controls whether/when Google should redirect a user to the IdP. Unset (defaults) is the recommended configuration. |
| `saml_sso_info` | String |  | SAML SSO details. Must be set if and only if `sso_mode` is set to `SAML_SSO`. |
| `customer` | String |  | Immutable. The customer. For example: `customers/C0123abc`. |
| `target_org_unit` | String |  | Immutable. Must be of the form `orgUnits/{org_unit}`. |
| `oidc_sso_info` | String |  | OpenID Connect SSO details. Must be set if and only if `sso_mode` is set to `OIDC_SSO`. |
| `target_group` | String |  | Immutable. Must be of the form `groups/{group}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rank` | i64 | Must be zero (which is the default value so it can be omitted) for assignments with `target_org_unit` set and must be greater-than-or-equal-to one for assignments with `target_group` set. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Inbound SSO Assignment. |
| `sso_mode` | String | Inbound SSO behavior. |
| `sign_in_behavior` | String | Assertions about users assigned to an IdP will always be accepted from that IdP. This controls whether/when Google should redirect a user to the IdP. Unset (defaults) is the recommended configuration. |
| `saml_sso_info` | String | SAML SSO details. Must be set if and only if `sso_mode` is set to `SAML_SSO`. |
| `customer` | String | Immutable. The customer. For example: `customers/C0123abc`. |
| `target_org_unit` | String | Immutable. Must be of the form `orgUnits/{org_unit}`. |
| `oidc_sso_info` | String | OpenID Connect SSO details. Must be set if and only if `sso_mode` is set to `OIDC_SSO`. |
| `target_group` | String | Immutable. Must be of the form `groups/{group}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create inbound_sso_assignment
inbound_sso_assignment = provider.cloudidentity_api.Inbound_sso_assignment {
}

# Access inbound_sso_assignment outputs
inbound_sso_assignment_id = inbound_sso_assignment.id
inbound_sso_assignment_rank = inbound_sso_assignment.rank
inbound_sso_assignment_name = inbound_sso_assignment.name
inbound_sso_assignment_sso_mode = inbound_sso_assignment.sso_mode
inbound_sso_assignment_sign_in_behavior = inbound_sso_assignment.sign_in_behavior
inbound_sso_assignment_saml_sso_info = inbound_sso_assignment.saml_sso_info
inbound_sso_assignment_customer = inbound_sso_assignment.customer
inbound_sso_assignment_target_org_unit = inbound_sso_assignment.target_org_unit
inbound_sso_assignment_oidc_sso_info = inbound_sso_assignment.oidc_sso_info
inbound_sso_assignment_target_group = inbound_sso_assignment.target_group
```

---


### Userinvitation

Cancels a UserInvitation that was already sent.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. `UserInvitation` name in the format `customers/{customer}/userinvitations/{user_email_address}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mails_sent_count` | String | Number of invitation emails sent to the user. |
| `state` | String | State of the `UserInvitation`. |
| `update_time` | String | Time when the `UserInvitation` was last updated. |
| `name` | String | Shall be of the form `customers/{customer}/userinvitations/{user_email_address}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create userinvitation
userinvitation = provider.cloudidentity_api.Userinvitation {
    name = "value"  # Required. `UserInvitation` name in the format `customers/{customer}/userinvitations/{user_email_address}`
}

# Access userinvitation outputs
userinvitation_id = userinvitation.id
userinvitation_mails_sent_count = userinvitation.mails_sent_count
userinvitation_state = userinvitation.state
userinvitation_update_time = userinvitation.update_time
userinvitation_name = userinvitation.name
```

---


### Idp_credential

Adds an IdpCredential. Up to 2 credentials are allowed. When the target customer has enabled [Multi-party approval for sensitive actions](https://support.google.com/a/answer/13790448), the `Operation` in the response will have `"done": false`, it will not have a response, and the metadata will have `"state": "awaiting-multi-party-approval"`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pem_data` | String |  | PEM encoded x509 certificate containing the public key for verifying IdP signatures. |
| `parent` | String | ✅ | Required. The InboundSamlSsoProfile that owns the IdpCredential. Format: `inboundSamlSsoProfiles/{sso_profile_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dsa_key_info` | String | Output only. Information of a DSA public key. |
| `rsa_key_info` | String | Output only. Information of a RSA public key. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the credential. |
| `update_time` | String | Output only. Time when the `IdpCredential` was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create idp_credential
idp_credential = provider.cloudidentity_api.Idp_credential {
    parent = "value"  # Required. The InboundSamlSsoProfile that owns the IdpCredential. Format: `inboundSamlSsoProfiles/{sso_profile_id}`
}

# Access idp_credential outputs
idp_credential_id = idp_credential.id
idp_credential_dsa_key_info = idp_credential.dsa_key_info
idp_credential_rsa_key_info = idp_credential.rsa_key_info
idp_credential_name = idp_credential.name
idp_credential_update_time = idp_credential.update_time
```

---


### Inbound_oidc_sso_profile

Creates an InboundOidcSsoProfile for a customer. When the target customer has enabled [Multi-party approval for sensitive actions](https://support.google.com/a/answer/13790448), the `Operation` in the response will have `"done": false`, it will not have a response, and the metadata will have `"state": "awaiting-multi-party-approval"`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Immutable. The customer. For example: `customers/C0123abc`. |
| `display_name` | String |  | Human-readable name of the OIDC SSO profile. |
| `idp_config` | String |  | OIDC identity provider configuration. |
| `name` | String |  | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the OIDC SSO profile. |
| `rp_config` | String |  | OIDC relying party (RP) configuration for this OIDC SSO profile. These are the RP details provided by Google that should be configured on the corresponding identity provider. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer` | String | Immutable. The customer. For example: `customers/C0123abc`. |
| `display_name` | String | Human-readable name of the OIDC SSO profile. |
| `idp_config` | String | OIDC identity provider configuration. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the OIDC SSO profile. |
| `rp_config` | String | OIDC relying party (RP) configuration for this OIDC SSO profile. These are the RP details provided by Google that should be configured on the corresponding identity provider. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create inbound_oidc_sso_profile
inbound_oidc_sso_profile = provider.cloudidentity_api.Inbound_oidc_sso_profile {
}

# Access inbound_oidc_sso_profile outputs
inbound_oidc_sso_profile_id = inbound_oidc_sso_profile.id
inbound_oidc_sso_profile_customer = inbound_oidc_sso_profile.customer
inbound_oidc_sso_profile_display_name = inbound_oidc_sso_profile.display_name
inbound_oidc_sso_profile_idp_config = inbound_oidc_sso_profile.idp_config
inbound_oidc_sso_profile_name = inbound_oidc_sso_profile.name
inbound_oidc_sso_profile_rp_config = inbound_oidc_sso_profile.rp_config
```

---


### Inbound_saml_sso_profile

Creates an InboundSamlSsoProfile for a customer. When the target customer has enabled [Multi-party approval for sensitive actions](https://support.google.com/a/answer/13790448), the `Operation` in the response will have `"done": false`, it will not have a response, and the metadata will have `"state": "awaiting-multi-party-approval"`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Human-readable name of the SAML SSO profile. |
| `idp_config` | String |  | SAML identity provider configuration. |
| `name` | String |  | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the SAML SSO profile. |
| `customer` | String |  | Immutable. The customer. For example: `customers/C0123abc`. |
| `sp_config` | String |  | SAML service provider configuration for this SAML SSO profile. These are the service provider details provided by Google that should be configured on the corresponding identity provider. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Human-readable name of the SAML SSO profile. |
| `idp_config` | String | SAML identity provider configuration. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the SAML SSO profile. |
| `customer` | String | Immutable. The customer. For example: `customers/C0123abc`. |
| `sp_config` | String | SAML service provider configuration for this SAML SSO profile. These are the service provider details provided by Google that should be configured on the corresponding identity provider. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create inbound_saml_sso_profile
inbound_saml_sso_profile = provider.cloudidentity_api.Inbound_saml_sso_profile {
}

# Access inbound_saml_sso_profile outputs
inbound_saml_sso_profile_id = inbound_saml_sso_profile.id
inbound_saml_sso_profile_display_name = inbound_saml_sso_profile.display_name
inbound_saml_sso_profile_idp_config = inbound_saml_sso_profile.idp_config
inbound_saml_sso_profile_name = inbound_saml_sso_profile.name
inbound_saml_sso_profile_customer = inbound_saml_sso_profile.customer
inbound_saml_sso_profile_sp_config = inbound_saml_sso_profile.sp_config
```

---


### Device

Creates a device. Only company-owned device may be created. **Note**: This method is available only to customers who have one of the following SKUs: Enterprise Standard, Enterprise Plus, Enterprise for Education, and Cloud Identity Premium

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `last_sync_time` | String |  | Most recent time when device synced with this service. |
| `owner_type` | String |  | Output only. Whether the device is owned by the company or an individual |
| `device_type` | String |  | Output only. Type of device. |
| `manufacturer` | String |  | Output only. Device manufacturer. Example: Motorola. |
| `name` | String |  | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}`, where device is the unique id assigned to the Device. Important: Device API scopes require that you use domain-wide delegation to access the API. For more information, see [Set up the Devices API](https://cloud.google.com/identity/docs/how-to/setup-devices). |
| `asset_tag` | String |  | Asset tag of the device. |
| `model` | String |  | Output only. Model name of device. Example: Pixel 3. |
| `brand` | String |  | Output only. Device brand. Example: Samsung. |
| `enabled_developer_options` | bool |  | Output only. Whether developer options is enabled on device. |
| `release_version` | String |  | Output only. OS release version. Example: 6.0. |
| `device_id` | String |  | Unique identifier for the device. |
| `baseband_version` | String |  | Output only. Baseband version of the device. |
| `create_time` | String |  | Output only. When the Company-Owned device was imported. This field is empty for BYOD devices. |
| `enabled_usb_debugging` | bool |  | Output only. Whether USB debugging is enabled on device. |
| `android_specific_attributes` | String |  | Output only. Attributes specific to Android devices. |
| `security_patch_time` | String |  | Output only. OS security patch update time on device. |
| `compromised_state` | String |  | Output only. Represents whether the Device is compromised. |
| `bootloader_version` | String |  | Output only. Device bootloader version. Example: 0.6.7. |
| `hostname` | String |  | Host name of the device. |
| `management_state` | String |  | Output only. Management state of the device |
| `network_operator` | String |  | Output only. Mobile or network operator of device, if available. |
| `other_accounts` | Vec<String> |  | Output only. Domain name for Google accounts on device. Type for other accounts on device. On Android, will only be populated if |ownership_privilege| is |PROFILE_OWNER| or |DEVICE_OWNER|. Does not include the account signed in to the device policy app if that account's domain has only one account. Examples: "com.example", "xyz.com". |
| `unified_device_id` | String |  | Output only. Unified device id of the device. |
| `wifi_mac_addresses` | Vec<String> |  | WiFi MAC addresses of device. |
| `os_version` | String |  | Output only. OS version of the device. Example: Android 8.1.0. |
| `serial_number` | String |  | Serial Number of device. Example: HT82V1A01076. |
| `build_number` | String |  | Output only. Build number of the device. |
| `imei` | String |  | Output only. IMEI number of device if GSM device; empty otherwise. |
| `meid` | String |  | Output only. MEID number of device if CDMA device; empty otherwise. |
| `endpoint_verification_specific_attributes` | String |  | Output only. Attributes specific to [Endpoint Verification](https://cloud.google.com/endpoint-verification/docs/overview) devices. |
| `kernel_version` | String |  | Output only. Kernel version of the device. |
| `encryption_state` | String |  | Output only. Device encryption state. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_sync_time` | String | Most recent time when device synced with this service. |
| `owner_type` | String | Output only. Whether the device is owned by the company or an individual |
| `device_type` | String | Output only. Type of device. |
| `manufacturer` | String | Output only. Device manufacturer. Example: Motorola. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}`, where device is the unique id assigned to the Device. Important: Device API scopes require that you use domain-wide delegation to access the API. For more information, see [Set up the Devices API](https://cloud.google.com/identity/docs/how-to/setup-devices). |
| `asset_tag` | String | Asset tag of the device. |
| `model` | String | Output only. Model name of device. Example: Pixel 3. |
| `brand` | String | Output only. Device brand. Example: Samsung. |
| `enabled_developer_options` | bool | Output only. Whether developer options is enabled on device. |
| `release_version` | String | Output only. OS release version. Example: 6.0. |
| `device_id` | String | Unique identifier for the device. |
| `baseband_version` | String | Output only. Baseband version of the device. |
| `create_time` | String | Output only. When the Company-Owned device was imported. This field is empty for BYOD devices. |
| `enabled_usb_debugging` | bool | Output only. Whether USB debugging is enabled on device. |
| `android_specific_attributes` | String | Output only. Attributes specific to Android devices. |
| `security_patch_time` | String | Output only. OS security patch update time on device. |
| `compromised_state` | String | Output only. Represents whether the Device is compromised. |
| `bootloader_version` | String | Output only. Device bootloader version. Example: 0.6.7. |
| `hostname` | String | Host name of the device. |
| `management_state` | String | Output only. Management state of the device |
| `network_operator` | String | Output only. Mobile or network operator of device, if available. |
| `other_accounts` | Vec<String> | Output only. Domain name for Google accounts on device. Type for other accounts on device. On Android, will only be populated if |ownership_privilege| is |PROFILE_OWNER| or |DEVICE_OWNER|. Does not include the account signed in to the device policy app if that account's domain has only one account. Examples: "com.example", "xyz.com". |
| `unified_device_id` | String | Output only. Unified device id of the device. |
| `wifi_mac_addresses` | Vec<String> | WiFi MAC addresses of device. |
| `os_version` | String | Output only. OS version of the device. Example: Android 8.1.0. |
| `serial_number` | String | Serial Number of device. Example: HT82V1A01076. |
| `build_number` | String | Output only. Build number of the device. |
| `imei` | String | Output only. IMEI number of device if GSM device; empty otherwise. |
| `meid` | String | Output only. MEID number of device if CDMA device; empty otherwise. |
| `endpoint_verification_specific_attributes` | String | Output only. Attributes specific to [Endpoint Verification](https://cloud.google.com/endpoint-verification/docs/overview) devices. |
| `kernel_version` | String | Output only. Kernel version of the device. |
| `encryption_state` | String | Output only. Device encryption state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device
device = provider.cloudidentity_api.Device {
}

# Access device outputs
device_id = device.id
device_last_sync_time = device.last_sync_time
device_owner_type = device.owner_type
device_device_type = device.device_type
device_manufacturer = device.manufacturer
device_name = device.name
device_asset_tag = device.asset_tag
device_model = device.model
device_brand = device.brand
device_enabled_developer_options = device.enabled_developer_options
device_release_version = device.release_version
device_device_id = device.device_id
device_baseband_version = device.baseband_version
device_create_time = device.create_time
device_enabled_usb_debugging = device.enabled_usb_debugging
device_android_specific_attributes = device.android_specific_attributes
device_security_patch_time = device.security_patch_time
device_compromised_state = device.compromised_state
device_bootloader_version = device.bootloader_version
device_hostname = device.hostname
device_management_state = device.management_state
device_network_operator = device.network_operator
device_other_accounts = device.other_accounts
device_unified_device_id = device.unified_device_id
device_wifi_mac_addresses = device.wifi_mac_addresses
device_os_version = device.os_version
device_serial_number = device.serial_number
device_build_number = device.build_number
device_imei = device.imei
device_meid = device.meid
device_endpoint_verification_specific_attributes = device.endpoint_verification_specific_attributes
device_kernel_version = device.kernel_version
device_encryption_state = device.encryption_state
```

---


### Client_state

Gets the client state for the device user

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_tags` | Vec<String> |  | The caller can specify asset tags for this resource |
| `managed` | String |  | The management state of the resource as specified by the API client. |
| `name` | String |  | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device}/deviceUsers/{device_user}/clientState/{partner}`, where partner corresponds to the partner storing the data. For partners belonging to the "BeyondCorp Alliance", this is the partner ID specified to you by Google. For all other callers, this is a string of the form: `{customer}-suffix`, where `customer` is your customer ID. The *suffix* is any string the caller specifies. This string will be displayed verbatim in the administration console. This suffix is used in setting up Custom Access Levels in Context-Aware Access. Your organization's customer ID can be obtained from the URL: `GET https://www.googleapis.com/admin/directory/v1/customers/my_customer` The `id` field in the response contains the customer ID starting with the letter 'C'. The customer ID to be used in this API is the string after the letter 'C' (not including 'C') |
| `compliance_state` | String |  | The compliance state of the resource as specified by the API client. |
| `key_value_pairs` | HashMap<String, String> |  | The map of key-value attributes stored by callers specific to a device. The total serialized length of this map may not exceed 10KB. No limit is placed on the number of attributes in a map. |
| `score_reason` | String |  | A descriptive cause of the health score. |
| `etag` | String |  | The token that needs to be passed back for concurrency control in updates. Token needs to be passed back in UpdateRequest |
| `custom_id` | String |  | This field may be used to store a unique identifier for the API resource within which these CustomAttributes are a field. |
| `last_update_time` | String |  | Output only. The time the client state data was last updated. |
| `create_time` | String |  | Output only. The time the client state data was created. |
| `health_score` | String |  | The Health score of the resource. The Health score is the callers specification of the condition of the device from a usability point of view. For example, a third-party device management provider may specify a health score based on its compliance with organizational policies. |
| `owner_type` | String |  | Output only. The owner of the ClientState |
| `name` | String | ✅ | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device}/deviceUsers/{device_user}/clientState/{partner}`, where partner corresponds to the partner storing the data. For partners belonging to the "BeyondCorp Alliance", this is the partner ID specified to you by Google. For all other callers, this is a string of the form: `{customer}-suffix`, where `customer` is your customer ID. The *suffix* is any string the caller specifies. This string will be displayed verbatim in the administration console. This suffix is used in setting up Custom Access Levels in Context-Aware Access. Your organization's customer ID can be obtained from the URL: `GET https://www.googleapis.com/admin/directory/v1/customers/my_customer` The `id` field in the response contains the customer ID starting with the letter 'C'. The customer ID to be used in this API is the string after the letter 'C' (not including 'C') |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_tags` | Vec<String> | The caller can specify asset tags for this resource |
| `managed` | String | The management state of the resource as specified by the API client. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device}/deviceUsers/{device_user}/clientState/{partner}`, where partner corresponds to the partner storing the data. For partners belonging to the "BeyondCorp Alliance", this is the partner ID specified to you by Google. For all other callers, this is a string of the form: `{customer}-suffix`, where `customer` is your customer ID. The *suffix* is any string the caller specifies. This string will be displayed verbatim in the administration console. This suffix is used in setting up Custom Access Levels in Context-Aware Access. Your organization's customer ID can be obtained from the URL: `GET https://www.googleapis.com/admin/directory/v1/customers/my_customer` The `id` field in the response contains the customer ID starting with the letter 'C'. The customer ID to be used in this API is the string after the letter 'C' (not including 'C') |
| `compliance_state` | String | The compliance state of the resource as specified by the API client. |
| `key_value_pairs` | HashMap<String, String> | The map of key-value attributes stored by callers specific to a device. The total serialized length of this map may not exceed 10KB. No limit is placed on the number of attributes in a map. |
| `score_reason` | String | A descriptive cause of the health score. |
| `etag` | String | The token that needs to be passed back for concurrency control in updates. Token needs to be passed back in UpdateRequest |
| `custom_id` | String | This field may be used to store a unique identifier for the API resource within which these CustomAttributes are a field. |
| `last_update_time` | String | Output only. The time the client state data was last updated. |
| `create_time` | String | Output only. The time the client state data was created. |
| `health_score` | String | The Health score of the resource. The Health score is the callers specification of the condition of the device from a usability point of view. For example, a third-party device management provider may specify a health score based on its compliance with organizational policies. |
| `owner_type` | String | Output only. The owner of the ClientState |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access client_state outputs
client_state_id = client_state.id
client_state_asset_tags = client_state.asset_tags
client_state_managed = client_state.managed
client_state_name = client_state.name
client_state_compliance_state = client_state.compliance_state
client_state_key_value_pairs = client_state.key_value_pairs
client_state_score_reason = client_state.score_reason
client_state_etag = client_state.etag
client_state_custom_id = client_state.custom_id
client_state_last_update_time = client_state.last_update_time
client_state_create_time = client_state.create_time
client_state_health_score = client_state.health_score
client_state_owner_type = client_state.owner_type
```

---


### Policie

Get a policy.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer` | String | Immutable. Customer that the Policy belongs to. The value is in the format 'customers/{customerId}'. The `customerId` must begin with "C" To find your customer ID in Admin Console see https://support.google.com/a/answer/10070793. |
| `policy_query` | String | Required. The PolicyQuery the Setting applies to. |
| `name` | String | Output only. Identifier. The [resource name](https://cloud.google.com/apis/design/resource_names) of the Policy. Format: policies/{policy}. |
| `setting` | String | Required. The Setting configured by this Policy. |
| `type` | String | Output only. The type of the policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access policie outputs
policie_id = policie.id
policie_customer = policie.customer
policie_policy_query = policie.policy_query
policie_name = policie.name
policie_setting = policie.setting
policie_type = policie.type
```

---


### Group

Creates a Group.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String |  | Required. Immutable. The resource name of the entity under which this `Group` resides in the Cloud Identity resource hierarchy. Must be of the form `identitysources/{identity_source}` for external [identity-mapped groups](https://support.google.com/a/answer/9039510) or `customers/{customer_id}` for Google Groups. The `customer_id` must begin with "C" (for example, 'C046psxkn'). [Find your customer ID.] (https://support.google.com/cloudidentity/answer/10070793) |
| `additional_group_keys` | Vec<String> |  | Output only. Additional group keys associated with the Group. |
| `group_key` | String |  | Required. The `EntityKey` of the `Group`. |
| `display_name` | String |  | The display name of the `Group`. |
| `update_time` | String |  | Output only. The time when the `Group` was last updated. |
| `dynamic_group_metadata` | String |  | Optional. Dynamic group metadata like queries and status. |
| `labels` | HashMap<String, String> |  | Required. One or more label entries that apply to the Group. Labels contain a key with an empty value. Google Groups are the default type of group and have a label with a key of `cloudidentity.googleapis.com/groups.discussion_forum` and an empty value. Existing Google Groups can have an additional label with a key of `cloudidentity.googleapis.com/groups.security` and an empty value added to them. **This is an immutable change and the security label cannot be removed once added.** Dynamic groups have a label with a key of `cloudidentity.googleapis.com/groups.dynamic`. Identity-mapped groups for Cloud Search have a label with a key of `system/groups/external` and an empty value. Google Groups can be [locked](https://support.google.com/a?p=locked-groups). To lock a group, add a label with a key of `cloudidentity.googleapis.com/groups.locked` and an empty value. Doing so locks the group. To unlock the group, remove this label. |
| `create_time` | String |  | Output only. The time when the `Group` was created. |
| `description` | String |  | An extended description to help users determine the purpose of a `Group`. Must not be longer than 4,096 characters. |
| `name` | String |  | Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Group`. Shall be of the form `groups/{group}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent` | String | Required. Immutable. The resource name of the entity under which this `Group` resides in the Cloud Identity resource hierarchy. Must be of the form `identitysources/{identity_source}` for external [identity-mapped groups](https://support.google.com/a/answer/9039510) or `customers/{customer_id}` for Google Groups. The `customer_id` must begin with "C" (for example, 'C046psxkn'). [Find your customer ID.] (https://support.google.com/cloudidentity/answer/10070793) |
| `additional_group_keys` | Vec<String> | Output only. Additional group keys associated with the Group. |
| `group_key` | String | Required. The `EntityKey` of the `Group`. |
| `display_name` | String | The display name of the `Group`. |
| `update_time` | String | Output only. The time when the `Group` was last updated. |
| `dynamic_group_metadata` | String | Optional. Dynamic group metadata like queries and status. |
| `labels` | HashMap<String, String> | Required. One or more label entries that apply to the Group. Labels contain a key with an empty value. Google Groups are the default type of group and have a label with a key of `cloudidentity.googleapis.com/groups.discussion_forum` and an empty value. Existing Google Groups can have an additional label with a key of `cloudidentity.googleapis.com/groups.security` and an empty value added to them. **This is an immutable change and the security label cannot be removed once added.** Dynamic groups have a label with a key of `cloudidentity.googleapis.com/groups.dynamic`. Identity-mapped groups for Cloud Search have a label with a key of `system/groups/external` and an empty value. Google Groups can be [locked](https://support.google.com/a?p=locked-groups). To lock a group, add a label with a key of `cloudidentity.googleapis.com/groups.locked` and an empty value. Doing so locks the group. To unlock the group, remove this label. |
| `create_time` | String | Output only. The time when the `Group` was created. |
| `description` | String | An extended description to help users determine the purpose of a `Group`. Must not be longer than 4,096 characters. |
| `name` | String | Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Group`. Shall be of the form `groups/{group}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.cloudidentity_api.Group {
}

# Access group outputs
group_id = group.id
group_parent = group.parent
group_additional_group_keys = group.additional_group_keys
group_group_key = group.group_key
group_display_name = group.display_name
group_update_time = group.update_time
group_dynamic_group_metadata = group.dynamic_group_metadata
group_labels = group.labels
group_create_time = group.create_time
group_description = group.description
group_name = group.name
```

---


### Device_user

Approves device to access user data.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Optional. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer}`, where customer is the customer to whom the device belongs. |
| `name` | String | ✅ | Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}/deviceUsers/{device_user}`, where device is the unique ID assigned to the Device, and device_user is the unique ID assigned to the User. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `password_state` | String | Password state of the DeviceUser object |
| `create_time` | String | When the user first signed in to the device |
| `language_code` | String | Output only. Default locale used on device, in IETF BCP-47 format. |
| `user_agent` | String | Output only. User agent on the device for this specific user |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the DeviceUser in format: `devices/{device}/deviceUsers/{device_user}`, where `device_user` uniquely identifies a user's use of a device. |
| `first_sync_time` | String | Output only. Most recent time when user registered with this service. |
| `last_sync_time` | String | Output only. Last time when user synced with policies. |
| `user_email` | String | Email address of the user registered on the device. |
| `compromised_state` | String | Compromised State of the DeviceUser object |
| `management_state` | String | Output only. Management state of the user on the device. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device_user
device_user = provider.cloudidentity_api.Device_user {
    name = "value"  # Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}/deviceUsers/{device_user}`, where device is the unique ID assigned to the Device, and device_user is the unique ID assigned to the User.
}

# Access device_user outputs
device_user_id = device_user.id
device_user_password_state = device_user.password_state
device_user_create_time = device_user.create_time
device_user_language_code = device_user.language_code
device_user_user_agent = device_user.user_agent
device_user_name = device_user.name
device_user_first_sync_time = device_user.first_sync_time
device_user_last_sync_time = device_user.last_sync_time
device_user_user_email = device_user.user_email
device_user_compromised_state = device_user.compromised_state
device_user_management_state = device_user.management_state
```

---


### Policie

Create a policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Immutable. Customer that the Policy belongs to. The value is in the format 'customers/{customerId}'. The `customerId` must begin with "C" To find your customer ID in Admin Console see https://support.google.com/a/answer/10070793. |
| `name` | String |  | Output only. Identifier. The [resource name](https://cloud.google.com/apis/design/resource_names) of the Policy. Format: policies/{policy}. |
| `policy_query` | String |  | Required. The PolicyQuery the Setting applies to. |
| `setting` | String |  | Required. The Setting configured by this Policy. |
| `type` | String |  | Output only. The type of the policy. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer` | String | Immutable. Customer that the Policy belongs to. The value is in the format 'customers/{customerId}'. The `customerId` must begin with "C" To find your customer ID in Admin Console see https://support.google.com/a/answer/10070793. |
| `name` | String | Output only. Identifier. The [resource name](https://cloud.google.com/apis/design/resource_names) of the Policy. Format: policies/{policy}. |
| `policy_query` | String | Required. The PolicyQuery the Setting applies to. |
| `setting` | String | Required. The Setting configured by this Policy. |
| `type` | String | Output only. The type of the policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.cloudidentity_api.Policie {
}

# Access policie outputs
policie_id = policie.id
policie_customer = policie.customer
policie_name = policie.name
policie_policy_query = policie.policy_query
policie_setting = policie.setting
policie_type = policie.type
```

---


### Userinvitation

Sends a UserInvitation to email. If the `UserInvitation` does not exist for this request and it is a valid request, the request creates a `UserInvitation`. **Note:** The `get` and `list` methods have a 48-hour delay where newly-created consumer accounts will not appear in the results. You can still send a `UserInvitation` to those accounts if you know the unmanaged email address and IsInvitableUser==True.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. `UserInvitation` name in the format `customers/{customer}/userinvitations/{user_email_address}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | State of the `UserInvitation`. |
| `name` | String | Shall be of the form `customers/{customer}/userinvitations/{user_email_address}`. |
| `update_time` | String | Time when the `UserInvitation` was last updated. |
| `mails_sent_count` | String | Number of invitation emails sent to the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create userinvitation
userinvitation = provider.cloudidentity_api.Userinvitation {
    name = "value"  # Required. `UserInvitation` name in the format `customers/{customer}/userinvitations/{user_email_address}`
}

# Access userinvitation outputs
userinvitation_id = userinvitation.id
userinvitation_state = userinvitation.state
userinvitation_name = userinvitation.name
userinvitation_update_time = userinvitation.update_time
userinvitation_mails_sent_count = userinvitation.mails_sent_count
```

---


### Group

Creates a `Group`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `additional_group_keys` | Vec<String> |  | Output only. Additional group keys associated with the Group. |
| `group_key` | String |  | Required. The `EntityKey` of the `Group`. |
| `labels` | HashMap<String, String> |  | Required. One or more label entries that apply to the Group. Labels contain a key with an empty value. Google Groups are the default type of group and have a label with a key of `cloudidentity.googleapis.com/groups.discussion_forum` and an empty value. Existing Google Groups can have an additional label with a key of `cloudidentity.googleapis.com/groups.security` and an empty value added to them. **This is an immutable change and the security label cannot be removed once added.** Dynamic groups have a label with a key of `cloudidentity.googleapis.com/groups.dynamic`. Identity-mapped groups for Cloud Search have a label with a key of `system/groups/external` and an empty value. Google Groups can be [locked](https://support.google.com/a?p=locked-groups). To lock a group, add a label with a key of `cloudidentity.googleapis.com/groups.locked` and an empty value. Doing so locks the group. To unlock the group, remove this label. |
| `posix_groups` | Vec<String> |  | Optional. The POSIX groups associated with the `Group`. |
| `parent` | String |  | Required. Immutable. The resource name of the entity under which this `Group` resides in the Cloud Identity resource hierarchy. Must be of the form `identitysources/{identity_source}` for external [identity-mapped groups](https://support.google.com/a/answer/9039510) or `customers/{customer_id}` for Google Groups. The `customer_id` must begin with "C" (for example, 'C046psxkn'). [Find your customer ID.] (https://support.google.com/cloudidentity/answer/10070793) |
| `dynamic_group_metadata` | String |  | Optional. Dynamic group metadata like queries and status. |
| `update_time` | String |  | Output only. The time when the `Group` was last updated. |
| `name` | String |  | Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Group`. Shall be of the form `groups/{group_id}`. |
| `create_time` | String |  | Output only. The time when the `Group` was created. |
| `description` | String |  | An extended description to help users determine the purpose of a `Group`. Must not be longer than 4,096 characters. |
| `display_name` | String |  | The display name of the `Group`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `additional_group_keys` | Vec<String> | Output only. Additional group keys associated with the Group. |
| `group_key` | String | Required. The `EntityKey` of the `Group`. |
| `labels` | HashMap<String, String> | Required. One or more label entries that apply to the Group. Labels contain a key with an empty value. Google Groups are the default type of group and have a label with a key of `cloudidentity.googleapis.com/groups.discussion_forum` and an empty value. Existing Google Groups can have an additional label with a key of `cloudidentity.googleapis.com/groups.security` and an empty value added to them. **This is an immutable change and the security label cannot be removed once added.** Dynamic groups have a label with a key of `cloudidentity.googleapis.com/groups.dynamic`. Identity-mapped groups for Cloud Search have a label with a key of `system/groups/external` and an empty value. Google Groups can be [locked](https://support.google.com/a?p=locked-groups). To lock a group, add a label with a key of `cloudidentity.googleapis.com/groups.locked` and an empty value. Doing so locks the group. To unlock the group, remove this label. |
| `posix_groups` | Vec<String> | Optional. The POSIX groups associated with the `Group`. |
| `parent` | String | Required. Immutable. The resource name of the entity under which this `Group` resides in the Cloud Identity resource hierarchy. Must be of the form `identitysources/{identity_source}` for external [identity-mapped groups](https://support.google.com/a/answer/9039510) or `customers/{customer_id}` for Google Groups. The `customer_id` must begin with "C" (for example, 'C046psxkn'). [Find your customer ID.] (https://support.google.com/cloudidentity/answer/10070793) |
| `dynamic_group_metadata` | String | Optional. Dynamic group metadata like queries and status. |
| `update_time` | String | Output only. The time when the `Group` was last updated. |
| `name` | String | Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Group`. Shall be of the form `groups/{group_id}`. |
| `create_time` | String | Output only. The time when the `Group` was created. |
| `description` | String | An extended description to help users determine the purpose of a `Group`. Must not be longer than 4,096 characters. |
| `display_name` | String | The display name of the `Group`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.cloudidentity_api.Group {
}

# Access group outputs
group_id = group.id
group_additional_group_keys = group.additional_group_keys
group_group_key = group.group_key
group_labels = group.labels
group_posix_groups = group.posix_groups
group_parent = group.parent
group_dynamic_group_metadata = group.dynamic_group_metadata
group_update_time = group.update_time
group_name = group.name
group_create_time = group.create_time
group_description = group.description
group_display_name = group.display_name
```

---


### Idp_credential

Adds an IdpCredential. Up to 2 credentials are allowed. When the target customer has enabled [Multi-party approval for sensitive actions](https://support.google.com/a/answer/13790448), the `Operation` in the response will have `"done": false`, it will not have a response, and the metadata will have `"state": "awaiting-multi-party-approval"`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pem_data` | String |  | PEM encoded x509 certificate containing the public key for verifying IdP signatures. |
| `parent` | String | ✅ | Required. The InboundSamlSsoProfile that owns the IdpCredential. Format: `inboundSamlSsoProfiles/{sso_profile_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dsa_key_info` | String | Output only. Information of a DSA public key. |
| `update_time` | String | Output only. Time when the `IdpCredential` was last updated. |
| `rsa_key_info` | String | Output only. Information of a RSA public key. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the credential. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create idp_credential
idp_credential = provider.cloudidentity_api.Idp_credential {
    parent = "value"  # Required. The InboundSamlSsoProfile that owns the IdpCredential. Format: `inboundSamlSsoProfiles/{sso_profile_id}`
}

# Access idp_credential outputs
idp_credential_id = idp_credential.id
idp_credential_dsa_key_info = idp_credential.dsa_key_info
idp_credential_update_time = idp_credential.update_time
idp_credential_rsa_key_info = idp_credential.rsa_key_info
idp_credential_name = idp_credential.name
```

---


### Device

Creates a device. Only company-owned device may be created. **Note**: This method is available only to customers who have one of the following SKUs: Enterprise Standard, Enterprise Plus, Enterprise for Education, and Cloud Identity Premium

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Optional. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs. |
| `device` | String |  | Required. The device to be created. The name field within this device is ignored in the create method. A new name is created by the method, and returned within the response. Only the fields `device_type`, `serial_number` and `asset_tag` (if present) are used to create the device. All other fields are ignored. The `device_type` and `serial_number` fields are required. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `build_number` | String | Output only. Build number of the device. |
| `create_time` | String | Output only. When the Company-Owned device was imported. This field is empty for BYOD devices. |
| `enabled_developer_options` | bool | Output only. Whether developer options is enabled on device. |
| `compromised_state` | String | Output only. Represents whether the Device is compromised. |
| `kernel_version` | String | Output only. Kernel version of the device. |
| `device_id` | String | Unique identifier for the device. |
| `asset_tag` | String | Asset tag of the device. |
| `manufacturer` | String | Output only. Device manufacturer. Example: Motorola. |
| `owner_type` | String | Output only. Whether the device is owned by the company or an individual |
| `security_patch_time` | String | Output only. OS security patch update time on device. |
| `other_accounts` | Vec<String> | Output only. Domain name for Google accounts on device. Type for other accounts on device. On Android, will only be populated if |ownership_privilege| is |PROFILE_OWNER| or |DEVICE_OWNER|. Does not include the account signed in to the device policy app if that account's domain has only one account. Examples: "com.example", "xyz.com". |
| `unified_device_id` | String | Output only. Unified device id of the device. |
| `wifi_mac_addresses` | Vec<String> | WiFi MAC addresses of device. |
| `client_types` | Vec<String> | List of the clients the device is reporting to. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device_id}`, where device_id is the unique id assigned to the Device. |
| `hostname` | String | Host name of the device. |
| `last_sync_time` | String | Most recent time when device synced with this service. |
| `encryption_state` | String | Output only. Device encryption state. |
| `android_specific_attributes` | String | Output only. Attributes specific to Android devices. |
| `os_version` | String | Output only. OS version of the device. Example: Android 8.1.0. |
| `meid` | String | Output only. MEID number of device if CDMA device; empty otherwise. |
| `imei` | String | Output only. IMEI number of device if GSM device; empty otherwise. |
| `model` | String | Output only. Model name of device. Example: Pixel 3. |
| `release_version` | String | Output only. OS release version. Example: 6.0. |
| `enabled_usb_debugging` | bool | Output only. Whether USB debugging is enabled on device. |
| `brand` | String | Output only. Device brand. Example: Samsung. |
| `device_type` | String | Output only. Type of device. |
| `network_operator` | String | Output only. Mobile or network operator of device, if available. |
| `baseband_version` | String | Output only. Baseband version of the device. |
| `serial_number` | String | Serial Number of device. Example: HT82V1A01076. |
| `management_state` | String | Output only. Management state of the device |
| `bootloader_version` | String | Output only. Device bootloader version. Example: 0.6.7. |
| `endpoint_verification_specific_attributes` | String | Output only. Attributes specific to [Endpoint Verification](https://cloud.google.com/endpoint-verification/docs/overview) devices. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device
device = provider.cloudidentity_api.Device {
}

# Access device outputs
device_id = device.id
device_build_number = device.build_number
device_create_time = device.create_time
device_enabled_developer_options = device.enabled_developer_options
device_compromised_state = device.compromised_state
device_kernel_version = device.kernel_version
device_device_id = device.device_id
device_asset_tag = device.asset_tag
device_manufacturer = device.manufacturer
device_owner_type = device.owner_type
device_security_patch_time = device.security_patch_time
device_other_accounts = device.other_accounts
device_unified_device_id = device.unified_device_id
device_wifi_mac_addresses = device.wifi_mac_addresses
device_client_types = device.client_types
device_name = device.name
device_hostname = device.hostname
device_last_sync_time = device.last_sync_time
device_encryption_state = device.encryption_state
device_android_specific_attributes = device.android_specific_attributes
device_os_version = device.os_version
device_meid = device.meid
device_imei = device.imei
device_model = device.model
device_release_version = device.release_version
device_enabled_usb_debugging = device.enabled_usb_debugging
device_brand = device.brand
device_device_type = device.device_type
device_network_operator = device.network_operator
device_baseband_version = device.baseband_version
device_serial_number = device.serial_number
device_management_state = device.management_state
device_bootloader_version = device.bootloader_version
device_endpoint_verification_specific_attributes = device.endpoint_verification_specific_attributes
```

---


### Client_state

Gets the client state for the device user

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `health_score` | String |  | The Health score of the resource |
| `score_reason` | String |  | A descriptive cause of the health score. |
| `custom_id` | String |  | This field may be used to store a unique identifier for the API resource within which these CustomAttributes are a field. |
| `etag` | String |  | The token that needs to be passed back for concurrency control in updates. Token needs to be passed back in UpdateRequest |
| `create_time` | String |  | Output only. The time the client state data was created. |
| `compliance_state` | String |  | The compliance state of the resource as specified by the API client. |
| `last_update_time` | String |  | Output only. The time the client state data was last updated. |
| `key_value_pairs` | HashMap<String, String> |  | The map of key-value attributes stored by callers specific to a device. The total serialized length of this map may not exceed 10KB. No limit is placed on the number of attributes in a map. |
| `managed` | String |  | The management state of the resource as specified by the API client. |
| `name` | String |  | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device_id}/deviceUsers/{device_user_id}/clientState/{partner_id}`, where partner_id corresponds to the partner storing the data. |
| `owner_type` | String |  | Output only. The owner of the ClientState |
| `asset_tags` | Vec<String> |  | The caller can specify asset tags for this resource |
| `name` | String | ✅ | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device_id}/deviceUsers/{device_user_id}/clientState/{partner_id}`, where partner_id corresponds to the partner storing the data. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `health_score` | String | The Health score of the resource |
| `score_reason` | String | A descriptive cause of the health score. |
| `custom_id` | String | This field may be used to store a unique identifier for the API resource within which these CustomAttributes are a field. |
| `etag` | String | The token that needs to be passed back for concurrency control in updates. Token needs to be passed back in UpdateRequest |
| `create_time` | String | Output only. The time the client state data was created. |
| `compliance_state` | String | The compliance state of the resource as specified by the API client. |
| `last_update_time` | String | Output only. The time the client state data was last updated. |
| `key_value_pairs` | HashMap<String, String> | The map of key-value attributes stored by callers specific to a device. The total serialized length of this map may not exceed 10KB. No limit is placed on the number of attributes in a map. |
| `managed` | String | The management state of the resource as specified by the API client. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device_id}/deviceUsers/{device_user_id}/clientState/{partner_id}`, where partner_id corresponds to the partner storing the data. |
| `owner_type` | String | Output only. The owner of the ClientState |
| `asset_tags` | Vec<String> | The caller can specify asset tags for this resource |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access client_state outputs
client_state_id = client_state.id
client_state_health_score = client_state.health_score
client_state_score_reason = client_state.score_reason
client_state_custom_id = client_state.custom_id
client_state_etag = client_state.etag
client_state_create_time = client_state.create_time
client_state_compliance_state = client_state.compliance_state
client_state_last_update_time = client_state.last_update_time
client_state_key_value_pairs = client_state.key_value_pairs
client_state_managed = client_state.managed
client_state_name = client_state.name
client_state_owner_type = client_state.owner_type
client_state_asset_tags = client_state.asset_tags
```

---


### Device_user

Approves device to access user data.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Optional. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs. |
| `name` | String | ✅ | Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device_id}/deviceUsers/{device_user_id}`, where device_id is the unique ID assigned to the Device, and device_user_id is the unique ID assigned to the User. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `first_sync_time` | String | Output only. Most recent time when user registered with this service. |
| `language_code` | String | Output only. Default locale used on device, in IETF BCP-47 format. |
| `management_state` | String | Output only. Management state of the user on the device. |
| `compromised_state` | String | Compromised State of the DeviceUser object |
| `last_sync_time` | String | Output only. Last time when user synced with policies. |
| `user_agent` | String | Output only. User agent on the device for this specific user |
| `password_state` | String | Password state of the DeviceUser object |
| `user_email` | String | Email address of the user registered on the device. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the DeviceUser in format: `devices/{device_id}/deviceUsers/{device_user_id}`, where `device_user_id` uniquely identifies a user's use of a device. |
| `create_time` | String | When the user first signed in to the device |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device_user
device_user = provider.cloudidentity_api.Device_user {
    name = "value"  # Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device_id}/deviceUsers/{device_user_id}`, where device_id is the unique ID assigned to the Device, and device_user_id is the unique ID assigned to the User.
}

# Access device_user outputs
device_user_id = device_user.id
device_user_first_sync_time = device_user.first_sync_time
device_user_language_code = device_user.language_code
device_user_management_state = device_user.management_state
device_user_compromised_state = device_user.compromised_state
device_user_last_sync_time = device_user.last_sync_time
device_user_user_agent = device_user.user_agent
device_user_password_state = device_user.password_state
device_user_user_email = device_user.user_email
device_user_name = device_user.name
device_user_create_time = device_user.create_time
```

---


### Inbound_oidc_sso_profile

Creates an InboundOidcSsoProfile for a customer. When the target customer has enabled [Multi-party approval for sensitive actions](https://support.google.com/a/answer/13790448), the `Operation` in the response will have `"done": false`, it will not have a response, and the metadata will have `"state": "awaiting-multi-party-approval"`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Immutable. The customer. For example: `customers/C0123abc`. |
| `name` | String |  | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the OIDC SSO profile. |
| `rp_config` | String |  | OIDC relying party (RP) configuration for this OIDC SSO profile. These are the RP details provided by Google that should be configured on the corresponding identity provider. |
| `idp_config` | String |  | OIDC identity provider configuration. |
| `display_name` | String |  | Human-readable name of the OIDC SSO profile. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer` | String | Immutable. The customer. For example: `customers/C0123abc`. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the OIDC SSO profile. |
| `rp_config` | String | OIDC relying party (RP) configuration for this OIDC SSO profile. These are the RP details provided by Google that should be configured on the corresponding identity provider. |
| `idp_config` | String | OIDC identity provider configuration. |
| `display_name` | String | Human-readable name of the OIDC SSO profile. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create inbound_oidc_sso_profile
inbound_oidc_sso_profile = provider.cloudidentity_api.Inbound_oidc_sso_profile {
}

# Access inbound_oidc_sso_profile outputs
inbound_oidc_sso_profile_id = inbound_oidc_sso_profile.id
inbound_oidc_sso_profile_customer = inbound_oidc_sso_profile.customer
inbound_oidc_sso_profile_name = inbound_oidc_sso_profile.name
inbound_oidc_sso_profile_rp_config = inbound_oidc_sso_profile.rp_config
inbound_oidc_sso_profile_idp_config = inbound_oidc_sso_profile.idp_config
inbound_oidc_sso_profile_display_name = inbound_oidc_sso_profile.display_name
```

---


### Inbound_saml_sso_profile

Creates an InboundSamlSsoProfile for a customer. When the target customer has enabled [Multi-party approval for sensitive actions](https://support.google.com/a/answer/13790448), the `Operation` in the response will have `"done": false`, it will not have a response, and the metadata will have `"state": "awaiting-multi-party-approval"`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Immutable. The customer. For example: `customers/C0123abc`. |
| `display_name` | String |  | Human-readable name of the SAML SSO profile. |
| `name` | String |  | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the SAML SSO profile. |
| `idp_config` | String |  | SAML identity provider configuration. |
| `sp_config` | String |  | SAML service provider configuration for this SAML SSO profile. These are the service provider details provided by Google that should be configured on the corresponding identity provider. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer` | String | Immutable. The customer. For example: `customers/C0123abc`. |
| `display_name` | String | Human-readable name of the SAML SSO profile. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the SAML SSO profile. |
| `idp_config` | String | SAML identity provider configuration. |
| `sp_config` | String | SAML service provider configuration for this SAML SSO profile. These are the service provider details provided by Google that should be configured on the corresponding identity provider. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create inbound_saml_sso_profile
inbound_saml_sso_profile = provider.cloudidentity_api.Inbound_saml_sso_profile {
}

# Access inbound_saml_sso_profile outputs
inbound_saml_sso_profile_id = inbound_saml_sso_profile.id
inbound_saml_sso_profile_customer = inbound_saml_sso_profile.customer
inbound_saml_sso_profile_display_name = inbound_saml_sso_profile.display_name
inbound_saml_sso_profile_name = inbound_saml_sso_profile.name
inbound_saml_sso_profile_idp_config = inbound_saml_sso_profile.idp_config
inbound_saml_sso_profile_sp_config = inbound_saml_sso_profile.sp_config
```

---


### Inbound_sso_assignment

Creates an InboundSsoAssignment for users and devices in a `Customer` under a given `Group` or `OrgUnit`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Immutable. The customer. For example: `customers/C0123abc`. |
| `rank` | i64 |  | Must be zero (which is the default value so it can be omitted) for assignments with `target_org_unit` set and must be greater-than-or-equal-to one for assignments with `target_group` set. |
| `oidc_sso_info` | String |  | OpenID Connect SSO details. Must be set if and only if `sso_mode` is set to `OIDC_SSO`. |
| `sign_in_behavior` | String |  | Assertions about users assigned to an IdP will always be accepted from that IdP. This controls whether/when Google should redirect a user to the IdP. Unset (defaults) is the recommended configuration. |
| `target_group` | String |  | Immutable. Must be of the form `groups/{group}`. |
| `sso_mode` | String |  | Inbound SSO behavior. |
| `name` | String |  | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Inbound SSO Assignment. |
| `target_org_unit` | String |  | Immutable. Must be of the form `orgUnits/{org_unit}`. |
| `saml_sso_info` | String |  | SAML SSO details. Must be set if and only if `sso_mode` is set to `SAML_SSO`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer` | String | Immutable. The customer. For example: `customers/C0123abc`. |
| `rank` | i64 | Must be zero (which is the default value so it can be omitted) for assignments with `target_org_unit` set and must be greater-than-or-equal-to one for assignments with `target_group` set. |
| `oidc_sso_info` | String | OpenID Connect SSO details. Must be set if and only if `sso_mode` is set to `OIDC_SSO`. |
| `sign_in_behavior` | String | Assertions about users assigned to an IdP will always be accepted from that IdP. This controls whether/when Google should redirect a user to the IdP. Unset (defaults) is the recommended configuration. |
| `target_group` | String | Immutable. Must be of the form `groups/{group}`. |
| `sso_mode` | String | Inbound SSO behavior. |
| `name` | String | Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Inbound SSO Assignment. |
| `target_org_unit` | String | Immutable. Must be of the form `orgUnits/{org_unit}`. |
| `saml_sso_info` | String | SAML SSO details. Must be set if and only if `sso_mode` is set to `SAML_SSO`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create inbound_sso_assignment
inbound_sso_assignment = provider.cloudidentity_api.Inbound_sso_assignment {
}

# Access inbound_sso_assignment outputs
inbound_sso_assignment_id = inbound_sso_assignment.id
inbound_sso_assignment_customer = inbound_sso_assignment.customer
inbound_sso_assignment_rank = inbound_sso_assignment.rank
inbound_sso_assignment_oidc_sso_info = inbound_sso_assignment.oidc_sso_info
inbound_sso_assignment_sign_in_behavior = inbound_sso_assignment.sign_in_behavior
inbound_sso_assignment_target_group = inbound_sso_assignment.target_group
inbound_sso_assignment_sso_mode = inbound_sso_assignment.sso_mode
inbound_sso_assignment_name = inbound_sso_assignment.name
inbound_sso_assignment_target_org_unit = inbound_sso_assignment.target_org_unit
inbound_sso_assignment_saml_sso_info = inbound_sso_assignment.saml_sso_info
```

---


### Membership

Creates a `Membership`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `member_key` | String |  | Immutable. The `EntityKey` of the member. Either `member_key` or `preferred_member_key` must be set when calling MembershipsService.CreateMembership but not both; both shall be set when returned. |
| `name` | String |  | Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Membership`. Shall be of the form `groups/{group_id}/memberships/{membership_id}`. |
| `type` | String |  | Output only. The type of the membership. |
| `delivery_setting` | String |  | Output only. Delivery setting associated with the membership. |
| `preferred_member_key` | String |  | Required. Immutable. The `EntityKey` of the member. Either `member_key` or `preferred_member_key` must be set when calling MembershipsService.CreateMembership but not both; both shall be set when returned. |
| `roles` | Vec<String> |  | The `MembershipRole`s that apply to the `Membership`. If unspecified, defaults to a single `MembershipRole` with `name` `MEMBER`. Must not contain duplicate `MembershipRole`s with the same `name`. |
| `update_time` | String |  | Output only. The time when the `Membership` was last updated. |
| `create_time` | String |  | Output only. The time when the `Membership` was created. |
| `parent` | String | ✅ | Required. The parent `Group` resource under which to create the `Membership`. Must be of the form `groups/{group_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `member_key` | String | Immutable. The `EntityKey` of the member. Either `member_key` or `preferred_member_key` must be set when calling MembershipsService.CreateMembership but not both; both shall be set when returned. |
| `name` | String | Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Membership`. Shall be of the form `groups/{group_id}/memberships/{membership_id}`. |
| `type` | String | Output only. The type of the membership. |
| `delivery_setting` | String | Output only. Delivery setting associated with the membership. |
| `preferred_member_key` | String | Required. Immutable. The `EntityKey` of the member. Either `member_key` or `preferred_member_key` must be set when calling MembershipsService.CreateMembership but not both; both shall be set when returned. |
| `roles` | Vec<String> | The `MembershipRole`s that apply to the `Membership`. If unspecified, defaults to a single `MembershipRole` with `name` `MEMBER`. Must not contain duplicate `MembershipRole`s with the same `name`. |
| `update_time` | String | Output only. The time when the `Membership` was last updated. |
| `create_time` | String | Output only. The time when the `Membership` was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create membership
membership = provider.cloudidentity_api.Membership {
    parent = "value"  # Required. The parent `Group` resource under which to create the `Membership`. Must be of the form `groups/{group_id}`.
}

# Access membership outputs
membership_id = membership.id
membership_member_key = membership.member_key
membership_name = membership.name
membership_type = membership.type
membership_delivery_setting = membership.delivery_setting
membership_preferred_member_key = membership.preferred_member_key
membership_roles = membership.roles
membership_update_time = membership.update_time
membership_create_time = membership.create_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple membership resources
membership_0 = provider.cloudidentity_api.Membership {
    parent = "value-0"
}
membership_1 = provider.cloudidentity_api.Membership {
    parent = "value-1"
}
membership_2 = provider.cloudidentity_api.Membership {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    membership = provider.cloudidentity_api.Membership {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudidentity_api Documentation](https://cloud.google.com/cloudidentity_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
