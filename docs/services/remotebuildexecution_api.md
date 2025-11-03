# Remotebuildexecution_api Service



**Resources**: 10

---

## Overview

The remotebuildexecution_api service provides access to 10 resource types:

- [Media](#media) [CR]
- [Operation](#operation) [CRD]
- [Instance](#instance) [CRUD]
- [Workerpool](#workerpool) [CRUD]
- [Operation](#operation) [R]
- [Remotebuildexecution](#remotebuildexecution) [R]
- [Operation](#operation) [C]
- [Action](#action) [C]
- [Action_result](#action_result) [RU]
- [Blob](#blob) [CR]

---

## Resources


### Media

Uploads media. Upload is supported on the URI `/upload/v1/media/{+name}`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_name` | String |  | Name of the media resource. |
| `resource_name` | String | ✅ | Name of the media that is being downloaded. See ReadRequest.resource_name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_name` | String | Name of the media resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.remotebuildexecution_api.Media {
    resource_name = "value"  # Name of the media that is being downloaded. See ReadRequest.resource_name.
}

# Access media outputs
media_id = media.id
media_resource_name = media.resource_name
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.remotebuildexecution_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
```

---


### Instance

Creates a new instance in the specified region. Returns a long running operation which contains an instance on completion. While the long running operation is in progress, any call to `GetInstance` returns an instance in state `CREATING`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String |  | ID of the created instance. A valid `instance_id` must: be 6-50 characters long, contain only lowercase letters, digits, hyphens and underscores, start with a lowercase letter, and end with a lowercase letter or a digit. |
| `parent` | String |  | Resource name of the project containing the instance. Format: `projects/[PROJECT_ID]`. |
| `instance` | String |  | Specifies the instance to create. The name in the instance, if specified in the instance, is ignored. |
| `parent` | String | ✅ | Resource name of the project containing the instance. Format: `projects/[PROJECT_ID]`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Instance resource name formatted as: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`. Name should not be populated when creating an instance since it is provided in the `instance_id` field. |
| `feature_policy` | String | The policy to define whether or not RBE features can be used or how they can be used. |
| `state` | String | Output only. State of the instance. |
| `location` | String | The location is a GCP region. Currently only `us-central1` is supported. |
| `logging_enabled` | bool | Output only. Whether stack driver logging is enabled for the instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.remotebuildexecution_api.Instance {
    parent = "value"  # Resource name of the project containing the instance. Format: `projects/[PROJECT_ID]`.
}

# Access instance outputs
instance_id = instance.id
instance_name = instance.name
instance_feature_policy = instance.feature_policy
instance_state = instance.state
instance_location = instance.location
instance_logging_enabled = instance.logging_enabled
```

---


### Workerpool

Creates a new worker pool with a specified size and configuration. Returns a long running operation which contains a worker pool on completion. While the long running operation is in progress, any call to `GetWorkerPool` returns a worker pool in state `CREATING`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pool_id` | String |  | ID of the created worker pool. A valid pool ID must: be 6-50 characters long, contain only lowercase letters, digits, hyphens and underscores, start with a lowercase letter, and end with a lowercase letter or a digit. |
| `parent` | String |  | Resource name of the instance in which to create the new worker pool. Format: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`. |
| `worker_pool` | String |  | Specifies the worker pool to create. The name in the worker pool, if specified, is ignored. |
| `parent` | String | ✅ | Resource name of the instance in which to create the new worker pool. Format: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | WorkerPool resource name formatted as: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]/workerpools/[POOL_ID]`. name should not be populated when creating a worker pool since it is provided in the `poolId` field. |
| `state` | String | Output only. State of the worker pool. |
| `worker_config` | String | Specifies the properties, such as machine type and disk size, used for creating workers in a worker pool. |
| `autoscale` | String | The autoscale policy to apply on a pool. |
| `channel` | String | Channel specifies the release channel of the pool. |
| `worker_count` | String | The desired number of workers in the worker pool. Must be a value between 0 and 15000. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workerpool
workerpool = provider.remotebuildexecution_api.Workerpool {
    parent = "value"  # Resource name of the instance in which to create the new worker pool. Format: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`.
}

# Access workerpool outputs
workerpool_id = workerpool.id
workerpool_name = workerpool.name
workerpool_state = workerpool.state
workerpool_worker_config = workerpool.worker_config
workerpool_autoscale = workerpool.autoscale
workerpool_channel = workerpool.channel
workerpool_worker_count = workerpool.worker_count
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


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
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Remotebuildexecution

GetCapabilities returns the server capabilities configuration of the remote endpoint. Only the capabilities of the services supported by the endpoint will be returned: * Execution + CAS + Action Cache endpoints should return both CacheCapabilities and ExecutionCapabilities. * Execution only endpoints should return ExecutionCapabilities. * CAS + Action Cache only endpoints should return CacheCapabilities.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deprecated_api_version` | String | Earliest RE API version supported, including deprecated versions. |
| `cache_capabilities` | String | Capabilities of the remote cache system. |
| `high_api_version` | String | Latest RE API version supported. |
| `execution_capabilities` | String | Capabilities of the remote execution system. |
| `low_api_version` | String | Earliest non-deprecated RE API version supported. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access remotebuildexecution outputs
remotebuildexecution_id = remotebuildexecution.id
remotebuildexecution_deprecated_api_version = remotebuildexecution.deprecated_api_version
remotebuildexecution_cache_capabilities = remotebuildexecution.cache_capabilities
remotebuildexecution_high_api_version = remotebuildexecution.high_api_version
remotebuildexecution_execution_capabilities = remotebuildexecution.execution_capabilities
remotebuildexecution_low_api_version = remotebuildexecution.low_api_version
```

---


### Operation

Wait for an execution operation to complete. When the client initially makes the request, the server immediately responds with the current status of the execution. The server will leave the request stream open until the operation completes, and then respond with the completed operation. The server MAY choose to stream additional updates as execution progresses, such as to provide an update as to the state of the execution.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the Operation returned by Execute. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.remotebuildexecution_api.Operation {
    name = "value"  # The name of the Operation returned by Execute.
}

```

---


### Action

Execute an action remotely. In order to execute an action, the client must first upload all of the inputs, the Command to run, and the Action into the ContentAddressableStorage. It then calls `Execute` with an `action_digest` referring to them. The server will run the action and eventually return the result. The input `Action`'s fields MUST meet the various canonicalization requirements specified in the documentation for their types so that it has the same digest as other logically equivalent `Action`s. The server MAY enforce the requirements and return errors if a non-canonical input is received. It MAY also proceed without verifying some or all of the requirements, such as for performance reasons. If the server does not verify the requirement, then it will treat the `Action` as distinct from another logically equivalent action if they hash differently. Returns a stream of google.longrunning.Operation messages describing the resulting execution, with eventual `response` ExecuteResponse. The `metadata` on the operation is of type ExecuteOperationMetadata. If the client remains connected after the first response is returned after the server, then updates are streamed as if the client had called WaitExecution until the execution completes or the request reaches an error. The operation can also be queried using Operations API. The server NEED NOT implement other methods or functionality of the Operations API. Errors discovered during creation of the `Operation` will be reported as gRPC Status errors, while errors that occurred while running the action will be reported in the `status` field of the `ExecuteResponse`. The server MUST NOT set the `error` field of the `Operation` proto. The possible errors include: * `INVALID_ARGUMENT`: One or more arguments are invalid. * `FAILED_PRECONDITION`: One or more errors occurred in setting up the action requested, such as a missing input or command or no worker being available. The client may be able to fix the errors and retry. * `RESOURCE_EXHAUSTED`: There is insufficient quota of some resource to run the action. * `UNAVAILABLE`: Due to a transient condition, such as all workers being occupied (and the server does not support a queue), the action could not be started. The client should retry. * `INTERNAL`: An internal error occurred in the execution engine or the worker. * `DEADLINE_EXCEEDED`: The execution timed out. * `CANCELLED`: The operation was cancelled by the client. This status is only possible if the server implements the Operations API CancelOperation method, and it was called for the current execution. In the case of a missing input or command, the server SHOULD additionally send a PreconditionFailure error detail where, for each requested blob not present in the CAS, there is a `Violation` with a `type` of `MISSING` and a `subject` of `"blobs/{hash}/{size}"` indicating the digest of the missing blob. The server does not need to guarantee that a call to this method leads to at most one execution of the action. The server MAY execute the action multiple times, potentially in parallel. These redundant executions MAY continue to run, even if the operation is completed.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `results_cache_policy` | String |  | An optional policy for the results of this execution in the remote cache. The server will have a default policy if this is not provided. This may be applied to both the ActionResult and the associated blobs. |
| `action_digest` | String |  | The digest of the Action to execute. |
| `execution_policy` | String |  | An optional policy for execution of the action. The server will have a default policy if this is not provided. |
| `skip_cache_lookup` | bool |  | If true, the action will be executed even if its result is already present in the ActionCache. The execution is still allowed to be merged with other in-flight executions of the same action, however - semantically, the service MUST only guarantee that the results of an execution with this field set were not visible before the corresponding execution request was sent. Note that actions from execution requests setting this field set are still eligible to be entered into the action cache upon completion, and services SHOULD overwrite any existing entries that may exist. This allows skip_cache_lookup requests to be used as a mechanism for replacing action cache entries that reference outputs no longer available or that are poisoned in any way. If false, the result may be served from the action cache. |
| `instance_name` | String | ✅ | The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create action
action = provider.remotebuildexecution_api.Action {
    instance_name = "value"  # The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
}

```

---


### Action_result

Retrieve a cached execution result. Implementations SHOULD ensure that any blobs referenced from the ContentAddressableStorage are available at the time of returning the ActionResult and will be for some period of time afterwards. The lifetimes of the referenced blobs SHOULD be increased if necessary and applicable. Errors: * `NOT_FOUND`: The requested `ActionResult` is not in the cache.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stderr_raw` | String |  | The standard error buffer of the action. The server SHOULD NOT inline stderr unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits. |
| `stdout_digest` | String |  | The digest for a blob containing the standard output of the action, which can be retrieved from the ContentAddressableStorage. |
| `output_directories` | Vec<String> |  | The output directories of the action. For each output directory requested in the `output_directories` or `output_paths` field of the Action, if the corresponding directory existed after the action completed, a single entry will be present in the output list, which will contain the digest of a Tree message containing the directory tree, and the path equal exactly to the corresponding Action output_directories member. As an example, suppose the Action had an output directory `a/b/dir` and the execution produced the following contents in `a/b/dir`: a file named `bar` and a directory named `foo` with an executable file named `baz`. Then, output_directory will contain (hashes shortened for readability): ```json // OutputDirectory proto: { path: "a/b/dir" tree_digest: { hash: "4a73bc9d03...", size: 55 } } // Tree proto with hash "4a73bc9d03..." and size 55: { root: { files: [ { name: "bar", digest: { hash: "4a73bc9d03...", size: 65534 } } ], directories: [ { name: "foo", digest: { hash: "4cf2eda940...", size: 43 } } ] } children : { // (Directory proto with hash "4cf2eda940..." and size 43) files: [ { name: "baz", digest: { hash: "b2c941073e...", size: 1294, }, is_executable: true } ] } } ``` If an output of the same name as listed in `output_files` of the Command was found in `output_directories`, but was not a directory, the server will return a FAILED_PRECONDITION. |
| `exit_code` | i64 |  | The exit code of the command. |
| `output_directory_symlinks` | Vec<String> |  | The output directories of the action that are symbolic links to other directories. Those may be links to other output directories, or input directories, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. For each output directory requested in the `output_directories` field of the Action, if the directory existed after the action completed, a single entry will be present either in this field, or in the `output_directories` field, if the directory was not a symbolic link. If an output of the same name was found, but was a symbolic link to a file instead of a directory, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. DEPRECATED as of v2.1. Servers that wish to be compatible with v2.0 API should still populate this field in addition to `output_symlinks`. |
| `output_symlinks` | Vec<String> |  | New in v2.1: this field will only be populated if the command `output_paths` field was used, and not the pre v2.1 `output_files` or `output_directories` fields. The output paths of the action that are symbolic links to other paths. Those may be links to other outputs, or inputs, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. A single entry for each output requested in `output_paths` field of the Action, if the corresponding path existed after the action completed and was a symbolic link. If the action does not produce a requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. |
| `execution_metadata` | String |  | The details of the execution that originally produced this result. |
| `stdout_raw` | String |  | The standard output buffer of the action. The server SHOULD NOT inline stdout unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits. |
| `output_file_symlinks` | Vec<String> |  | The output files of the action that are symbolic links to other files. Those may be links to other output files, or input files, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. For each output file requested in the `output_files` or `output_paths` field of the Action, if the corresponding file existed after the action completed, a single entry will be present either in this field, or in the `output_files` field, if the file was not a symbolic link. If an output symbolic link of the same name as listed in `output_files` of the Command was found, but its target type was not a regular file, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. DEPRECATED as of v2.1. Servers that wish to be compatible with v2.0 API should still populate this field in addition to `output_symlinks`. |
| `output_files` | Vec<String> |  | The output files of the action. For each output file requested in the `output_files` or `output_paths` field of the Action, if the corresponding file existed after the action completed, a single entry will be present either in this field, or the `output_file_symlinks` field if the file was a symbolic link to another file (`output_symlinks` field after v2.1). If an output listed in `output_files` was found, but was a directory rather than a regular file, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. |
| `stderr_digest` | String |  | The digest for a blob containing the standard error of the action, which can be retrieved from the ContentAddressableStorage. |
| `instance_name` | String | ✅ | The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted. |
| `hash` | String | ✅ | The hash. In the case of SHA-256, it will always be a lowercase hex string exactly 64 characters long. |
| `size_bytes` | String | ✅ | The size of the blob, in bytes. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stderr_raw` | String | The standard error buffer of the action. The server SHOULD NOT inline stderr unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits. |
| `stdout_digest` | String | The digest for a blob containing the standard output of the action, which can be retrieved from the ContentAddressableStorage. |
| `output_directories` | Vec<String> | The output directories of the action. For each output directory requested in the `output_directories` or `output_paths` field of the Action, if the corresponding directory existed after the action completed, a single entry will be present in the output list, which will contain the digest of a Tree message containing the directory tree, and the path equal exactly to the corresponding Action output_directories member. As an example, suppose the Action had an output directory `a/b/dir` and the execution produced the following contents in `a/b/dir`: a file named `bar` and a directory named `foo` with an executable file named `baz`. Then, output_directory will contain (hashes shortened for readability): ```json // OutputDirectory proto: { path: "a/b/dir" tree_digest: { hash: "4a73bc9d03...", size: 55 } } // Tree proto with hash "4a73bc9d03..." and size 55: { root: { files: [ { name: "bar", digest: { hash: "4a73bc9d03...", size: 65534 } } ], directories: [ { name: "foo", digest: { hash: "4cf2eda940...", size: 43 } } ] } children : { // (Directory proto with hash "4cf2eda940..." and size 43) files: [ { name: "baz", digest: { hash: "b2c941073e...", size: 1294, }, is_executable: true } ] } } ``` If an output of the same name as listed in `output_files` of the Command was found in `output_directories`, but was not a directory, the server will return a FAILED_PRECONDITION. |
| `exit_code` | i64 | The exit code of the command. |
| `output_directory_symlinks` | Vec<String> | The output directories of the action that are symbolic links to other directories. Those may be links to other output directories, or input directories, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. For each output directory requested in the `output_directories` field of the Action, if the directory existed after the action completed, a single entry will be present either in this field, or in the `output_directories` field, if the directory was not a symbolic link. If an output of the same name was found, but was a symbolic link to a file instead of a directory, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. DEPRECATED as of v2.1. Servers that wish to be compatible with v2.0 API should still populate this field in addition to `output_symlinks`. |
| `output_symlinks` | Vec<String> | New in v2.1: this field will only be populated if the command `output_paths` field was used, and not the pre v2.1 `output_files` or `output_directories` fields. The output paths of the action that are symbolic links to other paths. Those may be links to other outputs, or inputs, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. A single entry for each output requested in `output_paths` field of the Action, if the corresponding path existed after the action completed and was a symbolic link. If the action does not produce a requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. |
| `execution_metadata` | String | The details of the execution that originally produced this result. |
| `stdout_raw` | String | The standard output buffer of the action. The server SHOULD NOT inline stdout unless requested by the client in the GetActionResultRequest message. The server MAY omit inlining, even if requested, and MUST do so if inlining would cause the response to exceed message size limits. |
| `output_file_symlinks` | Vec<String> | The output files of the action that are symbolic links to other files. Those may be links to other output files, or input files, or even absolute paths outside of the working directory, if the server supports SymlinkAbsolutePathStrategy.ALLOWED. For each output file requested in the `output_files` or `output_paths` field of the Action, if the corresponding file existed after the action completed, a single entry will be present either in this field, or in the `output_files` field, if the file was not a symbolic link. If an output symbolic link of the same name as listed in `output_files` of the Command was found, but its target type was not a regular file, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. DEPRECATED as of v2.1. Servers that wish to be compatible with v2.0 API should still populate this field in addition to `output_symlinks`. |
| `output_files` | Vec<String> | The output files of the action. For each output file requested in the `output_files` or `output_paths` field of the Action, if the corresponding file existed after the action completed, a single entry will be present either in this field, or the `output_file_symlinks` field if the file was a symbolic link to another file (`output_symlinks` field after v2.1). If an output listed in `output_files` was found, but was a directory rather than a regular file, the server will return a FAILED_PRECONDITION. If the action does not produce the requested output, then that output will be omitted from the list. The server is free to arrange the output list as desired; clients MUST NOT assume that the output list is sorted. |
| `stderr_digest` | String | The digest for a blob containing the standard error of the action, which can be retrieved from the ContentAddressableStorage. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access action_result outputs
action_result_id = action_result.id
action_result_stderr_raw = action_result.stderr_raw
action_result_stdout_digest = action_result.stdout_digest
action_result_output_directories = action_result.output_directories
action_result_exit_code = action_result.exit_code
action_result_output_directory_symlinks = action_result.output_directory_symlinks
action_result_output_symlinks = action_result.output_symlinks
action_result_execution_metadata = action_result.execution_metadata
action_result_stdout_raw = action_result.stdout_raw
action_result_output_file_symlinks = action_result.output_file_symlinks
action_result_output_files = action_result.output_files
action_result_stderr_digest = action_result.stderr_digest
```

---


### Blob

Upload many blobs at once. The server may enforce a limit of the combined total size of blobs to be uploaded using this API. This limit may be obtained using the Capabilities API. Requests exceeding the limit should either be split into smaller chunks or uploaded using the ByteStream API, as appropriate. This request is equivalent to calling a Bytestream `Write` request on each individual blob, in parallel. The requests may succeed or fail independently. Errors: * `INVALID_ARGUMENT`: The client attempted to upload more than the server supported limit. Individual requests may return the following errors, additionally: * `RESOURCE_EXHAUSTED`: There is insufficient disk quota to store the blob. * `INVALID_ARGUMENT`: The Digest does not match the provided data.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | The individual upload requests. |
| `instance_name` | String | ✅ | The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If present, signifies that there are more results which the client can retrieve by passing this as the page_token in a subsequent request. If empty, signifies that this is the last page of results. |
| `directories` | Vec<String> | The directories descended from the requested root. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create blob
blob = provider.remotebuildexecution_api.Blob {
    instance_name = "value"  # The instance of the execution system to operate against. A server may support multiple instances of the execution system (with their own workers, storage, caches, etc.). The server MAY require use of this field to select between them in an implementation-defined fashion, otherwise it can be omitted.
}

# Access blob outputs
blob_id = blob.id
blob_next_page_token = blob.next_page_token
blob_directories = blob.directories
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple media resources
media_0 = provider.remotebuildexecution_api.Media {
    resource_name = "value-0"
}
media_1 = provider.remotebuildexecution_api.Media {
    resource_name = "value-1"
}
media_2 = provider.remotebuildexecution_api.Media {
    resource_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    media = provider.remotebuildexecution_api.Media {
        resource_name = "production-value"
    }
```

---

## Related Documentation

- [GCP Remotebuildexecution_api Documentation](https://cloud.google.com/remotebuildexecution_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
