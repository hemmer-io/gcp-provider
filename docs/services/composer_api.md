# Composer_api Service



**Resources**: 12

---

## Overview

The composer_api service provides access to 12 resource types:

- [Environment](#environment) [CRUD]
- [User_workloads_secret](#user_workloads_secret) [CRUD]
- [User_workloads_config_map](#user_workloads_config_map) [CRUD]
- [Workload](#workload) [R]
- [Operation](#operation) [RD]
- [Image_version](#image_version) [R]
- [Environment](#environment) [CRUD]
- [User_workloads_config_map](#user_workloads_config_map) [CRUD]
- [User_workloads_secret](#user_workloads_secret) [CRUD]
- [Operation](#operation) [RD]
- [Image_version](#image_version) [R]
- [Workload](#workload) [R]

---

## Resources


### Environment

Create a new environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uuid` | String |  | Output only. The UUID (Universally Unique IDentifier) associated with this environment. This value is generated when the environment is created. |
| `config` | String |  | Optional. Configuration parameters for this environment. |
| `storage_config` | String |  | Optional. Storage configuration for this environment. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `state` | String |  | The current state of the environment. |
| `name` | String |  | Identifier. The resource name of the environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}" EnvironmentId must start with a lowercase letter followed by up to 63 lowercase letters, numbers, or hyphens, and cannot end with a hyphen. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: * Keys must conform to regexp: \p{Ll}\p{Lo}{0,62} * Values must conform to regexp: [\p{Ll}\p{Lo}\p{N}_-]{0,63} * Both keys and values are additionally constrained to be <= 128 bytes in size. |
| `update_time` | String |  | Output only. The time at which this environment was last modified. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `create_time` | String |  | Output only. The time at which this environment was created. |
| `parent` | String | ✅ | The parent must be of the form "projects/{projectId}/locations/{locationId}". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uuid` | String | Output only. The UUID (Universally Unique IDentifier) associated with this environment. This value is generated when the environment is created. |
| `config` | String | Optional. Configuration parameters for this environment. |
| `storage_config` | String | Optional. Storage configuration for this environment. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `state` | String | The current state of the environment. |
| `name` | String | Identifier. The resource name of the environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}" EnvironmentId must start with a lowercase letter followed by up to 63 lowercase letters, numbers, or hyphens, and cannot end with a hyphen. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: * Keys must conform to regexp: \p{Ll}\p{Lo}{0,62} * Values must conform to regexp: [\p{Ll}\p{Lo}\p{N}_-]{0,63} * Both keys and values are additionally constrained to be <= 128 bytes in size. |
| `update_time` | String | Output only. The time at which this environment was last modified. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `create_time` | String | Output only. The time at which this environment was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.composer_api.Environment {
    parent = "value"  # The parent must be of the form "projects/{projectId}/locations/{locationId}".
}

# Access environment outputs
environment_id = environment.id
environment_uuid = environment.uuid
environment_config = environment.config
environment_storage_config = environment.storage_config
environment_satisfies_pzi = environment.satisfies_pzi
environment_state = environment.state
environment_name = environment.name
environment_labels = environment.labels
environment_update_time = environment.update_time
environment_satisfies_pzs = environment.satisfies_pzs
environment_create_time = environment.create_time
```

---


### User_workloads_secret

Creates a user workloads Secret. This method is supported for Cloud Composer environments in versions composer-3-airflow-*.*.*-build.* and newer.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | HashMap<String, String> |  | Optional. The "data" field of Kubernetes Secret, organized in key-value pairs, which can contain sensitive values such as a password, a token, or a key. The values for all keys have to be base64-encoded strings. For details see: https://kubernetes.io/docs/concepts/configuration/secret/ Example: { "example": "ZXhhbXBsZV92YWx1ZQ==", "another-example": "YW5vdGhlcl9leGFtcGxlX3ZhbHVl" } |
| `name` | String |  | Identifier. The resource name of the Secret, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsSecrets/{userWorkloadsSecretId}" |
| `parent` | String | ✅ | Required. The environment name to create a Secret for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | HashMap<String, String> | Optional. The "data" field of Kubernetes Secret, organized in key-value pairs, which can contain sensitive values such as a password, a token, or a key. The values for all keys have to be base64-encoded strings. For details see: https://kubernetes.io/docs/concepts/configuration/secret/ Example: { "example": "ZXhhbXBsZV92YWx1ZQ==", "another-example": "YW5vdGhlcl9leGFtcGxlX3ZhbHVl" } |
| `name` | String | Identifier. The resource name of the Secret, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsSecrets/{userWorkloadsSecretId}" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_workloads_secret
user_workloads_secret = provider.composer_api.User_workloads_secret {
    parent = "value"  # Required. The environment name to create a Secret for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
}

# Access user_workloads_secret outputs
user_workloads_secret_id = user_workloads_secret.id
user_workloads_secret_data = user_workloads_secret.data
user_workloads_secret_name = user_workloads_secret.name
```

---


### User_workloads_config_map

Creates a user workloads ConfigMap. This method is supported for Cloud Composer environments in versions composer-3-airflow-*.*.*-build.* and newer.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the ConfigMap, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsConfigMaps/{userWorkloadsConfigMapId}" |
| `data` | HashMap<String, String> |  | Optional. The "data" field of Kubernetes ConfigMap, organized in key-value pairs. For details see: https://kubernetes.io/docs/concepts/configuration/configmap/ Example: { "example_key": "example_value", "another_key": "another_value" } |
| `parent` | String | ✅ | Required. The environment name to create a ConfigMap for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the ConfigMap, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsConfigMaps/{userWorkloadsConfigMapId}" |
| `data` | HashMap<String, String> | Optional. The "data" field of Kubernetes ConfigMap, organized in key-value pairs. For details see: https://kubernetes.io/docs/concepts/configuration/configmap/ Example: { "example_key": "example_value", "another_key": "another_value" } |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_workloads_config_map
user_workloads_config_map = provider.composer_api.User_workloads_config_map {
    parent = "value"  # Required. The environment name to create a ConfigMap for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
}

# Access user_workloads_config_map outputs
user_workloads_config_map_id = user_workloads_config_map.id
user_workloads_config_map_name = user_workloads_config_map.name
user_workloads_config_map_data = user_workloads_config_map.data
```

---


### Workload

Lists workloads in a Cloud Composer environment. Workload is a unit that runs a single Composer component. This method is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The page token used to query for the next page if one exists. |
| `workloads` | Vec<String> | The list of environment workloads. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access workload outputs
workload_id = workload.id
workload_next_page_token = workload.next_page_token
workload_workloads = workload.workloads
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
operation_error = operation.error
```

---


### Image_version

List ImageVersions for provided location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The page token used to query for the next page if one exists. |
| `image_versions` | Vec<String> | The list of supported ImageVersions in a location. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access image_version outputs
image_version_id = image_version.id
image_version_next_page_token = image_version.next_page_token
image_version_image_versions = image_version.image_versions
```

---


### Environment

Create a new environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: * Keys must conform to regexp: \p{Ll}\p{Lo}{0,62} * Values must conform to regexp: [\p{Ll}\p{Lo}\p{N}_-]{0,63} * Both keys and values are additionally constrained to be <= 128 bytes in size. |
| `name` | String |  | Identifier. The resource name of the environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}" EnvironmentId must start with a lowercase letter followed by up to 63 lowercase letters, numbers, or hyphens, and cannot end with a hyphen. |
| `state` | String |  | The current state of the environment. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `storage_config` | String |  | Optional. Storage configuration for this environment. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. The time at which this environment was last modified. |
| `uuid` | String |  | Output only. The UUID (Universally Unique IDentifier) associated with this environment. This value is generated when the environment is created. |
| `config` | String |  | Optional. Configuration parameters for this environment. |
| `create_time` | String |  | Output only. The time at which this environment was created. |
| `parent` | String | ✅ | The parent must be of the form "projects/{projectId}/locations/{locationId}". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: * Keys must conform to regexp: \p{Ll}\p{Lo}{0,62} * Values must conform to regexp: [\p{Ll}\p{Lo}\p{N}_-]{0,63} * Both keys and values are additionally constrained to be <= 128 bytes in size. |
| `name` | String | Identifier. The resource name of the environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}" EnvironmentId must start with a lowercase letter followed by up to 63 lowercase letters, numbers, or hyphens, and cannot end with a hyphen. |
| `state` | String | The current state of the environment. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `storage_config` | String | Optional. Storage configuration for this environment. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. The time at which this environment was last modified. |
| `uuid` | String | Output only. The UUID (Universally Unique IDentifier) associated with this environment. This value is generated when the environment is created. |
| `config` | String | Optional. Configuration parameters for this environment. |
| `create_time` | String | Output only. The time at which this environment was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.composer_api.Environment {
    parent = "value"  # The parent must be of the form "projects/{projectId}/locations/{locationId}".
}

# Access environment outputs
environment_id = environment.id
environment_labels = environment.labels
environment_name = environment.name
environment_state = environment.state
environment_satisfies_pzi = environment.satisfies_pzi
environment_storage_config = environment.storage_config
environment_satisfies_pzs = environment.satisfies_pzs
environment_update_time = environment.update_time
environment_uuid = environment.uuid
environment_config = environment.config
environment_create_time = environment.create_time
```

---


### User_workloads_config_map

Creates a user workloads ConfigMap. This method is supported for Cloud Composer environments in versions composer-3-airflow-*.*.*-build.* and newer.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | HashMap<String, String> |  | Optional. The "data" field of Kubernetes ConfigMap, organized in key-value pairs. For details see: https://kubernetes.io/docs/concepts/configuration/configmap/ Example: { "example_key": "example_value", "another_key": "another_value" } |
| `name` | String |  | Identifier. The resource name of the ConfigMap, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsConfigMaps/{userWorkloadsConfigMapId}" |
| `parent` | String | ✅ | Required. The environment name to create a ConfigMap for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | HashMap<String, String> | Optional. The "data" field of Kubernetes ConfigMap, organized in key-value pairs. For details see: https://kubernetes.io/docs/concepts/configuration/configmap/ Example: { "example_key": "example_value", "another_key": "another_value" } |
| `name` | String | Identifier. The resource name of the ConfigMap, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsConfigMaps/{userWorkloadsConfigMapId}" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_workloads_config_map
user_workloads_config_map = provider.composer_api.User_workloads_config_map {
    parent = "value"  # Required. The environment name to create a ConfigMap for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
}

# Access user_workloads_config_map outputs
user_workloads_config_map_id = user_workloads_config_map.id
user_workloads_config_map_data = user_workloads_config_map.data
user_workloads_config_map_name = user_workloads_config_map.name
```

---


### User_workloads_secret

Creates a user workloads Secret. This method is supported for Cloud Composer environments in versions composer-3-airflow-*.*.*-build.* and newer.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | HashMap<String, String> |  | Optional. The "data" field of Kubernetes Secret, organized in key-value pairs, which can contain sensitive values such as a password, a token, or a key. The values for all keys have to be base64-encoded strings. For details see: https://kubernetes.io/docs/concepts/configuration/secret/ Example: { "example": "ZXhhbXBsZV92YWx1ZQ==", "another-example": "YW5vdGhlcl9leGFtcGxlX3ZhbHVl" } |
| `name` | String |  | Identifier. The resource name of the Secret, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsSecrets/{userWorkloadsSecretId}" |
| `parent` | String | ✅ | Required. The environment name to create a Secret for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | HashMap<String, String> | Optional. The "data" field of Kubernetes Secret, organized in key-value pairs, which can contain sensitive values such as a password, a token, or a key. The values for all keys have to be base64-encoded strings. For details see: https://kubernetes.io/docs/concepts/configuration/secret/ Example: { "example": "ZXhhbXBsZV92YWx1ZQ==", "another-example": "YW5vdGhlcl9leGFtcGxlX3ZhbHVl" } |
| `name` | String | Identifier. The resource name of the Secret, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsSecrets/{userWorkloadsSecretId}" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_workloads_secret
user_workloads_secret = provider.composer_api.User_workloads_secret {
    parent = "value"  # Required. The environment name to create a Secret for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
}

# Access user_workloads_secret outputs
user_workloads_secret_id = user_workloads_secret.id
user_workloads_secret_data = user_workloads_secret.data
user_workloads_secret_name = user_workloads_secret.name
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


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
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
```

---


### Image_version

List ImageVersions for provided location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_versions` | Vec<String> | The list of supported ImageVersions in a location. |
| `next_page_token` | String | The page token used to query for the next page if one exists. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access image_version outputs
image_version_id = image_version.id
image_version_image_versions = image_version.image_versions
image_version_next_page_token = image_version.next_page_token
```

---


### Workload

Lists workloads in a Cloud Composer environment. Workload is a unit that runs a single Composer component. This method is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workloads` | Vec<String> | The list of environment workloads. |
| `next_page_token` | String | The page token used to query for the next page if one exists. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access workload outputs
workload_id = workload.id
workload_workloads = workload.workloads
workload_next_page_token = workload.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple environment resources
environment_0 = provider.composer_api.Environment {
    parent = "value-0"
}
environment_1 = provider.composer_api.Environment {
    parent = "value-1"
}
environment_2 = provider.composer_api.Environment {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    environment = provider.composer_api.Environment {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Composer_api Documentation](https://cloud.google.com/composer_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
