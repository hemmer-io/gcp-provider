# Assuredworkloads_api Service



**Resources**: 8

---

## Overview

The assuredworkloads_api service provides access to 8 resource types:

- [Workload](#workload) [CRUD]
- [Violation](#violation) [CR]
- [Update](#update) [CR]
- [Operation](#operation) [R]
- [Workload](#workload) [CRUD]
- [Violation](#violation) [CR]
- [Operation](#operation) [R]
- [Update](#update) [CR]

---

## Resources


### Workload

Creates Assured Workload.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `billing_account` | String |  | Optional. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`. |
| `display_name` | String |  | Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload |
| `compliance_status` | String |  | Output only. Count of active Violations in the Workload. |
| `enable_sovereign_controls` | bool |  | Optional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers. |
| `ekm_provisioning_response` | String |  | Output only. Represents the Ekm Provisioning State of the given workload. |
| `etag` | String |  | Optional. ETag of the workload, it is calculated on the basis of the Workload contents. It will be used in Update & Delete operations. |
| `kms_settings` | String |  | Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS CMEK key is provisioned. This field is deprecated as of Feb 28, 2022. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field. |
| `labels` | HashMap<String, String> |  | Optional. Labels applied to the workload. |
| `name` | String |  | Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only. |
| `partner_permissions` | String |  | Optional. Permissions granted to the AW Partner SA account for the customer workload |
| `compliance_regime` | String |  | Required. Immutable. Compliance Regime associated with this workload. |
| `partner_services_billing_account` | String |  | Optional. Billing account necessary for purchasing services from Sovereign Partners. This field is required for creating SIA/PSN/CNTXT partner workloads. The caller should have 'billing.resourceAssociations.create' IAM permission on this billing-account. The format of this string is billingAccounts/AAAAAA-BBBBBB-CCCCCC |
| `partner` | String |  | Optional. Partner regime associated with this workload. |
| `resources` | Vec<String> |  | Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only. |
| `violation_notifications_enabled` | bool |  | Optional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload. |
| `workload_options` | String |  | Optional. Options to be set for the given created workload. |
| `saa_enrollment_response` | String |  | Output only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during GetWorkload call. In failure cases, user friendly error message is shown in SAA details page. |
| `create_time` | String |  | Output only. Immutable. The Workload creation timestamp. |
| `provisioned_resources_parent` | String |  | Input only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id} |
| `resource_monitoring_enabled` | bool |  | Output only. Indicates whether resource monitoring is enabled for workload or not. It is true when Resource feed is subscribed to AWM topic and AWM Service Agent Role is binded to AW Service Account for resource Assured workload. |
| `kaj_enrollment_state` | String |  | Output only. Represents the KAJ enrollment state of the given workload. |
| `compliant_but_disallowed_services` | Vec<String> |  | Output only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke RestrictAllowedResources endpoint to allow your project developers to use these services in their environment. |
| `resource_settings` | Vec<String> |  | Input only. Resource properties that are used to customize workload resources. These properties (such as custom project id) will be used to create workload resources if possible. This field is optional. |
| `parent` | String | ✅ | Required. The resource name of the new Workload's parent. Must be of the form `organizations/{org_id}/locations/{location_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `billing_account` | String | Optional. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`. |
| `display_name` | String | Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload |
| `compliance_status` | String | Output only. Count of active Violations in the Workload. |
| `enable_sovereign_controls` | bool | Optional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers. |
| `ekm_provisioning_response` | String | Output only. Represents the Ekm Provisioning State of the given workload. |
| `etag` | String | Optional. ETag of the workload, it is calculated on the basis of the Workload contents. It will be used in Update & Delete operations. |
| `kms_settings` | String | Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS CMEK key is provisioned. This field is deprecated as of Feb 28, 2022. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field. |
| `labels` | HashMap<String, String> | Optional. Labels applied to the workload. |
| `name` | String | Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only. |
| `partner_permissions` | String | Optional. Permissions granted to the AW Partner SA account for the customer workload |
| `compliance_regime` | String | Required. Immutable. Compliance Regime associated with this workload. |
| `partner_services_billing_account` | String | Optional. Billing account necessary for purchasing services from Sovereign Partners. This field is required for creating SIA/PSN/CNTXT partner workloads. The caller should have 'billing.resourceAssociations.create' IAM permission on this billing-account. The format of this string is billingAccounts/AAAAAA-BBBBBB-CCCCCC |
| `partner` | String | Optional. Partner regime associated with this workload. |
| `resources` | Vec<String> | Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only. |
| `violation_notifications_enabled` | bool | Optional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload. |
| `workload_options` | String | Optional. Options to be set for the given created workload. |
| `saa_enrollment_response` | String | Output only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during GetWorkload call. In failure cases, user friendly error message is shown in SAA details page. |
| `create_time` | String | Output only. Immutable. The Workload creation timestamp. |
| `provisioned_resources_parent` | String | Input only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id} |
| `resource_monitoring_enabled` | bool | Output only. Indicates whether resource monitoring is enabled for workload or not. It is true when Resource feed is subscribed to AWM topic and AWM Service Agent Role is binded to AW Service Account for resource Assured workload. |
| `kaj_enrollment_state` | String | Output only. Represents the KAJ enrollment state of the given workload. |
| `compliant_but_disallowed_services` | Vec<String> | Output only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke RestrictAllowedResources endpoint to allow your project developers to use these services in their environment. |
| `resource_settings` | Vec<String> | Input only. Resource properties that are used to customize workload resources. These properties (such as custom project id) will be used to create workload resources if possible. This field is optional. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workload
workload = provider.assuredworkloads_api.Workload {
    parent = "value"  # Required. The resource name of the new Workload's parent. Must be of the form `organizations/{org_id}/locations/{location_id}`.
}

# Access workload outputs
workload_id = workload.id
workload_billing_account = workload.billing_account
workload_display_name = workload.display_name
workload_compliance_status = workload.compliance_status
workload_enable_sovereign_controls = workload.enable_sovereign_controls
workload_ekm_provisioning_response = workload.ekm_provisioning_response
workload_etag = workload.etag
workload_kms_settings = workload.kms_settings
workload_labels = workload.labels
workload_name = workload.name
workload_partner_permissions = workload.partner_permissions
workload_compliance_regime = workload.compliance_regime
workload_partner_services_billing_account = workload.partner_services_billing_account
workload_partner = workload.partner
workload_resources = workload.resources
workload_violation_notifications_enabled = workload.violation_notifications_enabled
workload_workload_options = workload.workload_options
workload_saa_enrollment_response = workload.saa_enrollment_response
workload_create_time = workload.create_time
workload_provisioned_resources_parent = workload.provisioned_resources_parent
workload_resource_monitoring_enabled = workload.resource_monitoring_enabled
workload_kaj_enrollment_state = workload.kaj_enrollment_state
workload_compliant_but_disallowed_services = workload.compliant_but_disallowed_services
workload_resource_settings = workload.resource_settings
```

---


### Violation

Acknowledges an existing violation. By acknowledging a violation, users acknowledge the existence of a compliance violation in their workload and decide to ignore it due to a valid business justification. Acknowledgement is a permanent operation and it cannot be reverted.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `comment` | String |  | Required. Business justification explaining the need for violation acknowledgement |
| `non_compliant_org_policy` | String |  | Optional. This field is deprecated and will be removed in future version of the API. Name of the OrgPolicy which was modified with non-compliant change and resulted in this violation. Format: projects/{project_number}/policies/{constraint_name} folders/{folder_id}/policies/{constraint_name} organizations/{organization_id}/policies/{constraint_name} |
| `acknowledge_type` | String |  | Optional. Acknowledge type of specified violation. |
| `name` | String | ✅ | Required. The resource name of the Violation to acknowledge. Format: organizations/{organization}/locations/{location}/workloads/{workload}/violations/{violation} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `begin_time` | String | Output only. Time of the event which triggered the Violation. |
| `resolve_time` | String | Output only. Time of the event which fixed the Violation. If the violation is ACTIVE this will be empty. |
| `resource_type` | String | Optional. Output only. Type of the resource like compute.googleapis.com/Disk, etc. Empty for org-policy violations. |
| `non_compliant_org_policy` | String | Output only. Immutable. Name of the OrgPolicy which was modified with non-compliant change and resulted this violation. Format: projects/{project_number}/policies/{constraint_name} folders/{folder_id}/policies/{constraint_name} organizations/{organization_id}/policies/{constraint_name} |
| `associated_org_policy_violation_id` | String | Optional. Output only. Violation Id of the org-policy violation due to which the resource violation is caused. Empty for org-policy violations. |
| `audit_log_link` | String | Output only. Immutable. Audit Log Link for violated resource Format: https://console.cloud.google.com/logs/query;query={logName}{protoPayload.resourceName}{timeRange}{folder} |
| `remediation` | String | Output only. Compliance violation remediation |
| `state` | String | Output only. State of the violation |
| `parent_project_number` | String | Optional. Output only. Parent project number where resource is present. Empty for org-policy violations. |
| `exception_audit_log_link` | String | Output only. Immutable. Audit Log link to find business justification provided for violation exception. Format: https://console.cloud.google.com/logs/query;query={logName}{protoPayload.resourceName}{protoPayload.methodName}{timeRange}{organization} |
| `description` | String | Output only. Description for the Violation. e.g. OrgPolicy gcp.resourceLocations has non compliant value. |
| `name` | String | Output only. Immutable. Name of the Violation. Format: organizations/{organization}/locations/{location}/workloads/{workload_id}/violations/{violations_id} |
| `acknowledged` | bool | A boolean that indicates if the violation is acknowledged |
| `acknowledgement_time` | String | Optional. Timestamp when this violation was acknowledged first. Check exception_contexts to find the last time the violation was acknowledged when there are more than one violations. This field will be absent when acknowledged field is marked as false. |
| `exception_contexts` | Vec<String> | Output only. List of all the exception detail added for the violation. |
| `org_policy_constraint` | String | Output only. Immutable. The org-policy-constraint that was incorrectly changed, which resulted in this violation. |
| `violation_type` | String | Output only. Type of the violation |
| `category` | String | Output only. Category under which this violation is mapped. e.g. Location, Service Usage, Access, Encryption, etc. |
| `update_time` | String | Output only. The last time when the Violation record was updated. |
| `resource_name` | String | Optional. Output only. Name of the resource like //storage.googleapis.com/myprojectxyz-testbucket. Empty for org-policy violations. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create violation
violation = provider.assuredworkloads_api.Violation {
    name = "value"  # Required. The resource name of the Violation to acknowledge. Format: organizations/{organization}/locations/{location}/workloads/{workload}/violations/{violation}
}

# Access violation outputs
violation_id = violation.id
violation_begin_time = violation.begin_time
violation_resolve_time = violation.resolve_time
violation_resource_type = violation.resource_type
violation_non_compliant_org_policy = violation.non_compliant_org_policy
violation_associated_org_policy_violation_id = violation.associated_org_policy_violation_id
violation_audit_log_link = violation.audit_log_link
violation_remediation = violation.remediation
violation_state = violation.state
violation_parent_project_number = violation.parent_project_number
violation_exception_audit_log_link = violation.exception_audit_log_link
violation_description = violation.description
violation_name = violation.name
violation_acknowledged = violation.acknowledged
violation_acknowledgement_time = violation.acknowledgement_time
violation_exception_contexts = violation.exception_contexts
violation_org_policy_constraint = violation.org_policy_constraint
violation_violation_type = violation.violation_type
violation_category = violation.category
violation_update_time = violation.update_time
violation_resource_name = violation.resource_name
```

---


### Update

This endpoint creates a new operation to apply the given update.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String |  | The action to be performed on the update. |
| `name` | String | ✅ | Required. The resource name of the update. Format: organizations/{org_id}/locations/{location_id}/workloads/{workload_id}/updates/{update_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workload_updates` | Vec<String> | The list of workload updates for a given workload. |
| `next_page_token` | String | The next page token. Return empty if reached the last page. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create update
update = provider.assuredworkloads_api.Update {
    name = "value"  # Required. The resource name of the update. Format: organizations/{org_id}/locations/{location_id}/workloads/{workload_id}/updates/{update_id}
}

# Access update outputs
update_id = update.id
update_workload_updates = update.workload_updates
update_next_page_token = update.next_page_token
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Workload

Creates Assured Workload.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `partner_services_billing_account` | String |  | Optional. Billing account necessary for purchasing services from Sovereign Partners. This field is required for creating SIA/PSN/CNTXT partner workloads. The caller should have 'billing.resourceAssociations.create' IAM permission on this billing-account. The format of this string is billingAccounts/AAAAAA-BBBBBB-CCCCCC |
| `resource_monitoring_enabled` | bool |  | Output only. Indicates whether resource monitoring is enabled for workload or not. It is true when Resource feed is subscribed to AWM topic and AWM Service Agent Role is binded to AW Service Account for resource Assured workload. |
| `provisioned_resources_parent` | String |  | Input only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id} |
| `kaj_enrollment_state` | String |  | Output only. Represents the KAJ enrollment state of the given workload. |
| `il4_settings` | String |  | Input only. Immutable. Settings specific to resources needed for IL4. |
| `violation_notifications_enabled` | bool |  | Optional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload. |
| `fedramp_high_settings` | String |  | Input only. Immutable. Settings specific to resources needed for FedRAMP High. |
| `labels` | HashMap<String, String> |  | Optional. Labels applied to the workload. |
| `compliance_updates_enabled` | bool |  | Output only. Indicates whether the compliance updates feature is enabled for a workload. The compliance updates feature can be enabled via the EnableComplianceUpdates endpoint. |
| `resource_settings` | Vec<String> |  | Input only. Resource properties that are used to customize workload resources. These properties (such as custom project id) will be used to create workload resources if possible. This field is optional. |
| `compliance_regime` | String |  | Required. Immutable. Compliance Regime associated with this workload. |
| `compliance_status` | String |  | Output only. Count of active Violations in the Workload. |
| `compliant_but_disallowed_services` | Vec<String> |  | Output only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke RestrictAllowedResources endpoint to allow your project developers to use these services in their environment. |
| `billing_account` | String |  | Optional. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`. |
| `display_name` | String |  | Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload |
| `saa_enrollment_response` | String |  | Output only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during GetWorkload call. In failure cases, user friendly error message is shown in SAA details page. |
| `resources` | Vec<String> |  | Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only. |
| `create_time` | String |  | Output only. Immutable. The Workload creation timestamp. |
| `partner` | String |  | Optional. Partner regime associated with this workload. |
| `ekm_provisioning_response` | String |  | Output only. Represents the Ekm Provisioning State of the given workload. |
| `enable_sovereign_controls` | bool |  | Optional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers. |
| `fedramp_moderate_settings` | String |  | Input only. Immutable. Settings specific to resources needed for FedRAMP Moderate. |
| `partner_permissions` | String |  | Optional. Permissions granted to the AW Partner SA account for the customer workload |
| `kms_settings` | String |  | Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS CMEK key is provisioned. This field is deprecated as of Feb 28, 2022. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field. |
| `cjis_settings` | String |  | Input only. Immutable. Settings specific to resources needed for CJIS. |
| `etag` | String |  | Optional. ETag of the workload, it is calculated on the basis of the Workload contents. It will be used in Update & Delete operations. |
| `workload_options` | String |  | Optional. Options to be set for the given created workload. |
| `name` | String |  | Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only. |
| `available_updates` | i64 |  | Output only. The number of updates available for the workload. |
| `parent` | String | ✅ | Required. The resource name of the new Workload's parent. Must be of the form `organizations/{org_id}/locations/{location_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `partner_services_billing_account` | String | Optional. Billing account necessary for purchasing services from Sovereign Partners. This field is required for creating SIA/PSN/CNTXT partner workloads. The caller should have 'billing.resourceAssociations.create' IAM permission on this billing-account. The format of this string is billingAccounts/AAAAAA-BBBBBB-CCCCCC |
| `resource_monitoring_enabled` | bool | Output only. Indicates whether resource monitoring is enabled for workload or not. It is true when Resource feed is subscribed to AWM topic and AWM Service Agent Role is binded to AW Service Account for resource Assured workload. |
| `provisioned_resources_parent` | String | Input only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id} |
| `kaj_enrollment_state` | String | Output only. Represents the KAJ enrollment state of the given workload. |
| `il4_settings` | String | Input only. Immutable. Settings specific to resources needed for IL4. |
| `violation_notifications_enabled` | bool | Optional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload. |
| `fedramp_high_settings` | String | Input only. Immutable. Settings specific to resources needed for FedRAMP High. |
| `labels` | HashMap<String, String> | Optional. Labels applied to the workload. |
| `compliance_updates_enabled` | bool | Output only. Indicates whether the compliance updates feature is enabled for a workload. The compliance updates feature can be enabled via the EnableComplianceUpdates endpoint. |
| `resource_settings` | Vec<String> | Input only. Resource properties that are used to customize workload resources. These properties (such as custom project id) will be used to create workload resources if possible. This field is optional. |
| `compliance_regime` | String | Required. Immutable. Compliance Regime associated with this workload. |
| `compliance_status` | String | Output only. Count of active Violations in the Workload. |
| `compliant_but_disallowed_services` | Vec<String> | Output only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke RestrictAllowedResources endpoint to allow your project developers to use these services in their environment. |
| `billing_account` | String | Optional. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`. |
| `display_name` | String | Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload |
| `saa_enrollment_response` | String | Output only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during GetWorkload call. In failure cases, user friendly error message is shown in SAA details page. |
| `resources` | Vec<String> | Output only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only. |
| `create_time` | String | Output only. Immutable. The Workload creation timestamp. |
| `partner` | String | Optional. Partner regime associated with this workload. |
| `ekm_provisioning_response` | String | Output only. Represents the Ekm Provisioning State of the given workload. |
| `enable_sovereign_controls` | bool | Optional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers. |
| `fedramp_moderate_settings` | String | Input only. Immutable. Settings specific to resources needed for FedRAMP Moderate. |
| `partner_permissions` | String | Optional. Permissions granted to the AW Partner SA account for the customer workload |
| `kms_settings` | String | Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS CMEK key is provisioned. This field is deprecated as of Feb 28, 2022. In order to create a Keyring, callers should specify, ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field. |
| `cjis_settings` | String | Input only. Immutable. Settings specific to resources needed for CJIS. |
| `etag` | String | Optional. ETag of the workload, it is calculated on the basis of the Workload contents. It will be used in Update & Delete operations. |
| `workload_options` | String | Optional. Options to be set for the given created workload. |
| `name` | String | Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only. |
| `available_updates` | i64 | Output only. The number of updates available for the workload. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workload
workload = provider.assuredworkloads_api.Workload {
    parent = "value"  # Required. The resource name of the new Workload's parent. Must be of the form `organizations/{org_id}/locations/{location_id}`.
}

# Access workload outputs
workload_id = workload.id
workload_partner_services_billing_account = workload.partner_services_billing_account
workload_resource_monitoring_enabled = workload.resource_monitoring_enabled
workload_provisioned_resources_parent = workload.provisioned_resources_parent
workload_kaj_enrollment_state = workload.kaj_enrollment_state
workload_il4_settings = workload.il4_settings
workload_violation_notifications_enabled = workload.violation_notifications_enabled
workload_fedramp_high_settings = workload.fedramp_high_settings
workload_labels = workload.labels
workload_compliance_updates_enabled = workload.compliance_updates_enabled
workload_resource_settings = workload.resource_settings
workload_compliance_regime = workload.compliance_regime
workload_compliance_status = workload.compliance_status
workload_compliant_but_disallowed_services = workload.compliant_but_disallowed_services
workload_billing_account = workload.billing_account
workload_display_name = workload.display_name
workload_saa_enrollment_response = workload.saa_enrollment_response
workload_resources = workload.resources
workload_create_time = workload.create_time
workload_partner = workload.partner
workload_ekm_provisioning_response = workload.ekm_provisioning_response
workload_enable_sovereign_controls = workload.enable_sovereign_controls
workload_fedramp_moderate_settings = workload.fedramp_moderate_settings
workload_partner_permissions = workload.partner_permissions
workload_kms_settings = workload.kms_settings
workload_cjis_settings = workload.cjis_settings
workload_etag = workload.etag
workload_workload_options = workload.workload_options
workload_name = workload.name
workload_available_updates = workload.available_updates
```

---


### Violation

Acknowledges an existing violation. By acknowledging a violation, users acknowledge the existence of a compliance violation in their workload and decide to ignore it due to a valid business justification. Acknowledgement is a permanent operation and it cannot be reverted.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `non_compliant_org_policy` | String |  | Optional. This field is deprecated and will be removed in future version of the API. Name of the OrgPolicy which was modified with non-compliant change and resulted in this violation. Format: projects/{project_number}/policies/{constraint_name} folders/{folder_id}/policies/{constraint_name} organizations/{organization_id}/policies/{constraint_name} |
| `comment` | String |  | Required. Business justification explaining the need for violation acknowledgement |
| `acknowledge_type` | String |  | Optional. Acknowledge type of specified violation. |
| `name` | String | ✅ | Required. The resource name of the Violation to acknowledge. Format: organizations/{organization}/locations/{location}/workloads/{workload}/violations/{violation} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_type` | String | Optional. Output only. Type of the resource like compute.googleapis.com/Disk, etc. Empty for org-policy violations. |
| `update_time` | String | Output only. The last time when the Violation record was updated. |
| `begin_time` | String | Output only. Time of the event which triggered the Violation. |
| `acknowledged` | bool | A boolean that indicates if the violation is acknowledged |
| `non_compliant_org_policy` | String | Output only. Immutable. Name of the OrgPolicy which was modified with non-compliant change and resulted this violation. Format: projects/{project_number}/policies/{constraint_name} folders/{folder_id}/policies/{constraint_name} organizations/{organization_id}/policies/{constraint_name} |
| `remediation` | String | Output only. Compliance violation remediation |
| `name` | String | Output only. Immutable. Name of the Violation. Format: organizations/{organization}/locations/{location}/workloads/{workload_id}/violations/{violations_id} |
| `audit_log_link` | String | Output only. Immutable. Audit Log Link for violated resource Format: https://console.cloud.google.com/logs/query;query={logName}{protoPayload.resourceName}{timeRange}{folder} |
| `category` | String | Output only. Category under which this violation is mapped. e.g. Location, Service Usage, Access, Encryption, etc. |
| `parent_project_number` | String | Optional. Output only. Parent project number where resource is present. Empty for org-policy violations. |
| `state` | String | Output only. State of the violation |
| `description` | String | Output only. Description for the Violation. e.g. OrgPolicy gcp.resourceLocations has non compliant value. |
| `exception_contexts` | Vec<String> | Output only. List of all the exception detail added for the violation. |
| `violation_type` | String | Output only. Type of the violation |
| `associated_org_policy_violation_id` | String | Optional. Output only. Violation Id of the org-policy violation due to which the resource violation is caused. Empty for org-policy violations. |
| `exception_audit_log_link` | String | Output only. Immutable. Audit Log link to find business justification provided for violation exception. Format: https://console.cloud.google.com/logs/query;query={logName}{protoPayload.resourceName}{protoPayload.methodName}{timeRange}{organization} |
| `resolve_time` | String | Output only. Time of the event which fixed the Violation. If the violation is ACTIVE this will be empty. |
| `resource_name` | String | Optional. Output only. Name of the resource like //storage.googleapis.com/myprojectxyz-testbucket. Empty for org-policy violations. |
| `org_policy_constraint` | String | Output only. Immutable. The org-policy-constraint that was incorrectly changed, which resulted in this violation. |
| `acknowledgement_time` | String | Optional. Timestamp when this violation was acknowledged first. Check exception_contexts to find the last time the violation was acknowledged when there are more than one violations. This field will be absent when acknowledged field is marked as false. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create violation
violation = provider.assuredworkloads_api.Violation {
    name = "value"  # Required. The resource name of the Violation to acknowledge. Format: organizations/{organization}/locations/{location}/workloads/{workload}/violations/{violation}
}

# Access violation outputs
violation_id = violation.id
violation_resource_type = violation.resource_type
violation_update_time = violation.update_time
violation_begin_time = violation.begin_time
violation_acknowledged = violation.acknowledged
violation_non_compliant_org_policy = violation.non_compliant_org_policy
violation_remediation = violation.remediation
violation_name = violation.name
violation_audit_log_link = violation.audit_log_link
violation_category = violation.category
violation_parent_project_number = violation.parent_project_number
violation_state = violation.state
violation_description = violation.description
violation_exception_contexts = violation.exception_contexts
violation_violation_type = violation.violation_type
violation_associated_org_policy_violation_id = violation.associated_org_policy_violation_id
violation_exception_audit_log_link = violation.exception_audit_log_link
violation_resolve_time = violation.resolve_time
violation_resource_name = violation.resource_name
violation_org_policy_constraint = violation.org_policy_constraint
violation_acknowledgement_time = violation.acknowledgement_time
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
```

---


### Update

This endpoint creates a new operation to apply the given update.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String |  | The action to be performed on the update. |
| `name` | String | ✅ | Required. The resource name of the update. Format: organizations/{org_id}/locations/{location_id}/workloads/{workload_id}/updates/{update_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The next page token. Return empty if reached the last page. |
| `workload_updates` | Vec<String> | The list of workload updates for a given workload. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create update
update = provider.assuredworkloads_api.Update {
    name = "value"  # Required. The resource name of the update. Format: organizations/{org_id}/locations/{location_id}/workloads/{workload_id}/updates/{update_id}
}

# Access update outputs
update_id = update.id
update_next_page_token = update.next_page_token
update_workload_updates = update.workload_updates
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple workload resources
workload_0 = provider.assuredworkloads_api.Workload {
    parent = "value-0"
}
workload_1 = provider.assuredworkloads_api.Workload {
    parent = "value-1"
}
workload_2 = provider.assuredworkloads_api.Workload {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    workload = provider.assuredworkloads_api.Workload {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Assuredworkloads_api Documentation](https://cloud.google.com/assuredworkloads_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
