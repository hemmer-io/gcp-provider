# Firebaseappcheck_api Service



**Resources**: 26

---

## Overview

The firebaseappcheck_api service provides access to 26 resource types:

- [Play_integrity_config](#play_integrity_config) [RU]
- [Oauth_client](#oauth_client) [C]
- [Recaptcha_enterprise_config](#recaptcha_enterprise_config) [RU]
- [App](#app) [C]
- [Safety_net_config](#safety_net_config) [RU]
- [Service](#service) [CRU]
- [Resource_policie](#resource_policie) [CRUD]
- [Debug_token](#debug_token) [CRUD]
- [Jwk](#jwk) [R]
- [App_attest_config](#app_attest_config) [RU]
- [Device_check_config](#device_check_config) [RU]
- [Recaptcha_v3_config](#recaptcha_v3_config) [RU]
- [Project](#project) [C]
- [Recaptcha_v3_config](#recaptcha_v3_config) [RU]
- [Resource_policie](#resource_policie) [CRUD]
- [App](#app) [C]
- [Service](#service) [CRU]
- [Play_integrity_config](#play_integrity_config) [RU]
- [Recaptcha_config](#recaptcha_config) [RU]
- [Jwk](#jwk) [R]
- [Device_check_config](#device_check_config) [RU]
- [Debug_token](#debug_token) [CRUD]
- [Safety_net_config](#safety_net_config) [RU]
- [App_attest_config](#app_attest_config) [RU]
- [Recaptcha_enterprise_config](#recaptcha_enterprise_config) [RU]
- [Oauth_client](#oauth_client) [C]

---

## Resources


### Play_integrity_config

Gets the PlayIntegrityConfig for the specified app.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_integrity` | String |  | Specifies device integrity requirements for Android devices running your app. These settings correspond to requirements on the [**device integrity** field](https://developer.android.com/google/play/integrity/verdicts#device-integrity-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. Warning: There are also [conditional](https://developer.android.com/google/play/integrity/setup#conditional) as well as [optional](https://developer.android.com/google/play/integrity/setup#optional_device_information) responses that you can receive, but requires additional explicit opt-in from you. The App Check API is **not** responsible for any such opt-ins. The default values for these settings work for most apps, and are recommended. |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from Play Integrity tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `account_details` | String |  | Specifies account requirements for Android devices running your app. These settings correspond to requirements on the [**account details** field](https://developer.android.com/google/play/integrity/verdicts#account-details-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. The default values for these settings work for most apps, and are recommended. |
| `app_integrity` | String |  | Specifies application integrity requirements for Android devices running your app. These settings correspond to requirements on the [**application integrity** field](https://developer.android.com/google/play/integrity/verdicts#application-integrity-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. The default values for these settings work for most apps, and are recommended. |
| `name` | String |  | Required. The relative resource name of the Play Integrity configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/playIntegrityConfig ``` |
| `name` | String | ✅ | Required. The relative resource name of the Play Integrity configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/playIntegrityConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device_integrity` | String | Specifies device integrity requirements for Android devices running your app. These settings correspond to requirements on the [**device integrity** field](https://developer.android.com/google/play/integrity/verdicts#device-integrity-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. Warning: There are also [conditional](https://developer.android.com/google/play/integrity/setup#conditional) as well as [optional](https://developer.android.com/google/play/integrity/setup#optional_device_information) responses that you can receive, but requires additional explicit opt-in from you. The App Check API is **not** responsible for any such opt-ins. The default values for these settings work for most apps, and are recommended. |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from Play Integrity tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `account_details` | String | Specifies account requirements for Android devices running your app. These settings correspond to requirements on the [**account details** field](https://developer.android.com/google/play/integrity/verdicts#account-details-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. The default values for these settings work for most apps, and are recommended. |
| `app_integrity` | String | Specifies application integrity requirements for Android devices running your app. These settings correspond to requirements on the [**application integrity** field](https://developer.android.com/google/play/integrity/verdicts#application-integrity-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. The default values for these settings work for most apps, and are recommended. |
| `name` | String | Required. The relative resource name of the Play Integrity configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/playIntegrityConfig ``` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access play_integrity_config outputs
play_integrity_config_id = play_integrity_config.id
play_integrity_config_device_integrity = play_integrity_config.device_integrity
play_integrity_config_token_ttl = play_integrity_config.token_ttl
play_integrity_config_account_details = play_integrity_config.account_details
play_integrity_config_app_integrity = play_integrity_config.app_integrity
play_integrity_config_name = play_integrity_config.name
```

---


### Oauth_client

Accepts an App Attest CBOR attestation and verifies it with Apple using your preconfigured team and bundle IDs. If valid, returns an attestation artifact that can later be exchanged for an AppCheckToken using ExchangeAppAttestAssertion. For convenience and performance, this method's response object will also contain an AppCheckToken (if the verification is successful).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_id` | String |  | Required. The key ID generated by App Attest for the client app. |
| `attestation_statement` | String |  | Required. The App Attest statement returned by the client-side App Attest API. This is a base64url encoded CBOR object in the JSON response. |
| `challenge` | String |  | Required. A one-time challenge returned by an immediately prior call to GenerateAppAttestChallenge. |
| `limited_use` | bool |  | Specifies whether this attestation is for use in a *limited use* (`true`) or *session based* (`false`) context. To enable this attestation to be used with the *replay protection* feature, set this to `true`. The default value is `false`. |
| `app` | String | ✅ | Required. The relative resource name of the iOS app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create oauth_client
oauth_client = provider.firebaseappcheck_api.Oauth_client {
    app = "value"  # Required. The relative resource name of the iOS app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
}

```

---


### Recaptcha_enterprise_config

Gets the RecaptchaEnterpriseConfig for the specified app.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `risk_analysis` | String |  | Specifies risk tolerance and requirements for your application. These settings correspond to requirements on the [**`riskAnalysis`**](https://cloud.google.com/recaptcha/docs/interpret-assessment-website#interpret_assessment) tuple in the assessment obtained from reCAPTCHA Enterprise. The default values for these settings work for most apps, and are recommended. |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from reCAPTCHA Enterprise tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `name` | String |  | Required. The relative resource name of the reCAPTCHA Enterprise configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaEnterpriseConfig ``` |
| `site_key` | String |  | The score-based site key [created in reCAPTCHA Enterprise](https://cloud.google.com/recaptcha-enterprise/docs/create-key#creating_a_site_key) used to [invoke reCAPTCHA and generate the reCAPTCHA tokens](https://cloud.google.com/recaptcha-enterprise/docs/instrument-web-pages) for your application. Important: This is *not* the `site_secret` (as it is in reCAPTCHA v3), but rather your score-based reCAPTCHA Enterprise site key. |
| `name` | String | ✅ | Required. The relative resource name of the reCAPTCHA Enterprise configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaEnterpriseConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `risk_analysis` | String | Specifies risk tolerance and requirements for your application. These settings correspond to requirements on the [**`riskAnalysis`**](https://cloud.google.com/recaptcha/docs/interpret-assessment-website#interpret_assessment) tuple in the assessment obtained from reCAPTCHA Enterprise. The default values for these settings work for most apps, and are recommended. |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from reCAPTCHA Enterprise tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `name` | String | Required. The relative resource name of the reCAPTCHA Enterprise configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaEnterpriseConfig ``` |
| `site_key` | String | The score-based site key [created in reCAPTCHA Enterprise](https://cloud.google.com/recaptcha-enterprise/docs/create-key#creating_a_site_key) used to [invoke reCAPTCHA and generate the reCAPTCHA tokens](https://cloud.google.com/recaptcha-enterprise/docs/instrument-web-pages) for your application. Important: This is *not* the `site_secret` (as it is in reCAPTCHA v3), but rather your score-based reCAPTCHA Enterprise site key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access recaptcha_enterprise_config outputs
recaptcha_enterprise_config_id = recaptcha_enterprise_config.id
recaptcha_enterprise_config_risk_analysis = recaptcha_enterprise_config.risk_analysis
recaptcha_enterprise_config_token_ttl = recaptcha_enterprise_config.token_ttl
recaptcha_enterprise_config_name = recaptcha_enterprise_config.name
recaptcha_enterprise_config_site_key = recaptcha_enterprise_config.site_key
```

---


### App

Accepts an App Attest CBOR attestation and verifies it with Apple using your preconfigured team and bundle IDs. If valid, returns an attestation artifact that can later be exchanged for an AppCheckToken using ExchangeAppAttestAssertion. For convenience and performance, this method's response object will also contain an AppCheckToken (if the verification is successful).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_id` | String |  | Required. The key ID generated by App Attest for the client app. |
| `attestation_statement` | String |  | Required. The App Attest statement returned by the client-side App Attest API. This is a base64url encoded CBOR object in the JSON response. |
| `challenge` | String |  | Required. A one-time challenge returned by an immediately prior call to GenerateAppAttestChallenge. |
| `limited_use` | bool |  | Specifies whether this attestation is for use in a *limited use* (`true`) or *session based* (`false`) context. To enable this attestation to be used with the *replay protection* feature, set this to `true`. The default value is `false`. |
| `app` | String | ✅ | Required. The relative resource name of the iOS app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app
app = provider.firebaseappcheck_api.App {
    app = "value"  # Required. The relative resource name of the iOS app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
}

```

---


### Safety_net_config

Gets the SafetyNetConfig for the specified app.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The relative resource name of the SafetyNet configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/safetyNetConfig ``` |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from SafetyNet tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `name` | String | ✅ | Required. The relative resource name of the SafetyNet configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/safetyNetConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The relative resource name of the SafetyNet configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/safetyNetConfig ``` |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from SafetyNet tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access safety_net_config outputs
safety_net_config_id = safety_net_config.id
safety_net_config_name = safety_net_config.name
safety_net_config_token_ttl = safety_net_config.token_ttl
```

---


### Service

Atomically updates the specified Service configurations.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. The request messages specifying the Services to update. A maximum of 100 objects can be updated in a batch. |
| `update_mask` | String |  | Optional. A comma-separated list of names of fields in the Services to update. Example: `display_name`. If the `update_mask` field is set in both this request and any of the UpdateServiceRequest messages, they must match or the entire batch fails and no updates will be committed. |
| `parent` | String | ✅ | Required. The parent project name shared by all Service configurations being updated, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being updated must match this field, or the entire batch fails. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enforcement_mode` | String | Required. The App Check enforcement mode for this service. |
| `name` | String | Required. The relative resource name of the service configuration object, in the format: ``` projects/{project_number}/services/{service_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `firebasestorage.googleapis.com` (Cloud Storage for Firebase) * `firebasedatabase.googleapis.com` (Firebase Realtime Database) * `firestore.googleapis.com` (Cloud Firestore) * `oauth2.googleapis.com` (Google Identity for iOS) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.firebaseappcheck_api.Service {
    parent = "value"  # Required. The parent project name shared by all Service configurations being updated, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being updated must match this field, or the entire batch fails.
}

# Access service outputs
service_id = service.id
service_enforcement_mode = service.enforcement_mode
service_name = service.name
```

---


### Resource_policie

Creates the specified ResourcePolicy configuration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enforcement_mode` | String |  | Required. The App Check enforcement mode for this resource. This will override the EnforcementMode setting on the service. |
| `name` | String |  | Required. Identifier. The relative name of the resource policy object, in the format: ``` projects/{project_number}/services/{service_id}/resourcePolicies/{resource_policy_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `oauth2.googleapis.com` (Google Identity for iOS) `resource_policy_id` is a system-generated UID. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. This etag is strongly validated as defined by RFC 7232. |
| `update_time` | String |  | Output only. Timestamp when this resource policy configuration object was most recently updated. |
| `target_resource` | String |  | Required. Service specific name of the resource object to which this policy applies, in the format: * **iOS OAuth clients** (Google Identity for iOS): `//oauth2.googleapis.com/projects/{project_number}/oauthClients/{oauth_client_id}` Note that the resource must belong to the service specified in the `name` and be from the same project as this policy, but the resource is allowed to be missing at the time of creation of this policy; in that case, we make a best-effort attempt at respecting this policy, but it may not have any effect until the resource is fully created. |
| `parent` | String | ✅ | Required. The relative resource name of the parent Service in which the specified ResourcePolicy will be created, in the format: ``` projects/{project_number}/services/{service_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `oauth2.googleapis.com` (Google Identity for iOS) |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enforcement_mode` | String | Required. The App Check enforcement mode for this resource. This will override the EnforcementMode setting on the service. |
| `name` | String | Required. Identifier. The relative name of the resource policy object, in the format: ``` projects/{project_number}/services/{service_id}/resourcePolicies/{resource_policy_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `oauth2.googleapis.com` (Google Identity for iOS) `resource_policy_id` is a system-generated UID. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. This etag is strongly validated as defined by RFC 7232. |
| `update_time` | String | Output only. Timestamp when this resource policy configuration object was most recently updated. |
| `target_resource` | String | Required. Service specific name of the resource object to which this policy applies, in the format: * **iOS OAuth clients** (Google Identity for iOS): `//oauth2.googleapis.com/projects/{project_number}/oauthClients/{oauth_client_id}` Note that the resource must belong to the service specified in the `name` and be from the same project as this policy, but the resource is allowed to be missing at the time of creation of this policy; in that case, we make a best-effort attempt at respecting this policy, but it may not have any effect until the resource is fully created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create resource_policie
resource_policie = provider.firebaseappcheck_api.Resource_policie {
    parent = "value"  # Required. The relative resource name of the parent Service in which the specified ResourcePolicy will be created, in the format: ``` projects/{project_number}/services/{service_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `oauth2.googleapis.com` (Google Identity for iOS)
}

# Access resource_policie outputs
resource_policie_id = resource_policie.id
resource_policie_enforcement_mode = resource_policie.enforcement_mode
resource_policie_name = resource_policie.name
resource_policie_etag = resource_policie.etag
resource_policie_update_time = resource_policie.update_time
resource_policie_target_resource = resource_policie.target_resource
```

---


### Debug_token

Creates a new DebugToken for the specified app. For security reasons, after the creation operation completes, the `token` field cannot be updated or retrieved, but you can revoke the debug token using DeleteDebugToken. Each app can have a maximum of 20 debug tokens.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The relative resource name of the debug token, in the format: ``` projects/{project_number}/apps/{app_id}/debugTokens/{debug_token_id} ``` |
| `update_time` | String |  | Output only. Timestamp when this debug token was most recently updated. |
| `display_name` | String |  | Required. A human readable display name used to identify this debug token. |
| `token` | String |  | Required. Input only. Immutable. The secret token itself. Must be provided during creation, and must be a UUID4, case insensitive. This field is immutable once set, and cannot be provided during an UpdateDebugToken request. You can, however, delete this debug token using DeleteDebugToken to revoke it. For security reasons, this field will never be populated in any response. |
| `parent` | String | ✅ | Required. The relative resource name of the parent app in which the specified DebugToken will be created, in the format: ``` projects/{project_number}/apps/{app_id} ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The relative resource name of the debug token, in the format: ``` projects/{project_number}/apps/{app_id}/debugTokens/{debug_token_id} ``` |
| `update_time` | String | Output only. Timestamp when this debug token was most recently updated. |
| `display_name` | String | Required. A human readable display name used to identify this debug token. |
| `token` | String | Required. Input only. Immutable. The secret token itself. Must be provided during creation, and must be a UUID4, case insensitive. This field is immutable once set, and cannot be provided during an UpdateDebugToken request. You can, however, delete this debug token using DeleteDebugToken to revoke it. For security reasons, this field will never be populated in any response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create debug_token
debug_token = provider.firebaseappcheck_api.Debug_token {
    parent = "value"  # Required. The relative resource name of the parent app in which the specified DebugToken will be created, in the format: ``` projects/{project_number}/apps/{app_id} ```
}

# Access debug_token outputs
debug_token_id = debug_token.id
debug_token_name = debug_token.name
debug_token_update_time = debug_token.update_time
debug_token_display_name = debug_token.display_name
debug_token_token = debug_token.token
```

---


### Jwk

Returns a public JWK set as specified by [RFC 7517](https://tools.ietf.org/html/rfc7517) that can be used to verify App Check tokens. Exactly one of the public keys in the returned set will successfully validate any App Check token that is currently valid.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `keys` | Vec<String> | The set of public keys. See [section 5.1 of RFC 7517](https://tools.ietf.org/html/rfc7517#section-5). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access jwk outputs
jwk_id = jwk.id
jwk_keys = jwk.keys
```

---


### App_attest_config

Gets the AppAttestConfig for the specified app.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The relative resource name of the App Attest configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/appAttestConfig ``` |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from App Attest artifacts will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `name` | String | ✅ | Required. The relative resource name of the App Attest configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/appAttestConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The relative resource name of the App Attest configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/appAttestConfig ``` |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from App Attest artifacts will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access app_attest_config outputs
app_attest_config_id = app_attest_config.id
app_attest_config_name = app_attest_config.name
app_attest_config_token_ttl = app_attest_config.token_ttl
```

---


### Device_check_config

Gets the DeviceCheckConfig for the specified app. For security reasons, the `private_key` field is never populated in the response.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `private_key` | String |  | Required. Input only. The contents of the private key (`.p8`) file associated with the key specified by `key_id`. For security reasons, this field will never be populated in any response. |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from DeviceCheck tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `private_key_set` | bool |  | Output only. Whether the `private_key` field was previously set. Since we will never return the `private_key` field, this field is the only way to find out whether it was previously set. |
| `name` | String |  | Required. The relative resource name of the DeviceCheck configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/deviceCheckConfig ``` |
| `key_id` | String |  | Required. The key identifier of a private key enabled with DeviceCheck, created in your Apple Developer account. |
| `name` | String | ✅ | Required. The relative resource name of the DeviceCheck configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/deviceCheckConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `private_key` | String | Required. Input only. The contents of the private key (`.p8`) file associated with the key specified by `key_id`. For security reasons, this field will never be populated in any response. |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from DeviceCheck tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `private_key_set` | bool | Output only. Whether the `private_key` field was previously set. Since we will never return the `private_key` field, this field is the only way to find out whether it was previously set. |
| `name` | String | Required. The relative resource name of the DeviceCheck configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/deviceCheckConfig ``` |
| `key_id` | String | Required. The key identifier of a private key enabled with DeviceCheck, created in your Apple Developer account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access device_check_config outputs
device_check_config_id = device_check_config.id
device_check_config_private_key = device_check_config.private_key
device_check_config_token_ttl = device_check_config.token_ttl
device_check_config_private_key_set = device_check_config.private_key_set
device_check_config_name = device_check_config.name
device_check_config_key_id = device_check_config.key_id
```

---


### Recaptcha_v3_config

Gets the RecaptchaV3Config for the specified app. For security reasons, the `site_secret` field is never populated in the response.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `min_valid_score` | f64 |  | Specifies a minimum score required for a reCAPTCHA token to be considered valid. If its score is greater than or equal to this value, it will be accepted; otherwise, it will be rejected. The value must be between 0.0 and 1.0. The default value is 0.5. |
| `site_secret_set` | bool |  | Output only. Whether the `site_secret` field was previously set. Since we will never return the `site_secret` field, this field is the only way to find out whether it was previously set. |
| `name` | String |  | Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaV3Config ``` |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from reCAPTCHA tokens will be valid. If unset, a default value of 1 day is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `site_secret` | String |  | Required. Input only. The site secret used to identify your service for reCAPTCHA v3 verification. For security reasons, this field will never be populated in any response. |
| `name` | String | ✅ | Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaV3Config ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `min_valid_score` | f64 | Specifies a minimum score required for a reCAPTCHA token to be considered valid. If its score is greater than or equal to this value, it will be accepted; otherwise, it will be rejected. The value must be between 0.0 and 1.0. The default value is 0.5. |
| `site_secret_set` | bool | Output only. Whether the `site_secret` field was previously set. Since we will never return the `site_secret` field, this field is the only way to find out whether it was previously set. |
| `name` | String | Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaV3Config ``` |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from reCAPTCHA tokens will be valid. If unset, a default value of 1 day is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `site_secret` | String | Required. Input only. The site secret used to identify your service for reCAPTCHA v3 verification. For security reasons, this field will never be populated in any response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access recaptcha_v3_config outputs
recaptcha_v3_config_id = recaptcha_v3_config.id
recaptcha_v3_config_min_valid_score = recaptcha_v3_config.min_valid_score
recaptcha_v3_config_site_secret_set = recaptcha_v3_config.site_secret_set
recaptcha_v3_config_name = recaptcha_v3_config.name
recaptcha_v3_config_token_ttl = recaptcha_v3_config.token_ttl
recaptcha_v3_config_site_secret = recaptcha_v3_config.site_secret
```

---


### Project

Verifies the given App Check token and returns token usage signals that callers may act upon. This method currently only supports App Check tokens exchanged from the following attestation providers: * Play Integrity API * App Attest * DeviceCheck (`DCDevice` tokens) * reCAPTCHA Enterprise * reCAPTCHA v3 * Custom providers App Check tokens exchanged from debug secrets are also supported. Calling this method on an otherwise valid App Check token with an unsupported provider will cause an HTTP 400 error to be returned. Returns whether this token was already consumed before this call. If this is the first time this method has seen the given App Check token, the field `already_consumed` in the response will be absent. The given token will then be marked as `already_consumed` (set to `true`) for all future invocations of this method for that token. Note that if the given App Check token is invalid, an HTTP 403 error is returned instead of a response object, regardless whether the token was already consumed. Currently, when evaluating whether an App Check token was already consumed, only calls to this exact method are counted. Use of the App Check token elsewhere will not mark the token as being already consumed. The caller must have the [`firebaseappcheck.appCheckTokens.verify`](https://firebase.google.com/docs/projects/iam/permissions#app-check) permission to call this method. This permission is part of the [Firebase App Check Token Verifier role](https://firebase.google.com/docs/projects/iam/roles-predefined-product#app-check).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_check_token` | String |  | Required. The App Check token to verify. App Check tokens exchanged from the SafetyNet provider are not supported; an HTTP 400 error will be returned. |
| `project` | String | ✅ | Required. The relative resource name of the project for which the token was minted, in the format: ``` projects/{project_number} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.firebaseappcheck_api.Project {
    project = "value"  # Required. The relative resource name of the project for which the token was minted, in the format: ``` projects/{project_number} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
}

```

---


### Recaptcha_v3_config

Gets the RecaptchaV3Config for the specified app. For security reasons, the `site_secret` field is never populated in the response.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `site_secret_set` | bool |  | Output only. Whether the `site_secret` field was previously set. Since we will never return the `site_secret` field, this field is the only way to find out whether it was previously set. |
| `min_valid_score` | f64 |  | Specifies a minimum score required for a reCAPTCHA token to be considered valid. If its score is greater than or equal to this value, it will be accepted; otherwise, it will be rejected. The value must be between 0.0 and 1.0. The default value is 0.5. |
| `name` | String |  | Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaV3Config ``` |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from reCAPTCHA tokens will be valid. If unset, a default value of 1 day is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `site_secret` | String |  | Required. Input only. The site secret used to identify your service for reCAPTCHA v3 verification. For security reasons, this field will never be populated in any response. |
| `name` | String | ✅ | Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaV3Config ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `site_secret_set` | bool | Output only. Whether the `site_secret` field was previously set. Since we will never return the `site_secret` field, this field is the only way to find out whether it was previously set. |
| `min_valid_score` | f64 | Specifies a minimum score required for a reCAPTCHA token to be considered valid. If its score is greater than or equal to this value, it will be accepted; otherwise, it will be rejected. The value must be between 0.0 and 1.0. The default value is 0.5. |
| `name` | String | Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaV3Config ``` |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from reCAPTCHA tokens will be valid. If unset, a default value of 1 day is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `site_secret` | String | Required. Input only. The site secret used to identify your service for reCAPTCHA v3 verification. For security reasons, this field will never be populated in any response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access recaptcha_v3_config outputs
recaptcha_v3_config_id = recaptcha_v3_config.id
recaptcha_v3_config_site_secret_set = recaptcha_v3_config.site_secret_set
recaptcha_v3_config_min_valid_score = recaptcha_v3_config.min_valid_score
recaptcha_v3_config_name = recaptcha_v3_config.name
recaptcha_v3_config_token_ttl = recaptcha_v3_config.token_ttl
recaptcha_v3_config_site_secret = recaptcha_v3_config.site_secret
```

---


### Resource_policie

Creates the specified ResourcePolicy configuration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enforcement_mode` | String |  | Required. The App Check enforcement mode for this resource. This will override the EnforcementMode setting on the parent service. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. This etag is strongly validated as defined by RFC 7232. |
| `target_resource` | String |  | Required. Service specific name of the resource object to which this policy applies, in the format: * **iOS OAuth clients** (Google Identity for iOS): `//oauth2.googleapis.com/projects/{project_number}/oauthClients/{oauth_client_id}` Note that the resource must belong to the service specified in the `name` and be from the same project as this policy, but the resource is allowed to be missing at the time of creation of this policy; in that case, we make a best-effort attempt at respecting this policy, but it may not have any effect until the resource is fully created. |
| `update_time` | String |  | Output only. Timestamp when this resource policy configuration object was most recently updated. |
| `name` | String |  | Required. Identifier. The relative name of the resource policy object, in the format: ``` projects/{project_number}/services/{service_id}/resourcePolicies/{resource_policy_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `oauth2.googleapis.com` (Google Identity for iOS) `resource_policy_id` is a system-generated UID. |
| `parent` | String | ✅ | Required. The relative resource name of the parent Service in which the specified ResourcePolicy will be created, in the format: ``` projects/{project_number}/services/{service_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `oauth2.googleapis.com` (Google Identity for iOS) |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enforcement_mode` | String | Required. The App Check enforcement mode for this resource. This will override the EnforcementMode setting on the parent service. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. This etag is strongly validated as defined by RFC 7232. |
| `target_resource` | String | Required. Service specific name of the resource object to which this policy applies, in the format: * **iOS OAuth clients** (Google Identity for iOS): `//oauth2.googleapis.com/projects/{project_number}/oauthClients/{oauth_client_id}` Note that the resource must belong to the service specified in the `name` and be from the same project as this policy, but the resource is allowed to be missing at the time of creation of this policy; in that case, we make a best-effort attempt at respecting this policy, but it may not have any effect until the resource is fully created. |
| `update_time` | String | Output only. Timestamp when this resource policy configuration object was most recently updated. |
| `name` | String | Required. Identifier. The relative name of the resource policy object, in the format: ``` projects/{project_number}/services/{service_id}/resourcePolicies/{resource_policy_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `oauth2.googleapis.com` (Google Identity for iOS) `resource_policy_id` is a system-generated UID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create resource_policie
resource_policie = provider.firebaseappcheck_api.Resource_policie {
    parent = "value"  # Required. The relative resource name of the parent Service in which the specified ResourcePolicy will be created, in the format: ``` projects/{project_number}/services/{service_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `oauth2.googleapis.com` (Google Identity for iOS)
}

# Access resource_policie outputs
resource_policie_id = resource_policie.id
resource_policie_enforcement_mode = resource_policie.enforcement_mode
resource_policie_etag = resource_policie.etag
resource_policie_target_resource = resource_policie.target_resource
resource_policie_update_time = resource_policie.update_time
resource_policie_name = resource_policie.name
```

---


### App

Generates a challenge that protects the integrity of an immediately following call to ExchangeAppAttestAttestation or ExchangeAppAttestAssertion. A challenge should not be reused for multiple calls.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app` | String | ✅ | Required. The relative resource name of the iOS app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard. Alternatively, if this method is being called for an OAuth client protected by App Check, this field can also be in the format: ``` oauthClients/{oauth_client_id} ``` You can view the OAuth client ID for your OAuth clients in the Google Cloud console. Note that only iOS OAuth clients are supported at this time, and they must be linked to corresponding iOS Firebase apps. Please see [the documentation](https://developers.google.com/identity/sign-in/ios/appcheck/get-started#project-setup) for more information. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app
app = provider.firebaseappcheck_api.App {
    app = "value"  # Required. The relative resource name of the iOS app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard. Alternatively, if this method is being called for an OAuth client protected by App Check, this field can also be in the format: ``` oauthClients/{oauth_client_id} ``` You can view the OAuth client ID for your OAuth clients in the Google Cloud console. Note that only iOS OAuth clients are supported at this time, and they must be linked to corresponding iOS Firebase apps. Please see [the documentation](https://developers.google.com/identity/sign-in/ios/appcheck/get-started#project-setup) for more information.
}

```

---


### Service

Atomically updates the specified Service configurations.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. The request messages specifying the Services to update. A maximum of 100 objects can be updated in a batch. |
| `update_mask` | String |  | Optional. A comma-separated list of names of fields in the Services to update. Example: `display_name`. If the `update_mask` field is set in both this request and any of the UpdateServiceRequest messages, they must match or the entire batch fails and no updates will be committed. |
| `parent` | String | ✅ | Required. The parent project name shared by all Service configurations being updated, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being updated must match this field, or the entire batch fails. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enforcement_mode` | String | Required. The App Check enforcement mode for this service. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. This etag is strongly validated as defined by RFC 7232. |
| `name` | String | Required. The relative resource name of the service configuration object, in the format: ``` projects/{project_number}/services/{service_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `firebasestorage.googleapis.com` (Cloud Storage for Firebase) * `firebasedatabase.googleapis.com` (Firebase Realtime Database) * `firestore.googleapis.com` (Cloud Firestore) * `identitytoolkit.googleapis.com` (Firebase Authentication with Identity Platform) * `oauth2.googleapis.com` (Google Identity for iOS) |
| `update_time` | String | Output only. Timestamp when this service configuration object was most recently updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.firebaseappcheck_api.Service {
    parent = "value"  # Required. The parent project name shared by all Service configurations being updated, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being updated must match this field, or the entire batch fails.
}

# Access service outputs
service_id = service.id
service_enforcement_mode = service.enforcement_mode
service_etag = service.etag
service_name = service.name
service_update_time = service.update_time
```

---


### Play_integrity_config

Gets the PlayIntegrityConfig for the specified app.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_details` | String |  | Specifies account requirements for Android devices running your app. These settings correspond to requirements on the [**account details** field](https://developer.android.com/google/play/integrity/verdicts#account-details-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. The default values for these settings work for most apps, and are recommended. |
| `device_integrity` | String |  | Specifies device integrity requirements for Android devices running your app. These settings correspond to requirements on the [**device integrity** field](https://developer.android.com/google/play/integrity/verdicts#device-integrity-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. Warning: There are also [conditional](https://developer.android.com/google/play/integrity/setup#conditional) as well as [optional](https://developer.android.com/google/play/integrity/setup#optional_device_information) responses that you can receive, but requires additional explicit opt-in from you. The App Check API is **not** responsible for any such opt-ins. The default values for these settings work for most apps, and are recommended. |
| `name` | String |  | Required. The relative resource name of the Play Integrity configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/playIntegrityConfig ``` |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from Play Integrity tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `app_integrity` | String |  | Specifies application integrity requirements for Android devices running your app. These settings correspond to requirements on the [**application integrity** field](https://developer.android.com/google/play/integrity/verdicts#application-integrity-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. The default values for these settings work for most apps, and are recommended. |
| `name` | String | ✅ | Required. The relative resource name of the Play Integrity configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/playIntegrityConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_details` | String | Specifies account requirements for Android devices running your app. These settings correspond to requirements on the [**account details** field](https://developer.android.com/google/play/integrity/verdicts#account-details-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. The default values for these settings work for most apps, and are recommended. |
| `device_integrity` | String | Specifies device integrity requirements for Android devices running your app. These settings correspond to requirements on the [**device integrity** field](https://developer.android.com/google/play/integrity/verdicts#device-integrity-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. Warning: There are also [conditional](https://developer.android.com/google/play/integrity/setup#conditional) as well as [optional](https://developer.android.com/google/play/integrity/setup#optional_device_information) responses that you can receive, but requires additional explicit opt-in from you. The App Check API is **not** responsible for any such opt-ins. The default values for these settings work for most apps, and are recommended. |
| `name` | String | Required. The relative resource name of the Play Integrity configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/playIntegrityConfig ``` |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from Play Integrity tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `app_integrity` | String | Specifies application integrity requirements for Android devices running your app. These settings correspond to requirements on the [**application integrity** field](https://developer.android.com/google/play/integrity/verdicts#application-integrity-field) obtained from the Play Integrity API. See the [default responses table](https://developer.android.com/google/play/integrity/setup#default) for a quick summary. The default values for these settings work for most apps, and are recommended. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access play_integrity_config outputs
play_integrity_config_id = play_integrity_config.id
play_integrity_config_account_details = play_integrity_config.account_details
play_integrity_config_device_integrity = play_integrity_config.device_integrity
play_integrity_config_name = play_integrity_config.name
play_integrity_config_token_ttl = play_integrity_config.token_ttl
play_integrity_config_app_integrity = play_integrity_config.app_integrity
```

---


### Recaptcha_config

Gets the RecaptchaConfig for the specified app. For security reasons, the `site_secret` field is never populated in the response.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `min_valid_score` | f64 |  | Specifies a minimum score required for a reCAPTCHA token to be considered valid. If its score is greater than or equal to this value, it will be accepted; otherwise, it will be rejected. The value must be between 0.0 and 1.0. The default value is 0.5. |
| `site_secret` | String |  | Required. Input only. The site secret used to identify your service for reCAPTCHA v3 verification. For security reasons, this field will never be populated in any response. |
| `site_secret_set` | bool |  | Output only. Whether the `site_secret` field was previously set. Since we will never return the `site_secret` field, this field is the only way to find out whether it was previously set. |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from reCAPTCHA tokens will be valid. If unset, a default value of 1 day is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `name` | String |  | Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaConfig ``` |
| `name` | String | ✅ | Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `min_valid_score` | f64 | Specifies a minimum score required for a reCAPTCHA token to be considered valid. If its score is greater than or equal to this value, it will be accepted; otherwise, it will be rejected. The value must be between 0.0 and 1.0. The default value is 0.5. |
| `site_secret` | String | Required. Input only. The site secret used to identify your service for reCAPTCHA v3 verification. For security reasons, this field will never be populated in any response. |
| `site_secret_set` | bool | Output only. Whether the `site_secret` field was previously set. Since we will never return the `site_secret` field, this field is the only way to find out whether it was previously set. |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from reCAPTCHA tokens will be valid. If unset, a default value of 1 day is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `name` | String | Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaConfig ``` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access recaptcha_config outputs
recaptcha_config_id = recaptcha_config.id
recaptcha_config_min_valid_score = recaptcha_config.min_valid_score
recaptcha_config_site_secret = recaptcha_config.site_secret
recaptcha_config_site_secret_set = recaptcha_config.site_secret_set
recaptcha_config_token_ttl = recaptcha_config.token_ttl
recaptcha_config_name = recaptcha_config.name
```

---


### Jwk

Returns a public JWK set as specified by [RFC 7517](https://tools.ietf.org/html/rfc7517) that can be used to verify App Check tokens. Exactly one of the public keys in the returned set will successfully validate any App Check token that is currently valid.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `keys` | Vec<String> | The set of public keys. See [section 5.1 of RFC 7517](https://tools.ietf.org/html/rfc7517#section-5). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access jwk outputs
jwk_id = jwk.id
jwk_keys = jwk.keys
```

---


### Device_check_config

Gets the DeviceCheckConfig for the specified app. For security reasons, the `private_key` field is never populated in the response.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The relative resource name of the DeviceCheck configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/deviceCheckConfig ``` |
| `key_id` | String |  | Required. The key identifier of a private key enabled with DeviceCheck, created in your Apple Developer account. |
| `private_key_set` | bool |  | Output only. Whether the `private_key` field was previously set. Since we will never return the `private_key` field, this field is the only way to find out whether it was previously set. |
| `private_key` | String |  | Required. Input only. The contents of the private key (`.p8`) file associated with the key specified by `key_id`. For security reasons, this field will never be populated in any response. |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from DeviceCheck tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `name` | String | ✅ | Required. The relative resource name of the DeviceCheck configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/deviceCheckConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The relative resource name of the DeviceCheck configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/deviceCheckConfig ``` |
| `key_id` | String | Required. The key identifier of a private key enabled with DeviceCheck, created in your Apple Developer account. |
| `private_key_set` | bool | Output only. Whether the `private_key` field was previously set. Since we will never return the `private_key` field, this field is the only way to find out whether it was previously set. |
| `private_key` | String | Required. Input only. The contents of the private key (`.p8`) file associated with the key specified by `key_id`. For security reasons, this field will never be populated in any response. |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from DeviceCheck tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access device_check_config outputs
device_check_config_id = device_check_config.id
device_check_config_name = device_check_config.name
device_check_config_key_id = device_check_config.key_id
device_check_config_private_key_set = device_check_config.private_key_set
device_check_config_private_key = device_check_config.private_key
device_check_config_token_ttl = device_check_config.token_ttl
```

---


### Debug_token

Creates a new DebugToken for the specified app. For security reasons, after the creation operation completes, the `token` field cannot be updated or retrieved, but you can revoke the debug token using DeleteDebugToken. Each app can have a maximum of 20 debug tokens.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. A human readable display name used to identify this debug token. |
| `token` | String |  | Required. Input only. Immutable. The secret token itself. Must be provided during creation, and must be a UUID4, case insensitive. This field is immutable once set, and cannot be provided during an UpdateDebugToken request. You can, however, delete this debug token using DeleteDebugToken to revoke it. For security reasons, this field will never be populated in any response. |
| `update_time` | String |  | Output only. Timestamp when this debug token was most recently updated. |
| `name` | String |  | Required. The relative resource name of the debug token, in the format: ``` projects/{project_number}/apps/{app_id}/debugTokens/{debug_token_id} ``` |
| `parent` | String | ✅ | Required. The relative resource name of the parent app in which the specified DebugToken will be created, in the format: ``` projects/{project_number}/apps/{app_id} ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. A human readable display name used to identify this debug token. |
| `token` | String | Required. Input only. Immutable. The secret token itself. Must be provided during creation, and must be a UUID4, case insensitive. This field is immutable once set, and cannot be provided during an UpdateDebugToken request. You can, however, delete this debug token using DeleteDebugToken to revoke it. For security reasons, this field will never be populated in any response. |
| `update_time` | String | Output only. Timestamp when this debug token was most recently updated. |
| `name` | String | Required. The relative resource name of the debug token, in the format: ``` projects/{project_number}/apps/{app_id}/debugTokens/{debug_token_id} ``` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create debug_token
debug_token = provider.firebaseappcheck_api.Debug_token {
    parent = "value"  # Required. The relative resource name of the parent app in which the specified DebugToken will be created, in the format: ``` projects/{project_number}/apps/{app_id} ```
}

# Access debug_token outputs
debug_token_id = debug_token.id
debug_token_display_name = debug_token.display_name
debug_token_token = debug_token.token
debug_token_update_time = debug_token.update_time
debug_token_name = debug_token.name
```

---


### Safety_net_config

Gets the SafetyNetConfig for the specified app.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The relative resource name of the SafetyNet configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/safetyNetConfig ``` |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from SafetyNet tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `name` | String | ✅ | Required. The relative resource name of the SafetyNet configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/safetyNetConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The relative resource name of the SafetyNet configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/safetyNetConfig ``` |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from SafetyNet tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access safety_net_config outputs
safety_net_config_id = safety_net_config.id
safety_net_config_name = safety_net_config.name
safety_net_config_token_ttl = safety_net_config.token_ttl
```

---


### App_attest_config

Gets the AppAttestConfig for the specified app.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The relative resource name of the App Attest configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/appAttestConfig ``` |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from App Attest artifacts will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `name` | String | ✅ | Required. The relative resource name of the App Attest configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/appAttestConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The relative resource name of the App Attest configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/appAttestConfig ``` |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from App Attest artifacts will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access app_attest_config outputs
app_attest_config_id = app_attest_config.id
app_attest_config_name = app_attest_config.name
app_attest_config_token_ttl = app_attest_config.token_ttl
```

---


### Recaptcha_enterprise_config

Gets the RecaptchaEnterpriseConfig for the specified app.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The relative resource name of the reCAPTCHA Enterprise configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaEnterpriseConfig ``` |
| `risk_analysis` | String |  | Specifies risk tolerance and requirements for your application. These settings correspond to requirements on the [**`riskAnalysis`**](https://cloud.google.com/recaptcha/docs/interpret-assessment-website#interpret_assessment) tuple in the assessment obtained from reCAPTCHA Enterprise. The default values for these settings work for most apps, and are recommended. |
| `token_ttl` | String |  | Specifies the duration for which App Check tokens exchanged from reCAPTCHA Enterprise tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `site_key` | String |  | The score-based site key [created in reCAPTCHA Enterprise](https://cloud.google.com/recaptcha-enterprise/docs/create-key#creating_a_site_key) used to [invoke reCAPTCHA and generate the reCAPTCHA tokens](https://cloud.google.com/recaptcha-enterprise/docs/instrument-web-pages) for your application. Important: This is *not* the `site_secret` (as it is in reCAPTCHA v3), but rather your score-based reCAPTCHA Enterprise site key. |
| `name` | String | ✅ | Required. The relative resource name of the reCAPTCHA Enterprise configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaEnterpriseConfig ``` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The relative resource name of the reCAPTCHA Enterprise configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaEnterpriseConfig ``` |
| `risk_analysis` | String | Specifies risk tolerance and requirements for your application. These settings correspond to requirements on the [**`riskAnalysis`**](https://cloud.google.com/recaptcha/docs/interpret-assessment-website#interpret_assessment) tuple in the assessment obtained from reCAPTCHA Enterprise. The default values for these settings work for most apps, and are recommended. |
| `token_ttl` | String | Specifies the duration for which App Check tokens exchanged from reCAPTCHA Enterprise tokens will be valid. If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive. |
| `site_key` | String | The score-based site key [created in reCAPTCHA Enterprise](https://cloud.google.com/recaptcha-enterprise/docs/create-key#creating_a_site_key) used to [invoke reCAPTCHA and generate the reCAPTCHA tokens](https://cloud.google.com/recaptcha-enterprise/docs/instrument-web-pages) for your application. Important: This is *not* the `site_secret` (as it is in reCAPTCHA v3), but rather your score-based reCAPTCHA Enterprise site key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access recaptcha_enterprise_config outputs
recaptcha_enterprise_config_id = recaptcha_enterprise_config.id
recaptcha_enterprise_config_name = recaptcha_enterprise_config.name
recaptcha_enterprise_config_risk_analysis = recaptcha_enterprise_config.risk_analysis
recaptcha_enterprise_config_token_ttl = recaptcha_enterprise_config.token_ttl
recaptcha_enterprise_config_site_key = recaptcha_enterprise_config.site_key
```

---


### Oauth_client

Validates a debug token secret that you have previously created using CreateDebugToken. If valid, returns an AppCheckToken. Note that a restrictive quota is enforced on this method to prevent accidental exposure of the app to abuse.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `debug_token` | String |  | Required. A debug token secret. This string must match a debug token secret previously created using CreateDebugToken. |
| `limited_use` | bool |  | Specifies whether this attestation is for use in a *limited use* (`true`) or *session based* (`false`) context. To enable this attestation to be used with the *replay protection* feature, set this to `true`. The default value is `false`. |
| `app` | String | ✅ | Required. The relative resource name of the app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard. Alternatively, if this method is being called for an OAuth client protected by App Check, this field can also be in the format: ``` oauthClients/{oauth_client_id} ``` You can view the OAuth client ID for your OAuth clients in the Google Cloud console. Note that only iOS OAuth clients are supported at this time, and they must be linked to corresponding iOS Firebase apps. Please see [the documentation](https://developers.google.com/identity/sign-in/ios/appcheck/get-started#project-setup) for more information. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create oauth_client
oauth_client = provider.firebaseappcheck_api.Oauth_client {
    app = "value"  # Required. The relative resource name of the app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard. Alternatively, if this method is being called for an OAuth client protected by App Check, this field can also be in the format: ``` oauthClients/{oauth_client_id} ``` You can view the OAuth client ID for your OAuth clients in the Google Cloud console. Note that only iOS OAuth clients are supported at this time, and they must be linked to corresponding iOS Firebase apps. Please see [the documentation](https://developers.google.com/identity/sign-in/ios/appcheck/get-started#project-setup) for more information.
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple play_integrity_config resources
play_integrity_config_0 = provider.firebaseappcheck_api.Play_integrity_config {
    name = "value-0"
}
play_integrity_config_1 = provider.firebaseappcheck_api.Play_integrity_config {
    name = "value-1"
}
play_integrity_config_2 = provider.firebaseappcheck_api.Play_integrity_config {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    play_integrity_config = provider.firebaseappcheck_api.Play_integrity_config {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebaseappcheck_api Documentation](https://cloud.google.com/firebaseappcheck_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
