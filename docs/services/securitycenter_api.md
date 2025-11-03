# Securitycenter_api Service



**Resources**: 38

---

## Overview

The securitycenter_api service provides access to 38 resource types:

- [Asset](#asset) [CRU]
- [Notification_config](#notification_config) [CRUD]
- [External_system](#external_system) [U]
- [Operation](#operation) [CRD]
- [Effective_custom_module](#effective_custom_module) [R]
- [Finding](#finding) [CRU]
- [Organization](#organization) [RU]
- [Attack_path](#attack_path) [R]
- [Big_query_export](#big_query_export) [CRUD]
- [Event_threat_detection_setting](#event_threat_detection_setting) [C]
- [Valued_resource](#valued_resource) [R]
- [Custom_module](#custom_module) [CRUD]
- [Simulation](#simulation) [R]
- [Source](#source) [CRU]
- [Mute_config](#mute_config) [CRUD]
- [Resource_value_config](#resource_value_config) [CRUD]
- [Operation](#operation) [CRD]
- [Cluster](#cluster) [RU]
- [Virtual_machine_threat_detection_setting](#virtual_machine_threat_detection_setting) [R]
- [Project](#project) [RU]
- [Organization](#organization) [RU]
- [Rapid_vulnerability_detection_setting](#rapid_vulnerability_detection_setting) [R]
- [Folder](#folder) [RU]
- [Event_threat_detection_setting](#event_threat_detection_setting) [R]
- [Web_security_scanner_setting](#web_security_scanner_setting) [R]
- [Security_health_analytics_setting](#security_health_analytics_setting) [R]
- [Container_threat_detection_setting](#container_threat_detection_setting) [R]
- [Organization](#organization) [RU]
- [Finding](#finding) [CRU]
- [Source](#source) [CRU]
- [Operation](#operation) [CRD]
- [Asset](#asset) [CRU]
- [Finding](#finding) [CRU]
- [Asset](#asset) [CRU]
- [Notification_config](#notification_config) [CRUD]
- [Source](#source) [CRU]
- [Operation](#operation) [CRD]
- [Organization](#organization) [RU]

---

## Resources


### Asset

Filters an organization's assets and groups them by their specified properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `read_time` | String |  | Time used as a reference point when filtering assets. The filter is limited to assets existing at the supplied time and their values are those at that specific time. Absence of this field will default to the API's version of NOW. |
| `group_by` | String |  | Required. Expression that defines what assets fields to use for grouping. The string value should follow SQL syntax: comma separated list of fields. For example: "security_center_properties.resource_project,security_center_properties.project". The following fields are supported when compare_duration is not set: * security_center_properties.resource_project * security_center_properties.resource_project_display_name * security_center_properties.resource_type * security_center_properties.resource_parent * security_center_properties.resource_parent_display_name The following fields are supported when compare_duration is set: * security_center_properties.resource_type * security_center_properties.resource_project_display_name * security_center_properties.resource_parent_display_name |
| `page_size` | i64 |  | The maximum number of results to return in a single response. Default is 10, minimum is 1, maximum is 1000. |
| `page_token` | String |  | The value returned by the last `GroupAssetsResponse`; indicates that this is a continuation of a prior `GroupAssets` call, and that the system should return the next page of data. |
| `filter` | String |  | Expression that defines the filter to apply across assets. The expression is a list of zero or more restrictions combined via logical operators `AND` and `OR`. Parentheses are supported, and `OR` has higher precedence than `AND`. Restrictions have the form ` ` and may have a `-` character in front of them to indicate negation. The fields map to those defined in the Asset resource. Examples include: * name * security_center_properties.resource_name * resource_properties.a_property * security_marks.marks.marka The supported operators are: * `=` for all value types. * `>`, `<`, `>=`, `<=` for integer values. * `:`, meaning substring matching, for strings. The supported value types are: * string literals in quotes. * integer literals without quotes. * boolean literals `true` and `false` without quotes. The following field and operator combinations are supported: * name: `=` * update_time: `=`, `>`, `<`, `>=`, `<=` Usage: This should be milliseconds since epoch or an RFC3339 string. Examples: `update_time = "2019-06-10T16:07:18-07:00"` `update_time = 1560208038000` * create_time: `=`, `>`, `<`, `>=`, `<=` Usage: This should be milliseconds since epoch or an RFC3339 string. Examples: `create_time = "2019-06-10T16:07:18-07:00"` `create_time = 1560208038000` * iam_policy.policy_blob: `=`, `:` * resource_properties: `=`, `:`, `>`, `<`, `>=`, `<=` * security_marks.marks: `=`, `:` * security_center_properties.resource_name: `=`, `:` * security_center_properties.resource_display_name: `=`, `:` * security_center_properties.resource_type: `=`, `:` * security_center_properties.resource_parent: `=`, `:` * security_center_properties.resource_parent_display_name: `=`, `:` * security_center_properties.resource_project: `=`, `:` * security_center_properties.resource_project_display_name: `=`, `:` * security_center_properties.resource_owners: `=`, `:` For example, `resource_properties.size = 100` is a valid filter string. Use a partial match on the empty string to filter based on a property existing: `resource_properties.my_property : ""` Use a negated partial match on the empty string to filter based on a property not existing: `-resource_properties.my_property : ""` |
| `compare_duration` | String |  | When compare_duration is set, the GroupResult's "state_change" property is updated to indicate whether the asset was added, removed, or remained present during the compare_duration period of time that precedes the read_time. This is the time between (read_time - compare_duration) and read_time. The state change value is derived based on the presence of the asset at the two points in time. Intermediate state changes between the two times don't affect the result. For example, the results aren't affected if the asset is removed and re-created again. Possible "state_change" values when compare_duration is specified: * "ADDED": indicates that the asset was not present at the start of compare_duration, but present at reference_time. * "REMOVED": indicates that the asset was present at the start of compare_duration, but not present at reference_time. * "ACTIVE": indicates that the asset was present at both the start and the end of the time period defined by compare_duration and reference_time. If compare_duration is not specified, then the only possible state_change is "UNUSED", which will be the state_change set for all assets present at read_time. If this field is set then `state_change` must be a specified field in `group_by`. |
| `parent` | String | ✅ | Required. The name of the parent to group the assets by. Its format is `organizations/[organization_id]`, `folders/[folder_id]`, or `projects/[project_id]`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results. |
| `total_size` | i64 | The total number of assets matching the query. |
| `read_time` | String | Time used for executing the list request. |
| `list_assets_results` | Vec<String> | Assets matching the list request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create asset
asset = provider.securitycenter_api.Asset {
    parent = "value"  # Required. The name of the parent to group the assets by. Its format is `organizations/[organization_id]`, `folders/[folder_id]`, or `projects/[project_id]`.
}

# Access asset outputs
asset_id = asset.id
asset_next_page_token = asset.next_page_token
asset_total_size = asset.total_size
asset_read_time = asset.read_time
asset_list_assets_results = asset.list_assets_results
```

---


### Notification_config

Creates a notification config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `streaming_config` | String |  | The config for triggering streaming-based notifications. |
| `pubsub_topic` | String |  | The Pub/Sub topic to send notifications to. Its format is "projects/[project_id]/topics/[topic]". |
| `description` | String |  | The description of the notification config (max of 1024 characters). |
| `name` | String |  | The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket", "folders/{folder_id}/notificationConfigs/notify_public_bucket", or "projects/{project_id}/notificationConfigs/notify_public_bucket". |
| `service_account` | String |  | Output only. The service account that needs "pubsub.topics.publish" permission to publish to the Pub/Sub topic. |
| `parent` | String | ✅ | Required. Resource name of the new notification config's parent. Its format is `organizations/[organization_id]`, `folders/[folder_id]`, or `projects/[project_id]`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `streaming_config` | String | The config for triggering streaming-based notifications. |
| `pubsub_topic` | String | The Pub/Sub topic to send notifications to. Its format is "projects/[project_id]/topics/[topic]". |
| `description` | String | The description of the notification config (max of 1024 characters). |
| `name` | String | The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket", "folders/{folder_id}/notificationConfigs/notify_public_bucket", or "projects/{project_id}/notificationConfigs/notify_public_bucket". |
| `service_account` | String | Output only. The service account that needs "pubsub.topics.publish" permission to publish to the Pub/Sub topic. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notification_config
notification_config = provider.securitycenter_api.Notification_config {
    parent = "value"  # Required. Resource name of the new notification config's parent. Its format is `organizations/[organization_id]`, `folders/[folder_id]`, or `projects/[project_id]`.
}

# Access notification_config outputs
notification_config_id = notification_config.id
notification_config_streaming_config = notification_config.streaming_config
notification_config_pubsub_topic = notification_config.pubsub_topic
notification_config_description = notification_config.description
notification_config_name = notification_config.name
notification_config_service_account = notification_config.service_account
```

---


### External_system



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `case_sla` | String |  | The SLA of the finding's corresponding case in the external system. |
| `case_create_time` | String |  | The time when the case was created, as reported by the external system. |
| `case_close_time` | String |  | The time when the case was closed, as reported by the external system. |
| `case_priority` | String |  | The priority of the finding's corresponding case in the external system. |
| `case_uri` | String |  | The link to the finding's corresponding case in the external system. |
| `external_uid` | String |  | The identifier that's used to track the finding's corresponding case in the external system. |
| `name` | String |  | Full resource name of the external system, for example: "organizations/1234/sources/5678/findings/123456/externalSystems/jira", "folders/1234/sources/5678/findings/123456/externalSystems/jira", "projects/1234/sources/5678/findings/123456/externalSystems/jira" |
| `ticket_info` | String |  | Information about the ticket, if any, that is being used to track the resolution of the issue that is identified by this finding. |
| `assignees` | Vec<String> |  | References primary/secondary etc assignees in the external system. |
| `external_system_update_time` | String |  | The time when the case was last updated, as reported by the external system. |
| `status` | String |  | The most recent status of the finding's corresponding case, as reported by the external system. |
| `name` | String | ✅ | Full resource name of the external system, for example: "organizations/1234/sources/5678/findings/123456/externalSystems/jira", "folders/1234/sources/5678/findings/123456/externalSystems/jira", "projects/1234/sources/5678/findings/123456/externalSystems/jira" |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.securitycenter_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_metadata = operation.metadata
operation_response = operation.response
operation_name = operation.name
operation_done = operation.done
```

---


### Effective_custom_module

Retrieves an EffectiveSecurityHealthAnalyticsCustomModule.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_provider` | String | The cloud provider of the custom module. |
| `enablement_state` | String | Output only. The effective state of enablement for the module at the given level of the hierarchy. |
| `name` | String | Output only. The resource name of the custom module. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}", or "folders/{folder}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}" |
| `custom_config` | String | Output only. The user-specified configuration for the module. |
| `display_name` | String | Output only. The display name for the custom module. The name must be between 1 and 128 characters, start with a lowercase letter, and contain alphanumeric characters or underscores only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access effective_custom_module outputs
effective_custom_module_id = effective_custom_module.id
effective_custom_module_cloud_provider = effective_custom_module.cloud_provider
effective_custom_module_enablement_state = effective_custom_module.enablement_state
effective_custom_module_name = effective_custom_module.name
effective_custom_module_custom_config = effective_custom_module.custom_config
effective_custom_module_display_name = effective_custom_module.display_name
```

---


### Finding

Creates a finding. The corresponding source must exist for finding creation to succeed.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `contacts` | HashMap<String, String> |  | Output only. Map containing the points of contact for the given finding. The key represents the type of contact, while the value contains a list of all the contacts that pertain. Please refer to: https://cloud.google.com/resource-manager/docs/managing-notification-contacts#notification-categories { "security": { "contacts": [ { "email": "person1@company.com" }, { "email": "person2@company.com" } ] } } |
| `mute_update_time` | String |  | Output only. The most recent time this finding was muted or unmuted. |
| `database` | String |  | Database associated with the finding. |
| `data_flow_events` | Vec<String> |  | Data flow events associated with the finding. |
| `compliances` | Vec<String> |  | Contains compliance information for security standards associated to the finding. |
| `mute_initiator` | String |  | Records additional information about the mute operation, for example, the [mute configuration](/security-command-center/docs/how-to-mute-findings) that muted the finding and the user who muted the finding. |
| `notebook` | String |  | Notebook associated with the finding. |
| `next_steps` | String |  | Steps to address the finding. |
| `cloud_dlp_inspection` | String |  | Cloud Data Loss Prevention (Cloud DLP) inspection results that are associated with the finding. |
| `attack_exposure` | String |  | The results of an attack path simulation relevant to this finding. |
| `toxic_combination` | String |  | Contains details about a group of security issues that, when the issues occur together, represent a greater risk than when the issues occur independently. A group of such issues is referred to as a toxic combination. This field cannot be updated. Its value is ignored in all update requests. |
| `log_entries` | Vec<String> |  | Log entries that are relevant to the finding. |
| `indicator` | String |  | Represents what's commonly known as an *indicator of compromise* (IoC) in computer forensics. This is an artifact observed on a network or in an operating system that, with high confidence, indicates a computer intrusion. For more information, see [Indicator of compromise](https://en.wikipedia.org/wiki/Indicator_of_compromise). |
| `iam_bindings` | Vec<String> |  | Represents IAM bindings associated with the finding. |
| `source_properties` | HashMap<String, String> |  | Source specific properties. These properties are managed by the source that writes the finding. The key names in the source_properties map must be between 1 and 255 characters, and must start with a letter and contain alphanumeric characters or underscores only. |
| `resource_name` | String |  | For findings on Google Cloud resources, the full resource name of the Google Cloud resource this finding is for. See: https://cloud.google.com/apis/design/resource_names#full_resource_name When the finding is for a non-Google Cloud resource, the resourceName can be a customer or partner defined string. This field is immutable after creation time. |
| `vulnerability` | String |  | Represents vulnerability-specific fields like CVE and CVSS scores. CVE stands for Common Vulnerabilities and Exposures (https://cve.mitre.org/about/) |
| `canonical_name` | String |  | The canonical name of the finding. It's either "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}" or "projects/{project_number}/sources/{source_id}/findings/{finding_id}", depending on the closest CRM ancestor of the resource associated with the finding. |
| `ai_model` | String |  | The AI model associated with the finding. |
| `cloud_dlp_data_profile` | String |  | Cloud DLP data profile that is associated with the finding. |
| `exfiltration` | String |  | Represents exfiltrations associated with the finding. |
| `security_posture` | String |  | The security posture associated with the finding. |
| `category` | String |  | The additional taxonomy group within findings from a given source. This field is immutable after creation time. Example: "XSS_FLASH_INJECTION" |
| `module_name` | String |  | Unique identifier of the module which generated the finding. Example: folders/598186756061/securityHealthAnalyticsSettings/customModules/56799441161885 |
| `networks` | Vec<String> |  | Represents the VPC networks that the resource is attached to. |
| `create_time` | String |  | The time at which the finding was created in Security Command Center. |
| `security_marks` | String |  | Output only. User specified security marks. These marks are entirely managed by the user and come from the SecurityMarks resource that belongs to the finding. |
| `data_access_events` | Vec<String> |  | Data access events associated with the finding. |
| `cloud_armor` | String |  | Fields related to Cloud Armor findings. |
| `compliance_details` | String |  | Details about the compliance implications of the finding. |
| `group_memberships` | Vec<String> |  | Contains details about groups of which this finding is a member. A group is a collection of findings that are related in some way. This field cannot be updated. Its value is ignored in all update requests. |
| `containers` | Vec<String> |  | Containers associated with the finding. This field provides information for both Kubernetes and non-Kubernetes containers. |
| `kubernetes` | String |  | Kubernetes resources associated with the finding. |
| `data_retention_deletion_events` | Vec<String> |  | Data retention deletion events associated with the finding. |
| `disk` | String |  | Disk associated with the finding. |
| `finding_class` | String |  | The class of the finding. |
| `access` | String |  | Access details associated with the finding, such as more information on the caller, which method was accessed, and from where. |
| `affected_resources` | String |  | AffectedResources associated with the finding. |
| `job` | String |  | Job associated with the finding. |
| `mute` | String |  | Indicates the mute state of a finding (either muted, unmuted or undefined). Unlike other attributes of a finding, a finding provider shouldn't set the value of mute. |
| `parent` | String |  | The relative resource name of the source the finding belongs to. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name This field is immutable after creation time. For example: "organizations/{organization_id}/sources/{source_id}" |
| `kernel_rootkit` | String |  | Signature of the kernel rootkit. |
| `mute_info` | String |  | Output only. The mute information regarding this finding. |
| `vertex_ai` | String |  | VertexAi associated with the finding. |
| `external_uri` | String |  | The URI that, if available, points to a web page outside of Security Command Center where additional information about the finding can be found. This field is guaranteed to be either empty or a well formed URL. |
| `ip_rules` | String |  | IP rules associated with the finding. |
| `severity` | String |  | The severity of the finding. This field is managed by the source that writes the finding. |
| `load_balancers` | Vec<String> |  | The load balancers associated with the finding. |
| `mitre_attack` | String |  | MITRE ATT&CK tactics and techniques related to this finding. See: https://attack.mitre.org |
| `backup_disaster_recovery` | String |  | Fields related to Backup and DR findings. |
| `parent_display_name` | String |  | Output only. The human readable display name of the finding source such as "Event Threat Detection" or "Security Health Analytics". |
| `event_time` | String |  | The time the finding was first detected. If an existing finding is updated, then this is the time the update occurred. For example, if the finding represents an open firewall, this property captures the time the detector believes the firewall became open. The accuracy is determined by the detector. If the finding is later resolved, then this time reflects when the finding was resolved. This must not be set to a value greater than the current timestamp. |
| `processes` | Vec<String> |  | Represents operating system processes associated with the Finding. |
| `description` | String |  | Contains more details about the finding. |
| `name` | String |  | The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}". |
| `state` | String |  | The state of the finding. |
| `connections` | Vec<String> |  | Contains information about the IP connection associated with the finding. |
| `external_systems` | HashMap<String, String> |  | Output only. Third party SIEM/SOAR fields within SCC, contains external system information and external system finding fields. |
| `files` | Vec<String> |  | File associated with the finding. |
| `chokepoint` | String |  | Contains details about a chokepoint, which is a resource or resource group where high-risk attack paths converge, based on [attack path simulations] (https://cloud.google.com/security-command-center/docs/attack-exposure-learn#attack_path_simulations). This field cannot be updated. Its value is ignored in all update requests. |
| `org_policies` | Vec<String> |  | Contains information about the org policies associated with the finding. |
| `application` | String |  | Represents an application associated with the finding. |
| `parent` | String | ✅ | Required. Resource name of the new finding's parent. Its format should be `organizations/[organization_id]/sources/[source_id]`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `list_findings_results` | Vec<String> | Findings matching the list request. |
| `read_time` | String | Time used for executing the list request. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results. |
| `total_size` | i64 | The total number of findings matching the query. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create finding
finding = provider.securitycenter_api.Finding {
    parent = "value"  # Required. Resource name of the new finding's parent. Its format should be `organizations/[organization_id]/sources/[source_id]`.
}

# Access finding outputs
finding_id = finding.id
finding_list_findings_results = finding.list_findings_results
finding_read_time = finding.read_time
finding_next_page_token = finding.next_page_token
finding_total_size = finding.total_size
```

---


### Organization

Gets the settings for an organization.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings". |
| `asset_discovery_config` | String |  | The configuration used for Asset Discovery runs. |
| `enable_asset_discovery` | bool |  | A flag that indicates if Asset Discovery should be enabled. If the flag is set to `true`, then discovery of assets will occur. If it is set to `false`, all historical assets will remain, but discovery of future assets will not occur. |
| `name` | String | ✅ | The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings". |
| `asset_discovery_config` | String | The configuration used for Asset Discovery runs. |
| `enable_asset_discovery` | bool | A flag that indicates if Asset Discovery should be enabled. If the flag is set to `true`, then discovery of assets will occur. If it is set to `false`, all historical assets will remain, but discovery of future assets will not occur. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access organization outputs
organization_id = organization.id
organization_name = organization.name
organization_asset_discovery_config = organization.asset_discovery_config
organization_enable_asset_discovery = organization.enable_asset_discovery
```

---


### Attack_path

Lists the attack paths for a set of simulation results or valued resources and filter.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results. |
| `attack_paths` | Vec<String> | The attack paths that the attack path simulation identified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access attack_path outputs
attack_path_id = attack_path.id
attack_path_next_page_token = attack_path.next_page_token
attack_path_attack_paths = attack_path.attack_paths
```

---


### Big_query_export

Creates a BigQuery export.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dataset` | String |  | The dataset to write findings' updates to. Its format is "projects/[project_id]/datasets/[bigquery_dataset_id]". BigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). |
| `most_recent_editor` | String |  | Output only. Email address of the user who last edited the BigQuery export. This field is set by the server and will be ignored if provided on export creation or update. |
| `create_time` | String |  | Output only. The time at which the BigQuery export was created. This field is set by the server and will be ignored if provided on export on creation. |
| `update_time` | String |  | Output only. The most recent time at which the BigQuery export was updated. This field is set by the server and will be ignored if provided on export creation or update. |
| `description` | String |  | The description of the export (max of 1024 characters). |
| `filter` | String |  | Expression that defines the filter to apply across create/update events of findings. The expression is a list of zero or more restrictions combined via logical operators `AND` and `OR`. Parentheses are supported, and `OR` has higher precedence than `AND`. Restrictions have the form ` ` and may have a `-` character in front of them to indicate negation. The fields map to those defined in the corresponding resource. The supported operators are: * `=` for all value types. * `>`, `<`, `>=`, `<=` for integer values. * `:`, meaning substring matching, for strings. The supported value types are: * string literals in quotes. * integer literals without quotes. * boolean literals `true` and `false` without quotes. |
| `principal` | String |  | Output only. The service account that needs permission to create table and upload data to the BigQuery dataset. |
| `name` | String |  | The relative resource name of this export. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name. Example format: "organizations/{organization_id}/bigQueryExports/{export_id}" Example format: "folders/{folder_id}/bigQueryExports/{export_id}" Example format: "projects/{project_id}/bigQueryExports/{export_id}" This field is provided in responses, and is ignored when provided in create requests. |
| `parent` | String | ✅ | Required. The name of the parent resource of the new BigQuery export. Its format is `organizations/[organization_id]`, `folders/[folder_id]`, or `projects/[project_id]`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset` | String | The dataset to write findings' updates to. Its format is "projects/[project_id]/datasets/[bigquery_dataset_id]". BigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). |
| `most_recent_editor` | String | Output only. Email address of the user who last edited the BigQuery export. This field is set by the server and will be ignored if provided on export creation or update. |
| `create_time` | String | Output only. The time at which the BigQuery export was created. This field is set by the server and will be ignored if provided on export on creation. |
| `update_time` | String | Output only. The most recent time at which the BigQuery export was updated. This field is set by the server and will be ignored if provided on export creation or update. |
| `description` | String | The description of the export (max of 1024 characters). |
| `filter` | String | Expression that defines the filter to apply across create/update events of findings. The expression is a list of zero or more restrictions combined via logical operators `AND` and `OR`. Parentheses are supported, and `OR` has higher precedence than `AND`. Restrictions have the form ` ` and may have a `-` character in front of them to indicate negation. The fields map to those defined in the corresponding resource. The supported operators are: * `=` for all value types. * `>`, `<`, `>=`, `<=` for integer values. * `:`, meaning substring matching, for strings. The supported value types are: * string literals in quotes. * integer literals without quotes. * boolean literals `true` and `false` without quotes. |
| `principal` | String | Output only. The service account that needs permission to create table and upload data to the BigQuery dataset. |
| `name` | String | The relative resource name of this export. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name. Example format: "organizations/{organization_id}/bigQueryExports/{export_id}" Example format: "folders/{folder_id}/bigQueryExports/{export_id}" Example format: "projects/{project_id}/bigQueryExports/{export_id}" This field is provided in responses, and is ignored when provided in create requests. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create big_query_export
big_query_export = provider.securitycenter_api.Big_query_export {
    parent = "value"  # Required. The name of the parent resource of the new BigQuery export. Its format is `organizations/[organization_id]`, `folders/[folder_id]`, or `projects/[project_id]`.
}

# Access big_query_export outputs
big_query_export_id = big_query_export.id
big_query_export_dataset = big_query_export.dataset
big_query_export_most_recent_editor = big_query_export.most_recent_editor
big_query_export_create_time = big_query_export.create_time
big_query_export_update_time = big_query_export.update_time
big_query_export_description = big_query_export.description
big_query_export_filter = big_query_export.filter
big_query_export_principal = big_query_export.principal
big_query_export_name = big_query_export.name
```

---


### Event_threat_detection_setting

Validates the given Event Threat Detection custom module.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Required. The type of the module (e.g. CONFIGURABLE_BAD_IP). |
| `raw_text` | String |  | Required. The raw text of the module's contents. Used to generate error messages. |
| `parent` | String | ✅ | Required. Resource name of the parent to validate the Custom Module under. Its format is: * `organizations/{organization}/eventThreatDetectionSettings`. * `folders/{folder}/eventThreatDetectionSettings`. * `projects/{project}/eventThreatDetectionSettings`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create event_threat_detection_setting
event_threat_detection_setting = provider.securitycenter_api.Event_threat_detection_setting {
    parent = "value"  # Required. Resource name of the parent to validate the Custom Module under. Its format is: * `organizations/{organization}/eventThreatDetectionSettings`. * `folders/{folder}/eventThreatDetectionSettings`. * `projects/{project}/eventThreatDetectionSettings`.
}

```

---


### Valued_resource

Get the valued resource by name

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Human-readable name of the valued resource. |
| `resource` | String | The [full resource name](https://cloud.google.com/apis/design/resource_names#full_resource_name) of the valued resource. |
| `name` | String | Valued resource name, for example, e.g.: `organizations/123/simulations/456/valuedResources/789` |
| `exposed_score` | f64 | Exposed score for this valued resource. A value of 0 means no exposure was detected exposure. |
| `resource_type` | String | The [resource type](https://cloud.google.com/asset-inventory/docs/supported-asset-types) of the valued resource. |
| `resource_value` | String | How valuable this resource is. |
| `resource_value_configs_used` | Vec<String> | List of resource value configurations' metadata used to determine the value of this resource. Maximum of 100. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access valued_resource outputs
valued_resource_id = valued_resource.id
valued_resource_display_name = valued_resource.display_name
valued_resource_resource = valued_resource.resource
valued_resource_name = valued_resource.name
valued_resource_exposed_score = valued_resource.exposed_score
valued_resource_resource_type = valued_resource.resource_type
valued_resource_resource_value = valued_resource.resource_value
valued_resource_resource_value_configs_used = valued_resource.resource_value_configs_used
```

---


### Custom_module

Creates a resident SecurityHealthAnalyticsCustomModule at the scope of the given CRM parent, and also creates inherited SecurityHealthAnalyticsCustomModules for all CRM descendants of the given parent. These modules are enabled by default.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The resource name of the custom module. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", or "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}" The id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits. |
| `display_name` | String |  | The display name of the Security Health Analytics custom module. This display name becomes the finding category for all findings that are returned by this custom module. The display name must be between 1 and 128 characters, start with a lowercase letter, and contain alphanumeric characters or underscores only. |
| `custom_config` | String |  | The user specified custom configuration for the module. |
| `cloud_provider` | String |  | The cloud provider of the custom module. |
| `update_time` | String |  | Output only. The time at which the custom module was last updated. |
| `enablement_state` | String |  | The enablement state of the custom module. |
| `ancestor_module` | String |  | Output only. If empty, indicates that the custom module was created in the organization, folder, or project in which you are viewing the custom module. Otherwise, `ancestor_module` specifies the organization or folder from which the custom module is inherited. |
| `last_editor` | String |  | Output only. The editor that last updated the custom module. |
| `parent` | String | ✅ | Required. Resource name of the new custom module's parent. Its format is `organizations/{organization}/securityHealthAnalyticsSettings`, `folders/{folder}/securityHealthAnalyticsSettings`, or `projects/{project}/securityHealthAnalyticsSettings` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The resource name of the custom module. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", or "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}" The id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits. |
| `display_name` | String | The display name of the Security Health Analytics custom module. This display name becomes the finding category for all findings that are returned by this custom module. The display name must be between 1 and 128 characters, start with a lowercase letter, and contain alphanumeric characters or underscores only. |
| `custom_config` | String | The user specified custom configuration for the module. |
| `cloud_provider` | String | The cloud provider of the custom module. |
| `update_time` | String | Output only. The time at which the custom module was last updated. |
| `enablement_state` | String | The enablement state of the custom module. |
| `ancestor_module` | String | Output only. If empty, indicates that the custom module was created in the organization, folder, or project in which you are viewing the custom module. Otherwise, `ancestor_module` specifies the organization or folder from which the custom module is inherited. |
| `last_editor` | String | Output only. The editor that last updated the custom module. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_module
custom_module = provider.securitycenter_api.Custom_module {
    parent = "value"  # Required. Resource name of the new custom module's parent. Its format is `organizations/{organization}/securityHealthAnalyticsSettings`, `folders/{folder}/securityHealthAnalyticsSettings`, or `projects/{project}/securityHealthAnalyticsSettings`
}

# Access custom_module outputs
custom_module_id = custom_module.id
custom_module_name = custom_module.name
custom_module_display_name = custom_module.display_name
custom_module_custom_config = custom_module.custom_config
custom_module_cloud_provider = custom_module.cloud_provider
custom_module_update_time = custom_module.update_time
custom_module_enablement_state = custom_module.enablement_state
custom_module_ancestor_module = custom_module.ancestor_module
custom_module_last_editor = custom_module.last_editor
```

---


### Simulation

Get the simulation by name or the latest simulation for the given organization.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_provider` | String | Indicates which cloud provider was used in this simulation. |
| `create_time` | String | Output only. Time simulation was created |
| `name` | String | Full resource name of the Simulation: `organizations/123/simulations/456` |
| `resource_value_configs_metadata` | Vec<String> | Resource value configurations' metadata used in this simulation. Maximum of 100. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access simulation outputs
simulation_id = simulation.id
simulation_cloud_provider = simulation.cloud_provider
simulation_create_time = simulation.create_time
simulation_name = simulation.name
simulation_resource_value_configs_metadata = simulation.resource_value_configs_metadata
```

---


### Source

Creates a source.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The relative resource name of this source. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}" |
| `canonical_name` | String |  | The canonical name of the finding source. It's either "organizations/{organization_id}/sources/{source_id}", "folders/{folder_id}/sources/{source_id}", or "projects/{project_number}/sources/{source_id}", depending on the closest CRM ancestor of the resource associated with the finding. |
| `description` | String |  | The description of the source (max of 1024 characters). Example: "Web Security Scanner is a web security scanner for common vulnerabilities in App Engine applications. It can automatically scan and detect four common vulnerabilities, including cross-site-scripting (XSS), Flash injection, mixed content (HTTP in HTTPS), and outdated or insecure libraries." |
| `display_name` | String |  | The source's display name. A source's display name must be unique amongst its siblings, for example, two sources with the same parent can't share the same display name. The display name must have a length between 1 and 64 characters (inclusive). |
| `parent` | String | ✅ | Required. Resource name of the new source's parent. Its format should be `organizations/[organization_id]`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The relative resource name of this source. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}" |
| `canonical_name` | String | The canonical name of the finding source. It's either "organizations/{organization_id}/sources/{source_id}", "folders/{folder_id}/sources/{source_id}", or "projects/{project_number}/sources/{source_id}", depending on the closest CRM ancestor of the resource associated with the finding. |
| `description` | String | The description of the source (max of 1024 characters). Example: "Web Security Scanner is a web security scanner for common vulnerabilities in App Engine applications. It can automatically scan and detect four common vulnerabilities, including cross-site-scripting (XSS), Flash injection, mixed content (HTTP in HTTPS), and outdated or insecure libraries." |
| `display_name` | String | The source's display name. A source's display name must be unique amongst its siblings, for example, two sources with the same parent can't share the same display name. The display name must have a length between 1 and 64 characters (inclusive). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create source
source = provider.securitycenter_api.Source {
    parent = "value"  # Required. Resource name of the new source's parent. Its format should be `organizations/[organization_id]`.
}

# Access source outputs
source_id = source.id
source_name = source.name
source_canonical_name = source.canonical_name
source_description = source.description
source_display_name = source.display_name
```

---


### Mute_config

Creates a mute config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time at which the mute config was created. This field is set by the server and will be ignored if provided on config creation. |
| `expiry_time` | String |  | Optional. The expiry of the mute config. Only applicable for dynamic configs. If the expiry is set, when the config expires, it is removed from all findings. |
| `name` | String |  | This field will be ignored if provided on config creation. Format `organizations/{organization}/muteConfigs/{mute_config}` `folders/{folder}/muteConfigs/{mute_config}` `projects/{project}/muteConfigs/{mute_config}` `organizations/{organization}/locations/global/muteConfigs/{mute_config}` `folders/{folder}/locations/global/muteConfigs/{mute_config}` `projects/{project}/locations/global/muteConfigs/{mute_config}` |
| `filter` | String |  | Required. An expression that defines the filter to apply across create/update events of findings. While creating a filter string, be mindful of the scope in which the mute configuration is being created. E.g., If a filter contains project = X but is created under the project = Y scope, it might not match any findings. The following field and operator combinations are supported: * severity: `=`, `:` * category: `=`, `:` * resource.name: `=`, `:` * resource.project_name: `=`, `:` * resource.project_display_name: `=`, `:` * resource.folders.resource_folder: `=`, `:` * resource.parent_name: `=`, `:` * resource.parent_display_name: `=`, `:` * resource.type: `=`, `:` * finding_class: `=`, `:` * indicator.ip_addresses: `=`, `:` * indicator.domains: `=`, `:` |
| `most_recent_editor` | String |  | Output only. Email address of the user who last edited the mute config. This field is set by the server and will be ignored if provided on config creation or update. |
| `type` | String |  | Optional. The type of the mute config, which determines what type of mute state the config affects. The static mute state takes precedence over the dynamic mute state. Immutable after creation. STATIC by default if not set during creation. |
| `display_name` | String |  | The human readable name to be displayed for the mute config. |
| `description` | String |  | A description of the mute config. |
| `update_time` | String |  | Output only. The most recent time at which the mute config was updated. This field is set by the server and will be ignored if provided on config creation or update. |
| `parent` | String | ✅ | Required. Resource name of the new mute configs's parent. Its format is `organizations/[organization_id]`, `folders/[folder_id]`, or `projects/[project_id]`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time at which the mute config was created. This field is set by the server and will be ignored if provided on config creation. |
| `expiry_time` | String | Optional. The expiry of the mute config. Only applicable for dynamic configs. If the expiry is set, when the config expires, it is removed from all findings. |
| `name` | String | This field will be ignored if provided on config creation. Format `organizations/{organization}/muteConfigs/{mute_config}` `folders/{folder}/muteConfigs/{mute_config}` `projects/{project}/muteConfigs/{mute_config}` `organizations/{organization}/locations/global/muteConfigs/{mute_config}` `folders/{folder}/locations/global/muteConfigs/{mute_config}` `projects/{project}/locations/global/muteConfigs/{mute_config}` |
| `filter` | String | Required. An expression that defines the filter to apply across create/update events of findings. While creating a filter string, be mindful of the scope in which the mute configuration is being created. E.g., If a filter contains project = X but is created under the project = Y scope, it might not match any findings. The following field and operator combinations are supported: * severity: `=`, `:` * category: `=`, `:` * resource.name: `=`, `:` * resource.project_name: `=`, `:` * resource.project_display_name: `=`, `:` * resource.folders.resource_folder: `=`, `:` * resource.parent_name: `=`, `:` * resource.parent_display_name: `=`, `:` * resource.type: `=`, `:` * finding_class: `=`, `:` * indicator.ip_addresses: `=`, `:` * indicator.domains: `=`, `:` |
| `most_recent_editor` | String | Output only. Email address of the user who last edited the mute config. This field is set by the server and will be ignored if provided on config creation or update. |
| `type` | String | Optional. The type of the mute config, which determines what type of mute state the config affects. The static mute state takes precedence over the dynamic mute state. Immutable after creation. STATIC by default if not set during creation. |
| `display_name` | String | The human readable name to be displayed for the mute config. |
| `description` | String | A description of the mute config. |
| `update_time` | String | Output only. The most recent time at which the mute config was updated. This field is set by the server and will be ignored if provided on config creation or update. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mute_config
mute_config = provider.securitycenter_api.Mute_config {
    parent = "value"  # Required. Resource name of the new mute configs's parent. Its format is `organizations/[organization_id]`, `folders/[folder_id]`, or `projects/[project_id]`.
}

# Access mute_config outputs
mute_config_id = mute_config.id
mute_config_create_time = mute_config.create_time
mute_config_expiry_time = mute_config.expiry_time
mute_config_name = mute_config.name
mute_config_filter = mute_config.filter
mute_config_most_recent_editor = mute_config.most_recent_editor
mute_config_type = mute_config.type
mute_config_display_name = mute_config.display_name
mute_config_description = mute_config.description
mute_config_update_time = mute_config.update_time
```

---


### Resource_value_config

Creates a ResourceValueConfig for an organization. Maps user's tags to difference resource values for use by the attack path simulation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. The resource value configs to be created. |
| `parent` | String | ✅ | Required. Resource name of the new ResourceValueConfig's parent. The parent field in the CreateResourceValueConfigRequest messages must either be empty or match this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sensitive_data_protection_mapping` | String | A mapping of the sensitivity on Sensitive Data Protection finding to resource values. This mapping can only be used in combination with a resource_type that is related to BigQuery, e.g. "bigquery.googleapis.com/Dataset". |
| `scope` | String | Project or folder to scope this configuration to. For example, "project/456" would apply this configuration only to resources in "project/456" scope will be checked with `AND` of other resources. |
| `tag_values` | Vec<String> | Required. Tag values combined with `AND` to check against. For Google Cloud resources, they are tag value IDs in the form of "tagValues/123". Example: `[ "tagValues/123", "tagValues/456", "tagValues/789" ]` https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing |
| `description` | String | Description of the resource value configuration. |
| `create_time` | String | Output only. Timestamp this resource value configuration was created. |
| `resource_type` | String | Apply resource_value only to resources that match resource_type. resource_type will be checked with `AND` of other resources. For example, "storage.googleapis.com/Bucket" with resource_value "HIGH" will apply "HIGH" value only to "storage.googleapis.com/Bucket" resources. |
| `resource_labels_selector` | HashMap<String, String> | List of resource labels to search for, evaluated with `AND`. For example, `"resource_labels_selector": {"key": "value", "env": "prod"}` will match resources with labels "key": "value" `AND` "env": "prod" https://cloud.google.com/resource-manager/docs/creating-managing-labels |
| `resource_value` | String | Required. Resource value level this expression represents |
| `update_time` | String | Output only. Timestamp this resource value configuration was last updated. |
| `name` | String | Name for the resource value configuration |
| `cloud_provider` | String | Cloud provider this configuration applies to |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create resource_value_config
resource_value_config = provider.securitycenter_api.Resource_value_config {
    parent = "value"  # Required. Resource name of the new ResourceValueConfig's parent. The parent field in the CreateResourceValueConfigRequest messages must either be empty or match this field.
}

# Access resource_value_config outputs
resource_value_config_id = resource_value_config.id
resource_value_config_sensitive_data_protection_mapping = resource_value_config.sensitive_data_protection_mapping
resource_value_config_scope = resource_value_config.scope
resource_value_config_tag_values = resource_value_config.tag_values
resource_value_config_description = resource_value_config.description
resource_value_config_create_time = resource_value_config.create_time
resource_value_config_resource_type = resource_value_config.resource_type
resource_value_config_resource_labels_selector = resource_value_config.resource_labels_selector
resource_value_config_resource_value = resource_value_config.resource_value
resource_value_config_update_time = resource_value_config.update_time
resource_value_config_name = resource_value_config.name
resource_value_config_cloud_provider = resource_value_config.cloud_provider
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.securitycenter_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
```

---


### Cluster

Get the ContainerThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetContainerThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateContainerThreatDetectionSettings for this purpose.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_account` | String |  | Output only. The service account used by Container Threat Detection for scanning. Service accounts are scoped at the project level meaning this field will be empty at any level above a project. |
| `name` | String |  | Identifier. The resource name of the ContainerThreatDetectionSettings. Formats: * organizations/{organization}/containerThreatDetectionSettings * folders/{folder}/containerThreatDetectionSettings * projects/{project}/containerThreatDetectionSettings * projects/{project}/locations/{location}/clusters/{cluster}/containerThreatDetectionSettings |
| `modules` | HashMap<String, String> |  | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `service_enablement_state` | String |  | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `update_time` | String |  | Output only. The time the settings were last updated. |
| `name` | String | ✅ | Identifier. The resource name of the ContainerThreatDetectionSettings. Formats: * organizations/{organization}/containerThreatDetectionSettings * folders/{folder}/containerThreatDetectionSettings * projects/{project}/containerThreatDetectionSettings * projects/{project}/locations/{location}/clusters/{cluster}/containerThreatDetectionSettings |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account` | String | Output only. The service account used by Container Threat Detection for scanning. Service accounts are scoped at the project level meaning this field will be empty at any level above a project. |
| `name` | String | Identifier. The resource name of the ContainerThreatDetectionSettings. Formats: * organizations/{organization}/containerThreatDetectionSettings * folders/{folder}/containerThreatDetectionSettings * projects/{project}/containerThreatDetectionSettings * projects/{project}/locations/{location}/clusters/{cluster}/containerThreatDetectionSettings |
| `modules` | HashMap<String, String> | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `service_enablement_state` | String | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `update_time` | String | Output only. The time the settings were last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access cluster outputs
cluster_id = cluster.id
cluster_service_account = cluster.service_account
cluster_name = cluster.name
cluster_modules = cluster.modules
cluster_service_enablement_state = cluster.service_enablement_state
cluster_update_time = cluster.update_time
```

---


### Virtual_machine_threat_detection_setting

Calculates the effective VirtualMachineThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_enablement_state` | String | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `update_time` | String | Output only. The time the settings were last updated. |
| `modules` | HashMap<String, String> | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `name` | String | Identifier. The resource name of the VirtualMachineThreatDetectionSettings. Formats: * organizations/{organization}/virtualMachineThreatDetectionSettings * folders/{folder}/virtualMachineThreatDetectionSettings * projects/{project}/virtualMachineThreatDetectionSettings |
| `service_account` | String | Output only. The service account used by Virtual Machine Threat Detection detectors. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access virtual_machine_threat_detection_setting outputs
virtual_machine_threat_detection_setting_id = virtual_machine_threat_detection_setting.id
virtual_machine_threat_detection_setting_service_enablement_state = virtual_machine_threat_detection_setting.service_enablement_state
virtual_machine_threat_detection_setting_update_time = virtual_machine_threat_detection_setting.update_time
virtual_machine_threat_detection_setting_modules = virtual_machine_threat_detection_setting.modules
virtual_machine_threat_detection_setting_name = virtual_machine_threat_detection_setting.name
virtual_machine_threat_detection_setting_service_account = virtual_machine_threat_detection_setting.service_account
```

---


### Project

Get the ContainerThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetContainerThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateContainerThreatDetectionSettings for this purpose.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_account` | String |  | Output only. The service account used by Security Health Analytics detectors. |
| `update_time` | String |  | Output only. The time the settings were last updated. |
| `service_enablement_state` | String |  | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `name` | String |  | Identifier. The resource name of the SecurityHealthAnalyticsSettings. Formats: * organizations/{organization}/securityHealthAnalyticsSettings * folders/{folder}/securityHealthAnalyticsSettings * projects/{project}/securityHealthAnalyticsSettings |
| `modules` | HashMap<String, String> |  | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `name` | String | ✅ | Identifier. The resource name of the SecurityHealthAnalyticsSettings. Formats: * organizations/{organization}/securityHealthAnalyticsSettings * folders/{folder}/securityHealthAnalyticsSettings * projects/{project}/securityHealthAnalyticsSettings |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account` | String | Output only. The service account used by Container Threat Detection for scanning. Service accounts are scoped at the project level meaning this field will be empty at any level above a project. |
| `name` | String | Identifier. The resource name of the ContainerThreatDetectionSettings. Formats: * organizations/{organization}/containerThreatDetectionSettings * folders/{folder}/containerThreatDetectionSettings * projects/{project}/containerThreatDetectionSettings * projects/{project}/locations/{location}/clusters/{cluster}/containerThreatDetectionSettings |
| `modules` | HashMap<String, String> | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `service_enablement_state` | String | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `update_time` | String | Output only. The time the settings were last updated. |


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
project_service_account = project.service_account
project_name = project.name
project_modules = project.modules
project_service_enablement_state = project.service_enablement_state
project_update_time = project.update_time
```

---


### Organization

Get the WebSecurityScannerSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetWebSecurityScannerSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateWebSecurityScannerSettings for this purpose.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_account` | String |  | Output only. The service account used by Security Health Analytics detectors. |
| `update_time` | String |  | Output only. The time the settings were last updated. |
| `service_enablement_state` | String |  | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `name` | String |  | Identifier. The resource name of the SecurityHealthAnalyticsSettings. Formats: * organizations/{organization}/securityHealthAnalyticsSettings * folders/{folder}/securityHealthAnalyticsSettings * projects/{project}/securityHealthAnalyticsSettings |
| `modules` | HashMap<String, String> |  | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `name` | String | ✅ | Identifier. The resource name of the SecurityHealthAnalyticsSettings. Formats: * organizations/{organization}/securityHealthAnalyticsSettings * folders/{folder}/securityHealthAnalyticsSettings * projects/{project}/securityHealthAnalyticsSettings |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time the settings were last updated. |
| `name` | String | Identifier. The resource name of the WebSecurityScannerSettings. Formats: * organizations/{organization}/webSecurityScannerSettings * folders/{folder}/webSecurityScannerSettings * projects/{project}/webSecurityScannerSettings |
| `modules` | HashMap<String, String> | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `service_enablement_state` | String | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access organization outputs
organization_id = organization.id
organization_update_time = organization.update_time
organization_name = organization.name
organization_modules = organization.modules
organization_service_enablement_state = organization.service_enablement_state
```

---


### Rapid_vulnerability_detection_setting

Calculates the effective RapidVulnerabilityDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the RapidVulnerabilityDetectionSettings. Formats: * organizations/{organization}/rapidVulnerabilityDetectionSettings * folders/{folder}/rapidVulnerabilityDetectionSettings * projects/{project}/rapidVulnerabilityDetectionSettings |
| `modules` | HashMap<String, String> | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's. |
| `service_enablement_state` | String | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `update_time` | String | Output only. The time the settings were last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access rapid_vulnerability_detection_setting outputs
rapid_vulnerability_detection_setting_id = rapid_vulnerability_detection_setting.id
rapid_vulnerability_detection_setting_name = rapid_vulnerability_detection_setting.name
rapid_vulnerability_detection_setting_modules = rapid_vulnerability_detection_setting.modules
rapid_vulnerability_detection_setting_service_enablement_state = rapid_vulnerability_detection_setting.service_enablement_state
rapid_vulnerability_detection_setting_update_time = rapid_vulnerability_detection_setting.update_time
```

---


### Folder

Get the EventThreatDetectionSettings resource. In the returned settings response, a missing field only indicates that it was not explicitly set, so no assumption should be made about these fields. In other words, GetEventThreatDetectionSettings does not calculate the effective service settings for the resource, which accounts for inherited settings and defaults. Instead, use CalculateEventThreatDetectionSettings for this purpose.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_account` | String |  | Output only. The service account used by Container Threat Detection for scanning. Service accounts are scoped at the project level meaning this field will be empty at any level above a project. |
| `name` | String |  | Identifier. The resource name of the ContainerThreatDetectionSettings. Formats: * organizations/{organization}/containerThreatDetectionSettings * folders/{folder}/containerThreatDetectionSettings * projects/{project}/containerThreatDetectionSettings * projects/{project}/locations/{location}/clusters/{cluster}/containerThreatDetectionSettings |
| `modules` | HashMap<String, String> |  | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `service_enablement_state` | String |  | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `update_time` | String |  | Output only. The time the settings were last updated. |
| `name` | String | ✅ | Identifier. The resource name of the ContainerThreatDetectionSettings. Formats: * organizations/{organization}/containerThreatDetectionSettings * folders/{folder}/containerThreatDetectionSettings * projects/{project}/containerThreatDetectionSettings * projects/{project}/locations/{location}/clusters/{cluster}/containerThreatDetectionSettings |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the EventThreatDetectionSettings. Formats: * organizations/{organization}/eventThreatDetectionSettings * folders/{folder}/eventThreatDetectionSettings * projects/{project}/eventThreatDetectionSettings |
| `modules` | HashMap<String, String> | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `service_enablement_state` | String | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `update_time` | String | Output only. The time the settings were last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access folder outputs
folder_id = folder.id
folder_name = folder.name
folder_modules = folder.modules
folder_service_enablement_state = folder.service_enablement_state
folder_update_time = folder.update_time
```

---


### Event_threat_detection_setting

Calculates the effective EventThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the EventThreatDetectionSettings. Formats: * organizations/{organization}/eventThreatDetectionSettings * folders/{folder}/eventThreatDetectionSettings * projects/{project}/eventThreatDetectionSettings |
| `modules` | HashMap<String, String> | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `service_enablement_state` | String | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `update_time` | String | Output only. The time the settings were last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access event_threat_detection_setting outputs
event_threat_detection_setting_id = event_threat_detection_setting.id
event_threat_detection_setting_name = event_threat_detection_setting.name
event_threat_detection_setting_modules = event_threat_detection_setting.modules
event_threat_detection_setting_service_enablement_state = event_threat_detection_setting.service_enablement_state
event_threat_detection_setting_update_time = event_threat_detection_setting.update_time
```

---


### Web_security_scanner_setting

Calculates the effective WebSecurityScannerSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time the settings were last updated. |
| `name` | String | Identifier. The resource name of the WebSecurityScannerSettings. Formats: * organizations/{organization}/webSecurityScannerSettings * folders/{folder}/webSecurityScannerSettings * projects/{project}/webSecurityScannerSettings |
| `modules` | HashMap<String, String> | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `service_enablement_state` | String | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access web_security_scanner_setting outputs
web_security_scanner_setting_id = web_security_scanner_setting.id
web_security_scanner_setting_update_time = web_security_scanner_setting.update_time
web_security_scanner_setting_name = web_security_scanner_setting.name
web_security_scanner_setting_modules = web_security_scanner_setting.modules
web_security_scanner_setting_service_enablement_state = web_security_scanner_setting.service_enablement_state
```

---


### Security_health_analytics_setting

Calculates the effective SecurityHealthAnalyticsSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account` | String | Output only. The service account used by Security Health Analytics detectors. |
| `update_time` | String | Output only. The time the settings were last updated. |
| `service_enablement_state` | String | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `name` | String | Identifier. The resource name of the SecurityHealthAnalyticsSettings. Formats: * organizations/{organization}/securityHealthAnalyticsSettings * folders/{folder}/securityHealthAnalyticsSettings * projects/{project}/securityHealthAnalyticsSettings |
| `modules` | HashMap<String, String> | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access security_health_analytics_setting outputs
security_health_analytics_setting_id = security_health_analytics_setting.id
security_health_analytics_setting_service_account = security_health_analytics_setting.service_account
security_health_analytics_setting_update_time = security_health_analytics_setting.update_time
security_health_analytics_setting_service_enablement_state = security_health_analytics_setting.service_enablement_state
security_health_analytics_setting_name = security_health_analytics_setting.name
security_health_analytics_setting_modules = security_health_analytics_setting.modules
```

---


### Container_threat_detection_setting

Calculates the effective ContainerThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account` | String | Output only. The service account used by Container Threat Detection for scanning. Service accounts are scoped at the project level meaning this field will be empty at any level above a project. |
| `name` | String | Identifier. The resource name of the ContainerThreatDetectionSettings. Formats: * organizations/{organization}/containerThreatDetectionSettings * folders/{folder}/containerThreatDetectionSettings * projects/{project}/containerThreatDetectionSettings * projects/{project}/locations/{location}/clusters/{cluster}/containerThreatDetectionSettings |
| `modules` | HashMap<String, String> | The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's configuration. |
| `service_enablement_state` | String | The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED. |
| `update_time` | String | Output only. The time the settings were last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access container_threat_detection_setting outputs
container_threat_detection_setting_id = container_threat_detection_setting.id
container_threat_detection_setting_service_account = container_threat_detection_setting.service_account
container_threat_detection_setting_name = container_threat_detection_setting.name
container_threat_detection_setting_modules = container_threat_detection_setting.modules
container_threat_detection_setting_service_enablement_state = container_threat_detection_setting.service_enablement_state
container_threat_detection_setting_update_time = container_threat_detection_setting.update_time
```

---


### Organization

Gets the settings for an organization.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_discovery_config` | String |  | The configuration used for Asset Discovery runs. |
| `name` | String |  | The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings". |
| `enable_asset_discovery` | bool |  | A flag that indicates if Asset Discovery should be enabled. If the flag is set to `true`, then discovery of assets will occur. If it is set to `false`, all historical assets will remain, but discovery of future assets will not occur. |
| `name` | String | ✅ | The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_discovery_config` | String | The configuration used for Asset Discovery runs. |
| `name` | String | The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings". |
| `enable_asset_discovery` | bool | A flag that indicates if Asset Discovery should be enabled. If the flag is set to `true`, then discovery of assets will occur. If it is set to `false`, all historical assets will remain, but discovery of future assets will not occur. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access organization outputs
organization_id = organization.id
organization_asset_discovery_config = organization.asset_discovery_config
organization_name = organization.name
organization_enable_asset_discovery = organization.enable_asset_discovery
```

---


### Finding

Creates a finding. The corresponding source must exist for finding creation to succeed.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String |  | Immutable. The relative resource name of the source the finding belongs to. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name This field is immutable after creation time. For example: "organizations/{organization_id}/sources/{source_id}" |
| `name` | String |  | The relative resource name of this finding. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}" |
| `resource_name` | String |  | For findings on Google Cloud resources, the full resource name of the Google Cloud resource this finding is for. See: https://cloud.google.com/apis/design/resource_names#full_resource_name When the finding is for a non-Google Cloud resource, the resourceName can be a customer or partner defined string. This field is immutable after creation time. |
| `category` | String |  | The additional taxonomy group within findings from a given source. This field is immutable after creation time. Example: "XSS_FLASH_INJECTION" |
| `event_time` | String |  | The time at which the event took place, or when an update to the finding occurred. For example, if the finding represents an open firewall it would capture the time the detector believes the firewall became open. The accuracy is determined by the detector. If the finding were to be resolved afterward, this time would reflect when the finding was resolved. |
| `state` | String |  | The state of the finding. |
| `external_uri` | String |  | The URI that, if available, points to a web page outside of Security Command Center where additional information about the finding can be found. This field is guaranteed to be either empty or a well formed URL. |
| `source_properties` | HashMap<String, String> |  | Source specific properties. These properties are managed by the source that writes the finding. The key names in the source_properties map must be between 1 and 255 characters, and must start with a letter and contain alphanumeric characters or underscores only. |
| `create_time` | String |  | The time at which the finding was created in Security Command Center. |
| `security_marks` | String |  | Output only. User specified security marks. These marks are entirely managed by the user and come from the SecurityMarks resource that belongs to the finding. |
| `parent` | String | ✅ | Required. Resource name of the new finding's parent. Its format should be "organizations/[organization_id]/sources/[source_id]". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_size` | i64 | The total number of findings matching the query. |
| `findings` | Vec<String> | Findings matching the list request. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results. |
| `read_time` | String | Time used for executing the list request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create finding
finding = provider.securitycenter_api.Finding {
    parent = "value"  # Required. Resource name of the new finding's parent. Its format should be "organizations/[organization_id]/sources/[source_id]".
}

# Access finding outputs
finding_id = finding.id
finding_total_size = finding.total_size
finding_findings = finding.findings
finding_next_page_token = finding.next_page_token
finding_read_time = finding.read_time
```

---


### Source

Creates a source.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description of the source (max of 1024 characters). Example: "Web Security Scanner is a web security scanner for common vulnerabilities in App Engine applications. It can automatically scan and detect four common vulnerabilities, including cross-site-scripting (XSS), Flash injection, mixed content (HTTP in HTTPS), and outdated/insecure libraries." |
| `name` | String |  | The relative resource name of this source. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}" |
| `display_name` | String |  | The source's display name. A source's display name must be unique amongst its siblings, for example, two sources with the same parent can't share the same display name. The display name must have a length between 1 and 64 characters (inclusive). |
| `parent` | String | ✅ | Required. Resource name of the new source's parent. Its format should be "organizations/[organization_id]". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The description of the source (max of 1024 characters). Example: "Web Security Scanner is a web security scanner for common vulnerabilities in App Engine applications. It can automatically scan and detect four common vulnerabilities, including cross-site-scripting (XSS), Flash injection, mixed content (HTTP in HTTPS), and outdated/insecure libraries." |
| `name` | String | The relative resource name of this source. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}" |
| `display_name` | String | The source's display name. A source's display name must be unique amongst its siblings, for example, two sources with the same parent can't share the same display name. The display name must have a length between 1 and 64 characters (inclusive). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create source
source = provider.securitycenter_api.Source {
    parent = "value"  # Required. Resource name of the new source's parent. Its format should be "organizations/[organization_id]".
}

# Access source outputs
source_id = source.id
source_description = source.description
source_name = source.name
source_display_name = source.display_name
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.securitycenter_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_response = operation.response
```

---


### Asset

Runs asset discovery. The discovery is tracked with a long-running operation. This API can only be called with limited frequency for an organization. If it is called too frequently the caller will receive a TOO_MANY_REQUESTS error.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. Name of the organization to run asset discovery for. Its format is "organizations/[organization_id]". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `list_assets_results` | Vec<String> | Assets matching the list request. |
| `total_size` | i64 | The total number of assets matching the query. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results. |
| `read_time` | String | Time used for executing the list request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create asset
asset = provider.securitycenter_api.Asset {
    parent = "value"  # Required. Name of the organization to run asset discovery for. Its format is "organizations/[organization_id]".
}

# Access asset outputs
asset_id = asset.id
asset_list_assets_results = asset.list_assets_results
asset_total_size = asset.total_size
asset_next_page_token = asset.next_page_token
asset_read_time = asset.read_time
```

---


### Finding

Creates a finding. The corresponding source must exist for finding creation to succeed.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `external_uri` | String |  | The URI that, if available, points to a web page outside of Security Command Center where additional information about the finding can be found. This field is guaranteed to be either empty or a well formed URL. |
| `event_time` | String |  | The time at which the event took place, or when an update to the finding occurred. For example, if the finding represents an open firewall it would capture the time the detector believes the firewall became open. The accuracy is determined by the detector. If the finding were to be resolved afterward, this time would reflect when the finding was resolved. |
| `category` | String |  | The additional taxonomy group within findings from a given source. This field is immutable after creation time. Example: "XSS_FLASH_INJECTION" |
| `security_marks` | String |  | Output only. User specified security marks. These marks are entirely managed by the user and come from the SecurityMarks resource that belongs to the finding. |
| `parent` | String |  | The relative resource name of the source the finding belongs to. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name This field is immutable after creation time. For example: "organizations/{organization_id}/sources/{source_id}" |
| `severity` | String |  | The severity of the finding. |
| `state` | String |  | The state of the finding. |
| `resource_name` | String |  | For findings on Google Cloud resources, the full resource name of the Google Cloud resource this finding is for. See: https://cloud.google.com/apis/design/resource_names#full_resource_name When the finding is for a non-Google Cloud resource, the resourceName can be a customer or partner defined string. This field is immutable after creation time. |
| `name` | String |  | The relative resource name of this finding. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}" |
| `create_time` | String |  | The time at which the finding was created in Security Command Center. |
| `source_properties` | HashMap<String, String> |  | Source specific properties. These properties are managed by the source that writes the finding. The key names in the source_properties map must be between 1 and 255 characters, and must start with a letter and contain alphanumeric characters or underscores only. |
| `parent` | String | ✅ | Required. Resource name of the new finding's parent. Its format should be "organizations/[organization_id]/sources/[source_id]". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_size` | i64 | The total number of findings matching the query. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results. |
| `list_findings_results` | Vec<String> | Findings matching the list request. |
| `read_time` | String | Time used for executing the list request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create finding
finding = provider.securitycenter_api.Finding {
    parent = "value"  # Required. Resource name of the new finding's parent. Its format should be "organizations/[organization_id]/sources/[source_id]".
}

# Access finding outputs
finding_id = finding.id
finding_total_size = finding.total_size
finding_next_page_token = finding.next_page_token
finding_list_findings_results = finding.list_findings_results
finding_read_time = finding.read_time
```

---


### Asset

Runs asset discovery. The discovery is tracked with a long-running operation. This API can only be called with limited frequency for an organization. If it is called too frequently the caller will receive a TOO_MANY_REQUESTS error.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. Name of the organization to run asset discovery for. Its format is "organizations/[organization_id]". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `read_time` | String | Time used for executing the list request. |
| `list_assets_results` | Vec<String> | Assets matching the list request. |
| `total_size` | i64 | The total number of assets matching the query. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create asset
asset = provider.securitycenter_api.Asset {
    parent = "value"  # Required. Name of the organization to run asset discovery for. Its format is "organizations/[organization_id]".
}

# Access asset outputs
asset_id = asset.id
asset_read_time = asset.read_time
asset_list_assets_results = asset.list_assets_results
asset_total_size = asset.total_size
asset_next_page_token = asset.next_page_token
```

---


### Notification_config

Creates a notification config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description of the notification config (max of 1024 characters). |
| `pubsub_topic` | String |  | The Pub/Sub topic to send notifications to. Its format is "projects/[project_id]/topics/[topic]". |
| `service_account` | String |  | Output only. The service account that needs "pubsub.topics.publish" permission to publish to the Pub/Sub topic. |
| `event_type` | String |  | The type of events the config is for, e.g. FINDING. |
| `name` | String |  | The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket". |
| `streaming_config` | String |  | The config for triggering streaming-based notifications. |
| `parent` | String | ✅ | Required. Resource name of the new notification config's parent. Its format is "organizations/[organization_id]". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The description of the notification config (max of 1024 characters). |
| `pubsub_topic` | String | The Pub/Sub topic to send notifications to. Its format is "projects/[project_id]/topics/[topic]". |
| `service_account` | String | Output only. The service account that needs "pubsub.topics.publish" permission to publish to the Pub/Sub topic. |
| `event_type` | String | The type of events the config is for, e.g. FINDING. |
| `name` | String | The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket". |
| `streaming_config` | String | The config for triggering streaming-based notifications. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notification_config
notification_config = provider.securitycenter_api.Notification_config {
    parent = "value"  # Required. Resource name of the new notification config's parent. Its format is "organizations/[organization_id]".
}

# Access notification_config outputs
notification_config_id = notification_config.id
notification_config_description = notification_config.description
notification_config_pubsub_topic = notification_config.pubsub_topic
notification_config_service_account = notification_config.service_account
notification_config_event_type = notification_config.event_type
notification_config_name = notification_config.name
notification_config_streaming_config = notification_config.streaming_config
```

---


### Source

Creates a source.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description of the source (max of 1024 characters). Example: "Web Security Scanner is a web security scanner for common vulnerabilities in App Engine applications. It can automatically scan and detect four common vulnerabilities, including cross-site-scripting (XSS), Flash injection, mixed content (HTTP in HTTPS), and outdated/insecure libraries." |
| `name` | String |  | The relative resource name of this source. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}" |
| `display_name` | String |  | The source's display name. A source's display name must be unique amongst its siblings, for example, two sources with the same parent can't share the same display name. The display name must have a length between 1 and 64 characters (inclusive). |
| `parent` | String | ✅ | Required. Resource name of the new source's parent. Its format should be "organizations/[organization_id]". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The description of the source (max of 1024 characters). Example: "Web Security Scanner is a web security scanner for common vulnerabilities in App Engine applications. It can automatically scan and detect four common vulnerabilities, including cross-site-scripting (XSS), Flash injection, mixed content (HTTP in HTTPS), and outdated/insecure libraries." |
| `name` | String | The relative resource name of this source. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}" |
| `display_name` | String | The source's display name. A source's display name must be unique amongst its siblings, for example, two sources with the same parent can't share the same display name. The display name must have a length between 1 and 64 characters (inclusive). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create source
source = provider.securitycenter_api.Source {
    parent = "value"  # Required. Resource name of the new source's parent. Its format should be "organizations/[organization_id]".
}

# Access source outputs
source_id = source.id
source_description = source.description
source_name = source.name
source_display_name = source.display_name
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.securitycenter_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Organization

Gets the settings for an organization.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_discovery_config` | String |  | The configuration used for Asset Discovery runs. |
| `enable_asset_discovery` | bool |  | A flag that indicates if Asset Discovery should be enabled. If the flag is set to `true`, then discovery of assets will occur. If it is set to `false, all historical assets will remain, but discovery of future assets will not occur. |
| `name` | String |  | The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings". |
| `name` | String | ✅ | The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_discovery_config` | String | The configuration used for Asset Discovery runs. |
| `enable_asset_discovery` | bool | A flag that indicates if Asset Discovery should be enabled. If the flag is set to `true`, then discovery of assets will occur. If it is set to `false, all historical assets will remain, but discovery of future assets will not occur. |
| `name` | String | The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access organization outputs
organization_id = organization.id
organization_asset_discovery_config = organization.asset_discovery_config
organization_enable_asset_discovery = organization.enable_asset_discovery
organization_name = organization.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple asset resources
asset_0 = provider.securitycenter_api.Asset {
    parent = "value-0"
}
asset_1 = provider.securitycenter_api.Asset {
    parent = "value-1"
}
asset_2 = provider.securitycenter_api.Asset {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    asset = provider.securitycenter_api.Asset {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Securitycenter_api Documentation](https://cloud.google.com/securitycenter_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
