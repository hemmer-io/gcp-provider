# Workflows_api Service



**Resources**: 6

---

## Overview

The workflows_api service provides access to 6 resource types:

- [Operation](#operation) [RD]
- [Location](#location) [R]
- [Workflow](#workflow) [CRUD]
- [Workflow](#workflow) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [RD]

---

## Resources


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
operation_error = operation.error
operation_name = operation.name
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_metadata = location.metadata
location_display_name = location.display_name
location_name = location.name
location_location_id = location.location_id
location_labels = location.labels
```

---


### Workflow

Creates a new workflow. If a workflow with the specified name already exists in the specified project and location, the long running operation returns a ALREADY_EXISTS error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the workflow deployment. |
| `source_contents` | String |  | Workflow code to be executed. The size limit is 128KB. |
| `state_error` | String |  | Output only. Error regarding the state of the workflow. For example, this field will have error details if the execution data is unavailable due to revoked KMS key permissions. |
| `all_kms_keys_versions` | Vec<String> |  | Output only. A list of all KMS crypto key versions used to encrypt or decrypt the data associated with the workflow. |
| `crypto_key_version` | String |  | Output only. The resource name of a KMS crypto key version used to encrypt or decrypt the data associated with the workflow. Format: projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}/cryptoKeyVersions/{cryptoKeyVersion} |
| `create_time` | String |  | Output only. The timestamp for when the workflow was created. This is a workflow-wide field and is not tied to a specific revision. |
| `all_kms_keys` | Vec<String> |  | Output only. A list of all KMS crypto keys used to encrypt or decrypt the data associated with the workflow. |
| `labels` | HashMap<String, String> |  | Labels associated with this workflow. Labels can contain at most 64 entries. Keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores, and dashes. Label keys must start with a letter. International characters are allowed. This is a workflow-wide field and is not tied to a specific revision. |
| `name` | String |  | The resource name of the workflow. Format: projects/{project}/locations/{location}/workflows/{workflow}. This is a workflow-wide field and is not tied to a specific revision. |
| `service_account` | String |  | The service account associated with the latest workflow version. This service account represents the identity of the workflow and determines what permissions the workflow has. Format: projects/{project}/serviceAccounts/{account} or {account} Using `-` as a wildcard for the `{project}` or not providing one at all will infer the project from the account. The `{account}` value can be the `email` address or the `unique_id` of the service account. If not provided, workflow will use the project's default service account. Modifying this field for an existing workflow results in a new workflow revision. |
| `update_time` | String |  | Output only. The timestamp for when the workflow was last updated. This is a workflow-wide field and is not tied to a specific revision. |
| `call_log_level` | String |  | Optional. Describes the level of platform logging to apply to calls and call responses during executions of this workflow. If both the workflow and the execution specify a logging level, the execution level takes precedence. |
| `revision_id` | String |  | Output only. The revision of the workflow. A new revision of a workflow is created as a result of updating the following properties of a workflow: - Service account - Workflow code to be executed The format is "000001-a4d", where the first six characters define the zero-padded revision ordinal number. They are followed by a hyphen and three hexadecimal random characters. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tags associated with this workflow. |
| `description` | String |  | Description of the workflow provided by the user. Must be at most 1000 Unicode characters long. This is a workflow-wide field and is not tied to a specific revision. |
| `user_env_vars` | HashMap<String, String> |  | Optional. User-defined environment variables associated with this workflow revision. This map has a maximum length of 20. Each string can take up to 4KiB. Keys cannot be empty strings and cannot start with "GOOGLE" or "WORKFLOWS". |
| `crypto_key_name` | String |  | Optional. The resource name of a KMS crypto key used to encrypt or decrypt the data associated with the workflow. Format: projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey} Using `-` as a wildcard for the `{project}` or not providing one at all will infer the project from the account. If not provided, data associated with the workflow will not be CMEK-encrypted. |
| `execution_history_level` | String |  | Optional. Describes the execution history level to apply to this workflow. |
| `revision_create_time` | String |  | Output only. The timestamp for the latest revision of the workflow's creation. |
| `parent` | String | ✅ | Required. Project and location in which the workflow should be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the workflow deployment. |
| `source_contents` | String | Workflow code to be executed. The size limit is 128KB. |
| `state_error` | String | Output only. Error regarding the state of the workflow. For example, this field will have error details if the execution data is unavailable due to revoked KMS key permissions. |
| `all_kms_keys_versions` | Vec<String> | Output only. A list of all KMS crypto key versions used to encrypt or decrypt the data associated with the workflow. |
| `crypto_key_version` | String | Output only. The resource name of a KMS crypto key version used to encrypt or decrypt the data associated with the workflow. Format: projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}/cryptoKeyVersions/{cryptoKeyVersion} |
| `create_time` | String | Output only. The timestamp for when the workflow was created. This is a workflow-wide field and is not tied to a specific revision. |
| `all_kms_keys` | Vec<String> | Output only. A list of all KMS crypto keys used to encrypt or decrypt the data associated with the workflow. |
| `labels` | HashMap<String, String> | Labels associated with this workflow. Labels can contain at most 64 entries. Keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores, and dashes. Label keys must start with a letter. International characters are allowed. This is a workflow-wide field and is not tied to a specific revision. |
| `name` | String | The resource name of the workflow. Format: projects/{project}/locations/{location}/workflows/{workflow}. This is a workflow-wide field and is not tied to a specific revision. |
| `service_account` | String | The service account associated with the latest workflow version. This service account represents the identity of the workflow and determines what permissions the workflow has. Format: projects/{project}/serviceAccounts/{account} or {account} Using `-` as a wildcard for the `{project}` or not providing one at all will infer the project from the account. The `{account}` value can be the `email` address or the `unique_id` of the service account. If not provided, workflow will use the project's default service account. Modifying this field for an existing workflow results in a new workflow revision. |
| `update_time` | String | Output only. The timestamp for when the workflow was last updated. This is a workflow-wide field and is not tied to a specific revision. |
| `call_log_level` | String | Optional. Describes the level of platform logging to apply to calls and call responses during executions of this workflow. If both the workflow and the execution specify a logging level, the execution level takes precedence. |
| `revision_id` | String | Output only. The revision of the workflow. A new revision of a workflow is created as a result of updating the following properties of a workflow: - Service account - Workflow code to be executed The format is "000001-a4d", where the first six characters define the zero-padded revision ordinal number. They are followed by a hyphen and three hexadecimal random characters. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tags associated with this workflow. |
| `description` | String | Description of the workflow provided by the user. Must be at most 1000 Unicode characters long. This is a workflow-wide field and is not tied to a specific revision. |
| `user_env_vars` | HashMap<String, String> | Optional. User-defined environment variables associated with this workflow revision. This map has a maximum length of 20. Each string can take up to 4KiB. Keys cannot be empty strings and cannot start with "GOOGLE" or "WORKFLOWS". |
| `crypto_key_name` | String | Optional. The resource name of a KMS crypto key used to encrypt or decrypt the data associated with the workflow. Format: projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey} Using `-` as a wildcard for the `{project}` or not providing one at all will infer the project from the account. If not provided, data associated with the workflow will not be CMEK-encrypted. |
| `execution_history_level` | String | Optional. Describes the execution history level to apply to this workflow. |
| `revision_create_time` | String | Output only. The timestamp for the latest revision of the workflow's creation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workflow
workflow = provider.workflows_api.Workflow {
    parent = "value"  # Required. Project and location in which the workflow should be created. Format: projects/{project}/locations/{location}
}

# Access workflow outputs
workflow_id = workflow.id
workflow_state = workflow.state
workflow_source_contents = workflow.source_contents
workflow_state_error = workflow.state_error
workflow_all_kms_keys_versions = workflow.all_kms_keys_versions
workflow_crypto_key_version = workflow.crypto_key_version
workflow_create_time = workflow.create_time
workflow_all_kms_keys = workflow.all_kms_keys
workflow_labels = workflow.labels
workflow_name = workflow.name
workflow_service_account = workflow.service_account
workflow_update_time = workflow.update_time
workflow_call_log_level = workflow.call_log_level
workflow_revision_id = workflow.revision_id
workflow_tags = workflow.tags
workflow_description = workflow.description
workflow_user_env_vars = workflow.user_env_vars
workflow_crypto_key_name = workflow.crypto_key_name
workflow_execution_history_level = workflow.execution_history_level
workflow_revision_create_time = workflow.revision_create_time
```

---


### Workflow

Creates a new workflow. If a workflow with the specified name already exists in the specified project and location, the long running operation will return ALREADY_EXISTS error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Labels associated with this workflow. Labels can contain at most 64 entries. Keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores and dashes. Label keys must start with a letter. International characters are allowed. |
| `name` | String |  | The resource name of the workflow. Format: projects/{project}/locations/{location}/workflows/{workflow} |
| `revision_id` | String |  | Output only. The revision of the workflow. A new revision of a workflow is created as a result of updating the following properties of a workflow: - Service account - Workflow code to be executed The format is "000001-a4d", where the first 6 characters define the zero-padded revision ordinal number. They are followed by a hyphen and 3 hexadecimal random characters. |
| `description` | String |  | Description of the workflow provided by the user. Must be at most 1000 unicode characters long. |
| `update_time` | String |  | Output only. The last update timestamp of the workflow. |
| `state` | String |  | Output only. State of the workflow deployment. |
| `service_account` | String |  | The service account associated with the latest workflow version. This service account represents the identity of the workflow and determines what permissions the workflow has. Format: projects/{project}/serviceAccounts/{account} or {account} Using `-` as a wildcard for the `{project}` or not providing one at all will infer the project from the account. The `{account}` value can be the `email` address or the `unique_id` of the service account. If not provided, workflow will use the project's default service account. Modifying this field for an existing workflow results in a new workflow revision. |
| `source_contents` | String |  | Workflow code to be executed. The size limit is 128KB. |
| `create_time` | String |  | Output only. The timestamp of when the workflow was created. |
| `revision_create_time` | String |  | Output only. The timestamp that the latest revision of the workflow was created. |
| `parent` | String | ✅ | Required. Project and location in which the workflow should be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Labels associated with this workflow. Labels can contain at most 64 entries. Keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores and dashes. Label keys must start with a letter. International characters are allowed. |
| `name` | String | The resource name of the workflow. Format: projects/{project}/locations/{location}/workflows/{workflow} |
| `revision_id` | String | Output only. The revision of the workflow. A new revision of a workflow is created as a result of updating the following properties of a workflow: - Service account - Workflow code to be executed The format is "000001-a4d", where the first 6 characters define the zero-padded revision ordinal number. They are followed by a hyphen and 3 hexadecimal random characters. |
| `description` | String | Description of the workflow provided by the user. Must be at most 1000 unicode characters long. |
| `update_time` | String | Output only. The last update timestamp of the workflow. |
| `state` | String | Output only. State of the workflow deployment. |
| `service_account` | String | The service account associated with the latest workflow version. This service account represents the identity of the workflow and determines what permissions the workflow has. Format: projects/{project}/serviceAccounts/{account} or {account} Using `-` as a wildcard for the `{project}` or not providing one at all will infer the project from the account. The `{account}` value can be the `email` address or the `unique_id` of the service account. If not provided, workflow will use the project's default service account. Modifying this field for an existing workflow results in a new workflow revision. |
| `source_contents` | String | Workflow code to be executed. The size limit is 128KB. |
| `create_time` | String | Output only. The timestamp of when the workflow was created. |
| `revision_create_time` | String | Output only. The timestamp that the latest revision of the workflow was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workflow
workflow = provider.workflows_api.Workflow {
    parent = "value"  # Required. Project and location in which the workflow should be created. Format: projects/{project}/locations/{location}
}

# Access workflow outputs
workflow_id = workflow.id
workflow_labels = workflow.labels
workflow_name = workflow.name
workflow_revision_id = workflow.revision_id
workflow_description = workflow.description
workflow_update_time = workflow.update_time
workflow_state = workflow.state
workflow_service_account = workflow.service_account
workflow_source_contents = workflow.source_contents
workflow_create_time = workflow.create_time
workflow_revision_create_time = workflow.revision_create_time
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_name = location.name
location_metadata = location.metadata
location_display_name = location.display_name
location_location_id = location.location_id
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
operation_metadata = operation.metadata
operation_response = operation.response
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
operation_0 = provider.workflows_api.Operation {
}
operation_1 = provider.workflows_api.Operation {
}
operation_2 = provider.workflows_api.Operation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.workflows_api.Operation {
    }
```

---

## Related Documentation

- [GCP Workflows_api Documentation](https://cloud.google.com/workflows_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
