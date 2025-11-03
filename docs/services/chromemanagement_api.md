# Chromemanagement_api Service



**Resources**: 14

---

## Overview

The chromemanagement_api service provides access to 14 resource types:

- [Profile](#profile) [RD]
- [Device](#device) [R]
- [Third_party_profile_user](#third_party_profile_user) [C]
- [Chrome](#chrome) [R]
- [Operation](#operation) [CRD]
- [Event](#event) [R]
- [User](#user) [R]
- [Certificate_provisioning_processe](#certificate_provisioning_processe) [CR]
- [Command](#command) [CR]
- [Notification_config](#notification_config) [CRD]
- [App](#app) [R]
- [Report](#report) [R]
- [Android](#android) [R]
- [Web](#web) [R]

---

## Resources


### Profile

Gets a Chrome browser profile with customer ID and profile permanent ID.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `browser_version` | String | Output only. Version of the browser on which the profile exists. |
| `display_name` | String | Output only. Profile display name set by client. |
| `annotated_user` | String | Optional. User of the profile annotated by the admin. |
| `last_status_report_time` | String | Output only. Timestamp of the latest status report by the profile. |
| `supports_fcm_notifications` | bool | Output only. Whether the profile supports FCM notifications. |
| `user_id` | String | Output only. Unique Directory API ID of the user that can be used in Admin SDK Users API. |
| `policy_count` | String | Output only. Number of policies applied on the profile. |
| `profile_id` | String | Output only. Chrome client side profile ID. |
| `profile_permanent_id` | String | Output only. Profile permanent ID is the unique identifier of a profile within one customer. |
| `device_info` | String | Output only. Basic information of the device on which the profile exists. This information is only available for the affiliated profiles. |
| `os_platform_type` | String | Output only. OS platform of the device on which the profile exists. |
| `reporting_data` | String | Output only. Detailed reporting data of the profile. This information is only available when the profile reporting policy is enabled. |
| `user_email` | String | Output only. Email address of the user to which the profile belongs. |
| `attestation_credential` | String | Output only. Attestation credential information of the profile. |
| `last_policy_sync_time` | String | Output only. Timestamp of the latest policy sync by the profile. |
| `last_policy_fetch_time` | String | Output only. Timestamp of the latest policy fetch by the profile. |
| `name` | String | Identifier. Format: customers/{customer_id}/profiles/{profile_permanent_id} |
| `os_platform_version` | String | Output only. Major OS platform version of the device on which the profile exists, from profile reporting. |
| `os_version` | String | Output only. OS version of the device on which the profile exists. |
| `last_activity_time` | String | Output only. Timestamp of the latest activity by the profile. |
| `first_enrollment_time` | String | Output only. Timestamp of the first enrollment of the profile. |
| `affiliation_state` | String | Output only. The specific affiliation state of the profile. |
| `browser_channel` | String | Output only. Channel of the browser on which the profile exists. |
| `annotated_location` | String | Optional. Location of the profile annotated by the admin. |
| `identity_provider` | String | Output only. Identify provider of the profile. |
| `etag` | String | Output only. Etag of this ChromeBrowserProfile resource. This etag can be used with UPDATE operation to ensure consistency. |
| `extension_count` | String | Output only. Number of extensions installed on the profile. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access profile outputs
profile_id = profile.id
profile_browser_version = profile.browser_version
profile_display_name = profile.display_name
profile_annotated_user = profile.annotated_user
profile_last_status_report_time = profile.last_status_report_time
profile_supports_fcm_notifications = profile.supports_fcm_notifications
profile_user_id = profile.user_id
profile_policy_count = profile.policy_count
profile_profile_id = profile.profile_id
profile_profile_permanent_id = profile.profile_permanent_id
profile_device_info = profile.device_info
profile_os_platform_type = profile.os_platform_type
profile_reporting_data = profile.reporting_data
profile_user_email = profile.user_email
profile_attestation_credential = profile.attestation_credential
profile_last_policy_sync_time = profile.last_policy_sync_time
profile_last_policy_fetch_time = profile.last_policy_fetch_time
profile_name = profile.name
profile_os_platform_version = profile.os_platform_version
profile_os_version = profile.os_version
profile_last_activity_time = profile.last_activity_time
profile_first_enrollment_time = profile.first_enrollment_time
profile_affiliation_state = profile.affiliation_state
profile_browser_channel = profile.browser_channel
profile_annotated_location = profile.annotated_location
profile_identity_provider = profile.identity_provider
profile_etag = profile.etag
profile_extension_count = profile.extension_count
```

---


### Device

Get telemetry device.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `os_update_status` | Vec<String> | Output only. Contains relevant information regarding ChromeOS update status. |
| `memory_status_report` | Vec<String> | Output only. Memory status reports collected periodically sorted decreasing by report_time. |
| `cpu_status_report` | Vec<String> | Output only. CPU status reports collected periodically sorted in a decreasing order of report_time. |
| `network_bandwidth_report` | Vec<String> | Output only. Network bandwidth reports collected periodically sorted in a decreasing order of report_time. |
| `audio_status_report` | Vec<String> | Output only. Audio reports collected periodically sorted in a decreasing order of report_time. |
| `storage_info` | String | Output only. Information of storage specs for the device. |
| `storage_status_report` | Vec<String> | Output only. Storage reports collected periodically. |
| `serial_number` | String | Output only. Device serial number. This value is the same as the Admin Console's Serial Number in the ChromeOS Devices tab. |
| `network_status_report` | Vec<String> | Output only. Network specs collected periodically. |
| `boot_performance_report` | Vec<String> | Output only. Boot performance reports of the device. |
| `org_unit_id` | String | Output only. Organization unit ID of the device. |
| `app_report` | Vec<String> | Output only. App reports collected periodically sorted in a decreasing order of report_time. |
| `device_id` | String | Output only. The unique Directory API ID of the device. This value is the same as the Admin Console's Directory API ID in the ChromeOS Devices tab |
| `battery_status_report` | Vec<String> | Output only. Battery reports collected periodically. |
| `battery_info` | Vec<String> | Output only. Information on battery specs for the device. |
| `network_diagnostics_report` | Vec<String> | Output only. Network diagnostics collected periodically. |
| `name` | String | Output only. Resource name of the device. |
| `cpu_info` | Vec<String> | Output only. Information regarding CPU specs for the device. |
| `heartbeat_status_report` | Vec<String> | Output only. Heartbeat status report containing timestamps periodically sorted in decreasing order of report_time |
| `network_info` | String | Output only. Network devices information. |
| `peripherals_report` | Vec<String> | Output only. Peripherals reports collected periodically sorted in a decreasing order of report_time. |
| `graphics_info` | String | Output only. Contains information regarding Graphic peripherals for the device. |
| `thunderbolt_info` | Vec<String> | Output only. Information on Thunderbolt bus. |
| `memory_info` | String | Output only. Information regarding memory specs for the device. |
| `customer` | String | Output only. Google Workspace Customer whose enterprise enrolled the device. |
| `runtime_counters_report` | Vec<String> | Output only. Runtime counters reports collected device lifetime runtime, as well as the counts of S0->S3, S0->S4, and S0->S5 transitions, meaning entering into sleep, hibernation, and power-off states |
| `graphics_status_report` | Vec<String> | Output only. Graphics reports collected periodically. |
| `kiosk_app_status_report` | Vec<String> | Output only. Kiosk app status report for the kiosk device |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access device outputs
device_id = device.id
device_os_update_status = device.os_update_status
device_memory_status_report = device.memory_status_report
device_cpu_status_report = device.cpu_status_report
device_network_bandwidth_report = device.network_bandwidth_report
device_audio_status_report = device.audio_status_report
device_storage_info = device.storage_info
device_storage_status_report = device.storage_status_report
device_serial_number = device.serial_number
device_network_status_report = device.network_status_report
device_boot_performance_report = device.boot_performance_report
device_org_unit_id = device.org_unit_id
device_app_report = device.app_report
device_device_id = device.device_id
device_battery_status_report = device.battery_status_report
device_battery_info = device.battery_info
device_network_diagnostics_report = device.network_diagnostics_report
device_name = device.name
device_cpu_info = device.cpu_info
device_heartbeat_status_report = device.heartbeat_status_report
device_network_info = device.network_info
device_peripherals_report = device.peripherals_report
device_graphics_info = device.graphics_info
device_thunderbolt_info = device.thunderbolt_info
device_memory_info = device.memory_info
device_customer = device.customer
device_runtime_counters_report = device.runtime_counters_report
device_graphics_status_report = device.graphics_status_report
device_kiosk_app_status_report = device.kiosk_app_status_report
```

---


### Third_party_profile_user

Moves a third party chrome profile user to a destination OU. All profiles associated to that user will be moved to the destination OU.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_org_unit` | String |  | Required. Destination organizational unit where the third party chrome profile user will be moved to. |
| `name` | String | ✅ | Required. Format: customers/{customer_id}/thirdPartyProfileUsers/{third_party_profile_user_id} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create third_party_profile_user
third_party_profile_user = provider.chromemanagement_api.Third_party_profile_user {
    name = "value"  # Required. Format: customers/{customer_id}/thirdPartyProfileUsers/{third_party_profile_user_id}
}

```

---


### Chrome

Get a specific app for a customer by its resource name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `publisher` | String | Output only. The publisher of the item. |
| `service_error` | String | Output only. Information about a partial service error if applicable. |
| `first_publish_time` | String | Output only. First published time. |
| `latest_publish_time` | String | Output only. Latest published time. |
| `revision_id` | String | Output only. App version. A new revision is committed whenever a new version of the app is published. |
| `review_rating` | f64 | Output only. The rating of the app (on 5 stars). Chrome Web Store review information will always be for the latest version of an app. |
| `type` | String | Output only. App type. |
| `description` | String | Output only. App's description. |
| `name` | String | Output only. Format: name=customers/{customer_id}/apps/{chrome|android|web}/{app_id}@{version} |
| `android_app_info` | String | Output only. Android app information. |
| `app_id` | String | Output only. Unique store identifier for the item. Examples: "gmbmikajjgmnabiglmofipeabaddhgne" for the Save to Google Drive Chrome extension, "com.google.android.apps.docs" for the Google Drive Android app. |
| `icon_uri` | String | Output only. A link to an image that can be used as an icon for the product. |
| `chrome_app_info` | String | Output only. Chrome Web Store app information. |
| `is_paid_app` | bool | Output only. Indicates if the app has to be paid for OR has paid content. |
| `display_name` | String | Output only. App's display name. |
| `privacy_policy_uri` | String | Output only. The URI pointing to the privacy policy of the app, if it was provided by the developer. Version-specific field that will only be set when the requested app version is found. |
| `detail_uri` | String | Output only. The uri for the detail page of the item. |
| `review_number` | String | Output only. Number of reviews received. Chrome Web Store review information will always be for the latest version of an app. |
| `homepage_uri` | String | Output only. Home page or Website uri. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access chrome outputs
chrome_id = chrome.id
chrome_publisher = chrome.publisher
chrome_service_error = chrome.service_error
chrome_first_publish_time = chrome.first_publish_time
chrome_latest_publish_time = chrome.latest_publish_time
chrome_revision_id = chrome.revision_id
chrome_review_rating = chrome.review_rating
chrome_type = chrome.type
chrome_description = chrome.description
chrome_name = chrome.name
chrome_android_app_info = chrome.android_app_info
chrome_app_id = chrome.app_id
chrome_icon_uri = chrome.icon_uri
chrome_chrome_app_info = chrome.chrome_app_info
chrome_is_paid_app = chrome.is_paid_app
chrome_display_name = chrome.display_name
chrome_privacy_policy_uri = chrome.privacy_policy_uri
chrome_detail_uri = chrome.detail_uri
chrome_review_number = chrome.review_number
chrome_homepage_uri = chrome.homepage_uri
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.chromemanagement_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_name = operation.name
operation_error = operation.error
operation_done = operation.done
operation_response = operation.response
```

---


### Event

List telemetry events.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to specify next page in the list. |
| `telemetry_events` | Vec<String> | Telemetry events returned in the response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access event outputs
event_id = event.id
event_next_page_token = event.next_page_token
event_telemetry_events = event.telemetry_events
```

---


### User

Get telemetry user.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name of the user. |
| `org_unit_id` | String | Organization unit of the user. |
| `customer` | String | G Suite Customer whose enterprise enrolled the device. |
| `user_device` | Vec<String> | Telemetry data collected from a managed user and device. |
| `user_email` | String | Email address of the user. |
| `user_id` | String | Directory ID of the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user outputs
user_id = user.id
user_name = user.name
user_org_unit_id = user.org_unit_id
user_customer = user.customer
user_user_device = user.user_device
user_user_email = user.user_email
user_user_id = user.user_id
```

---


### Certificate_provisioning_processe

Uploads a successfully issued certificate for a certificate provisioning process.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_pem` | String |  | Required. The issued certificate in PEM format. |
| `name` | String | ✅ | Required. Resource name of the `CertificateProvisioningProcess` to return. The name pattern is given as `customers/{customer}/certificateProvisioningProcesses/{certificate_provisioning_process}` with `{customer}` being the obfuscated customer id and `{certificate_provisioning_process}` being the certificate provisioning process id. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `generic_ca_connection` | String | Output only. The CA connection is a generic CA connection. |
| `signature` | String | Output only. The signature of `signature_algorithm`, generated using the client's private key using `signature_algorithm`. This field is only present after the `SignData` operation has finished. |
| `name` | String | Identifier. Resource name of the `CertificateProvisioningProcess`. The name pattern is given as `customers/{customer}/certificateProvisioningProcesses/{certificate_provisioning_process}` with `{customer}` being the obfuscated customer id and `{certificate_provisioning_process}` being the certificate provisioning process id. |
| `sign_data` | String | Output only. The data that the client was asked to sign. This field is only present after the `SignData` operation has been initiated. |
| `subject_public_key_info` | String | Output only. The public key for which a certificate should be provisioned. Represented as a DER-encoded X.509 SubjectPublicKeyInfo. |
| `chrome_os_user_session` | String | Output only. The client certificate is being provisioned for a ChromeOS user. This contains information about the current user session. |
| `failure_message` | String | Output only. A message describing why this `CertificateProvisioningProcess` has failed. Presence of this field indicates that the `CertificateProvisioningProcess` has failed. |
| `start_time` | String | Output only. Server-generated timestamp of when the certificate provisioning process has been created. |
| `provisioning_profile_id` | String | Output only. The ID of the certificate provisioning profile. |
| `scep_ca_connection` | String | Output only. The CA connection is a SCEP CA connection. |
| `chrome_os_device` | String | Output only. The client certificate is being provisioned for a ChromeOS device. This contains information about the device. |
| `generic_profile` | String | Output only. The profile is a generic certificate provisioning profile. |
| `scep_profile` | String | Output only. The profile is a SCEP certificate provisioning profile. |
| `issued_certificate` | String | Output only. The issued certificate for this `CertificateProvisioningProcess` in PEM format. |
| `signature_algorithm` | String | Output only. The signature algorithm that the client and backend components use when processing `sign_data`. If the `profile_type` is a `GenericProfile`, this field will only be present after the `SignData` operation was initiated. If the `profile_type` is a `ScepProfile`, the field will always be present. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate_provisioning_processe
certificate_provisioning_processe = provider.chromemanagement_api.Certificate_provisioning_processe {
    name = "value"  # Required. Resource name of the `CertificateProvisioningProcess` to return. The name pattern is given as `customers/{customer}/certificateProvisioningProcesses/{certificate_provisioning_process}` with `{customer}` being the obfuscated customer id and `{certificate_provisioning_process}` being the certificate provisioning process id.
}

# Access certificate_provisioning_processe outputs
certificate_provisioning_processe_id = certificate_provisioning_processe.id
certificate_provisioning_processe_generic_ca_connection = certificate_provisioning_processe.generic_ca_connection
certificate_provisioning_processe_signature = certificate_provisioning_processe.signature
certificate_provisioning_processe_name = certificate_provisioning_processe.name
certificate_provisioning_processe_sign_data = certificate_provisioning_processe.sign_data
certificate_provisioning_processe_subject_public_key_info = certificate_provisioning_processe.subject_public_key_info
certificate_provisioning_processe_chrome_os_user_session = certificate_provisioning_processe.chrome_os_user_session
certificate_provisioning_processe_failure_message = certificate_provisioning_processe.failure_message
certificate_provisioning_processe_start_time = certificate_provisioning_processe.start_time
certificate_provisioning_processe_provisioning_profile_id = certificate_provisioning_processe.provisioning_profile_id
certificate_provisioning_processe_scep_ca_connection = certificate_provisioning_processe.scep_ca_connection
certificate_provisioning_processe_chrome_os_device = certificate_provisioning_processe.chrome_os_device
certificate_provisioning_processe_generic_profile = certificate_provisioning_processe.generic_profile
certificate_provisioning_processe_scep_profile = certificate_provisioning_processe.scep_profile
certificate_provisioning_processe_issued_certificate = certificate_provisioning_processe.issued_certificate
certificate_provisioning_processe_signature_algorithm = certificate_provisioning_processe.signature_algorithm
```

---


### Command

Creates a Chrome browser profile remote command.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `payload` | HashMap<String, String> |  | Required. Payload of the remote command. The payload for "clearBrowsingData" command supports: - fields "clearCache" and "clearCookies" - values of boolean type. |
| `valid_duration` | String |  | Output only. Valid duration of the remote command. |
| `command_type` | String |  | Required. Type of the remote command. The only supported command_type is "clearBrowsingData". |
| `issue_time` | String |  | Output only. Timestamp of the issurance of the remote command. |
| `command_state` | String |  | Output only. State of the remote command. |
| `name` | String |  | Identifier. Format: customers/{customer_id}/profiles/{profile_permanent_id}/commands/{command_id} |
| `command_result` | String |  | Output only. Result of the remote command. |
| `parent` | String | ✅ | Required. Format: customers/{customer_id}/profiles/{profile_permanent_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `payload` | HashMap<String, String> | Required. Payload of the remote command. The payload for "clearBrowsingData" command supports: - fields "clearCache" and "clearCookies" - values of boolean type. |
| `valid_duration` | String | Output only. Valid duration of the remote command. |
| `command_type` | String | Required. Type of the remote command. The only supported command_type is "clearBrowsingData". |
| `issue_time` | String | Output only. Timestamp of the issurance of the remote command. |
| `command_state` | String | Output only. State of the remote command. |
| `name` | String | Identifier. Format: customers/{customer_id}/profiles/{profile_permanent_id}/commands/{command_id} |
| `command_result` | String | Output only. Result of the remote command. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create command
command = provider.chromemanagement_api.Command {
    parent = "value"  # Required. Format: customers/{customer_id}/profiles/{profile_permanent_id}
}

# Access command outputs
command_id = command.id
command_payload = command.payload
command_valid_duration = command.valid_duration
command_command_type = command.command_type
command_issue_time = command.issue_time
command_command_state = command.command_state
command_name = command.name
command_command_result = command.command_result
```

---


### Notification_config

Create a telemetry notification config.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Output only. Google Workspace customer that owns the resource. |
| `filter` | String |  | Only send notifications for telemetry data matching this filter. |
| `name` | String |  | Output only. Resource name of the notification configuration. |
| `google_cloud_pubsub_topic` | String |  | The pubsub topic to which notifications are published to. |
| `parent` | String | ✅ | Required. The parent resource where this notification config will be created. Format: `customers/{customer}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `telemetry_notification_configs` | Vec<String> | The telemetry notification configs from the specified customer. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notification_config
notification_config = provider.chromemanagement_api.Notification_config {
    parent = "value"  # Required. The parent resource where this notification config will be created. Format: `customers/{customer}`
}

# Access notification_config outputs
notification_config_id = notification_config.id
notification_config_next_page_token = notification_config.next_page_token
notification_config_telemetry_notification_configs = notification_config.telemetry_notification_configs
```

---


### App

Generate summary of app installation requests.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to specify the next page in the list. |
| `requested_apps` | Vec<String> | Count of requested apps matching request. |
| `total_size` | i64 | Total number of matching app requests. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access app outputs
app_id = app.id
app_next_page_token = app.next_page_token
app_requested_apps = app.requested_apps
app_total_size = app.total_size
```

---


### Report

Counts of devices with a specific hardware specification from the requested hardware type (for example model name, processor type). Further information can be found here https://support.google.com/chrome/a/answer/10564947

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_reports` | Vec<String> | The DeviceHardwareCountReport for device model type (for example Acer C7 Chromebook). |
| `cpu_reports` | Vec<String> | The DeviceHardwareCountReport for device cpu type (for example Intel(R) Core(TM) i7-10610U CPU @ 1.80GHz). |
| `storage_reports` | Vec<String> | The DeviceHardwareCountReport for device storage amount in gigabytes (for example 128). |
| `memory_reports` | Vec<String> | The DeviceHardwareCountReport for device memory amount in gigabytes (for example 16). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access report outputs
report_id = report.id
report_model_reports = report.model_reports
report_cpu_reports = report.cpu_reports
report_storage_reports = report.storage_reports
report_memory_reports = report.memory_reports
```

---


### Android

Get a specific app for a customer by its resource name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `publisher` | String | Output only. The publisher of the item. |
| `service_error` | String | Output only. Information about a partial service error if applicable. |
| `first_publish_time` | String | Output only. First published time. |
| `latest_publish_time` | String | Output only. Latest published time. |
| `revision_id` | String | Output only. App version. A new revision is committed whenever a new version of the app is published. |
| `review_rating` | f64 | Output only. The rating of the app (on 5 stars). Chrome Web Store review information will always be for the latest version of an app. |
| `type` | String | Output only. App type. |
| `description` | String | Output only. App's description. |
| `name` | String | Output only. Format: name=customers/{customer_id}/apps/{chrome|android|web}/{app_id}@{version} |
| `android_app_info` | String | Output only. Android app information. |
| `app_id` | String | Output only. Unique store identifier for the item. Examples: "gmbmikajjgmnabiglmofipeabaddhgne" for the Save to Google Drive Chrome extension, "com.google.android.apps.docs" for the Google Drive Android app. |
| `icon_uri` | String | Output only. A link to an image that can be used as an icon for the product. |
| `chrome_app_info` | String | Output only. Chrome Web Store app information. |
| `is_paid_app` | bool | Output only. Indicates if the app has to be paid for OR has paid content. |
| `display_name` | String | Output only. App's display name. |
| `privacy_policy_uri` | String | Output only. The URI pointing to the privacy policy of the app, if it was provided by the developer. Version-specific field that will only be set when the requested app version is found. |
| `detail_uri` | String | Output only. The uri for the detail page of the item. |
| `review_number` | String | Output only. Number of reviews received. Chrome Web Store review information will always be for the latest version of an app. |
| `homepage_uri` | String | Output only. Home page or Website uri. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access android outputs
android_id = android.id
android_publisher = android.publisher
android_service_error = android.service_error
android_first_publish_time = android.first_publish_time
android_latest_publish_time = android.latest_publish_time
android_revision_id = android.revision_id
android_review_rating = android.review_rating
android_type = android.type
android_description = android.description
android_name = android.name
android_android_app_info = android.android_app_info
android_app_id = android.app_id
android_icon_uri = android.icon_uri
android_chrome_app_info = android.chrome_app_info
android_is_paid_app = android.is_paid_app
android_display_name = android.display_name
android_privacy_policy_uri = android.privacy_policy_uri
android_detail_uri = android.detail_uri
android_review_number = android.review_number
android_homepage_uri = android.homepage_uri
```

---


### Web

Get a specific app for a customer by its resource name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `publisher` | String | Output only. The publisher of the item. |
| `service_error` | String | Output only. Information about a partial service error if applicable. |
| `first_publish_time` | String | Output only. First published time. |
| `latest_publish_time` | String | Output only. Latest published time. |
| `revision_id` | String | Output only. App version. A new revision is committed whenever a new version of the app is published. |
| `review_rating` | f64 | Output only. The rating of the app (on 5 stars). Chrome Web Store review information will always be for the latest version of an app. |
| `type` | String | Output only. App type. |
| `description` | String | Output only. App's description. |
| `name` | String | Output only. Format: name=customers/{customer_id}/apps/{chrome|android|web}/{app_id}@{version} |
| `android_app_info` | String | Output only. Android app information. |
| `app_id` | String | Output only. Unique store identifier for the item. Examples: "gmbmikajjgmnabiglmofipeabaddhgne" for the Save to Google Drive Chrome extension, "com.google.android.apps.docs" for the Google Drive Android app. |
| `icon_uri` | String | Output only. A link to an image that can be used as an icon for the product. |
| `chrome_app_info` | String | Output only. Chrome Web Store app information. |
| `is_paid_app` | bool | Output only. Indicates if the app has to be paid for OR has paid content. |
| `display_name` | String | Output only. App's display name. |
| `privacy_policy_uri` | String | Output only. The URI pointing to the privacy policy of the app, if it was provided by the developer. Version-specific field that will only be set when the requested app version is found. |
| `detail_uri` | String | Output only. The uri for the detail page of the item. |
| `review_number` | String | Output only. Number of reviews received. Chrome Web Store review information will always be for the latest version of an app. |
| `homepage_uri` | String | Output only. Home page or Website uri. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access web outputs
web_id = web.id
web_publisher = web.publisher
web_service_error = web.service_error
web_first_publish_time = web.first_publish_time
web_latest_publish_time = web.latest_publish_time
web_revision_id = web.revision_id
web_review_rating = web.review_rating
web_type = web.type
web_description = web.description
web_name = web.name
web_android_app_info = web.android_app_info
web_app_id = web.app_id
web_icon_uri = web.icon_uri
web_chrome_app_info = web.chrome_app_info
web_is_paid_app = web.is_paid_app
web_display_name = web.display_name
web_privacy_policy_uri = web.privacy_policy_uri
web_detail_uri = web.detail_uri
web_review_number = web.review_number
web_homepage_uri = web.homepage_uri
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple profile resources
profile_0 = provider.chromemanagement_api.Profile {
}
profile_1 = provider.chromemanagement_api.Profile {
}
profile_2 = provider.chromemanagement_api.Profile {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    profile = provider.chromemanagement_api.Profile {
    }
```

---

## Related Documentation

- [GCP Chromemanagement_api Documentation](https://cloud.google.com/chromemanagement_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
