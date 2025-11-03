# Accessapproval_api Service



**Resources**: 4

---

## Overview

The accessapproval_api service provides access to 4 resource types:

- [Organization](#organization) [RUD]
- [Folder](#folder) [RUD]
- [Project](#project) [RUD]
- [Approval_request](#approval_request) [CR]

---

## Resources


### Organization

Gets the Access Approval settings associated with a project, folder, or organization.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_pubsub_topic` | String |  | Optional. A pubsub topic that notifications relating to access approval are published to. Notifications include pre-approved accesses. |
| `request_scope_max_width_preference` | String |  | Optional. A setting that indicates the maximum scope of an Access Approval request: either organization, folder, or project. Google administrators will be asked to send requests no broader than the configured scope. |
| `ancestor_has_active_key_version` | bool |  | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that an ancestor of this Project or Folder has set active_key_version (this field will always be unset for the organization since organizations do not have ancestors). |
| `enrolled_services` | Vec<String> |  | A list of Google Cloud Services for which the given resource has Access Approval enrolled. Access requests for the resource given by name against any of these services contained here will be required to have explicit approval. If name refers to an organization, enrollment can be done for individual services. If name refers to a folder or project, enrollment can only be done on an all or nothing basis. If a cloud_product is repeated in this list, the first entry will be honored and all following entries will be discarded. |
| `approval_policy` | String |  | Optional. Policy configuration for Access Approval that sets the operating mode. The available policies are Transparency, Streamlined Support, and Approval Required. |
| `prefer_no_broad_approval_requests` | bool |  | This field is used to set a preference for granularity of an access approval request. If true, Google personnel will be asked to send resource-level requests when possible. If false, Google personnel will be asked to send requests at the project level. |
| `require_customer_visible_justification` | bool |  | Optional. When enabled, Google will only be able to send approval requests for access reasons with a customer accessible case ID in the reason detail. Also known as "Require customer initiated support case justification" |
| `active_key_version` | String |  | The asymmetric crypto key version to use for signing approval requests. Empty active_key_version indicates that a Google-managed key should be used for signing. This property will be ignored if set by an ancestor of this resource, and new non-empty values may not be set. |
| `name` | String |  | The resource name of the settings. Format is one of: * "projects/{project}/accessApprovalSettings" * "folders/{folder}/accessApprovalSettings" * "organizations/{organization}/accessApprovalSettings" |
| `enrolled_ancestor` | bool |  | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that at least one service is enrolled for Access Approval in one or more ancestors of the Project or Folder (this field will always be unset for the organization since organizations do not have ancestors). |
| `preferred_request_expiration_days` | i64 |  | Set the default access approval request expiration time. This value is able to be set directly by the customer at the time of approval, overriding this suggested value. We recommend setting this value to 30 days. |
| `invalid_key_version` | bool |  | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that there is some configuration issue with the active_key_version configured at this level in the resource hierarchy (e.g. it doesn't exist or the Access Approval service account doesn't have the correct permissions on it, etc.) This key version is not necessarily the effective key version at this level, as key versions are inherited top-down. |
| `effective_approval_policy` | String |  | Output only. Effective policy applied for Access Approval, inclusive of inheritance. |
| `notification_emails` | Vec<String> |  | A list of email addresses to which notifications relating to approval requests should be sent. Notifications relating to a resource will be sent to all emails in the settings of ancestor resources of that resource. A maximum of 50 email addresses are allowed. |
| `name` | String | ✅ | The resource name of the settings. Format is one of: * "projects/{project}/accessApprovalSettings" * "folders/{folder}/accessApprovalSettings" * "organizations/{organization}/accessApprovalSettings" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_pubsub_topic` | String | Optional. A pubsub topic that notifications relating to access approval are published to. Notifications include pre-approved accesses. |
| `request_scope_max_width_preference` | String | Optional. A setting that indicates the maximum scope of an Access Approval request: either organization, folder, or project. Google administrators will be asked to send requests no broader than the configured scope. |
| `ancestor_has_active_key_version` | bool | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that an ancestor of this Project or Folder has set active_key_version (this field will always be unset for the organization since organizations do not have ancestors). |
| `enrolled_services` | Vec<String> | A list of Google Cloud Services for which the given resource has Access Approval enrolled. Access requests for the resource given by name against any of these services contained here will be required to have explicit approval. If name refers to an organization, enrollment can be done for individual services. If name refers to a folder or project, enrollment can only be done on an all or nothing basis. If a cloud_product is repeated in this list, the first entry will be honored and all following entries will be discarded. |
| `approval_policy` | String | Optional. Policy configuration for Access Approval that sets the operating mode. The available policies are Transparency, Streamlined Support, and Approval Required. |
| `prefer_no_broad_approval_requests` | bool | This field is used to set a preference for granularity of an access approval request. If true, Google personnel will be asked to send resource-level requests when possible. If false, Google personnel will be asked to send requests at the project level. |
| `require_customer_visible_justification` | bool | Optional. When enabled, Google will only be able to send approval requests for access reasons with a customer accessible case ID in the reason detail. Also known as "Require customer initiated support case justification" |
| `active_key_version` | String | The asymmetric crypto key version to use for signing approval requests. Empty active_key_version indicates that a Google-managed key should be used for signing. This property will be ignored if set by an ancestor of this resource, and new non-empty values may not be set. |
| `name` | String | The resource name of the settings. Format is one of: * "projects/{project}/accessApprovalSettings" * "folders/{folder}/accessApprovalSettings" * "organizations/{organization}/accessApprovalSettings" |
| `enrolled_ancestor` | bool | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that at least one service is enrolled for Access Approval in one or more ancestors of the Project or Folder (this field will always be unset for the organization since organizations do not have ancestors). |
| `preferred_request_expiration_days` | i64 | Set the default access approval request expiration time. This value is able to be set directly by the customer at the time of approval, overriding this suggested value. We recommend setting this value to 30 days. |
| `invalid_key_version` | bool | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that there is some configuration issue with the active_key_version configured at this level in the resource hierarchy (e.g. it doesn't exist or the Access Approval service account doesn't have the correct permissions on it, etc.) This key version is not necessarily the effective key version at this level, as key versions are inherited top-down. |
| `effective_approval_policy` | String | Output only. Effective policy applied for Access Approval, inclusive of inheritance. |
| `notification_emails` | Vec<String> | A list of email addresses to which notifications relating to approval requests should be sent. Notifications relating to a resource will be sent to all emails in the settings of ancestor resources of that resource. A maximum of 50 email addresses are allowed. |


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
organization_notification_pubsub_topic = organization.notification_pubsub_topic
organization_request_scope_max_width_preference = organization.request_scope_max_width_preference
organization_ancestor_has_active_key_version = organization.ancestor_has_active_key_version
organization_enrolled_services = organization.enrolled_services
organization_approval_policy = organization.approval_policy
organization_prefer_no_broad_approval_requests = organization.prefer_no_broad_approval_requests
organization_require_customer_visible_justification = organization.require_customer_visible_justification
organization_active_key_version = organization.active_key_version
organization_name = organization.name
organization_enrolled_ancestor = organization.enrolled_ancestor
organization_preferred_request_expiration_days = organization.preferred_request_expiration_days
organization_invalid_key_version = organization.invalid_key_version
organization_effective_approval_policy = organization.effective_approval_policy
organization_notification_emails = organization.notification_emails
```

---


### Folder

Retrieves the service account that is used by Access Approval to access KMS keys for signing approved approval requests.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_pubsub_topic` | String |  | Optional. A pubsub topic that notifications relating to access approval are published to. Notifications include pre-approved accesses. |
| `request_scope_max_width_preference` | String |  | Optional. A setting that indicates the maximum scope of an Access Approval request: either organization, folder, or project. Google administrators will be asked to send requests no broader than the configured scope. |
| `ancestor_has_active_key_version` | bool |  | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that an ancestor of this Project or Folder has set active_key_version (this field will always be unset for the organization since organizations do not have ancestors). |
| `enrolled_services` | Vec<String> |  | A list of Google Cloud Services for which the given resource has Access Approval enrolled. Access requests for the resource given by name against any of these services contained here will be required to have explicit approval. If name refers to an organization, enrollment can be done for individual services. If name refers to a folder or project, enrollment can only be done on an all or nothing basis. If a cloud_product is repeated in this list, the first entry will be honored and all following entries will be discarded. |
| `approval_policy` | String |  | Optional. Policy configuration for Access Approval that sets the operating mode. The available policies are Transparency, Streamlined Support, and Approval Required. |
| `prefer_no_broad_approval_requests` | bool |  | This field is used to set a preference for granularity of an access approval request. If true, Google personnel will be asked to send resource-level requests when possible. If false, Google personnel will be asked to send requests at the project level. |
| `require_customer_visible_justification` | bool |  | Optional. When enabled, Google will only be able to send approval requests for access reasons with a customer accessible case ID in the reason detail. Also known as "Require customer initiated support case justification" |
| `active_key_version` | String |  | The asymmetric crypto key version to use for signing approval requests. Empty active_key_version indicates that a Google-managed key should be used for signing. This property will be ignored if set by an ancestor of this resource, and new non-empty values may not be set. |
| `name` | String |  | The resource name of the settings. Format is one of: * "projects/{project}/accessApprovalSettings" * "folders/{folder}/accessApprovalSettings" * "organizations/{organization}/accessApprovalSettings" |
| `enrolled_ancestor` | bool |  | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that at least one service is enrolled for Access Approval in one or more ancestors of the Project or Folder (this field will always be unset for the organization since organizations do not have ancestors). |
| `preferred_request_expiration_days` | i64 |  | Set the default access approval request expiration time. This value is able to be set directly by the customer at the time of approval, overriding this suggested value. We recommend setting this value to 30 days. |
| `invalid_key_version` | bool |  | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that there is some configuration issue with the active_key_version configured at this level in the resource hierarchy (e.g. it doesn't exist or the Access Approval service account doesn't have the correct permissions on it, etc.) This key version is not necessarily the effective key version at this level, as key versions are inherited top-down. |
| `effective_approval_policy` | String |  | Output only. Effective policy applied for Access Approval, inclusive of inheritance. |
| `notification_emails` | Vec<String> |  | A list of email addresses to which notifications relating to approval requests should be sent. Notifications relating to a resource will be sent to all emails in the settings of ancestor resources of that resource. A maximum of 50 email addresses are allowed. |
| `name` | String | ✅ | The resource name of the settings. Format is one of: * "projects/{project}/accessApprovalSettings" * "folders/{folder}/accessApprovalSettings" * "organizations/{organization}/accessApprovalSettings" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the Access Approval service account. Format is one of: * "projects/{project}/serviceAccount" * "folders/{folder}/serviceAccount" * "organizations/{organization}/serviceAccount" |
| `account_email` | String | Email address of the service account. |


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
folder_account_email = folder.account_email
```

---


### Project

Retrieves the service account that is used by Access Approval to access KMS keys for signing approved approval requests.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_pubsub_topic` | String |  | Optional. A pubsub topic that notifications relating to access approval are published to. Notifications include pre-approved accesses. |
| `request_scope_max_width_preference` | String |  | Optional. A setting that indicates the maximum scope of an Access Approval request: either organization, folder, or project. Google administrators will be asked to send requests no broader than the configured scope. |
| `ancestor_has_active_key_version` | bool |  | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that an ancestor of this Project or Folder has set active_key_version (this field will always be unset for the organization since organizations do not have ancestors). |
| `enrolled_services` | Vec<String> |  | A list of Google Cloud Services for which the given resource has Access Approval enrolled. Access requests for the resource given by name against any of these services contained here will be required to have explicit approval. If name refers to an organization, enrollment can be done for individual services. If name refers to a folder or project, enrollment can only be done on an all or nothing basis. If a cloud_product is repeated in this list, the first entry will be honored and all following entries will be discarded. |
| `approval_policy` | String |  | Optional. Policy configuration for Access Approval that sets the operating mode. The available policies are Transparency, Streamlined Support, and Approval Required. |
| `prefer_no_broad_approval_requests` | bool |  | This field is used to set a preference for granularity of an access approval request. If true, Google personnel will be asked to send resource-level requests when possible. If false, Google personnel will be asked to send requests at the project level. |
| `require_customer_visible_justification` | bool |  | Optional. When enabled, Google will only be able to send approval requests for access reasons with a customer accessible case ID in the reason detail. Also known as "Require customer initiated support case justification" |
| `active_key_version` | String |  | The asymmetric crypto key version to use for signing approval requests. Empty active_key_version indicates that a Google-managed key should be used for signing. This property will be ignored if set by an ancestor of this resource, and new non-empty values may not be set. |
| `name` | String |  | The resource name of the settings. Format is one of: * "projects/{project}/accessApprovalSettings" * "folders/{folder}/accessApprovalSettings" * "organizations/{organization}/accessApprovalSettings" |
| `enrolled_ancestor` | bool |  | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that at least one service is enrolled for Access Approval in one or more ancestors of the Project or Folder (this field will always be unset for the organization since organizations do not have ancestors). |
| `preferred_request_expiration_days` | i64 |  | Set the default access approval request expiration time. This value is able to be set directly by the customer at the time of approval, overriding this suggested value. We recommend setting this value to 30 days. |
| `invalid_key_version` | bool |  | Output only. This field is read only (not settable via UpdateAccessApprovalSettings method). If the field is true, that indicates that there is some configuration issue with the active_key_version configured at this level in the resource hierarchy (e.g. it doesn't exist or the Access Approval service account doesn't have the correct permissions on it, etc.) This key version is not necessarily the effective key version at this level, as key versions are inherited top-down. |
| `effective_approval_policy` | String |  | Output only. Effective policy applied for Access Approval, inclusive of inheritance. |
| `notification_emails` | Vec<String> |  | A list of email addresses to which notifications relating to approval requests should be sent. Notifications relating to a resource will be sent to all emails in the settings of ancestor resources of that resource. A maximum of 50 email addresses are allowed. |
| `name` | String | ✅ | The resource name of the settings. Format is one of: * "projects/{project}/accessApprovalSettings" * "folders/{folder}/accessApprovalSettings" * "organizations/{organization}/accessApprovalSettings" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the Access Approval service account. Format is one of: * "projects/{project}/serviceAccount" * "folders/{folder}/serviceAccount" * "organizations/{organization}/serviceAccount" |
| `account_email` | String | Email address of the service account. |


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
project_account_email = project.account_email
```

---


### Approval_request

Approves a request and returns the updated ApprovalRequest. Returns NOT_FOUND if the request does not exist. Returns FAILED_PRECONDITION if the request exists but is not in a pending state.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expire_time` | String |  | The expiration time of this approval. |
| `name` | String | ✅ | Name of the approval request to approve. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `requested_duration` | String | The requested access duration. |
| `approve` | String | Access was approved. |
| `dismiss` | String | The request was dismissed. |
| `request_time` | String | The time at which approval was requested. |
| `requested_resource_name` | String | The resource for which approval is being requested. The format of the resource name is defined at https://cloud.google.com/apis/design/resource_names. The resource name here may either be a "full" resource name (e.g. "//library.googleapis.com/shelves/shelf1/books/book2") or a "relative" resource name (e.g. "shelves/shelf1/books/book2") as described in the resource name specification. |
| `requested_resource_properties` | String | Properties related to the resource represented by requested_resource_name. |
| `requested_expiration` | String | The original requested expiration for the approval. Calculated by adding the requested_duration to the request_time. |
| `requested_augmented_info` | String | This field contains the augmented information of the request. |
| `requested_locations` | String | The locations for which approval is being requested. |
| `name` | String | The resource name of the request. Format is "{projects|folders|organizations}/{id}/approvalRequests/{approval_request}". |
| `requested_reason` | String | The access reason for which approval is being requested. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create approval_request
approval_request = provider.accessapproval_api.Approval_request {
    name = "value"  # Name of the approval request to approve.
}

# Access approval_request outputs
approval_request_id = approval_request.id
approval_request_requested_duration = approval_request.requested_duration
approval_request_approve = approval_request.approve
approval_request_dismiss = approval_request.dismiss
approval_request_request_time = approval_request.request_time
approval_request_requested_resource_name = approval_request.requested_resource_name
approval_request_requested_resource_properties = approval_request.requested_resource_properties
approval_request_requested_expiration = approval_request.requested_expiration
approval_request_requested_augmented_info = approval_request.requested_augmented_info
approval_request_requested_locations = approval_request.requested_locations
approval_request_name = approval_request.name
approval_request_requested_reason = approval_request.requested_reason
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple organization resources
organization_0 = provider.accessapproval_api.Organization {
    name = "value-0"
}
organization_1 = provider.accessapproval_api.Organization {
    name = "value-1"
}
organization_2 = provider.accessapproval_api.Organization {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    organization = provider.accessapproval_api.Organization {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Accessapproval_api Documentation](https://cloud.google.com/accessapproval_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
