# Deploymentmanager_api Service



**Resources**: 19

---

## Overview

The deploymentmanager_api service provides access to 19 resource types:

- [Operation](#operation) [R]
- [Deployment](#deployment) [CRUD]
- [Resource](#resource) [R]
- [Manifest](#manifest) [R]
- [Composite_type](#composite_type) [CRUD]
- [Type](#type) [R]
- [Type_provider](#type_provider) [CRUD]
- [Type_provider](#type_provider) [CRUD]
- [Operation](#operation) [R]
- [Deployment](#deployment) [CRUD]
- [Type](#type) [R]
- [Manifest](#manifest) [R]
- [Composite_type](#composite_type) [CRUD]
- [Resource](#resource) [R]
- [Type](#type) [R]
- [Deployment](#deployment) [CRUD]
- [Operation](#operation) [R]
- [Resource](#resource) [R]
- [Manifest](#manifest) [R]

---

## Resources


### Operation

Gets information about a specific operation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format. |
| `user` | String | [Output Only] User who requested the operation, for example: `user@example.com` or `alice_smith_identifier (global/workforcePools/example-com-us-employees)`. |
| `zone` | String | [Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations. |
| `self_link_with_id` | String | [Output Only] Server-defined URL for this resource with the resource id. |
| `operation_group_id` | String | [Output Only] An ID that represents a group of operations, such as when a group of operations results from a `bulkInsert` API request. |
| `creation_timestamp` | String | [Deprecated] This field is deprecated. |
| `id` | String | [Output Only] The unique identifier for the operation. This identifier is defined by the server. |
| `progress` | i64 | [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses. |
| `set_autoscaler_link_operation_metadata` | String | This field is used internally by the Autoscaler team and should not be promoted to "alpha/beta/v1". |
| `operation_type` | String | [Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on. |
| `target_link` | String | [Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the disk that the snapshot was created from. |
| `status` | String | [Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`. |
| `status_message` | String | [Output Only] An optional textual description of the current status of the operation. |
| `error` | String | [Output Only] If errors are generated during processing of the operation, this field will be populated. |
| `instances_bulk_insert_operation_metadata` | String |  |
| `name` | String | [Output Only] Name of the operation. |
| `http_error_message` | String | [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as `NOT FOUND`. |
| `end_time` | String | [Output Only] The time that this operation was completed. This value is in RFC3339 text format. |
| `set_common_instance_metadata_operation_metadata` | String | [Output Only] If the operation is for projects.setCommonInstanceMetadata, this field will contain information on all underlying zonal actions and their state. |
| `client_operation_id` | String | [Output Only] The value of `requestId` if you provided it in the request. Not present otherwise. |
| `self_link` | String | [Output Only] Server-defined URL for the resource. |
| `warnings` | Vec<String> | [Output Only] If warning messages are generated during processing of the operation, this field will be populated. |
| `http_error_status_code` | i64 | [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a `404` means the resource was not found. |
| `region` | String | [Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations. |
| `description` | String | [Output Only] A textual description of the operation, which is set when the operation is created. |
| `kind` | String | [Output Only] Type of the resource. Always `compute#operation` for Operation resources. |
| `insert_time` | String | [Output Only] The time that this operation was requested. This value is in RFC3339 text format. |
| `target_id` | String | [Output Only] The unique target ID, which identifies a specific incarnation of the target resource. |


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
operation_start_time = operation.start_time
operation_user = operation.user
operation_zone = operation.zone
operation_self_link_with_id = operation.self_link_with_id
operation_operation_group_id = operation.operation_group_id
operation_creation_timestamp = operation.creation_timestamp
operation_id = operation.id
operation_progress = operation.progress
operation_set_autoscaler_link_operation_metadata = operation.set_autoscaler_link_operation_metadata
operation_operation_type = operation.operation_type
operation_target_link = operation.target_link
operation_status = operation.status
operation_status_message = operation.status_message
operation_error = operation.error
operation_instances_bulk_insert_operation_metadata = operation.instances_bulk_insert_operation_metadata
operation_name = operation.name
operation_http_error_message = operation.http_error_message
operation_end_time = operation.end_time
operation_set_common_instance_metadata_operation_metadata = operation.set_common_instance_metadata_operation_metadata
operation_client_operation_id = operation.client_operation_id
operation_self_link = operation.self_link
operation_warnings = operation.warnings
operation_http_error_status_code = operation.http_error_status_code
operation_region = operation.region
operation_description = operation.description
operation_kind = operation.kind
operation_insert_time = operation.insert_time
operation_target_id = operation.target_id
```

---


### Deployment

Creates a deployment and all of the resources described by the deployment manifest.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `self_link` | String |  | Output only. Server defined URL for the resource. |
| `update` | String |  | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `fingerprint` | String |  | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `description` | String |  | An optional user-provided description of the deployment. |
| `name` | String |  | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `update_time` | String |  | Output only. Update timestamp in RFC3339 text format. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `id` | String |  |  |
| `labels` | Vec<String> |  | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `outputs` | Vec<String> |  | Output only. List of outputs from the last manifest that deployed successfully. |
| `credential` | String |  | User provided default credential for the deployment. |
| `target` | String |  | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `manifest` | String |  | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | Output only. Server defined URL for the resource. |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `fingerprint` | String | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `description` | String | An optional user-provided description of the deployment. |
| `name` | String | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `id` | String |  |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `outputs` | Vec<String> | Output only. List of outputs from the last manifest that deployed successfully. |
| `credential` | String | User provided default credential for the deployment. |
| `target` | String | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `manifest` | String | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |


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
deployment_self_link = deployment.self_link
deployment_update = deployment.update
deployment_fingerprint = deployment.fingerprint
deployment_operation = deployment.operation
deployment_description = deployment.description
deployment_name = deployment.name
deployment_update_time = deployment.update_time
deployment_insert_time = deployment.insert_time
deployment_id = deployment.id
deployment_labels = deployment.labels
deployment_outputs = deployment.outputs
deployment_credential = deployment.credential
deployment_target = deployment.target
deployment_manifest = deployment.manifest
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
| `final_properties` | String | Output only. The evaluated properties of the resource with references expanded. Returned as serialized YAML. |
| `access_control` | String | The Access Control Policy set on this resource. |
| `manifest` | String | Output only. URL of the manifest representing the current configuration of this resource. |
| `name` | String | Output only. The name of the resource as it appears in the YAML config. |
| `id` | String |  |
| `last_used_credential` | String | Output only. The last used credential that successfully created/updated the resource. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `runtime_policies` | Vec<String> | Output only. In case this is an action, it will show the runtimePolicies on which this action will run in the deployment |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this resource, the updated configuration appears here. |
| `type` | String | Output only. The type of the resource, for example `compute.v1.instance`, or `cloudfunctions.v1beta1.function`. |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |
| `properties` | String | Output only. The current properties of the resource before any references have been filled in. Returned as serialized YAML. |
| `url` | String | Output only. The URL of the actual resource. |
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
resource_final_properties = resource.final_properties
resource_access_control = resource.access_control
resource_manifest = resource.manifest
resource_name = resource.name
resource_id = resource.id
resource_last_used_credential = resource.last_used_credential
resource_insert_time = resource.insert_time
resource_runtime_policies = resource.runtime_policies
resource_update = resource.update
resource_type = resource.type
resource_update_time = resource.update_time
resource_properties = resource.properties
resource_url = resource.url
resource_warnings = resource.warnings
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
| `id` | String |  |
| `name` | String | Output only. The name of the manifest. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `manifest_size_limit_bytes` | String | Output only. The size limit for expanded manifests in the project. |
| `config` | String | Output only. The YAML configuration for this manifest. |
| `self_link` | String | Output only. Self link for the manifest. |
| `manifest_size_bytes` | String | Output only. The computed size of the fully expanded manifest. |
| `layout` | String | Output only. The YAML layout for this manifest. |
| `expanded_config` | String | Output only. The fully-expanded configuration file, including any templates and references. |
| `imports` | Vec<String> | Output only. The imported files for this manifest. |


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
manifest_id = manifest.id
manifest_name = manifest.name
manifest_insert_time = manifest.insert_time
manifest_manifest_size_limit_bytes = manifest.manifest_size_limit_bytes
manifest_config = manifest.config
manifest_self_link = manifest.self_link
manifest_manifest_size_bytes = manifest.manifest_size_bytes
manifest_layout = manifest.layout
manifest_expanded_config = manifest.expanded_config
manifest_imports = manifest.imports
```

---


### Composite_type

Creates a composite type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_contents` | String |  | Files for the template type. |
| `self_link` | String |  | Output only. Server defined URL for the resource. |
| `labels` | Vec<String> |  | Map of labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `id` | String |  |  |
| `status` | String |  |  |
| `name` | String |  | Name of the composite type, must follow the expression: `[a-z]([-a-z0-9_.]{0,61}[a-z0-9])?`. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this composite type. |
| `description` | String |  | An optional textual description of the resource; provided by the client when the resource is created. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `template_contents` | String | Files for the template type. |
| `self_link` | String | Output only. Server defined URL for the resource. |
| `labels` | Vec<String> | Map of labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `id` | String |  |
| `status` | String |  |
| `name` | String | Name of the composite type, must follow the expression: `[a-z]([-a-z0-9_.]{0,61}[a-z0-9])?`. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this composite type. |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |


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
composite_type_template_contents = composite_type.template_contents
composite_type_self_link = composite_type.self_link
composite_type_labels = composite_type.labels
composite_type_id = composite_type.id
composite_type_status = composite_type.status
composite_type_name = composite_type.name
composite_type_operation = composite_type.operation
composite_type_description = composite_type.description
composite_type_insert_time = composite_type.insert_time
```

---


### Type

Gets information about a specific type.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this type. |
| `self_link` | String | Output only. Server defined URL for the resource. |
| `id` | String |  |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |
| `name` | String | Name of the type. |
| `base` | String | Base Type (configurable service) that backs this Type. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |


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
type_operation = type.operation
type_self_link = type.self_link
type_id = type.id
type_insert_time = type.insert_time
type_description = type.description
type_name = type.name
type_base = type.base
type_labels = type.labels
```

---


### Type_provider

Creates a type provider.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `credential` | String |  | Credential used when interacting with this type. |
| `options` | String |  | Options to apply when handling any resources in this service. |
| `self_link` | String |  | Output only. Self link for the type provider. |
| `id` | String |  | Output only. Unique identifier for the resource defined by the server. |
| `collection_overrides` | Vec<String> |  | Allows resource handling overrides for specific collections |
| `custom_certificate_authority_roots` | Vec<String> |  | List of up to 2 custom certificate authority roots to use for TLS authentication when making calls on behalf of this type provider. If set, TLS authentication will exclusively use these roots instead of relying on publicly trusted certificate authorities when validating TLS certificate authenticity. The certificates must be in base64-encoded PEM format. The maximum size of each certificate must not exceed 10KB. |
| `labels` | Vec<String> |  | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?` |
| `name` | String |  | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this type provider. |
| `descriptor_url` | String |  | Descriptor Url for the this type provider. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `description` | String |  | An optional textual description of the resource; provided by the client when the resource is created. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `credential` | String | Credential used when interacting with this type. |
| `options` | String | Options to apply when handling any resources in this service. |
| `self_link` | String | Output only. Self link for the type provider. |
| `id` | String | Output only. Unique identifier for the resource defined by the server. |
| `collection_overrides` | Vec<String> | Allows resource handling overrides for specific collections |
| `custom_certificate_authority_roots` | Vec<String> | List of up to 2 custom certificate authority roots to use for TLS authentication when making calls on behalf of this type provider. If set, TLS authentication will exclusively use these roots instead of relying on publicly trusted certificate authorities when validating TLS certificate authenticity. The certificates must be in base64-encoded PEM format. The maximum size of each certificate must not exceed 10KB. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?` |
| `name` | String | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this type provider. |
| `descriptor_url` | String | Descriptor Url for the this type provider. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |


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
type_provider_credential = type_provider.credential
type_provider_options = type_provider.options
type_provider_self_link = type_provider.self_link
type_provider_id = type_provider.id
type_provider_collection_overrides = type_provider.collection_overrides
type_provider_custom_certificate_authority_roots = type_provider.custom_certificate_authority_roots
type_provider_labels = type_provider.labels
type_provider_name = type_provider.name
type_provider_operation = type_provider.operation
type_provider_descriptor_url = type_provider.descriptor_url
type_provider_insert_time = type_provider.insert_time
type_provider_description = type_provider.description
```

---


### Type_provider

Creates a type provider.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_certificate_authority_roots` | Vec<String> |  | List of up to 2 custom certificate authority roots to use for TLS authentication when making calls on behalf of this type provider. If set, TLS authentication will exclusively use these roots instead of relying on publicly trusted certificate authorities when validating TLS certificate authenticity. The certificates must be in base64-encoded PEM format. The maximum size of each certificate must not exceed 10KB. |
| `collection_overrides` | Vec<String> |  | Allows resource handling overrides for specific collections |
| `credential` | String |  | Credential used when interacting with this type. |
| `descriptor_url` | String |  | Descriptor Url for the this type provider. |
| `name` | String |  | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this type provider. |
| `labels` | Vec<String> |  | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?` |
| `description` | String |  | An optional textual description of the resource; provided by the client when the resource is created. |
| `self_link` | String |  | Output only. Self link for the type provider. |
| `id` | String |  | Output only. Unique identifier for the resource defined by the server. |
| `options` | String |  | Options to apply when handling any resources in this service. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_certificate_authority_roots` | Vec<String> | List of up to 2 custom certificate authority roots to use for TLS authentication when making calls on behalf of this type provider. If set, TLS authentication will exclusively use these roots instead of relying on publicly trusted certificate authorities when validating TLS certificate authenticity. The certificates must be in base64-encoded PEM format. The maximum size of each certificate must not exceed 10KB. |
| `collection_overrides` | Vec<String> | Allows resource handling overrides for specific collections |
| `credential` | String | Credential used when interacting with this type. |
| `descriptor_url` | String | Descriptor Url for the this type provider. |
| `name` | String | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this type provider. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?` |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |
| `self_link` | String | Output only. Self link for the type provider. |
| `id` | String | Output only. Unique identifier for the resource defined by the server. |
| `options` | String | Options to apply when handling any resources in this service. |


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
type_provider_custom_certificate_authority_roots = type_provider.custom_certificate_authority_roots
type_provider_collection_overrides = type_provider.collection_overrides
type_provider_credential = type_provider.credential
type_provider_descriptor_url = type_provider.descriptor_url
type_provider_name = type_provider.name
type_provider_insert_time = type_provider.insert_time
type_provider_operation = type_provider.operation
type_provider_labels = type_provider.labels
type_provider_description = type_provider.description
type_provider_self_link = type_provider.self_link
type_provider_id = type_provider.id
type_provider_options = type_provider.options
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
| `user` | String | [Output Only] User who requested the operation, for example: `user@example.com` or `alice_smith_identifier (global/workforcePools/example-com-us-employees)`. |
| `warnings` | Vec<String> | [Output Only] If warning messages are generated during processing of the operation, this field will be populated. |
| `zone` | String | [Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations. |
| `http_error_message` | String | [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as `NOT FOUND`. |
| `name` | String | [Output Only] Name of the operation. |
| `operation_type` | String | [Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on. |
| `set_autoscaler_link_operation_metadata` | String | This field is used internally by the Autoscaler team and should not be promoted to "alpha/beta/v1". |
| `creation_timestamp` | String | [Deprecated] This field is deprecated. |
| `operation_group_id` | String | [Output Only] An ID that represents a group of operations, such as when a group of operations results from a `bulkInsert` API request. |
| `status_message` | String | [Output Only] An optional textual description of the current status of the operation. |
| `error` | String | [Output Only] If errors are generated during processing of the operation, this field will be populated. |
| `progress` | i64 | [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses. |
| `description` | String | [Output Only] A textual description of the operation, which is set when the operation is created. |
| `kind` | String | [Output Only] Type of the resource. Always `compute#operation` for Operation resources. |
| `client_operation_id` | String | [Output Only] The value of `requestId` if you provided it in the request. Not present otherwise. |
| `http_error_status_code` | i64 | [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a `404` means the resource was not found. |
| `start_time` | String | [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format. |
| `instances_bulk_insert_operation_metadata` | String |  |
| `self_link_with_id` | String | [Output Only] Server-defined URL for this resource with the resource id. |
| `end_time` | String | [Output Only] The time that this operation was completed. This value is in RFC3339 text format. |
| `insert_time` | String | [Output Only] The time that this operation was requested. This value is in RFC3339 text format. |
| `target_id` | String | [Output Only] The unique target ID, which identifies a specific incarnation of the target resource. |
| `id` | String | [Output Only] The unique identifier for the operation. This identifier is defined by the server. |
| `set_common_instance_metadata_operation_metadata` | String | [Output Only] If the operation is for projects.setCommonInstanceMetadata, this field will contain information on all underlying zonal actions and their state. |
| `status` | String | [Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`. |
| `self_link` | String | [Output Only] Server-defined URL for the resource. |
| `region` | String | [Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations. |
| `target_link` | String | [Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the disk that the snapshot was created from. |


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
operation_user = operation.user
operation_warnings = operation.warnings
operation_zone = operation.zone
operation_http_error_message = operation.http_error_message
operation_name = operation.name
operation_operation_type = operation.operation_type
operation_set_autoscaler_link_operation_metadata = operation.set_autoscaler_link_operation_metadata
operation_creation_timestamp = operation.creation_timestamp
operation_operation_group_id = operation.operation_group_id
operation_status_message = operation.status_message
operation_error = operation.error
operation_progress = operation.progress
operation_description = operation.description
operation_kind = operation.kind
operation_client_operation_id = operation.client_operation_id
operation_http_error_status_code = operation.http_error_status_code
operation_start_time = operation.start_time
operation_instances_bulk_insert_operation_metadata = operation.instances_bulk_insert_operation_metadata
operation_self_link_with_id = operation.self_link_with_id
operation_end_time = operation.end_time
operation_insert_time = operation.insert_time
operation_target_id = operation.target_id
operation_id = operation.id
operation_set_common_instance_metadata_operation_metadata = operation.set_common_instance_metadata_operation_metadata
operation_status = operation.status
operation_self_link = operation.self_link
operation_region = operation.region
operation_target_link = operation.target_link
```

---


### Deployment

Creates a deployment and all of the resources described by the deployment manifest.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | An optional user-provided description of the deployment. |
| `fingerprint` | String |  | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `self_link` | String |  | Output only. Server defined URL for the resource. |
| `manifest` | String |  | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `labels` | Vec<String> |  | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `id` | String |  |  |
| `target` | String |  | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `update` | String |  | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `name` | String |  | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `update_time` | String |  | Output only. Update timestamp in RFC3339 text format. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | An optional user-provided description of the deployment. |
| `fingerprint` | String | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `self_link` | String | Output only. Server defined URL for the resource. |
| `manifest` | String | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `id` | String |  |
| `target` | String | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `name` | String | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
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
deployment_description = deployment.description
deployment_fingerprint = deployment.fingerprint
deployment_self_link = deployment.self_link
deployment_manifest = deployment.manifest
deployment_labels = deployment.labels
deployment_operation = deployment.operation
deployment_id = deployment.id
deployment_target = deployment.target
deployment_update = deployment.update
deployment_insert_time = deployment.insert_time
deployment_name = deployment.name
deployment_update_time = deployment.update_time
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


### Manifest

Gets information about a specific manifest.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | Output only. Self link for the manifest. |
| `manifest_size_limit_bytes` | String | Output only. The size limit for expanded manifests in the project. |
| `imports` | Vec<String> | Output only. The imported files for this manifest. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `layout` | String | Output only. The YAML layout for this manifest. |
| `id` | String |  |
| `config` | String | Output only. The YAML configuration for this manifest. |
| `name` | String | Output only. The name of the manifest. |
| `expanded_config` | String | Output only. The fully-expanded configuration file, including any templates and references. |
| `manifest_size_bytes` | String | Output only. The computed size of the fully expanded manifest. |


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
manifest_self_link = manifest.self_link
manifest_manifest_size_limit_bytes = manifest.manifest_size_limit_bytes
manifest_imports = manifest.imports
manifest_insert_time = manifest.insert_time
manifest_layout = manifest.layout
manifest_id = manifest.id
manifest_config = manifest.config
manifest_name = manifest.name
manifest_expanded_config = manifest.expanded_config
manifest_manifest_size_bytes = manifest.manifest_size_bytes
```

---


### Composite_type

Creates a composite type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String |  |  |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this composite type. |
| `id` | String |  |  |
| `name` | String |  | Name of the composite type, must follow the expression: `[a-z]([-a-z0-9_.]{0,61}[a-z0-9])?`. |
| `self_link` | String |  | Output only. Server defined URL for the resource. |
| `template_contents` | String |  | Files for the template type. |
| `description` | String |  | An optional textual description of the resource; provided by the client when the resource is created. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `labels` | Vec<String> |  | Map of labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String |  |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this composite type. |
| `id` | String |  |
| `name` | String | Name of the composite type, must follow the expression: `[a-z]([-a-z0-9_.]{0,61}[a-z0-9])?`. |
| `self_link` | String | Output only. Server defined URL for the resource. |
| `template_contents` | String | Files for the template type. |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `labels` | Vec<String> | Map of labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |


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
composite_type_status = composite_type.status
composite_type_operation = composite_type.operation
composite_type_id = composite_type.id
composite_type_name = composite_type.name
composite_type_self_link = composite_type.self_link
composite_type_template_contents = composite_type.template_contents
composite_type_description = composite_type.description
composite_type_insert_time = composite_type.insert_time
composite_type_labels = composite_type.labels
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
| `manifest` | String | Output only. URL of the manifest representing the current configuration of this resource. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `name` | String | Output only. The name of the resource as it appears in the YAML config. |
| `access_control` | String | The Access Control Policy set on this resource. |
| `properties` | String | Output only. The current properties of the resource before any references have been filled in. Returned as serialized YAML. |
| `type` | String | Output only. The type of the resource, for example `compute.v1.instance`, or `cloudfunctions.v1beta1.function`. |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this resource, the updated configuration appears here. |
| `final_properties` | String | Output only. The evaluated properties of the resource with references expanded. Returned as serialized YAML. |
| `id` | String |  |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |
| `warnings` | Vec<String> | Output only. If warning messages are generated during processing of this resource, this field will be populated. |
| `url` | String | Output only. The URL of the actual resource. |


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
resource_manifest = resource.manifest
resource_insert_time = resource.insert_time
resource_name = resource.name
resource_access_control = resource.access_control
resource_properties = resource.properties
resource_type = resource.type
resource_update = resource.update
resource_final_properties = resource.final_properties
resource_id = resource.id
resource_update_time = resource.update_time
resource_warnings = resource.warnings
resource_url = resource.url
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
| `description` | String |  | An optional user-provided description of the deployment. |
| `labels` | Vec<String> |  | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `id` | String |  |  |
| `update` | String |  | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `operation` | String |  | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `insert_time` | String |  | Output only. Creation timestamp in RFC3339 text format. |
| `update_time` | String |  | Output only. Update timestamp in RFC3339 text format. |
| `manifest` | String |  | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `name` | String |  | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `fingerprint` | String |  | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
| `self_link` | String |  | Output only. Server defined URL for the resource. |
| `project` | String | ✅ | The project ID for this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target` | String | [Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates. |
| `description` | String | An optional user-provided description of the deployment. |
| `labels` | Vec<String> | Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. |
| `id` | String |  |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here. |
| `operation` | String | Output only. The Operation that most recently ran, or is currently running, on this deployment. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |
| `manifest` | String | Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent. |
| `name` | String | Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. |
| `fingerprint` | String | Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment. |
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
deployment_description = deployment.description
deployment_labels = deployment.labels
deployment_id = deployment.id
deployment_update = deployment.update
deployment_operation = deployment.operation
deployment_insert_time = deployment.insert_time
deployment_update_time = deployment.update_time
deployment_manifest = deployment.manifest
deployment_name = deployment.name
deployment_fingerprint = deployment.fingerprint
deployment_self_link = deployment.self_link
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
| `start_time` | String | [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format. |
| `instances_bulk_insert_operation_metadata` | String |  |
| `target_link` | String | [Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the disk that the snapshot was created from. |
| `insert_time` | String | [Output Only] The time that this operation was requested. This value is in RFC3339 text format. |
| `region` | String | [Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations. |
| `kind` | String | [Output Only] Type of the resource. Always `compute#operation` for Operation resources. |
| `warnings` | Vec<String> | [Output Only] If warning messages are generated during processing of the operation, this field will be populated. |
| `error` | String | [Output Only] If errors are generated during processing of the operation, this field will be populated. |
| `set_autoscaler_link_operation_metadata` | String | This field is used internally by the Autoscaler team and should not be promoted to "alpha/beta/v1". |
| `http_error_message` | String | [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as `NOT FOUND`. |
| `target_id` | String | [Output Only] The unique target ID, which identifies a specific incarnation of the target resource. |
| `end_time` | String | [Output Only] The time that this operation was completed. This value is in RFC3339 text format. |
| `client_operation_id` | String | [Output Only] The value of `requestId` if you provided it in the request. Not present otherwise. |
| `description` | String | [Output Only] A textual description of the operation, which is set when the operation is created. |
| `status` | String | [Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`. |
| `status_message` | String | [Output Only] An optional textual description of the current status of the operation. |
| `operation_group_id` | String | [Output Only] An ID that represents a group of operations, such as when a group of operations results from a `bulkInsert` API request. |
| `user` | String | [Output Only] User who requested the operation, for example: `user@example.com` or `alice_smith_identifier (global/workforcePools/example-com-us-employees)`. |
| `zone` | String | [Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations. |
| `creation_timestamp` | String | [Deprecated] This field is deprecated. |
| `http_error_status_code` | i64 | [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a `404` means the resource was not found. |
| `id` | String | [Output Only] The unique identifier for the operation. This identifier is defined by the server. |
| `name` | String | [Output Only] Name of the operation. |
| `self_link` | String | [Output Only] Server-defined URL for the resource. |
| `operation_type` | String | [Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on. |
| `progress` | i64 | [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses. |
| `self_link_with_id` | String | [Output Only] Server-defined URL for this resource with the resource id. |
| `set_common_instance_metadata_operation_metadata` | String | [Output Only] If the operation is for projects.setCommonInstanceMetadata, this field will contain information on all underlying zonal actions and their state. |


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
operation_start_time = operation.start_time
operation_instances_bulk_insert_operation_metadata = operation.instances_bulk_insert_operation_metadata
operation_target_link = operation.target_link
operation_insert_time = operation.insert_time
operation_region = operation.region
operation_kind = operation.kind
operation_warnings = operation.warnings
operation_error = operation.error
operation_set_autoscaler_link_operation_metadata = operation.set_autoscaler_link_operation_metadata
operation_http_error_message = operation.http_error_message
operation_target_id = operation.target_id
operation_end_time = operation.end_time
operation_client_operation_id = operation.client_operation_id
operation_description = operation.description
operation_status = operation.status
operation_status_message = operation.status_message
operation_operation_group_id = operation.operation_group_id
operation_user = operation.user
operation_zone = operation.zone
operation_creation_timestamp = operation.creation_timestamp
operation_http_error_status_code = operation.http_error_status_code
operation_id = operation.id
operation_name = operation.name
operation_self_link = operation.self_link
operation_operation_type = operation.operation_type
operation_progress = operation.progress
operation_self_link_with_id = operation.self_link_with_id
operation_set_common_instance_metadata_operation_metadata = operation.set_common_instance_metadata_operation_metadata
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
| `id` | String |  |
| `manifest` | String | Output only. URL of the manifest representing the current configuration of this resource. |
| `name` | String | Output only. The name of the resource as it appears in the YAML config. |
| `properties` | String | Output only. The current properties of the resource before any references have been filled in. Returned as serialized YAML. |
| `type` | String | Output only. The type of the resource, for example `compute.v1.instance`, or `cloudfunctions.v1beta1.function`. |
| `update` | String | Output only. If Deployment Manager is currently updating or previewing an update to this resource, the updated configuration appears here. |
| `final_properties` | String | Output only. The evaluated properties of the resource with references expanded. Returned as serialized YAML. |
| `access_control` | String | The Access Control Policy set on this resource. |
| `update_time` | String | Output only. Update timestamp in RFC3339 text format. |
| `url` | String | Output only. The URL of the actual resource. |
| `warnings` | Vec<String> | Output only. If warning messages are generated during processing of this resource, this field will be populated. |
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |


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
resource_id = resource.id
resource_manifest = resource.manifest
resource_name = resource.name
resource_properties = resource.properties
resource_type = resource.type
resource_update = resource.update
resource_final_properties = resource.final_properties
resource_access_control = resource.access_control
resource_update_time = resource.update_time
resource_url = resource.url
resource_warnings = resource.warnings
resource_insert_time = resource.insert_time
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
| `insert_time` | String | Output only. Creation timestamp in RFC3339 text format. |
| `manifest_size_limit_bytes` | String | Output only. The size limit for expanded manifests in the project. |
| `manifest_size_bytes` | String | Output only. The computed size of the fully expanded manifest. |
| `imports` | Vec<String> | Output only. The imported files for this manifest. |
| `expanded_config` | String | Output only. The fully-expanded configuration file, including any templates and references. |
| `id` | String |  |
| `name` | String | Output only. The name of the manifest. |
| `self_link` | String | Output only. Self link for the manifest. |
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
manifest_insert_time = manifest.insert_time
manifest_manifest_size_limit_bytes = manifest.manifest_size_limit_bytes
manifest_manifest_size_bytes = manifest.manifest_size_bytes
manifest_imports = manifest.imports
manifest_expanded_config = manifest.expanded_config
manifest_id = manifest.id
manifest_name = manifest.name
manifest_self_link = manifest.self_link
manifest_layout = manifest.layout
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple operation resources
operation_0 = provider.deploymentmanager_api.Operation {
}
operation_1 = provider.deploymentmanager_api.Operation {
}
operation_2 = provider.deploymentmanager_api.Operation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.deploymentmanager_api.Operation {
    }
```

---

## Related Documentation

- [GCP Deploymentmanager_api Documentation](https://cloud.google.com/deploymentmanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
