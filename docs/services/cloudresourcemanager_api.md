# Cloudresourcemanager_api Service



**Resources**: 24

---

## Overview

The cloudresourcemanager_api service provides access to 24 resource types:

- [Organization](#organization) [CR]
- [Project](#project) [CRUD]
- [Folder](#folder) [C]
- [Operation](#operation) [R]
- [Lien](#lien) [CRD]
- [Operation](#operation) [R]
- [Folder](#folder) [CRUD]
- [Folder](#folder) [CRUD]
- [Operation](#operation) [R]
- [Organization](#organization) [CRU]
- [Project](#project) [CRUD]
- [Effective_tag](#effective_tag) [R]
- [Operation](#operation) [R]
- [Tag_binding_collection](#tag_binding_collection) [RU]
- [Tag_value](#tag_value) [CRUD]
- [Tag_hold](#tag_hold) [CRD]
- [Lien](#lien) [CRD]
- [Effective_tag_binding_collection](#effective_tag_binding_collection) [R]
- [Tag_key](#tag_key) [CRUD]
- [Folder](#folder) [CRUD]
- [Capabilitie](#capabilitie) [RU]
- [Tag_binding](#tag_binding) [CRD]
- [Organization](#organization) [CR]
- [Project](#project) [CRUD]

---

## Resources


### Organization

Gets the access control policy for an Organization resource. May be empty if no such policy or resource exists. The `resource` field should be the organization's resource name, e.g. "organizations/123". Authorization requires the Google IAM permission `resourcemanager.organizations.getIamPolicy` on the specified organization

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `options` | String |  | OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `owner` | String | The owner of this Organization. The owner should be specified on creation. Once set, it cannot be changed. This field is required. |
| `name` | String | Output only. The resource name of the organization. This is the organization's relative path in the API. Its format is "organizations/[organization_id]". For example, "organizations/1234". |
| `display_name` | String | A human-readable string that refers to the Organization in the Google Cloud console. This string is set by the server and cannot be changed. The string will be set to the primary domain (for example, "google.com") of the G Suite customer that owns the organization. |
| `creation_time` | String | Timestamp when the Organization was created. Assigned by the server. |
| `lifecycle_state` | String | The organization's current lifecycle state. Assigned by the server. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create organization
organization = provider.cloudresourcemanager_api.Organization {
    resource = "value"  # REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access organization outputs
organization_id = organization.id
organization_owner = organization.owner
organization_name = organization.name
organization_display_name = organization.display_name
organization_creation_time = organization.creation_time
organization_lifecycle_state = organization.lifecycle_state
```

---


### Project

Request that a new Project be created. The result is an Operation which can be used to track the creation process. This process usually takes a few seconds, but can sometimes take much longer. The tracking Operation is automatically deleted after a few hours, so there is no need to call DeleteOperation. Authorization requires the Google IAM permission `resourcemanager.projects.create` on the specified parent for the new project. The parent is identified by a specified ResourceId, which must include both an ID and a type, such as organization. This method does not associate the new project with a billing account. You can set or update the billing account associated with a project using the [`projects.updateBillingInfo`] (/billing/reference/rest/v1/projects/updateBillingInfo) method.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String |  | An optional reference to a parent Resource. Supported parent types include "organization" and "folder". Once set, the parent cannot be cleared. The `parent` can be set on creation or using the `UpdateProject` method; the end user must have the `resourcemanager.projects.create` permission on the parent. |
| `configured_capabilities` | Vec<String> |  | Output only. If this project is a Management Project, list of capabilities configured on the parent folder. Note, presence of any capability implies that this is a Management Project. Example: `folders/123/capabilities/app-management`. OUTPUT ONLY. |
| `lifecycle_state` | String |  | The Project lifecycle state. Read-only. |
| `project_id` | String |  | The unique, user-assigned ID of the Project. It must be 6 to 30 lowercase letters, digits, or hyphens. It must start with a letter. Trailing hyphens are prohibited. Example: `tokyo-rain-123` Read-only after creation. |
| `project_number` | String |  | The number uniquely identifying the project. Example: `415104041262` Read-only. |
| `create_time` | String |  | Creation time. Read-only. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this project. Each item in the map must be expressed as " : ". For example: "123/environment" : "production", "123/costCenter" : "marketing" Note: Currently this field is in Preview. |
| `name` | String |  | The optional user-assigned display name of the Project. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `My Project` Read-write. |
| `labels` | HashMap<String, String> |  | The labels associated with this Project. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: a-z{0,62}. Label values must be between 0 and 63 characters long and must conform to the regular expression [a-z0-9_-]{0,63}. A label value can be empty. No more than 256 labels can be associated with a given resource. Clients should store labels in a representation such as JSON that does not depend on specific characters being disallowed. Example: "environment" : "dev" Read-write. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent` | String | An optional reference to a parent Resource. Supported parent types include "organization" and "folder". Once set, the parent cannot be cleared. The `parent` can be set on creation or using the `UpdateProject` method; the end user must have the `resourcemanager.projects.create` permission on the parent. |
| `configured_capabilities` | Vec<String> | Output only. If this project is a Management Project, list of capabilities configured on the parent folder. Note, presence of any capability implies that this is a Management Project. Example: `folders/123/capabilities/app-management`. OUTPUT ONLY. |
| `lifecycle_state` | String | The Project lifecycle state. Read-only. |
| `project_id` | String | The unique, user-assigned ID of the Project. It must be 6 to 30 lowercase letters, digits, or hyphens. It must start with a letter. Trailing hyphens are prohibited. Example: `tokyo-rain-123` Read-only after creation. |
| `project_number` | String | The number uniquely identifying the project. Example: `415104041262` Read-only. |
| `create_time` | String | Creation time. Read-only. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this project. Each item in the map must be expressed as " : ". For example: "123/environment" : "production", "123/costCenter" : "marketing" Note: Currently this field is in Preview. |
| `name` | String | The optional user-assigned display name of the Project. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `My Project` Read-write. |
| `labels` | HashMap<String, String> | The labels associated with this Project. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: a-z{0,62}. Label values must be between 0 and 63 characters long and must conform to the regular expression [a-z0-9_-]{0,63}. A label value can be empty. No more than 256 labels can be associated with a given resource. Clients should store labels in a representation such as JSON that does not depend on specific characters being disallowed. Example: "environment" : "dev" Read-write. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.cloudresourcemanager_api.Project {
}

# Access project outputs
project_id = project.id
project_parent = project.parent
project_configured_capabilities = project.configured_capabilities
project_lifecycle_state = project.lifecycle_state
project_project_id = project.project_id
project_project_number = project.project_number
project_create_time = project.create_time
project_tags = project.tags
project_name = project.name
project_labels = project.labels
```

---


### Folder

Updates the specified `Policy` on the resource. Creates a new `Policy` for that `Constraint` on the resource if one does not exist. Not supplying an `etag` on the request `Policy` results in an unconditional write of the `Policy`.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | `Policy` to set on the resource. |
| `resource` | String | ✅ | Resource name of the resource to attach the `Policy`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create folder
folder = provider.cloudresourcemanager_api.Folder {
    resource = "value"  # Resource name of the resource to attach the `Policy`.
}

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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


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
operation_name = operation.name
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
```

---


### Lien

Create a Lien which applies to the resource denoted by the `parent` field. Callers of this method will require permission on the `parent` resource. For example, applying to `projects/1234` requires permission `resourcemanager.projects.updateLiens`. NOTE: Some resources may limit the number of Liens which may be applied.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `restrictions` | Vec<String> |  | The types of operations which should be blocked as a result of this Lien. Each value should correspond to an IAM permission. The server will validate the permissions against those for which Liens are supported. An empty list is meaningless and will be rejected. Example: ['resourcemanager.projects.delete'] |
| `reason` | String |  | Concise user-visible strings indicating why an action cannot be performed on a resource. Maximum length of 200 characters. Example: 'Holds production API key' |
| `parent` | String |  | A reference to the resource this Lien is attached to. The server will validate the parent against those for which Liens are supported. Example: `projects/1234` |
| `name` | String |  | A system-generated unique identifier for this Lien. Example: `liens/1234abcd` |
| `origin` | String |  | A stable, user-visible/meaningful string identifying the origin of the Lien, intended to be inspected programmatically. Maximum length of 200 characters. Example: 'compute.googleapis.com' |
| `create_time` | String |  | The creation time of this Lien. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `restrictions` | Vec<String> | The types of operations which should be blocked as a result of this Lien. Each value should correspond to an IAM permission. The server will validate the permissions against those for which Liens are supported. An empty list is meaningless and will be rejected. Example: ['resourcemanager.projects.delete'] |
| `reason` | String | Concise user-visible strings indicating why an action cannot be performed on a resource. Maximum length of 200 characters. Example: 'Holds production API key' |
| `parent` | String | A reference to the resource this Lien is attached to. The server will validate the parent against those for which Liens are supported. Example: `projects/1234` |
| `name` | String | A system-generated unique identifier for this Lien. Example: `liens/1234abcd` |
| `origin` | String | A stable, user-visible/meaningful string identifying the origin of the Lien, intended to be inspected programmatically. Maximum length of 200 characters. Example: 'compute.googleapis.com' |
| `create_time` | String | The creation time of this Lien. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lien
lien = provider.cloudresourcemanager_api.Lien {
}

# Access lien outputs
lien_id = lien.id
lien_restrictions = lien.restrictions
lien_reason = lien.reason
lien_parent = lien.parent
lien_name = lien.name
lien_origin = lien.origin
lien_create_time = lien.create_time
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


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
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
```

---


### Folder

Creates a Folder in the resource hierarchy. Returns an Operation which can be used to track the progress of the folder creation workflow. Upon success the Operation.response field will be populated with the created Folder. In order to succeed, the addition of this new Folder must not violate the Folder naming, height or fanout constraints. + The Folder's display_name must be distinct from all other Folders that share its parent. + The addition of the Folder must not cause the active Folder hierarchy to exceed a height of 10. Note, the full active + deleted Folder hierarchy is allowed to reach a height of 20; this provides additional headroom when moving folders that contain deleted folders. + The addition of the Folder must not cause the total number of Folders under its parent to exceed 300. If the operation fails due to a folder constraint violation, some errors may be returned by the CreateFolder request, with status code FAILED_PRECONDITION and an error description. Other folder constraint violations will be communicated in the Operation, with the specific PreconditionFailure returned via the details list in the Operation.error field. The caller must have `resourcemanager.folders.create` permission on the identified parent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lifecycle_state` | String |  | Output only. The lifecycle state of the folder. Updates to the lifecycle_state must be performed via DeleteFolder and UndeleteFolder. |
| `management_project` | String |  | Output only. Management Project associated with this folder (if app-management capability is enabled). Example: `projects/google-mp-123` OUTPUT ONLY. |
| `create_time` | String |  | Output only. Timestamp when the Folder was created. Assigned by the server. |
| `display_name` | String |  | The folder's display name. A folder's display name must be unique amongst its siblings, e.g. no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters. This is captured by the regular expression: `[\p{L}\p{N}]([\p{L}\p{N}_- ]{0,28}[\p{L}\p{N}])?`. |
| `configured_capabilities` | Vec<String> |  | Output only. Optional capabilities configured for this folder (via UpdateCapability API). Example: `folders/123/capabilities/app-management`. |
| `parent` | String |  | Required. The Folder's parent's resource name. Updates to the folder's parent must be performed via MoveFolder. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this folder. Each item in the map must be expressed as " : ". For example: "123/environment" : "production", "123/costCenter" : "marketing" Note: Currently this field is in Preview. |
| `name` | String |  | Output only. The resource name of the Folder. Its format is `folders/{folder_id}`, for example: "folders/1234". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lifecycle_state` | String | Output only. The lifecycle state of the folder. Updates to the lifecycle_state must be performed via DeleteFolder and UndeleteFolder. |
| `management_project` | String | Output only. Management Project associated with this folder (if app-management capability is enabled). Example: `projects/google-mp-123` OUTPUT ONLY. |
| `create_time` | String | Output only. Timestamp when the Folder was created. Assigned by the server. |
| `display_name` | String | The folder's display name. A folder's display name must be unique amongst its siblings, e.g. no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters. This is captured by the regular expression: `[\p{L}\p{N}]([\p{L}\p{N}_- ]{0,28}[\p{L}\p{N}])?`. |
| `configured_capabilities` | Vec<String> | Output only. Optional capabilities configured for this folder (via UpdateCapability API). Example: `folders/123/capabilities/app-management`. |
| `parent` | String | Required. The Folder's parent's resource name. Updates to the folder's parent must be performed via MoveFolder. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this folder. Each item in the map must be expressed as " : ". For example: "123/environment" : "production", "123/costCenter" : "marketing" Note: Currently this field is in Preview. |
| `name` | String | Output only. The resource name of the Folder. Its format is `folders/{folder_id}`, for example: "folders/1234". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create folder
folder = provider.cloudresourcemanager_api.Folder {
}

# Access folder outputs
folder_id = folder.id
folder_lifecycle_state = folder.lifecycle_state
folder_management_project = folder.management_project
folder_create_time = folder.create_time
folder_display_name = folder.display_name
folder_configured_capabilities = folder.configured_capabilities
folder_parent = folder.parent
folder_tags = folder.tags
folder_name = folder.name
```

---


### Folder

Creates a Folder in the resource hierarchy. Returns an Operation which can be used to track the progress of the folder creation workflow. Upon success the Operation.response field will be populated with the created Folder. In order to succeed, the addition of this new Folder must not violate the Folder naming, height or fanout constraints. + The Folder's display_name must be distinct from all other Folders that share its parent. + The addition of the Folder must not cause the active Folder hierarchy to exceed a height of 10. Note, the full active + deleted Folder hierarchy is allowed to reach a height of 20; this provides additional headroom when moving folders that contain deleted folders. + The addition of the Folder must not cause the total number of Folders under its parent to exceed 300. If the operation fails due to a folder constraint violation, some errors may be returned by the CreateFolder request, with status code FAILED_PRECONDITION and an error description. Other folder constraint violations will be communicated in the Operation, with the specific PreconditionFailure returned via the details list in the Operation.error field. The caller must have `resourcemanager.folders.create` permission on the identified parent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The folder's display name. A folder's display name must be unique amongst its siblings, e.g. no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters. This is captured by the regular expression: `[\p{L}\p{N}]([\p{L}\p{N}_- ]{0,28}[\p{L}\p{N}])?`. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this folder. Each item in the map must be expressed as " : ". For example: "123/environment" : "production", "123/costCenter" : "marketing" Note: Currently this field is in Preview. |
| `create_time` | String |  | Output only. Timestamp when the Folder was created. Assigned by the server. |
| `lifecycle_state` | String |  | Output only. The lifecycle state of the folder. Updates to the lifecycle_state must be performed via DeleteFolder and UndeleteFolder. |
| `name` | String |  | Output only. The resource name of the Folder. Its format is `folders/{folder_id}`, for example: "folders/1234". |
| `management_project` | String |  | Output only. Management Project associated with this folder (if app-management capability is enabled). Example: `projects/google-mp-123` OUTPUT ONLY. |
| `configured_capabilities` | Vec<String> |  | Output only. Optional capabilities configured for this folder (via UpdateCapability API). Example: `folders/123/capabilities/app-management`. |
| `parent` | String |  | Required. The Folder's parent's resource name. Updates to the folder's parent must be performed via MoveFolder. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The folder's display name. A folder's display name must be unique amongst its siblings, e.g. no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters. This is captured by the regular expression: `[\p{L}\p{N}]([\p{L}\p{N}_- ]{0,28}[\p{L}\p{N}])?`. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this folder. Each item in the map must be expressed as " : ". For example: "123/environment" : "production", "123/costCenter" : "marketing" Note: Currently this field is in Preview. |
| `create_time` | String | Output only. Timestamp when the Folder was created. Assigned by the server. |
| `lifecycle_state` | String | Output only. The lifecycle state of the folder. Updates to the lifecycle_state must be performed via DeleteFolder and UndeleteFolder. |
| `name` | String | Output only. The resource name of the Folder. Its format is `folders/{folder_id}`, for example: "folders/1234". |
| `management_project` | String | Output only. Management Project associated with this folder (if app-management capability is enabled). Example: `projects/google-mp-123` OUTPUT ONLY. |
| `configured_capabilities` | Vec<String> | Output only. Optional capabilities configured for this folder (via UpdateCapability API). Example: `folders/123/capabilities/app-management`. |
| `parent` | String | Required. The Folder's parent's resource name. Updates to the folder's parent must be performed via MoveFolder. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create folder
folder = provider.cloudresourcemanager_api.Folder {
}

# Access folder outputs
folder_id = folder.id
folder_display_name = folder.display_name
folder_tags = folder.tags
folder_create_time = folder.create_time
folder_lifecycle_state = folder.lifecycle_state
folder_name = folder.name
folder_management_project = folder.management_project
folder_configured_capabilities = folder.configured_capabilities
folder_parent = folder.parent
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Organization

Returns permissions that a caller has on the specified Organization. The `resource` field should be the organization's resource name, e.g. "organizations/123".

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `owner` | String | The owner of this Organization. The owner should be specified on creation. Once set, it cannot be changed. This field is required. |
| `creation_time` | String | Timestamp when the Organization was created. Assigned by the server. |
| `name` | String | Output only. The resource name of the organization. This is the organization's relative path in the API. Its format is "organizations/[organization_id]". For example, "organizations/1234". |
| `lifecycle_state` | String | The organization's current lifecycle state. Assigned by the server. |
| `display_name` | String | A human-readable string that refers to the Organization in the Google Cloud console. This string is set by the server and cannot be changed. The string will be set to the primary domain (for example, "google.com") of the G Suite customer that owns the organization. |
| `organization_id` | String | An immutable id for the Organization that is assigned on creation. This should be omitted when creating a new Organization. This field is read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create organization
organization = provider.cloudresourcemanager_api.Organization {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access organization outputs
organization_id = organization.id
organization_owner = organization.owner
organization_creation_time = organization.creation_time
organization_name = organization.name
organization_lifecycle_state = organization.lifecycle_state
organization_display_name = organization.display_name
organization_organization_id = organization.organization_id
```

---


### Project

Creates a Project resource. Initially, the Project resource is owned by its creator exclusively. The creator can later grant permission to others to read or update the Project. Several APIs are activated automatically for the Project, including Google Cloud Storage. The parent is identified by a specified ResourceId, which must include both an ID and a type, such as project, folder, or organization. This method does not associate the new project with a billing account. You can set or update the billing account associated with a project using the [`projects.updateBillingInfo`] (/billing/reference/rest/v1/projects/updateBillingInfo) method.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The optional user-assigned display name of the Project. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `My Project` Read-write. |
| `parent` | String |  | An optional reference to a parent Resource. Supported parent types include "organization" and "folder". Once set, the parent cannot be cleared. The `parent` can be set on creation or using the `UpdateProject` method; the end user must have the `resourcemanager.projects.create` permission on the parent. Read-write. |
| `create_time` | String |  | Creation time. Read-only. |
| `project_number` | String |  | The number uniquely identifying the project. Example: `415104041262` Read-only. |
| `lifecycle_state` | String |  | The Project lifecycle state. Read-only. |
| `project_id` | String |  | The unique, user-assigned ID of the Project. It must be 6 to 30 lowercase letters, digits, or hyphens. It must start with a letter. Trailing hyphens are prohibited. Example: `tokyo-rain-123` Read-only after creation. |
| `labels` | HashMap<String, String> |  | The labels associated with this Project. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: a-z{0,62}. Label values must be between 0 and 63 characters long and must conform to the regular expression [a-z0-9_-]{0,63}. A label value can be empty. No more than 256 labels can be associated with a given resource. Clients should store labels in a representation such as JSON that does not depend on specific characters being disallowed. Example: `"environment" : "dev"` Read-write. |
| `configured_capabilities` | Vec<String> |  | Output only. If this project is a Management Project, list of capabilities configured on the parent folder. Note, presence of any capability implies that this is a Management Project. Example: `folders/123/capabilities/app-management`. OUTPUT ONLY. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The optional user-assigned display name of the Project. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `My Project` Read-write. |
| `parent` | String | An optional reference to a parent Resource. Supported parent types include "organization" and "folder". Once set, the parent cannot be cleared. The `parent` can be set on creation or using the `UpdateProject` method; the end user must have the `resourcemanager.projects.create` permission on the parent. Read-write. |
| `create_time` | String | Creation time. Read-only. |
| `project_number` | String | The number uniquely identifying the project. Example: `415104041262` Read-only. |
| `lifecycle_state` | String | The Project lifecycle state. Read-only. |
| `project_id` | String | The unique, user-assigned ID of the Project. It must be 6 to 30 lowercase letters, digits, or hyphens. It must start with a letter. Trailing hyphens are prohibited. Example: `tokyo-rain-123` Read-only after creation. |
| `labels` | HashMap<String, String> | The labels associated with this Project. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: a-z{0,62}. Label values must be between 0 and 63 characters long and must conform to the regular expression [a-z0-9_-]{0,63}. A label value can be empty. No more than 256 labels can be associated with a given resource. Clients should store labels in a representation such as JSON that does not depend on specific characters being disallowed. Example: `"environment" : "dev"` Read-write. |
| `configured_capabilities` | Vec<String> | Output only. If this project is a Management Project, list of capabilities configured on the parent folder. Note, presence of any capability implies that this is a Management Project. Example: `folders/123/capabilities/app-management`. OUTPUT ONLY. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.cloudresourcemanager_api.Project {
}

# Access project outputs
project_id = project.id
project_name = project.name
project_parent = project.parent
project_create_time = project.create_time
project_project_number = project.project_number
project_lifecycle_state = project.lifecycle_state
project_project_id = project.project_id
project_labels = project.labels
project_configured_capabilities = project.configured_capabilities
```

---


### Effective_tag

Return a list of effective tags for the given Google Cloud resource, as specified in `parent`.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Pagination token. If the result set is too large to fit in a single response, this token is returned. It encodes the position of the current result cursor. Feeding this value into a new list request with the `page_token` parameter gives the next page of the results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. Pagination tokens have a limited lifetime. |
| `effective_tags` | Vec<String> | A possibly paginated list of effective tags for the specified resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access effective_tag outputs
effective_tag_id = effective_tag.id
effective_tag_next_page_token = effective_tag.next_page_token
effective_tag_effective_tags = effective_tag.effective_tags
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |


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
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
```

---


### Tag_binding_collection

Returns tag bindings directly attached to a GCP resource.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `full_resource_name` | String |  | The full resource name of the resource the TagBindings are bound to. E.g. `//cloudresourcemanager.googleapis.com/projects/123` |
| `etag` | String |  | Optional. A checksum based on the current bindings which can be passed to prevent race conditions. This field is always set in server responses. |
| `tags` | HashMap<String, String> |  | Tag keys/values directly bound to this resource, specified in namespaced format. For example: "123/environment": "production" |
| `name` | String |  | Identifier. The name of the TagBindingCollection, following the convention: `locations/{location}/tagBindingCollections/{encoded-full-resource-name}` where the encoded-full-resource-name is the UTF-8 encoded name of the GCP resource the TagBindings are bound to. "locations/global/tagBindingCollections/%2f%2fcloudresourcemanager.googleapis.com%2fprojects%2f123" |
| `name` | String | ✅ | Identifier. The name of the TagBindingCollection, following the convention: `locations/{location}/tagBindingCollections/{encoded-full-resource-name}` where the encoded-full-resource-name is the UTF-8 encoded name of the GCP resource the TagBindings are bound to. "locations/global/tagBindingCollections/%2f%2fcloudresourcemanager.googleapis.com%2fprojects%2f123" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `full_resource_name` | String | The full resource name of the resource the TagBindings are bound to. E.g. `//cloudresourcemanager.googleapis.com/projects/123` |
| `etag` | String | Optional. A checksum based on the current bindings which can be passed to prevent race conditions. This field is always set in server responses. |
| `tags` | HashMap<String, String> | Tag keys/values directly bound to this resource, specified in namespaced format. For example: "123/environment": "production" |
| `name` | String | Identifier. The name of the TagBindingCollection, following the convention: `locations/{location}/tagBindingCollections/{encoded-full-resource-name}` where the encoded-full-resource-name is the UTF-8 encoded name of the GCP resource the TagBindings are bound to. "locations/global/tagBindingCollections/%2f%2fcloudresourcemanager.googleapis.com%2fprojects%2f123" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access tag_binding_collection outputs
tag_binding_collection_id = tag_binding_collection.id
tag_binding_collection_full_resource_name = tag_binding_collection.full_resource_name
tag_binding_collection_etag = tag_binding_collection.etag
tag_binding_collection_tags = tag_binding_collection.tags
tag_binding_collection_name = tag_binding_collection.name
```

---


### Tag_value

Creates a TagValue as a child of the specified TagKey. If a another request with the same parameters is sent while the original request is in process the second request will receive an error. A maximum of 1000 TagValues can exist under a TagKey at any given time.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Creation time. |
| `etag` | String |  | Optional. Entity tag which users can pass to prevent race conditions. This field is always set in server responses. See UpdateTagValueRequest for details. |
| `name` | String |  | Immutable. Resource name for TagValue in the format `tagValues/456`. |
| `description` | String |  | Optional. User-assigned description of the TagValue. Must not exceed 256 characters. Read-write. |
| `short_name` | String |  | Required. Immutable. User-assigned short name for TagValue. The short name should be unique for TagValues within the same parent TagKey. The short name must be 256 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `namespaced_name` | String |  | Output only. The namespaced name of the TagValue. Can be in the form `{organization_id}/{tag_key_short_name}/{tag_value_short_name}` or `{project_id}/{tag_key_short_name}/{tag_value_short_name}` or `{project_number}/{tag_key_short_name}/{tag_value_short_name}`. |
| `parent` | String |  | Immutable. The resource name of the new TagValue's parent TagKey. Must be of the form `tagKeys/{tag_key_id}`. |
| `update_time` | String |  | Output only. Update time. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Creation time. |
| `etag` | String | Optional. Entity tag which users can pass to prevent race conditions. This field is always set in server responses. See UpdateTagValueRequest for details. |
| `name` | String | Immutable. Resource name for TagValue in the format `tagValues/456`. |
| `description` | String | Optional. User-assigned description of the TagValue. Must not exceed 256 characters. Read-write. |
| `short_name` | String | Required. Immutable. User-assigned short name for TagValue. The short name should be unique for TagValues within the same parent TagKey. The short name must be 256 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `namespaced_name` | String | Output only. The namespaced name of the TagValue. Can be in the form `{organization_id}/{tag_key_short_name}/{tag_value_short_name}` or `{project_id}/{tag_key_short_name}/{tag_value_short_name}` or `{project_number}/{tag_key_short_name}/{tag_value_short_name}`. |
| `parent` | String | Immutable. The resource name of the new TagValue's parent TagKey. Must be of the form `tagKeys/{tag_key_id}`. |
| `update_time` | String | Output only. Update time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag_value
tag_value = provider.cloudresourcemanager_api.Tag_value {
}

# Access tag_value outputs
tag_value_id = tag_value.id
tag_value_create_time = tag_value.create_time
tag_value_etag = tag_value.etag
tag_value_name = tag_value.name
tag_value_description = tag_value.description
tag_value_short_name = tag_value.short_name
tag_value_namespaced_name = tag_value.namespaced_name
tag_value_parent = tag_value.parent
tag_value_update_time = tag_value.update_time
```

---


### Tag_hold

Creates a TagHold. Returns ALREADY_EXISTS if a TagHold with the same resource and origin exists under the same TagValue.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `help_link` | String |  | Optional. A URL where an end user can learn more about removing this hold. E.g. `https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing` |
| `create_time` | String |  | Output only. The time this TagHold was created. |
| `name` | String |  | Output only. The resource name of a TagHold. This is a String of the form: `tagValues/{tag-value-id}/tagHolds/{tag-hold-id}` (e.g. `tagValues/123/tagHolds/456`). This resource name is generated by the server. |
| `holder` | String |  | Required. The name of the resource where the TagValue is being used. Must be less than 200 characters. E.g. `//compute.googleapis.com/compute/projects/myproject/regions/us-east-1/instanceGroupManagers/instance-group` |
| `origin` | String |  | Optional. An optional string representing the origin of this request. This field should include human-understandable information to distinguish origins from each other. Must be less than 200 characters. E.g. `migs-35678234` |
| `parent` | String | ✅ | Required. The resource name of the TagHold's parent TagValue. Must be of the form: `tagValues/{tag-value-id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Pagination token. If the result set is too large to fit in a single response, this token is returned. It encodes the position of the current result cursor. Feeding this value into a new list request with the `page_token` parameter gives the next page of the results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. Pagination tokens have a limited lifetime. |
| `tag_holds` | Vec<String> | A possibly paginated list of TagHolds. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag_hold
tag_hold = provider.cloudresourcemanager_api.Tag_hold {
    parent = "value"  # Required. The resource name of the TagHold's parent TagValue. Must be of the form: `tagValues/{tag-value-id}`.
}

# Access tag_hold outputs
tag_hold_id = tag_hold.id
tag_hold_next_page_token = tag_hold.next_page_token
tag_hold_tag_holds = tag_hold.tag_holds
```

---


### Lien

Create a Lien which applies to the resource denoted by the `parent` field. Callers of this method will require permission on the `parent` resource. For example, applying to `projects/1234` requires permission `resourcemanager.projects.updateLiens`. NOTE: Some resources may limit the number of Liens which may be applied.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `origin` | String |  | A stable, user-visible/meaningful string identifying the origin of the Lien, intended to be inspected programmatically. Maximum length of 200 characters. Example: 'compute.googleapis.com' |
| `parent` | String |  | A reference to the resource this Lien is attached to. The server will validate the parent against those for which Liens are supported. Example: `projects/1234` |
| `name` | String |  | A system-generated unique identifier for this Lien. Example: `liens/1234abcd` |
| `reason` | String |  | Concise user-visible strings indicating why an action cannot be performed on a resource. Maximum length of 200 characters. Example: 'Holds production API key' |
| `restrictions` | Vec<String> |  | The types of operations which should be blocked as a result of this Lien. Each value should correspond to an IAM permission. The server will validate the permissions against those for which Liens are supported. An empty list is meaningless and will be rejected. Example: ['resourcemanager.projects.delete'] |
| `create_time` | String |  | The creation time of this Lien. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `origin` | String | A stable, user-visible/meaningful string identifying the origin of the Lien, intended to be inspected programmatically. Maximum length of 200 characters. Example: 'compute.googleapis.com' |
| `parent` | String | A reference to the resource this Lien is attached to. The server will validate the parent against those for which Liens are supported. Example: `projects/1234` |
| `name` | String | A system-generated unique identifier for this Lien. Example: `liens/1234abcd` |
| `reason` | String | Concise user-visible strings indicating why an action cannot be performed on a resource. Maximum length of 200 characters. Example: 'Holds production API key' |
| `restrictions` | Vec<String> | The types of operations which should be blocked as a result of this Lien. Each value should correspond to an IAM permission. The server will validate the permissions against those for which Liens are supported. An empty list is meaningless and will be rejected. Example: ['resourcemanager.projects.delete'] |
| `create_time` | String | The creation time of this Lien. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lien
lien = provider.cloudresourcemanager_api.Lien {
}

# Access lien outputs
lien_id = lien.id
lien_origin = lien.origin
lien_parent = lien.parent
lien_name = lien.name
lien_reason = lien.reason
lien_restrictions = lien.restrictions
lien_create_time = lien.create_time
```

---


### Effective_tag_binding_collection

Returns effective tag bindings on a GCP resource.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The name of the EffectiveTagBindingCollection, following the convention: `locations/{location}/effectiveTagBindingCollections/{encoded-full-resource-name}` where the encoded-full-resource-name is the UTF-8 encoded name of the GCP resource the TagBindings are bound to. E.g. "locations/global/effectiveTagBindingCollections/%2f%2fcloudresourcemanager.googleapis.com%2fprojects%2f123" |
| `effective_tags` | HashMap<String, String> | Tag keys/values effectively bound to this resource, specified in namespaced format. For example: "123/environment": "production" |
| `full_resource_name` | String | The full resource name of the resource the TagBindings are bound to. E.g. `//cloudresourcemanager.googleapis.com/projects/123` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access effective_tag_binding_collection outputs
effective_tag_binding_collection_id = effective_tag_binding_collection.id
effective_tag_binding_collection_name = effective_tag_binding_collection.name
effective_tag_binding_collection_effective_tags = effective_tag_binding_collection.effective_tags
effective_tag_binding_collection_full_resource_name = effective_tag_binding_collection.full_resource_name
```

---


### Tag_key

Creates a new TagKey. If another request with the same parameters is sent while the original request is in process, the second request will receive an error. A maximum of 1000 TagKeys can exist under a parent at any given time.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Creation time. |
| `short_name` | String |  | Required. Immutable. The user friendly name for a TagKey. The short name should be unique for TagKeys within the same tag namespace. The short name must be 1-256 characters, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `name` | String |  | Immutable. The resource name for a TagKey. Must be in the format `tagKeys/{tag_key_id}`, where `tag_key_id` is the generated numeric id for the TagKey. |
| `etag` | String |  | Optional. Entity tag which users can pass to prevent race conditions. This field is always set in server responses. See UpdateTagKeyRequest for details. |
| `parent` | String |  | Immutable. The resource name of the TagKey's parent. A TagKey can be parented by an Organization or a Project. For a TagKey parented by an Organization, its parent must be in the form `organizations/{org_id}`. For a TagKey parented by a Project, its parent can be in the form `projects/{project_id}` or `projects/{project_number}`. |
| `namespaced_name` | String |  | Output only. Immutable. Namespaced name of the TagKey. |
| `purpose_data` | HashMap<String, String> |  | Optional. Purpose data corresponds to the policy system that the tag is intended for. See documentation for `Purpose` for formatting of this field. Purpose data cannot be changed once set. |
| `purpose` | String |  | Optional. A purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag. A purpose does not grant a policy engine exclusive rights to the Tag, and it may be referenced by other policy engines. A purpose cannot be changed once set. |
| `update_time` | String |  | Output only. Update time. |
| `description` | String |  | Optional. User-assigned description of the TagKey. Must not exceed 256 characters. Read-write. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Creation time. |
| `short_name` | String | Required. Immutable. The user friendly name for a TagKey. The short name should be unique for TagKeys within the same tag namespace. The short name must be 1-256 characters, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `name` | String | Immutable. The resource name for a TagKey. Must be in the format `tagKeys/{tag_key_id}`, where `tag_key_id` is the generated numeric id for the TagKey. |
| `etag` | String | Optional. Entity tag which users can pass to prevent race conditions. This field is always set in server responses. See UpdateTagKeyRequest for details. |
| `parent` | String | Immutable. The resource name of the TagKey's parent. A TagKey can be parented by an Organization or a Project. For a TagKey parented by an Organization, its parent must be in the form `organizations/{org_id}`. For a TagKey parented by a Project, its parent can be in the form `projects/{project_id}` or `projects/{project_number}`. |
| `namespaced_name` | String | Output only. Immutable. Namespaced name of the TagKey. |
| `purpose_data` | HashMap<String, String> | Optional. Purpose data corresponds to the policy system that the tag is intended for. See documentation for `Purpose` for formatting of this field. Purpose data cannot be changed once set. |
| `purpose` | String | Optional. A purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag. A purpose does not grant a policy engine exclusive rights to the Tag, and it may be referenced by other policy engines. A purpose cannot be changed once set. |
| `update_time` | String | Output only. Update time. |
| `description` | String | Optional. User-assigned description of the TagKey. Must not exceed 256 characters. Read-write. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag_key
tag_key = provider.cloudresourcemanager_api.Tag_key {
}

# Access tag_key outputs
tag_key_id = tag_key.id
tag_key_create_time = tag_key.create_time
tag_key_short_name = tag_key.short_name
tag_key_name = tag_key.name
tag_key_etag = tag_key.etag
tag_key_parent = tag_key.parent
tag_key_namespaced_name = tag_key.namespaced_name
tag_key_purpose_data = tag_key.purpose_data
tag_key_purpose = tag_key.purpose
tag_key_update_time = tag_key.update_time
tag_key_description = tag_key.description
```

---


### Folder

Creates a folder in the resource hierarchy. Returns an `Operation` which can be used to track the progress of the folder creation workflow. Upon success, the `Operation.response` field will be populated with the created Folder. In order to succeed, the addition of this new folder must not violate the folder naming, height, or fanout constraints. + The folder's `display_name` must be distinct from all other folders that share its parent. + The addition of the folder must not cause the active folder hierarchy to exceed a height of 10. Note, the full active + deleted folder hierarchy is allowed to reach a height of 20; this provides additional headroom when moving folders that contain deleted folders. + The addition of the folder must not cause the total number of folders under its parent to exceed 300. If the operation fails due to a folder constraint violation, some errors may be returned by the `CreateFolder` request, with status code `FAILED_PRECONDITION` and an error description. Other folder constraint violations will be communicated in the `Operation`, with the specific `PreconditionFailure` returned in the details list in the `Operation.error` field. The caller must have `resourcemanager.folders.create` permission on the identified parent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Output only. A checksum computed by the server based on the current value of the folder resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `create_time` | String |  | Output only. Timestamp when the folder was created. |
| `state` | String |  | Output only. The lifecycle state of the folder. Updates to the state must be performed using DeleteFolder and UndeleteFolder. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this folder. Each item in the map must be expressed as " : ". For example: "123/environment" : "production", "123/costCenter" : "marketing" Note: Currently this field is in Preview. |
| `delete_time` | String |  | Output only. Timestamp when the folder was requested to be deleted. |
| `update_time` | String |  | Output only. Timestamp when the folder was last modified. |
| `parent` | String |  | Required. The folder's parent's resource name. Updates to the folder's parent must be performed using MoveFolder. |
| `configured_capabilities` | Vec<String> |  | Output only. Optional capabilities configured for this folder (via UpdateCapability API). Example: `folders/123/capabilities/app-management`. |
| `name` | String |  | Identifier. The resource name of the folder. Its format is `folders/{folder_id}`, for example: "folders/1234". |
| `management_project` | String |  | Output only. Management Project associated with this folder (if app-management capability is enabled). Example: `projects/google-mp-123` OUTPUT ONLY. |
| `display_name` | String |  | The folder's display name. A folder's display name must be unique amongst its siblings. For example, no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters. This is captured by the regular expression: `[\p{L}\p{N}]([\p{L}\p{N}_- ]{0,28}[\p{L}\p{N}])?`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. A checksum computed by the server based on the current value of the folder resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `create_time` | String | Output only. Timestamp when the folder was created. |
| `state` | String | Output only. The lifecycle state of the folder. Updates to the state must be performed using DeleteFolder and UndeleteFolder. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this folder. Each item in the map must be expressed as " : ". For example: "123/environment" : "production", "123/costCenter" : "marketing" Note: Currently this field is in Preview. |
| `delete_time` | String | Output only. Timestamp when the folder was requested to be deleted. |
| `update_time` | String | Output only. Timestamp when the folder was last modified. |
| `parent` | String | Required. The folder's parent's resource name. Updates to the folder's parent must be performed using MoveFolder. |
| `configured_capabilities` | Vec<String> | Output only. Optional capabilities configured for this folder (via UpdateCapability API). Example: `folders/123/capabilities/app-management`. |
| `name` | String | Identifier. The resource name of the folder. Its format is `folders/{folder_id}`, for example: "folders/1234". |
| `management_project` | String | Output only. Management Project associated with this folder (if app-management capability is enabled). Example: `projects/google-mp-123` OUTPUT ONLY. |
| `display_name` | String | The folder's display name. A folder's display name must be unique amongst its siblings. For example, no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters. This is captured by the regular expression: `[\p{L}\p{N}]([\p{L}\p{N}_- ]{0,28}[\p{L}\p{N}])?`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create folder
folder = provider.cloudresourcemanager_api.Folder {
}

# Access folder outputs
folder_id = folder.id
folder_etag = folder.etag
folder_create_time = folder.create_time
folder_state = folder.state
folder_tags = folder.tags
folder_delete_time = folder.delete_time
folder_update_time = folder.update_time
folder_parent = folder.parent
folder_configured_capabilities = folder.configured_capabilities
folder_name = folder.name
folder_management_project = folder.management_project
folder_display_name = folder.display_name
```

---


### Capabilitie

Retrieves the Capability identified by the supplied resource name.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. Identifier. The resource name of the capability. Must be in the following form: * `folders/{folder_id}/capabilities/{capability_name}` For example, `folders/123/capabilities/app-management` Following are the allowed {capability_name} values: * `app-management` |
| `value` | bool |  | Required. The configured value of the capability at the given parent resource. |
| `name` | String | ✅ | Immutable. Identifier. The resource name of the capability. Must be in the following form: * `folders/{folder_id}/capabilities/{capability_name}` For example, `folders/123/capabilities/app-management` Following are the allowed {capability_name} values: * `app-management` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Identifier. The resource name of the capability. Must be in the following form: * `folders/{folder_id}/capabilities/{capability_name}` For example, `folders/123/capabilities/app-management` Following are the allowed {capability_name} values: * `app-management` |
| `value` | bool | Required. The configured value of the capability at the given parent resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access capabilitie outputs
capabilitie_id = capabilitie.id
capabilitie_name = capabilitie.name
capabilitie_value = capabilitie.value
```

---


### Tag_binding

Creates a TagBinding between a TagValue and a Google Cloud resource.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of the TagBinding. This is a String of the form: `tagBindings/{full-resource-name}/{tag-value-name}` (e.g. `tagBindings/%2F%2Fcloudresourcemanager.googleapis.com%2Fprojects%2F123/tagValues/456`). |
| `tag_value_namespaced_name` | String |  | The namespaced name for the TagValue of the TagBinding. Must be in the format `{parent_id}/{tag_key_short_name}/{short_name}`. For methods that support TagValue namespaced name, only one of tag_value_namespaced_name or tag_value may be filled. Requests with both fields will be rejected. |
| `parent` | String |  | The full resource name of the resource the TagValue is bound to. E.g. `//cloudresourcemanager.googleapis.com/projects/123` |
| `tag_value` | String |  | The TagValue of the TagBinding. Must be of the form `tagValues/456`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Pagination token. If the result set is too large to fit in a single response, this token is returned. It encodes the position of the current result cursor. Feeding this value into a new list request with the `page_token` parameter gives the next page of the results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. Pagination tokens have a limited lifetime. |
| `tag_bindings` | Vec<String> | A possibly paginated list of TagBindings for the specified resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag_binding
tag_binding = provider.cloudresourcemanager_api.Tag_binding {
}

# Access tag_binding outputs
tag_binding_id = tag_binding.id
tag_binding_next_page_token = tag_binding.next_page_token
tag_binding_tag_bindings = tag_binding.tag_bindings
```

---


### Organization

Sets the access control policy on an organization resource. Replaces any existing policy. The `resource` field should be the organization's resource name, for example: "organizations/123". Authorization requires the IAM permission `resourcemanager.organizations.setIamPolicy` on the specified organization.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. Timestamp when the Organization was requested for deletion. |
| `update_time` | String | Output only. Timestamp when the Organization was last modified. |
| `name` | String | Output only. The resource name of the organization. This is the organization's relative path in the API. Its format is "organizations/[organization_id]". For example, "organizations/1234". |
| `directory_customer_id` | String | Immutable. The G Suite / Workspace customer id used in the Directory API. |
| `create_time` | String | Output only. Timestamp when the Organization was created. |
| `display_name` | String | Output only. A human-readable string that refers to the organization in the Google Cloud Console. This string is set by the server and cannot be changed. The string will be set to the primary domain (for example, "google.com") of the Google Workspace customer that owns the organization. |
| `state` | String | Output only. The organization's current lifecycle state. |
| `etag` | String | Output only. A checksum computed by the server based on the current value of the Organization resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create organization
organization = provider.cloudresourcemanager_api.Organization {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access organization outputs
organization_id = organization.id
organization_delete_time = organization.delete_time
organization_update_time = organization.update_time
organization_name = organization.name
organization_directory_customer_id = organization.directory_customer_id
organization_create_time = organization.create_time
organization_display_name = organization.display_name
organization_state = organization.state
organization_etag = organization.etag
```

---


### Project

Request that a new project be created. The result is an `Operation` which can be used to track the creation process. This process usually takes a few seconds, but can sometimes take much longer. The tracking `Operation` is automatically deleted after a few hours, so there is no need to call `DeleteOperation`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this project. Each item in the map must be expressed as " : ". For example: "123/environment" : "production", "123/costCenter" : "marketing" Note: Currently this field is in Preview. |
| `delete_time` | String |  | Output only. The time at which this resource was requested for deletion. |
| `update_time` | String |  | Output only. The most recent time this resource was modified. |
| `create_time` | String |  | Output only. Creation time. |
| `labels` | HashMap<String, String> |  | Optional. The labels associated with this project. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: \[a-z\](\[-a-z0-9\]*\[a-z0-9\])?. Label values must be between 0 and 63 characters long and must conform to the regular expression (\[a-z\](\[-a-z0-9\]*\[a-z0-9\])?)?. No more than 64 labels can be associated with a given resource. Clients should store labels in a representation such as JSON that does not depend on specific characters being disallowed. Example: `"myBusinessDimension" : "businessValue"` |
| `parent` | String |  | Optional. A reference to a parent Resource. eg., `organizations/123` or `folders/876`. |
| `display_name` | String |  | Optional. A user-assigned display name of the project. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `My Project` |
| `name` | String |  | Output only. The unique resource name of the project. It is an int64 generated number prefixed by "projects/". Example: `projects/415104041262` |
| `configured_capabilities` | Vec<String> |  | Output only. If this project is a Management Project, list of capabilities configured on the parent folder. Note, presence of any capability implies that this is a Management Project. Example: `folders/123/capabilities/app-management`. OUTPUT ONLY. |
| `project_id` | String |  | Immutable. The unique, user-assigned id of the project. It must be 6 to 30 lowercase ASCII letters, digits, or hyphens. It must start with a letter. Trailing hyphens are prohibited. Example: `tokyo-rain-123` |
| `state` | String |  | Output only. The project lifecycle state. |
| `etag` | String |  | Output only. A checksum computed by the server based on the current value of the Project resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this project. Each item in the map must be expressed as " : ". For example: "123/environment" : "production", "123/costCenter" : "marketing" Note: Currently this field is in Preview. |
| `delete_time` | String | Output only. The time at which this resource was requested for deletion. |
| `update_time` | String | Output only. The most recent time this resource was modified. |
| `create_time` | String | Output only. Creation time. |
| `labels` | HashMap<String, String> | Optional. The labels associated with this project. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: \[a-z\](\[-a-z0-9\]*\[a-z0-9\])?. Label values must be between 0 and 63 characters long and must conform to the regular expression (\[a-z\](\[-a-z0-9\]*\[a-z0-9\])?)?. No more than 64 labels can be associated with a given resource. Clients should store labels in a representation such as JSON that does not depend on specific characters being disallowed. Example: `"myBusinessDimension" : "businessValue"` |
| `parent` | String | Optional. A reference to a parent Resource. eg., `organizations/123` or `folders/876`. |
| `display_name` | String | Optional. A user-assigned display name of the project. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `My Project` |
| `name` | String | Output only. The unique resource name of the project. It is an int64 generated number prefixed by "projects/". Example: `projects/415104041262` |
| `configured_capabilities` | Vec<String> | Output only. If this project is a Management Project, list of capabilities configured on the parent folder. Note, presence of any capability implies that this is a Management Project. Example: `folders/123/capabilities/app-management`. OUTPUT ONLY. |
| `project_id` | String | Immutable. The unique, user-assigned id of the project. It must be 6 to 30 lowercase ASCII letters, digits, or hyphens. It must start with a letter. Trailing hyphens are prohibited. Example: `tokyo-rain-123` |
| `state` | String | Output only. The project lifecycle state. |
| `etag` | String | Output only. A checksum computed by the server based on the current value of the Project resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.cloudresourcemanager_api.Project {
}

# Access project outputs
project_id = project.id
project_tags = project.tags
project_delete_time = project.delete_time
project_update_time = project.update_time
project_create_time = project.create_time
project_labels = project.labels
project_parent = project.parent
project_display_name = project.display_name
project_name = project.name
project_configured_capabilities = project.configured_capabilities
project_project_id = project.project_id
project_state = project.state
project_etag = project.etag
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
organization_0 = provider.cloudresourcemanager_api.Organization {
    resource = "value-0"
}
organization_1 = provider.cloudresourcemanager_api.Organization {
    resource = "value-1"
}
organization_2 = provider.cloudresourcemanager_api.Organization {
    resource = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    organization = provider.cloudresourcemanager_api.Organization {
        resource = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudresourcemanager_api Documentation](https://cloud.google.com/cloudresourcemanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
