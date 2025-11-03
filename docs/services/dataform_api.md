# Dataform_api Service



**Resources**: 20

---

## Overview

The dataform_api service provides access to 20 resource types:

- [Release_config](#release_config) [CRUD]
- [Operation](#operation) [CRD]
- [Folder](#folder) [CR]
- [Workflow_invocation](#workflow_invocation) [CRD]
- [Team_folder](#team_folder) [CR]
- [Workflow_config](#workflow_config) [CRUD]
- [Compilation_result](#compilation_result) [CR]
- [Location](#location) [RU]
- [Repositorie](#repositorie) [CRUD]
- [Workspace](#workspace) [CRD]
- [Operation](#operation) [CRD]
- [Location](#location) [RU]
- [Workspace](#workspace) [CRD]
- [Workflow_config](#workflow_config) [CRUD]
- [Compilation_result](#compilation_result) [CR]
- [Folder](#folder) [CR]
- [Repositorie](#repositorie) [CRUD]
- [Release_config](#release_config) [CRUD]
- [Team_folder](#team_folder) [CR]
- [Workflow_invocation](#workflow_invocation) [CRD]

---

## Resources


### Release_config

Creates a new ReleaseConfig in a given Repository.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `git_commitish` | String |  | Required. Git commit/tag/branch name at which the repository should be compiled. Must exist in the remote repository. Examples: - a commit SHA: `12ade345` - a tag: `tag1` - a branch name: `branch1` |
| `time_zone` | String |  | Optional. Specifies the time zone to be used when interpreting cron_schedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC. |
| `name` | String |  | Identifier. The release config's name. |
| `disabled` | bool |  | Optional. Disables automatic creation of compilation results. |
| `release_compilation_result` | String |  | Optional. The name of the currently released compilation result for this release config. This value is updated when a compilation result is automatically created from this release config (using cron_schedule), or when this resource is updated by API call (perhaps to roll back to an earlier release). The compilation result must have been created using this release config. Must be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `code_compilation_config` | String |  | Optional. If set, fields of `code_compilation_config` override the default compilation settings that are specified in dataform.json. |
| `recent_scheduled_release_records` | Vec<String> |  | Output only. Records of the 10 most recent scheduled release attempts, ordered in descending order of `release_time`. Updated whenever automatic creation of a compilation result is triggered by cron_schedule. |
| `cron_schedule` | String |  | Optional. Optional schedule (in cron format) for automatic creation of compilation results. |
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `parent` | String | ✅ | Required. The repository in which to create the release config. Must be in the format `projects/*/locations/*/repositories/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `git_commitish` | String | Required. Git commit/tag/branch name at which the repository should be compiled. Must exist in the remote repository. Examples: - a commit SHA: `12ade345` - a tag: `tag1` - a branch name: `branch1` |
| `time_zone` | String | Optional. Specifies the time zone to be used when interpreting cron_schedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC. |
| `name` | String | Identifier. The release config's name. |
| `disabled` | bool | Optional. Disables automatic creation of compilation results. |
| `release_compilation_result` | String | Optional. The name of the currently released compilation result for this release config. This value is updated when a compilation result is automatically created from this release config (using cron_schedule), or when this resource is updated by API call (perhaps to roll back to an earlier release). The compilation result must have been created using this release config. Must be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `code_compilation_config` | String | Optional. If set, fields of `code_compilation_config` override the default compilation settings that are specified in dataform.json. |
| `recent_scheduled_release_records` | Vec<String> | Output only. Records of the 10 most recent scheduled release attempts, ordered in descending order of `release_time`. Updated whenever automatic creation of a compilation result is triggered by cron_schedule. |
| `cron_schedule` | String | Optional. Optional schedule (in cron format) for automatic creation of compilation results. |
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create release_config
release_config = provider.dataform_api.Release_config {
    parent = "value"  # Required. The repository in which to create the release config. Must be in the format `projects/*/locations/*/repositories/*`.
}

# Access release_config outputs
release_config_id = release_config.id
release_config_git_commitish = release_config.git_commitish
release_config_time_zone = release_config.time_zone
release_config_name = release_config.name
release_config_disabled = release_config.disabled
release_config_release_compilation_result = release_config.release_compilation_result
release_config_code_compilation_config = release_config.code_compilation_config
release_config_recent_scheduled_release_records = release_config.recent_scheduled_release_records
release_config_cron_schedule = release_config.cron_schedule
release_config_internal_metadata = release_config.internal_metadata
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.dataform_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
```

---


### Folder

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create folder
folder = provider.dataform_api.Folder {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access folder outputs
folder_id = folder.id
folder_etag = folder.etag
folder_version = folder.version
folder_bindings = folder.bindings
```

---


### Workflow_invocation

Creates a new WorkflowInvocation in a given Repository.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `data_encryption_state` | String |  | Output only. Only set if the repository has a KMS Key. |
| `workflow_config` | String |  | Immutable. The name of the workflow config to invoke. Must be in the format `projects/*/locations/*/repositories/*/workflowConfigs/*`. |
| `invocation_config` | String |  | Immutable. If left unset, a default InvocationConfig will be used. |
| `state` | String |  | Output only. This workflow invocation's current state. |
| `compilation_result` | String |  | Immutable. The name of the compilation result to use for this invocation. Must be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `resolved_compilation_result` | String |  | Output only. The resolved compilation result that was used to create this invocation. Will be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `name` | String |  | Output only. The workflow invocation's name. |
| `invocation_timing` | String |  | Output only. This workflow invocation's timing details. |
| `parent` | String | ✅ | Required. The repository in which to create the workflow invocation. Must be in the format `projects/*/locations/*/repositories/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `data_encryption_state` | String | Output only. Only set if the repository has a KMS Key. |
| `workflow_config` | String | Immutable. The name of the workflow config to invoke. Must be in the format `projects/*/locations/*/repositories/*/workflowConfigs/*`. |
| `invocation_config` | String | Immutable. If left unset, a default InvocationConfig will be used. |
| `state` | String | Output only. This workflow invocation's current state. |
| `compilation_result` | String | Immutable. The name of the compilation result to use for this invocation. Must be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `resolved_compilation_result` | String | Output only. The resolved compilation result that was used to create this invocation. Will be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `name` | String | Output only. The workflow invocation's name. |
| `invocation_timing` | String | Output only. This workflow invocation's timing details. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workflow_invocation
workflow_invocation = provider.dataform_api.Workflow_invocation {
    parent = "value"  # Required. The repository in which to create the workflow invocation. Must be in the format `projects/*/locations/*/repositories/*`.
}

# Access workflow_invocation outputs
workflow_invocation_id = workflow_invocation.id
workflow_invocation_internal_metadata = workflow_invocation.internal_metadata
workflow_invocation_data_encryption_state = workflow_invocation.data_encryption_state
workflow_invocation_workflow_config = workflow_invocation.workflow_config
workflow_invocation_invocation_config = workflow_invocation.invocation_config
workflow_invocation_state = workflow_invocation.state
workflow_invocation_compilation_result = workflow_invocation.compilation_result
workflow_invocation_resolved_compilation_result = workflow_invocation.resolved_compilation_result
workflow_invocation_name = workflow_invocation.name
workflow_invocation_invocation_timing = workflow_invocation.invocation_timing
```

---


### Team_folder

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create team_folder
team_folder = provider.dataform_api.Team_folder {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access team_folder outputs
team_folder_id = team_folder.id
team_folder_etag = team_folder.etag
team_folder_version = team_folder.version
team_folder_bindings = team_folder.bindings
```

---


### Workflow_config

Creates a new WorkflowConfig in a given Repository.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The workflow config's name. |
| `cron_schedule` | String |  | Optional. Optional schedule (in cron format) for automatic execution of this workflow config. |
| `recent_scheduled_execution_records` | Vec<String> |  | Output only. Records of the 10 most recent scheduled execution attempts, ordered in descending order of `execution_time`. Updated whenever automatic creation of a workflow invocation is triggered by cron_schedule. |
| `release_config` | String |  | Required. The name of the release config whose release_compilation_result should be executed. Must be in the format `projects/*/locations/*/repositories/*/releaseConfigs/*`. |
| `disabled` | bool |  | Optional. Disables automatic creation of workflow invocations. |
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `time_zone` | String |  | Optional. Specifies the time zone to be used when interpreting cron_schedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC. |
| `update_time` | String |  | Output only. The timestamp of when the WorkflowConfig was last updated. |
| `create_time` | String |  | Output only. The timestamp of when the WorkflowConfig was created. |
| `invocation_config` | String |  | Optional. If left unset, a default InvocationConfig will be used. |
| `parent` | String | ✅ | Required. The repository in which to create the workflow config. Must be in the format `projects/*/locations/*/repositories/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The workflow config's name. |
| `cron_schedule` | String | Optional. Optional schedule (in cron format) for automatic execution of this workflow config. |
| `recent_scheduled_execution_records` | Vec<String> | Output only. Records of the 10 most recent scheduled execution attempts, ordered in descending order of `execution_time`. Updated whenever automatic creation of a workflow invocation is triggered by cron_schedule. |
| `release_config` | String | Required. The name of the release config whose release_compilation_result should be executed. Must be in the format `projects/*/locations/*/repositories/*/releaseConfigs/*`. |
| `disabled` | bool | Optional. Disables automatic creation of workflow invocations. |
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `time_zone` | String | Optional. Specifies the time zone to be used when interpreting cron_schedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC. |
| `update_time` | String | Output only. The timestamp of when the WorkflowConfig was last updated. |
| `create_time` | String | Output only. The timestamp of when the WorkflowConfig was created. |
| `invocation_config` | String | Optional. If left unset, a default InvocationConfig will be used. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workflow_config
workflow_config = provider.dataform_api.Workflow_config {
    parent = "value"  # Required. The repository in which to create the workflow config. Must be in the format `projects/*/locations/*/repositories/*`.
}

# Access workflow_config outputs
workflow_config_id = workflow_config.id
workflow_config_name = workflow_config.name
workflow_config_cron_schedule = workflow_config.cron_schedule
workflow_config_recent_scheduled_execution_records = workflow_config.recent_scheduled_execution_records
workflow_config_release_config = workflow_config.release_config
workflow_config_disabled = workflow_config.disabled
workflow_config_internal_metadata = workflow_config.internal_metadata
workflow_config_time_zone = workflow_config.time_zone
workflow_config_update_time = workflow_config.update_time
workflow_config_create_time = workflow_config.create_time
workflow_config_invocation_config = workflow_config.invocation_config
```

---


### Compilation_result

Creates a new CompilationResult in a given project and location.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `create_time` | String |  | Output only. The timestamp of when the compilation result was created. |
| `git_commitish` | String |  | Immutable. Git commit/tag/branch name at which the repository should be compiled. Must exist in the remote repository. Examples: - a commit SHA: `12ade345` - a tag: `tag1` - a branch name: `branch1` |
| `compilation_errors` | Vec<String> |  | Output only. Errors encountered during project compilation. |
| `data_encryption_state` | String |  | Output only. Only set if the repository has a KMS Key. |
| `code_compilation_config` | String |  | Immutable. If set, fields of `code_compilation_config` override the default compilation settings that are specified in dataform.json. |
| `dataform_core_version` | String |  | Output only. The version of `@dataform/core` that was used for compilation. |
| `release_config` | String |  | Immutable. The name of the release config to compile. Must be in the format `projects/*/locations/*/repositories/*/releaseConfigs/*`. |
| `resolved_git_commit_sha` | String |  | Output only. The fully resolved Git commit SHA of the code that was compiled. Not set for compilation results whose source is a workspace. |
| `workspace` | String |  | Immutable. The name of the workspace to compile. Must be in the format `projects/*/locations/*/repositories/*/workspaces/*`. |
| `name` | String |  | Output only. The compilation result's name. |
| `parent` | String | ✅ | Required. The repository in which to create the compilation result. Must be in the format `projects/*/locations/*/repositories/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `create_time` | String | Output only. The timestamp of when the compilation result was created. |
| `git_commitish` | String | Immutable. Git commit/tag/branch name at which the repository should be compiled. Must exist in the remote repository. Examples: - a commit SHA: `12ade345` - a tag: `tag1` - a branch name: `branch1` |
| `compilation_errors` | Vec<String> | Output only. Errors encountered during project compilation. |
| `data_encryption_state` | String | Output only. Only set if the repository has a KMS Key. |
| `code_compilation_config` | String | Immutable. If set, fields of `code_compilation_config` override the default compilation settings that are specified in dataform.json. |
| `dataform_core_version` | String | Output only. The version of `@dataform/core` that was used for compilation. |
| `release_config` | String | Immutable. The name of the release config to compile. Must be in the format `projects/*/locations/*/repositories/*/releaseConfigs/*`. |
| `resolved_git_commit_sha` | String | Output only. The fully resolved Git commit SHA of the code that was compiled. Not set for compilation results whose source is a workspace. |
| `workspace` | String | Immutable. The name of the workspace to compile. Must be in the format `projects/*/locations/*/repositories/*/workspaces/*`. |
| `name` | String | Output only. The compilation result's name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create compilation_result
compilation_result = provider.dataform_api.Compilation_result {
    parent = "value"  # Required. The repository in which to create the compilation result. Must be in the format `projects/*/locations/*/repositories/*`.
}

# Access compilation_result outputs
compilation_result_id = compilation_result.id
compilation_result_internal_metadata = compilation_result.internal_metadata
compilation_result_create_time = compilation_result.create_time
compilation_result_git_commitish = compilation_result.git_commitish
compilation_result_compilation_errors = compilation_result.compilation_errors
compilation_result_data_encryption_state = compilation_result.data_encryption_state
compilation_result_code_compilation_config = compilation_result.code_compilation_config
compilation_result_dataform_core_version = compilation_result.dataform_core_version
compilation_result_release_config = compilation_result.release_config
compilation_result_resolved_git_commit_sha = compilation_result.resolved_git_commit_sha
compilation_result_workspace = compilation_result.workspace
compilation_result_name = compilation_result.name
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `name` | String |  | Identifier. The config name. |
| `default_kms_key_name` | String |  | Optional. The default KMS key that is used if no encryption key is provided when a repository is created. |
| `name` | String | ✅ | Identifier. The config name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |


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
location_name = location.name
location_location_id = location.location_id
location_display_name = location.display_name
location_labels = location.labels
location_metadata = location.metadata
```

---


### Repositorie

Creates a new Repository in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `service_account` | String |  | Optional. The service account to run workflow invocations under. |
| `kms_key_name` | String |  | Optional. The reference to a KMS encryption key. If provided, it will be used to encrypt user data in the repository and all child resources. It is not possible to add or update the encryption key after the repository is created. Example: `projects/{kms_project}/locations/{location}/keyRings/{key_location}/cryptoKeys/{key}` |
| `data_encryption_state` | String |  | Output only. A data encryption state of a Git repository if this Repository is protected by a KMS key. |
| `display_name` | String |  | Optional. The repository's user-friendly name. |
| `name` | String |  | Identifier. The repository's name. |
| `workspace_compilation_overrides` | String |  | Optional. If set, fields of `workspace_compilation_overrides` override the default compilation settings that are specified in dataform.json when creating workspace-scoped compilation results. See documentation for `WorkspaceCompilationOverrides` for more information. |
| `labels` | HashMap<String, String> |  | Optional. Repository user labels. |
| `set_authenticated_user_admin` | bool |  | Optional. Input only. If set to true, the authenticated user will be granted the roles/dataform.admin role on the created repository. |
| `npmrc_environment_variables_secret_version` | String |  | Optional. The name of the Secret Manager secret version to be used to interpolate variables into the .npmrc file for package installation operations. Must be in the format `projects/*/secrets/*/versions/*`. The file itself must be in a JSON format. |
| `create_time` | String |  | Output only. The timestamp of when the repository was created. |
| `git_remote_settings` | String |  | Optional. If set, configures this repository to be linked to a Git remote. |
| `parent` | String | ✅ | Required. The location in which to create the repository. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `service_account` | String | Optional. The service account to run workflow invocations under. |
| `kms_key_name` | String | Optional. The reference to a KMS encryption key. If provided, it will be used to encrypt user data in the repository and all child resources. It is not possible to add or update the encryption key after the repository is created. Example: `projects/{kms_project}/locations/{location}/keyRings/{key_location}/cryptoKeys/{key}` |
| `data_encryption_state` | String | Output only. A data encryption state of a Git repository if this Repository is protected by a KMS key. |
| `display_name` | String | Optional. The repository's user-friendly name. |
| `name` | String | Identifier. The repository's name. |
| `workspace_compilation_overrides` | String | Optional. If set, fields of `workspace_compilation_overrides` override the default compilation settings that are specified in dataform.json when creating workspace-scoped compilation results. See documentation for `WorkspaceCompilationOverrides` for more information. |
| `labels` | HashMap<String, String> | Optional. Repository user labels. |
| `set_authenticated_user_admin` | bool | Optional. Input only. If set to true, the authenticated user will be granted the roles/dataform.admin role on the created repository. |
| `npmrc_environment_variables_secret_version` | String | Optional. The name of the Secret Manager secret version to be used to interpolate variables into the .npmrc file for package installation operations. Must be in the format `projects/*/secrets/*/versions/*`. The file itself must be in a JSON format. |
| `create_time` | String | Output only. The timestamp of when the repository was created. |
| `git_remote_settings` | String | Optional. If set, configures this repository to be linked to a Git remote. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create repositorie
repositorie = provider.dataform_api.Repositorie {
    parent = "value"  # Required. The location in which to create the repository. Must be in the format `projects/*/locations/*`.
}

# Access repositorie outputs
repositorie_id = repositorie.id
repositorie_internal_metadata = repositorie.internal_metadata
repositorie_service_account = repositorie.service_account
repositorie_kms_key_name = repositorie.kms_key_name
repositorie_data_encryption_state = repositorie.data_encryption_state
repositorie_display_name = repositorie.display_name
repositorie_name = repositorie.name
repositorie_workspace_compilation_overrides = repositorie.workspace_compilation_overrides
repositorie_labels = repositorie.labels
repositorie_set_authenticated_user_admin = repositorie.set_authenticated_user_admin
repositorie_npmrc_environment_variables_secret_version = repositorie.npmrc_environment_variables_secret_version
repositorie_create_time = repositorie.create_time
repositorie_git_remote_settings = repositorie.git_remote_settings
```

---


### Workspace

Creates a new Workspace in a given Repository.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp of when the workspace was created. |
| `data_encryption_state` | String |  | Output only. A data encryption state of a Git repository if this Workspace is protected by a KMS key. |
| `name` | String |  | Identifier. The workspace's name. |
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `parent` | String | ✅ | Required. The repository in which to create the workspace. Must be in the format `projects/*/locations/*/repositories/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp of when the workspace was created. |
| `data_encryption_state` | String | Output only. A data encryption state of a Git repository if this Workspace is protected by a KMS key. |
| `name` | String | Identifier. The workspace's name. |
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workspace
workspace = provider.dataform_api.Workspace {
    parent = "value"  # Required. The repository in which to create the workspace. Must be in the format `projects/*/locations/*/repositories/*`.
}

# Access workspace outputs
workspace_id = workspace.id
workspace_create_time = workspace.create_time
workspace_data_encryption_state = workspace.data_encryption_state
workspace_name = workspace.name
workspace_internal_metadata = workspace.internal_metadata
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.dataform_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_kms_key_name` | String |  | Optional. The default KMS key that is used if no encryption key is provided when a repository is created. |
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `name` | String |  | Identifier. The config name. |
| `name` | String | ✅ | Identifier. The config name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_metadata = location.metadata
location_display_name = location.display_name
location_location_id = location.location_id
location_name = location.name
```

---


### Workspace

Creates a new Workspace in a given Repository.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_encryption_state` | String |  | Output only. A data encryption state of a Git repository if this Workspace is protected by a KMS key. |
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `create_time` | String |  | Output only. The timestamp of when the workspace was created. |
| `name` | String |  | Identifier. The workspace's name. |
| `parent` | String | ✅ | Required. The repository in which to create the workspace. Must be in the format `projects/*/locations/*/repositories/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_encryption_state` | String | Output only. A data encryption state of a Git repository if this Workspace is protected by a KMS key. |
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `create_time` | String | Output only. The timestamp of when the workspace was created. |
| `name` | String | Identifier. The workspace's name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workspace
workspace = provider.dataform_api.Workspace {
    parent = "value"  # Required. The repository in which to create the workspace. Must be in the format `projects/*/locations/*/repositories/*`.
}

# Access workspace outputs
workspace_id = workspace.id
workspace_data_encryption_state = workspace.data_encryption_state
workspace_internal_metadata = workspace.internal_metadata
workspace_create_time = workspace.create_time
workspace_name = workspace.name
```

---


### Workflow_config

Creates a new WorkflowConfig in a given Repository.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `invocation_config` | String |  | Optional. If left unset, a default InvocationConfig will be used. |
| `release_config` | String |  | Required. The name of the release config whose release_compilation_result should be executed. Must be in the format `projects/*/locations/*/repositories/*/releaseConfigs/*`. |
| `time_zone` | String |  | Optional. Specifies the time zone to be used when interpreting cron_schedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC. |
| `recent_scheduled_execution_records` | Vec<String> |  | Output only. Records of the 10 most recent scheduled execution attempts, ordered in descending order of `execution_time`. Updated whenever automatic creation of a workflow invocation is triggered by cron_schedule. |
| `update_time` | String |  | Output only. The timestamp of when the WorkflowConfig was last updated. |
| `name` | String |  | Identifier. The workflow config's name. |
| `disabled` | bool |  | Optional. Disables automatic creation of workflow invocations. |
| `create_time` | String |  | Output only. The timestamp of when the WorkflowConfig was created. |
| `cron_schedule` | String |  | Optional. Optional schedule (in cron format) for automatic execution of this workflow config. |
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `parent` | String | ✅ | Required. The repository in which to create the workflow config. Must be in the format `projects/*/locations/*/repositories/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invocation_config` | String | Optional. If left unset, a default InvocationConfig will be used. |
| `release_config` | String | Required. The name of the release config whose release_compilation_result should be executed. Must be in the format `projects/*/locations/*/repositories/*/releaseConfigs/*`. |
| `time_zone` | String | Optional. Specifies the time zone to be used when interpreting cron_schedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC. |
| `recent_scheduled_execution_records` | Vec<String> | Output only. Records of the 10 most recent scheduled execution attempts, ordered in descending order of `execution_time`. Updated whenever automatic creation of a workflow invocation is triggered by cron_schedule. |
| `update_time` | String | Output only. The timestamp of when the WorkflowConfig was last updated. |
| `name` | String | Identifier. The workflow config's name. |
| `disabled` | bool | Optional. Disables automatic creation of workflow invocations. |
| `create_time` | String | Output only. The timestamp of when the WorkflowConfig was created. |
| `cron_schedule` | String | Optional. Optional schedule (in cron format) for automatic execution of this workflow config. |
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workflow_config
workflow_config = provider.dataform_api.Workflow_config {
    parent = "value"  # Required. The repository in which to create the workflow config. Must be in the format `projects/*/locations/*/repositories/*`.
}

# Access workflow_config outputs
workflow_config_id = workflow_config.id
workflow_config_invocation_config = workflow_config.invocation_config
workflow_config_release_config = workflow_config.release_config
workflow_config_time_zone = workflow_config.time_zone
workflow_config_recent_scheduled_execution_records = workflow_config.recent_scheduled_execution_records
workflow_config_update_time = workflow_config.update_time
workflow_config_name = workflow_config.name
workflow_config_disabled = workflow_config.disabled
workflow_config_create_time = workflow_config.create_time
workflow_config_cron_schedule = workflow_config.cron_schedule
workflow_config_internal_metadata = workflow_config.internal_metadata
```

---


### Compilation_result

Creates a new CompilationResult in a given project and location.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workspace` | String |  | Immutable. The name of the workspace to compile. Must be in the format `projects/*/locations/*/repositories/*/workspaces/*`. |
| `compilation_errors` | Vec<String> |  | Output only. Errors encountered during project compilation. |
| `git_commitish` | String |  | Immutable. Git commit/tag/branch name at which the repository should be compiled. Must exist in the remote repository. Examples: - a commit SHA: `12ade345` - a tag: `tag1` - a branch name: `branch1` |
| `release_config` | String |  | Immutable. The name of the release config to compile. Must be in the format `projects/*/locations/*/repositories/*/releaseConfigs/*`. |
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `create_time` | String |  | Output only. The timestamp of when the compilation result was created. |
| `code_compilation_config` | String |  | Immutable. If set, fields of `code_compilation_config` override the default compilation settings that are specified in dataform.json. |
| `dataform_core_version` | String |  | Output only. The version of `@dataform/core` that was used for compilation. |
| `data_encryption_state` | String |  | Output only. Only set if the repository has a KMS Key. |
| `name` | String |  | Output only. The compilation result's name. |
| `resolved_git_commit_sha` | String |  | Output only. The fully resolved Git commit SHA of the code that was compiled. Not set for compilation results whose source is a workspace. |
| `parent` | String | ✅ | Required. The repository in which to create the compilation result. Must be in the format `projects/*/locations/*/repositories/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workspace` | String | Immutable. The name of the workspace to compile. Must be in the format `projects/*/locations/*/repositories/*/workspaces/*`. |
| `compilation_errors` | Vec<String> | Output only. Errors encountered during project compilation. |
| `git_commitish` | String | Immutable. Git commit/tag/branch name at which the repository should be compiled. Must exist in the remote repository. Examples: - a commit SHA: `12ade345` - a tag: `tag1` - a branch name: `branch1` |
| `release_config` | String | Immutable. The name of the release config to compile. Must be in the format `projects/*/locations/*/repositories/*/releaseConfigs/*`. |
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `create_time` | String | Output only. The timestamp of when the compilation result was created. |
| `code_compilation_config` | String | Immutable. If set, fields of `code_compilation_config` override the default compilation settings that are specified in dataform.json. |
| `dataform_core_version` | String | Output only. The version of `@dataform/core` that was used for compilation. |
| `data_encryption_state` | String | Output only. Only set if the repository has a KMS Key. |
| `name` | String | Output only. The compilation result's name. |
| `resolved_git_commit_sha` | String | Output only. The fully resolved Git commit SHA of the code that was compiled. Not set for compilation results whose source is a workspace. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create compilation_result
compilation_result = provider.dataform_api.Compilation_result {
    parent = "value"  # Required. The repository in which to create the compilation result. Must be in the format `projects/*/locations/*/repositories/*`.
}

# Access compilation_result outputs
compilation_result_id = compilation_result.id
compilation_result_workspace = compilation_result.workspace
compilation_result_compilation_errors = compilation_result.compilation_errors
compilation_result_git_commitish = compilation_result.git_commitish
compilation_result_release_config = compilation_result.release_config
compilation_result_internal_metadata = compilation_result.internal_metadata
compilation_result_create_time = compilation_result.create_time
compilation_result_code_compilation_config = compilation_result.code_compilation_config
compilation_result_dataform_core_version = compilation_result.dataform_core_version
compilation_result_data_encryption_state = compilation_result.data_encryption_state
compilation_result_name = compilation_result.name
compilation_result_resolved_git_commit_sha = compilation_result.resolved_git_commit_sha
```

---


### Folder

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create folder
folder = provider.dataform_api.Folder {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access folder outputs
folder_id = folder.id
folder_etag = folder.etag
folder_bindings = folder.bindings
folder_version = folder.version
```

---


### Repositorie

Creates a new Repository in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp of when the repository was created. |
| `service_account` | String |  | Optional. The service account to run workflow invocations under. |
| `name` | String |  | Identifier. The repository's name. |
| `npmrc_environment_variables_secret_version` | String |  | Optional. The name of the Secret Manager secret version to be used to interpolate variables into the .npmrc file for package installation operations. Must be in the format `projects/*/secrets/*/versions/*`. The file itself must be in a JSON format. |
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `data_encryption_state` | String |  | Output only. A data encryption state of a Git repository if this Repository is protected by a KMS key. |
| `set_authenticated_user_admin` | bool |  | Optional. Input only. If set to true, the authenticated user will be granted the roles/dataform.admin role on the created repository. To modify access to the created repository later apply setIamPolicy from https://cloud.google.com/dataform/reference/rest#rest-resource:-v1beta1.projects.locations.repositories |
| `git_remote_settings` | String |  | Optional. If set, configures this repository to be linked to a Git remote. |
| `workspace_compilation_overrides` | String |  | Optional. If set, fields of `workspace_compilation_overrides` override the default compilation settings that are specified in dataform.json when creating workspace-scoped compilation results. See documentation for `WorkspaceCompilationOverrides` for more information. |
| `display_name` | String |  | Optional. The repository's user-friendly name. |
| `labels` | HashMap<String, String> |  | Optional. Repository user labels. |
| `kms_key_name` | String |  | Optional. The reference to a KMS encryption key. If provided, it will be used to encrypt user data in the repository and all child resources. It is not possible to add or update the encryption key after the repository is created. Example: `projects/{kms_project}/locations/{location}/keyRings/{key_location}/cryptoKeys/{key}` |
| `parent` | String | ✅ | Required. The location in which to create the repository. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp of when the repository was created. |
| `service_account` | String | Optional. The service account to run workflow invocations under. |
| `name` | String | Identifier. The repository's name. |
| `npmrc_environment_variables_secret_version` | String | Optional. The name of the Secret Manager secret version to be used to interpolate variables into the .npmrc file for package installation operations. Must be in the format `projects/*/secrets/*/versions/*`. The file itself must be in a JSON format. |
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `data_encryption_state` | String | Output only. A data encryption state of a Git repository if this Repository is protected by a KMS key. |
| `set_authenticated_user_admin` | bool | Optional. Input only. If set to true, the authenticated user will be granted the roles/dataform.admin role on the created repository. To modify access to the created repository later apply setIamPolicy from https://cloud.google.com/dataform/reference/rest#rest-resource:-v1beta1.projects.locations.repositories |
| `git_remote_settings` | String | Optional. If set, configures this repository to be linked to a Git remote. |
| `workspace_compilation_overrides` | String | Optional. If set, fields of `workspace_compilation_overrides` override the default compilation settings that are specified in dataform.json when creating workspace-scoped compilation results. See documentation for `WorkspaceCompilationOverrides` for more information. |
| `display_name` | String | Optional. The repository's user-friendly name. |
| `labels` | HashMap<String, String> | Optional. Repository user labels. |
| `kms_key_name` | String | Optional. The reference to a KMS encryption key. If provided, it will be used to encrypt user data in the repository and all child resources. It is not possible to add or update the encryption key after the repository is created. Example: `projects/{kms_project}/locations/{location}/keyRings/{key_location}/cryptoKeys/{key}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create repositorie
repositorie = provider.dataform_api.Repositorie {
    parent = "value"  # Required. The location in which to create the repository. Must be in the format `projects/*/locations/*`.
}

# Access repositorie outputs
repositorie_id = repositorie.id
repositorie_create_time = repositorie.create_time
repositorie_service_account = repositorie.service_account
repositorie_name = repositorie.name
repositorie_npmrc_environment_variables_secret_version = repositorie.npmrc_environment_variables_secret_version
repositorie_internal_metadata = repositorie.internal_metadata
repositorie_data_encryption_state = repositorie.data_encryption_state
repositorie_set_authenticated_user_admin = repositorie.set_authenticated_user_admin
repositorie_git_remote_settings = repositorie.git_remote_settings
repositorie_workspace_compilation_overrides = repositorie.workspace_compilation_overrides
repositorie_display_name = repositorie.display_name
repositorie_labels = repositorie.labels
repositorie_kms_key_name = repositorie.kms_key_name
```

---


### Release_config

Creates a new ReleaseConfig in a given Repository.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The release config's name. |
| `cron_schedule` | String |  | Optional. Optional schedule (in cron format) for automatic creation of compilation results. |
| `code_compilation_config` | String |  | Optional. If set, fields of `code_compilation_config` override the default compilation settings that are specified in dataform.json. |
| `recent_scheduled_release_records` | Vec<String> |  | Output only. Records of the 10 most recent scheduled release attempts, ordered in descending order of `release_time`. Updated whenever automatic creation of a compilation result is triggered by cron_schedule. |
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `time_zone` | String |  | Optional. Specifies the time zone to be used when interpreting cron_schedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC. |
| `release_compilation_result` | String |  | Optional. The name of the currently released compilation result for this release config. This value is updated when a compilation result is automatically created from this release config (using cron_schedule), or when this resource is updated by API call (perhaps to roll back to an earlier release). The compilation result must have been created using this release config. Must be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `git_commitish` | String |  | Required. Git commit/tag/branch name at which the repository should be compiled. Must exist in the remote repository. Examples: - a commit SHA: `12ade345` - a tag: `tag1` - a branch name: `branch1` |
| `disabled` | bool |  | Optional. Disables automatic creation of compilation results. |
| `parent` | String | ✅ | Required. The repository in which to create the release config. Must be in the format `projects/*/locations/*/repositories/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The release config's name. |
| `cron_schedule` | String | Optional. Optional schedule (in cron format) for automatic creation of compilation results. |
| `code_compilation_config` | String | Optional. If set, fields of `code_compilation_config` override the default compilation settings that are specified in dataform.json. |
| `recent_scheduled_release_records` | Vec<String> | Output only. Records of the 10 most recent scheduled release attempts, ordered in descending order of `release_time`. Updated whenever automatic creation of a compilation result is triggered by cron_schedule. |
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `time_zone` | String | Optional. Specifies the time zone to be used when interpreting cron_schedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC. |
| `release_compilation_result` | String | Optional. The name of the currently released compilation result for this release config. This value is updated when a compilation result is automatically created from this release config (using cron_schedule), or when this resource is updated by API call (perhaps to roll back to an earlier release). The compilation result must have been created using this release config. Must be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `git_commitish` | String | Required. Git commit/tag/branch name at which the repository should be compiled. Must exist in the remote repository. Examples: - a commit SHA: `12ade345` - a tag: `tag1` - a branch name: `branch1` |
| `disabled` | bool | Optional. Disables automatic creation of compilation results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create release_config
release_config = provider.dataform_api.Release_config {
    parent = "value"  # Required. The repository in which to create the release config. Must be in the format `projects/*/locations/*/repositories/*`.
}

# Access release_config outputs
release_config_id = release_config.id
release_config_name = release_config.name
release_config_cron_schedule = release_config.cron_schedule
release_config_code_compilation_config = release_config.code_compilation_config
release_config_recent_scheduled_release_records = release_config.recent_scheduled_release_records
release_config_internal_metadata = release_config.internal_metadata
release_config_time_zone = release_config.time_zone
release_config_release_compilation_result = release_config.release_compilation_result
release_config_git_commitish = release_config.git_commitish
release_config_disabled = release_config.disabled
```

---


### Team_folder

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create team_folder
team_folder = provider.dataform_api.Team_folder {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access team_folder outputs
team_folder_id = team_folder.id
team_folder_etag = team_folder.etag
team_folder_bindings = team_folder.bindings
team_folder_version = team_folder.version
```

---


### Workflow_invocation

Creates a new WorkflowInvocation in a given Repository.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `invocation_config` | String |  | Immutable. If left unset, a default InvocationConfig will be used. |
| `state` | String |  | Output only. This workflow invocation's current state. |
| `workflow_config` | String |  | Immutable. The name of the workflow config to invoke. Must be in the format `projects/*/locations/*/repositories/*/workflowConfigs/*`. |
| `resolved_compilation_result` | String |  | Output only. The resolved compilation result that was used to create this invocation. Will be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `compilation_result` | String |  | Immutable. The name of the compilation result to use for this invocation. Must be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `data_encryption_state` | String |  | Output only. Only set if the repository has a KMS Key. |
| `internal_metadata` | String |  | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `invocation_timing` | String |  | Output only. This workflow invocation's timing details. |
| `name` | String |  | Output only. The workflow invocation's name. |
| `parent` | String | ✅ | Required. The repository in which to create the workflow invocation. Must be in the format `projects/*/locations/*/repositories/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invocation_config` | String | Immutable. If left unset, a default InvocationConfig will be used. |
| `state` | String | Output only. This workflow invocation's current state. |
| `workflow_config` | String | Immutable. The name of the workflow config to invoke. Must be in the format `projects/*/locations/*/repositories/*/workflowConfigs/*`. |
| `resolved_compilation_result` | String | Output only. The resolved compilation result that was used to create this invocation. Will be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `compilation_result` | String | Immutable. The name of the compilation result to use for this invocation. Must be in the format `projects/*/locations/*/repositories/*/compilationResults/*`. |
| `data_encryption_state` | String | Output only. Only set if the repository has a KMS Key. |
| `internal_metadata` | String | Output only. All the metadata information that is used internally to serve the resource. For example: timestamps, flags, status fields, etc. The format of this field is a JSON string. |
| `invocation_timing` | String | Output only. This workflow invocation's timing details. |
| `name` | String | Output only. The workflow invocation's name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workflow_invocation
workflow_invocation = provider.dataform_api.Workflow_invocation {
    parent = "value"  # Required. The repository in which to create the workflow invocation. Must be in the format `projects/*/locations/*/repositories/*`.
}

# Access workflow_invocation outputs
workflow_invocation_id = workflow_invocation.id
workflow_invocation_invocation_config = workflow_invocation.invocation_config
workflow_invocation_state = workflow_invocation.state
workflow_invocation_workflow_config = workflow_invocation.workflow_config
workflow_invocation_resolved_compilation_result = workflow_invocation.resolved_compilation_result
workflow_invocation_compilation_result = workflow_invocation.compilation_result
workflow_invocation_data_encryption_state = workflow_invocation.data_encryption_state
workflow_invocation_internal_metadata = workflow_invocation.internal_metadata
workflow_invocation_invocation_timing = workflow_invocation.invocation_timing
workflow_invocation_name = workflow_invocation.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple release_config resources
release_config_0 = provider.dataform_api.Release_config {
    parent = "value-0"
}
release_config_1 = provider.dataform_api.Release_config {
    parent = "value-1"
}
release_config_2 = provider.dataform_api.Release_config {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    release_config = provider.dataform_api.Release_config {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Dataform_api Documentation](https://cloud.google.com/dataform_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
