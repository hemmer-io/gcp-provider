# Deploymentmanager_api Service



**Resources**: 19

---

## Overview

The deploymentmanager_api service provides access to 19 resource types:

- [Type](#type) [R]
- [Type_provider](#type_provider) [CRUD]
- [Operation](#operation) [R]
- [Manifest](#manifest) [R]
- [Resource](#resource) [R]
- [Deployment](#deployment) [CRUD]
- [Composite_type](#composite_type) [CRUD]
- [Operation](#operation) [R]
- [Manifest](#manifest) [R]
- [Resource](#resource) [R]
- [Type](#type) [R]
- [Deployment](#deployment) [CRUD]
- [Composite_type](#composite_type) [CRUD]
- [Type_provider](#type_provider) [CRUD]
- [Deployment](#deployment) [CRUD]
- [Manifest](#manifest) [R]
- [Resource](#resource) [R]
- [Operation](#operation) [R]
- [Type](#type) [R]

---

## Resources


### Type

Gets information about a specific type.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `base` | String | Base Type (configurable service) that backs this Type. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this type. |
| `self_link` | String | Output only. Server defined URL for the resource. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |
| `id` | String |  |
| `name` | String | Name of the type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access type outputs
type_id = type.id
type_base = type.base
type_operation = type.operation
type_self_link = type.self_link
type_insert_time = type.insert_time
type_labels = type.labels
type_description = type.description
type_id = type.id
type_name = type.name
```

---


### Type_provider

Creates a type provider.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `credential` | String |  | Credential used when interacting with this type. |
| `labels` | Vec<String> |  | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?` |
| `options` | String |  | Options to apply when handling any resources in this service. |
| `collection_overrides` | Vec<String> |  | Allows resource handling overrides for specific collections |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this type provider. |
| `custom_certificate_authority_roots` | Vec<String> |  | List of up to 2 custom certificate authority roots to use for TLS authentication when making calls on behalf of this type provider. If set, TLS authentication will exclusively use these roots instead of relying on publicly trusted certificate authorities when validating TLS certificate authenticity. The certificates must be in base64-encoded PEM format. The maximum size of each certificate must not exceed 10KB. |
| `description` | String |  | An optional textual description of the resource; provided by the client when the resource is created. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `id` | String |  | Output only. Unique identifier for the resource defined by the server. |
| `self_link` | String |  | Output only. Self link for the type provider. |
| `descriptor_url` | String |  | Descriptor Url for the this type provider. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `credential` | String | Credential used when interacting with this type. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?` |
| `options` | String | Options to apply when handling any resources in this service. |
| `collection_overrides` | Vec<String> | Allows resource handling overrides for specific collections |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this type provider. |
| `custom_certificate_authority_roots` | Vec<String> | List of up to 2 custom certificate authority roots to use for TLS authentication when making calls on behalf of this type provider. If set, TLS authentication will exclusively use these roots instead of relying on publicly trusted certificate authorities when validating TLS certificate authenticity. The certificates must be in base64-encoded PEM format. The maximum size of each certificate must not exceed 10KB. |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `id` | String | Output only. Unique identifier for the resource defined by the server. |
| `self_link` | String | Output only. Self link for the type provider. |
| `descriptor_url` | String | Descriptor Url for the this type provider. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create type_provider
type_provider = provider.deploymentmanager_api.Type_provider {
    project = "value"  # The project ID for this request.
}

# Access type_provider outputs
type_provider_id = type_provider.id
type_provider_name = type_provider.name
type_provider_credential = type_provider.credential
type_provider_labels = type_provider.labels
type_provider_options = type_provider.options
type_provider_collection_overrides = type_provider.collection_overrides
type_provider_operation = type_provider.operation
type_provider_custom_certificate_authority_roots = type_provider.custom_certificate_authority_roots
type_provider_description = type_provider.description
type_provider_insert_time = type_provider.insert_time
type_provider_id = type_provider.id
type_provider_self_link = type_provider.self_link
type_provider_descriptor_url = type_provider.descriptor_url
```

---


### Operation

Gets information about a specific operation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instances_bulk_insert_operation_metadata` | String |  |
| `id` | String | [Output Only] The unique identifier for the operation. This identifier is defined by the server. |
| `operation_group_id` | String | [Output Only] An ID that represents a group of operations, such as when a group of operations results from a `bulkInsert` API request. |
| `region` | String | [Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations. |
| `http_error_message` | String | [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as `NOT FOUND`. |
| `client_operation_id` | String | [Output Only] The value of `requestId` if you provided it in the request. Not present otherwise. |
| `insert_time` | String | [Output Only] The time that this operation was requested. This value is in RFC3339 text format. |
| `self_link_with_id` | String | [Output Only] Server-defined URL for this resource with the resource id. |
| `description` | String | [Output Only] A textual description of the operation, which is set when the operation is created. |
| `kind` | String | [Output Only] Type of the resource. Always `compute#operation` for Operation resources. |
| `name` | String | [Output Only] Name of the operation. |
| `status` | String | [Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`. |
| `http_error_status_code` | i64 | [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a `404` means the resource was not found. |
| `operation_type` | String | [Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on. |
| `error` | String | [Output Only] If errors are generated during processing of the operation, this field will be populated. |
| `set_autoscaler_link_operation_metadata` | String | This field is used internally by the Autoscaler team and should not be promoted to "alpha/beta/v1". |
| `target_id` | String | [Output Only] The unique target ID, which identifies a specific incarnation of the target resource. |
| `target_link` | String | [Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the disk that the snapshot was created from. |
| `user` | String | [Output Only] User who requested the operation, for example: `user@example.com` or `alice_smith_identifier (global/workforcePools/example-com-us-employees)`. |
| `warnings` | Vec<String> | [Output Only] If warning messages are generated during processing of the operation, this field will be populated. |
| `set_common_instance_metadata_operation_metadata` | String | [Output Only] If the operation is for projects.setCommonInstanceMetadata, this field will contain information on all underlying zonal actions and their state. |
| `progress` | i64 | [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses. |
| `end_time` | String | [Output Only] The time that this operation was completed. This value is in RFC3339 text format. |
| `start_time` | String | [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format. |
| `self_link` | String | [Output Only] Server-defined URL for the resource. |
| `creation_timestamp` | String | [Deprecated] This field is deprecated. |
| `status_message` | String | [Output Only] An optional textual description of the current status of the operation. |
| `zone` | String | [Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations. |


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
operation_instances_bulk_insert_operation_metadata = operation.instances_bulk_insert_operation_metadata
operation_id = operation.id
operation_operation_group_id = operation.operation_group_id
operation_region = operation.region
operation_http_error_message = operation.http_error_message
operation_client_operation_id = operation.client_operation_id
operation_insert_time = operation.insert_time
operation_self_link_with_id = operation.self_link_with_id
operation_description = operation.description
operation_kind = operation.kind
operation_name = operation.name
operation_status = operation.status
operation_http_error_status_code = operation.http_error_status_code
operation_operation_type = operation.operation_type
operation_error = operation.error
operation_set_autoscaler_link_operation_metadata = operation.set_autoscaler_link_operation_metadata
operation_target_id = operation.target_id
operation_target_link = operation.target_link
operation_user = operation.user
operation_warnings = operation.warnings
operation_set_common_instance_metadata_operation_metadata = operation.set_common_instance_metadata_operation_metadata
operation_progress = operation.progress
operation_end_time = operation.end_time
operation_start_time = operation.start_time
operation_self_link = operation.self_link
operation_creation_timestamp = operation.creation_timestamp
operation_status_message = operation.status_message
operation_zone = operation.zone
```

---


### Manifest

Gets information about a specific manifest.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expanded_config` | String | Output only. The fully-expanded configuration file, including any templates and references. |
| `id` | String |  |
| `manifest_size_limit_bytes` | String | Output only. The size limit for expanded manifests in the project. |
| `manifest_size_bytes` | String | Output only. The computed size of the fully expanded manifest. |
| `config` | String | Output only. The YAML configuration for this manifest. |
| `imports` | Vec<String> | Output only. The imported files for this manifest. |
| `layout` | String | Output only. The YAML layout for this manifest. |
| `name` | String | Output only. The name of the manifest. |
| `self_link` | String | Output only. Self link for the manifest. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access manifest outputs
manifest_id = manifest.id
manifest_expanded_config = manifest.expanded_config
manifest_id = manifest.id
manifest_manifest_size_limit_bytes = manifest.manifest_size_limit_bytes
manifest_manifest_size_bytes = manifest.manifest_size_bytes
manifest_config = manifest.config
manifest_imports = manifest.imports
manifest_layout = manifest.layout
manifest_name = manifest.name
manifest_self_link = manifest.self_link
manifest_insert_time = manifest.insert_time
```

---


### Resource

Gets information about a single resource.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `properties` | String | Output only. The current properties of the resource before any references have been filled in. Returned as serialized YAML. |
| `id` | String |  |
| `url` | String | Output only. The URL of the actual resource. |
| `name` | String | Output only. The name of the resource as it appears in the YAML config. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `manifest` | String | Output only. URL of the manifest representing the current configuration of this resource. |
| `final_properties` | String | Output only. The evaluated properties of the resource with references expanded. Returned as serialized YAML. |
| `access_control` | String | The Access Control Policy set on this resource. |
| `runtime_policies` | Vec<String> | Output only. In case this is an action, it will show the runtimePolicies on which this action will run in the deployment |
| `type` | String | Output only. The type of the resource, for example `compute.v1.instance`, or `cloudfunctions.v1beta1.function`. |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this resource, the updated configuration appears here. |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |
| `last_used_credential` | String | Output only. The last used credential that successfully created/updated the resource. |
| `warnings` | Vec<String> | Output only. If warning messages are generated during processing of this resource, this field will be populated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access resource outputs
resource_id = resource.id
resource_properties = resource.properties
resource_id = resource.id
resource_url = resource.url
resource_name = resource.name
resource_insert_time = resource.insert_time
resource_manifest = resource.manifest
resource_final_properties = resource.final_properties
resource_access_control = resource.access_control
resource_runtime_policies = resource.runtime_policies
resource_type = resource.type
resource_update = resource.update
resource_update_time = resource.update_time
resource_last_used_credential = resource.last_used_credential
resource_warnings = resource.warnings
```

---


### Deployment

Creates a deployment and all of the resources described by the deployment manifest.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `update` | String |  | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `description` | String |  | An optional user-provided description of the deployment. |
| `id` | String |  |  |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `credential` | String |  | User provided default credential for the deployment. |
| `labels` | Vec<String> |  | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `name` | String |  | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `outputs` | Vec<String> |  | Output only. List of outputs from the last manifest that deployed successfully. |
| `target` | String |  | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `update_time` | String |  | Output only. Update timestamp in RFC3339 text format. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `manifest` | String |  | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `self_link` | String |  | Output only. Server defined URL for the resource. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fingerprint` | String | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `description` | String | An optional user-provided description of the deployment. |
| `id` | String |  |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `credential` | String | User provided default credential for the deployment. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `name` | String | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `outputs` | Vec<String> | Output only. List of outputs from the last manifest that deployed successfully. |
| `target` | String | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `manifest` | String | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `self_link` | String | Output only. Server defined URL for the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment
deployment = provider.deploymentmanager_api.Deployment {
    project = "value"  # The project ID for this request.
}

# Access deployment outputs
deployment_id = deployment.id
deployment_fingerprint = deployment.fingerprint
deployment_update = deployment.update
deployment_description = deployment.description
deployment_id = deployment.id
deployment_insert_time = deployment.insert_time
deployment_credential = deployment.credential
deployment_labels = deployment.labels
deployment_name = deployment.name
deployment_outputs = deployment.outputs
deployment_target = deployment.target
deployment_update_time = deployment.update_time
deployment_operation = deployment.operation
deployment_manifest = deployment.manifest
deployment_self_link = deployment.self_link
```

---


### Composite_type

Creates a composite type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `self_link` | String |  | Output only. Server defined URL for the resource. |
| `status` | String |  |  |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `description` | String |  | An optional textual description of the resource; provided by the client when the resource is created. |
| `labels` | Vec<String> |  | Map of labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `id` | String |  |  |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this composite type. |
| `name` | String |  | Name of the composite type, must follow the expression: `[a-z]([-a-z0-9_.]{0,61}[a-z0-9])?`. |
| `template_contents` | String |  | Files for the template type. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | Output only. Server defined URL for the resource. |
| `status` | String |  |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |
| `labels` | Vec<String> | Map of labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `id` | String |  |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this composite type. |
| `name` | String | Name of the composite type, must follow the expression: `[a-z]([-a-z0-9_.]{0,61}[a-z0-9])?`. |
| `template_contents` | String | Files for the template type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create composite_type
composite_type = provider.deploymentmanager_api.Composite_type {
    project = "value"  # The project ID for this request.
}

# Access composite_type outputs
composite_type_id = composite_type.id
composite_type_self_link = composite_type.self_link
composite_type_status = composite_type.status
composite_type_insert_time = composite_type.insert_time
composite_type_description = composite_type.description
composite_type_labels = composite_type.labels
composite_type_id = composite_type.id
composite_type_operation = composite_type.operation
composite_type_name = composite_type.name
composite_type_template_contents = composite_type.template_contents
```

---


### Operation

Gets information about a specific operation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instances_bulk_insert_operation_metadata` | String |  |
| `http_error_message` | String | [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as `NOT FOUND`. |
| `set_common_instance_metadata_operation_metadata` | String | [Output Only] If the operation is for projects.setCommonInstanceMetadata, this field will contain information on all underlying zonal actions and their state. |
| `self_link` | String | [Output Only] Server-defined URL for the resource. |
| `self_link_with_id` | String | [Output Only] Server-defined URL for this resource with the resource id. |
| `client_operation_id` | String | [Output Only] The value of `requestId` if you provided it in the request. Not present otherwise. |
| `operation_type` | String | [Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on. |
| `operation_group_id` | String | [Output Only] An ID that represents a group of operations, such as when a group of operations results from a `bulkInsert` API request. |
| `description` | String | [Output Only] A textual description of the operation, which is set when the operation is created. |
| `progress` | i64 | [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses. |
| `start_time` | String | [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format. |
| `status_message` | String | [Output Only] An optional textual description of the current status of the operation. |
| `target_link` | String | [Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the disk that the snapshot was created from. |
| `zone` | String | [Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations. |
| `region` | String | [Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations. |
| `end_time` | String | [Output Only] The time that this operation was completed. This value is in RFC3339 text format. |
| `kind` | String | [Output Only] Type of the resource. Always `compute#operation` for Operation resources. |
| `http_error_status_code` | i64 | [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a `404` means the resource was not found. |
| `error` | String | [Output Only] If errors are generated during processing of the operation, this field will be populated. |
| `id` | String | [Output Only] The unique identifier for the operation. This identifier is defined by the server. |
| `warnings` | Vec<String> | [Output Only] If warning messages are generated during processing of the operation, this field will be populated. |
| `creation_timestamp` | String | [Deprecated] This field is deprecated. |
| `insert_time` | String | [Output Only] The time that this operation was requested. This value is in RFC3339 text format. |
| `set_autoscaler_link_operation_metadata` | String | This field is used internally by the Autoscaler team and should not be promoted to "alpha/beta/v1". |
| `user` | String | [Output Only] User who requested the operation, for example: `user@example.com` or `alice_smith_identifier (global/workforcePools/example-com-us-employees)`. |
| `name` | String | [Output Only] Name of the operation. |
| `target_id` | String | [Output Only] The unique target ID, which identifies a specific incarnation of the target resource. |
| `status` | String | [Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`. |


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
operation_instances_bulk_insert_operation_metadata = operation.instances_bulk_insert_operation_metadata
operation_http_error_message = operation.http_error_message
operation_set_common_instance_metadata_operation_metadata = operation.set_common_instance_metadata_operation_metadata
operation_self_link = operation.self_link
operation_self_link_with_id = operation.self_link_with_id
operation_client_operation_id = operation.client_operation_id
operation_operation_type = operation.operation_type
operation_operation_group_id = operation.operation_group_id
operation_description = operation.description
operation_progress = operation.progress
operation_start_time = operation.start_time
operation_status_message = operation.status_message
operation_target_link = operation.target_link
operation_zone = operation.zone
operation_region = operation.region
operation_end_time = operation.end_time
operation_kind = operation.kind
operation_http_error_status_code = operation.http_error_status_code
operation_error = operation.error
operation_id = operation.id
operation_warnings = operation.warnings
operation_creation_timestamp = operation.creation_timestamp
operation_insert_time = operation.insert_time
operation_set_autoscaler_link_operation_metadata = operation.set_autoscaler_link_operation_metadata
operation_user = operation.user
operation_name = operation.name
operation_target_id = operation.target_id
operation_status = operation.status
```

---


### Manifest

Gets information about a specific manifest.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The name of the manifest. |
| `id` | String |  |
| `manifest_size_limit_bytes` | String | Output only. The size limit for expanded manifests in the project. |
| `manifest_size_bytes` | String | Output only. The computed size of the fully expanded manifest. |
| `config` | String | Output only. The YAML configuration for this manifest. |
| `self_link` | String | Output only. Self link for the manifest. |
| `expanded_config` | String | Output only. The fully-expanded configuration file, including any templates and references. |
| `imports` | Vec<String> | Output only. The imported files for this manifest. |
| `layout` | String | Output only. The YAML layout for this manifest. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access manifest outputs
manifest_id = manifest.id
manifest_name = manifest.name
manifest_id = manifest.id
manifest_manifest_size_limit_bytes = manifest.manifest_size_limit_bytes
manifest_manifest_size_bytes = manifest.manifest_size_bytes
manifest_config = manifest.config
manifest_self_link = manifest.self_link
manifest_expanded_config = manifest.expanded_config
manifest_imports = manifest.imports
manifest_layout = manifest.layout
manifest_insert_time = manifest.insert_time
```

---


### Resource

Gets information about a single resource.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `url` | String | Output only. The URL of the actual resource. |
| `final_properties` | String | Output only. The evaluated properties of the resource with references expanded. Returned as serialized YAML. |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `manifest` | String | Output only. URL of the manifest representing the current configuration of this resource. |
| `type` | String | Output only. The type of the resource, for example `compute.v1.instance`, or `cloudfunctions.v1beta1.function`. |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this resource, the updated configuration appears here. |
| `properties` | String | Output only. The current properties of the resource before any references have been filled in. Returned as serialized YAML. |
| `access_control` | String | The Access Control Policy set on this resource. |
| `id` | String |  |
| `name` | String | Output only. The name of the resource as it appears in the YAML config. |
| `warnings` | Vec<String> | Output only. If warning messages are generated during processing of this resource, this field will be populated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access resource outputs
resource_id = resource.id
resource_url = resource.url
resource_final_properties = resource.final_properties
resource_update_time = resource.update_time
resource_insert_time = resource.insert_time
resource_manifest = resource.manifest
resource_type = resource.type
resource_update = resource.update
resource_properties = resource.properties
resource_access_control = resource.access_control
resource_id = resource.id
resource_name = resource.name
resource_warnings = resource.warnings
```

---


### Type

Lists all resource types for Deployment Manager.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `types` | Vec<String> | Output only. A list of resource types supported by Deployment Manager. |
| `next_page_token` | String | A token used to continue a truncated list request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access type outputs
type_id = type.id
type_types = type.types
type_next_page_token = type.next_page_token
```

---


### Deployment

Creates a deployment and all of the resources described by the deployment manifest.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target` | String |  | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `update_time` | String |  | Output only. Update timestamp in RFC3339 text format. |
| `name` | String |  | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `labels` | Vec<String> |  | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `manifest` | String |  | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `description` | String |  | An optional user-provided description of the deployment. |
| `fingerprint` | String |  | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `update` | String |  | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `id` | String |  |  |
| `self_link` | String |  | Output only. Server defined URL for the resource. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target` | String | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |
| `name` | String | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `manifest` | String | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `description` | String | An optional user-provided description of the deployment. |
| `fingerprint` | String | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `id` | String |  |
| `self_link` | String | Output only. Server defined URL for the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment
deployment = provider.deploymentmanager_api.Deployment {
    project = "value"  # The project ID for this request.
}

# Access deployment outputs
deployment_id = deployment.id
deployment_target = deployment.target
deployment_update_time = deployment.update_time
deployment_name = deployment.name
deployment_labels = deployment.labels
deployment_manifest = deployment.manifest
deployment_description = deployment.description
deployment_fingerprint = deployment.fingerprint
deployment_update = deployment.update
deployment_insert_time = deployment.insert_time
deployment_operation = deployment.operation
deployment_id = deployment.id
deployment_self_link = deployment.self_link
```

---


### Composite_type

Creates a composite type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Name of the composite type, must follow the expression: `[a-z]([-a-z0-9_.]{0,61}[a-z0-9])?`. |
| `labels` | Vec<String> |  | Map of labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this composite type. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `self_link` | String |  | Output only. Server defined URL for the resource. |
| `status` | String |  |  |
| `template_contents` | String |  | Files for the template type. |
| `id` | String |  |  |
| `description` | String |  | An optional textual description of the resource; provided by the client when the resource is created. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Name of the composite type, must follow the expression: `[a-z]([-a-z0-9_.]{0,61}[a-z0-9])?`. |
| `labels` | Vec<String> | Map of labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this composite type. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `self_link` | String | Output only. Server defined URL for the resource. |
| `status` | String |  |
| `template_contents` | String | Files for the template type. |
| `id` | String |  |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create composite_type
composite_type = provider.deploymentmanager_api.Composite_type {
    project = "value"  # The project ID for this request.
}

# Access composite_type outputs
composite_type_id = composite_type.id
composite_type_name = composite_type.name
composite_type_labels = composite_type.labels
composite_type_operation = composite_type.operation
composite_type_insert_time = composite_type.insert_time
composite_type_self_link = composite_type.self_link
composite_type_status = composite_type.status
composite_type_template_contents = composite_type.template_contents
composite_type_id = composite_type.id
composite_type_description = composite_type.description
```

---


### Type_provider

Creates a type provider.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `descriptor_url` | String |  | Descriptor Url for the this type provider. |
| `collection_overrides` | Vec<String> |  | Allows resource handling overrides for specific collections |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `custom_certificate_authority_roots` | Vec<String> |  | List of up to 2 custom certificate authority roots to use for TLS authentication when making calls on behalf of this type provider. If set, TLS authentication will exclusively use these roots instead of relying on publicly trusted certificate authorities when validating TLS certificate authenticity. The certificates must be in base64-encoded PEM format. The maximum size of each certificate must not exceed 10KB. |
| `credential` | String |  | Credential used when interacting with this type. |
| `description` | String |  | An optional textual description of the resource; provided by the client when the resource is created. |
| `labels` | Vec<String> |  | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?` |
| `id` | String |  | Output only. Unique identifier for the resource defined by the server. |
| `name` | String |  | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `options` | String |  | Options to apply when handling any resources in this service. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this type provider. |
| `self_link` | String |  | Output only. Self link for the type provider. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `descriptor_url` | String | Descriptor Url for the this type provider. |
| `collection_overrides` | Vec<String> | Allows resource handling overrides for specific collections |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `custom_certificate_authority_roots` | Vec<String> | List of up to 2 custom certificate authority roots to use for TLS authentication when making calls on behalf of this type provider. If set, TLS authentication will exclusively use these roots instead of relying on publicly trusted certificate authorities when validating TLS certificate authenticity. The certificates must be in base64-encoded PEM format. The maximum size of each certificate must not exceed 10KB. |
| `credential` | String | Credential used when interacting with this type. |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?` |
| `id` | String | Output only. Unique identifier for the resource defined by the server. |
| `name` | String | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `options` | String | Options to apply when handling any resources in this service. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this type provider. |
| `self_link` | String | Output only. Self link for the type provider. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create type_provider
type_provider = provider.deploymentmanager_api.Type_provider {
    project = "value"  # The project ID for this request.
}

# Access type_provider outputs
type_provider_id = type_provider.id
type_provider_descriptor_url = type_provider.descriptor_url
type_provider_collection_overrides = type_provider.collection_overrides
type_provider_insert_time = type_provider.insert_time
type_provider_custom_certificate_authority_roots = type_provider.custom_certificate_authority_roots
type_provider_credential = type_provider.credential
type_provider_description = type_provider.description
type_provider_labels = type_provider.labels
type_provider_id = type_provider.id
type_provider_name = type_provider.name
type_provider_options = type_provider.options
type_provider_operation = type_provider.operation
type_provider_self_link = type_provider.self_link
```

---


### Deployment

Creates a deployment and all of the resources described by the deployment manifest.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  |  |
| `target` | String |  | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `self_link` | String |  | Output only. Server defined URL for the resource. |
| `labels` | Vec<String> |  | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `fingerprint` | String |  | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `manifest` | String |  | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `update` | String |  | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `name` | String |  | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `description` | String |  | An optional user-provided description of the deployment. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `update_time` | String |  | Output only. Update timestamp in RFC3339 text format. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String |  |
| `target` | String | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `self_link` | String | Output only. Server defined URL for the resource. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `fingerprint` | String | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `manifest` | String | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `name` | String | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `description` | String | An optional user-provided description of the deployment. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment
deployment = provider.deploymentmanager_api.Deployment {
    project = "value"  # The project ID for this request.
}

# Access deployment outputs
deployment_id = deployment.id
deployment_id = deployment.id
deployment_target = deployment.target
deployment_insert_time = deployment.insert_time
deployment_self_link = deployment.self_link
deployment_labels = deployment.labels
deployment_fingerprint = deployment.fingerprint
deployment_manifest = deployment.manifest
deployment_update = deployment.update
deployment_name = deployment.name
deployment_description = deployment.description
deployment_operation = deployment.operation
deployment_update_time = deployment.update_time
```

---


### Manifest

Gets information about a specific manifest.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `config` | String | Output only. The YAML configuration for this manifest. |
| `expanded_config` | String | Output only. The fully-expanded configuration file, including any templates and references. |
| `name` | String | Output only. The name of the manifest. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `imports` | Vec<String> | Output only. The imported files for this manifest. |
| `manifest_size_limit_bytes` | String | Output only. The size limit for expanded manifests in the project. |
| `self_link` | String | Output only. Self link for the manifest. |
| `manifest_size_bytes` | String | Output only. The computed size of the fully expanded manifest. |
| `id` | String |  |
| `layout` | String | Output only. The YAML layout for this manifest. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access manifest outputs
manifest_id = manifest.id
manifest_config = manifest.config
manifest_expanded_config = manifest.expanded_config
manifest_name = manifest.name
manifest_insert_time = manifest.insert_time
manifest_imports = manifest.imports
manifest_manifest_size_limit_bytes = manifest.manifest_size_limit_bytes
manifest_self_link = manifest.self_link
manifest_manifest_size_bytes = manifest.manifest_size_bytes
manifest_id = manifest.id
manifest_layout = manifest.layout
```

---


### Resource

Gets information about a single resource.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `warnings` | Vec<String> | Output only. If warning messages are generated during processing of this resource, this field will be populated. |
| `id` | String |  |
| `name` | String | Output only. The name of the resource as it appears in the YAML config. |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this resource, the updated configuration appears here. |
| `url` | String | Output only. The URL of the actual resource. |
| `final_properties` | String | Output only. The evaluated properties of the resource with references expanded. Returned as serialized YAML. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `properties` | String | Output only. The current properties of the resource before any references have been filled in. Returned as serialized YAML. |
| `type` | String | Output only. The type of the resource, for example `compute.v1.instance`, or `cloudfunctions.v1beta1.function`. |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |
| `access_control` | String | The Access Control Policy set on this resource. |
| `manifest` | String | Output only. URL of the manifest representing the current configuration of this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access resource outputs
resource_id = resource.id
resource_warnings = resource.warnings
resource_id = resource.id
resource_name = resource.name
resource_update = resource.update
resource_url = resource.url
resource_final_properties = resource.final_properties
resource_insert_time = resource.insert_time
resource_properties = resource.properties
resource_type = resource.type
resource_update_time = resource.update_time
resource_access_control = resource.access_control
resource_manifest = resource.manifest
```

---


### Operation

Gets information about a specific operation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_operation_id` | String | [Output Only] The value of `requestId` if you provided it in the request. Not present otherwise. |
| `kind` | String | [Output Only] Type of the resource. Always `compute#operation` for Operation resources. |
| `zone` | String | [Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations. |
| `insert_time` | String | [Output Only] The time that this operation was requested. This value is in RFC3339 text format. |
| `name` | String | [Output Only] Name of the operation. |
| `id` | String | [Output Only] The unique identifier for the operation. This identifier is defined by the server. |
| `start_time` | String | [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format. |
| `http_error_message` | String | [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as `NOT FOUND`. |
| `status_message` | String | [Output Only] An optional textual description of the current status of the operation. |
| `creation_timestamp` | String | [Deprecated] This field is deprecated. |
| `http_error_status_code` | i64 | [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a `404` means the resource was not found. |
| `error` | String | [Output Only] If errors are generated during processing of the operation, this field will be populated. |
| `end_time` | String | [Output Only] The time that this operation was completed. This value is in RFC3339 text format. |
| `operation_group_id` | String | [Output Only] An ID that represents a group of operations, such as when a group of operations results from a `bulkInsert` API request. |
| `self_link` | String | [Output Only] Server-defined URL for the resource. |
| `warnings` | Vec<String> | [Output Only] If warning messages are generated during processing of the operation, this field will be populated. |
| `operation_type` | String | [Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on. |
| `region` | String | [Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations. |
| `self_link_with_id` | String | [Output Only] Server-defined URL for this resource with the resource id. |
| `progress` | i64 | [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses. |
| `set_autoscaler_link_operation_metadata` | String | This field is used internally by the Autoscaler team and should not be promoted to "alpha/beta/v1". |
| `set_common_instance_metadata_operation_metadata` | String | [Output Only] If the operation is for projects.setCommonInstanceMetadata, this field will contain information on all underlying zonal actions and their state. |
| `target_id` | String | [Output Only] The unique target ID, which identifies a specific incarnation of the target resource. |
| `status` | String | [Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`. |
| `instances_bulk_insert_operation_metadata` | String |  |
| `target_link` | String | [Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the disk that the snapshot was created from. |
| `user` | String | [Output Only] User who requested the operation, for example: `user@example.com` or `alice_smith_identifier (global/workforcePools/example-com-us-employees)`. |
| `description` | String | [Output Only] A textual description of the operation, which is set when the operation is created. |


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
operation_client_operation_id = operation.client_operation_id
operation_kind = operation.kind
operation_zone = operation.zone
operation_insert_time = operation.insert_time
operation_name = operation.name
operation_id = operation.id
operation_start_time = operation.start_time
operation_http_error_message = operation.http_error_message
operation_status_message = operation.status_message
operation_creation_timestamp = operation.creation_timestamp
operation_http_error_status_code = operation.http_error_status_code
operation_error = operation.error
operation_end_time = operation.end_time
operation_operation_group_id = operation.operation_group_id
operation_self_link = operation.self_link
operation_warnings = operation.warnings
operation_operation_type = operation.operation_type
operation_region = operation.region
operation_self_link_with_id = operation.self_link_with_id
operation_progress = operation.progress
operation_set_autoscaler_link_operation_metadata = operation.set_autoscaler_link_operation_metadata
operation_set_common_instance_metadata_operation_metadata = operation.set_common_instance_metadata_operation_metadata
operation_target_id = operation.target_id
operation_status = operation.status
operation_instances_bulk_insert_operation_metadata = operation.instances_bulk_insert_operation_metadata
operation_target_link = operation.target_link
operation_user = operation.user
operation_description = operation.description
```

---


### Type

Lists all resource types for Deployment Manager.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token used to continue a truncated list request. |
| `types` | Vec<String> | Output only. A list of resource types supported by Deployment Manager. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access type outputs
type_id = type.id
type_next_page_token = type.next_page_token
type_types = type.types
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple type resources
type_0 = provider.deploymentmanager_api.Type {
}
type_1 = provider.deploymentmanager_api.Type {
}
type_2 = provider.deploymentmanager_api.Type {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    type = provider.deploymentmanager_api.Type {
    }
```

---

## Related Documentation

- [GCP Deploymentmanager_api Documentation](https://cloud.google.com/deploymentmanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
