# Batch_api Service



**Resources**: 5

---

## Overview

The batch_api service provides access to 5 resource types:

- [Task](#task) [R]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [State](#state) [C]
- [Job](#job) [CRD]

---

## Resources


### Task

Return a single Task.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Task name. The name is generated from the parent TaskGroup name and 'id' field. For example: "projects/123456/locations/us-west1/jobs/job01/taskGroups/group01/tasks/task01". |
| `status` | String | Task Status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access task outputs
task_id = task.id
task_name = task.name
task_status = task.status
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

# Create operation
operation = provider.batch_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
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


### State

Report agent's state, e.g. agent status and tasks information

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `agent_info` | String |  | Agent info. |
| `agent_timing_info` | String |  | Agent timing info. |
| `metadata` | String |  | Agent metadata. |
| `parent` | String | ✅ | Required. Format: projects/{project}/locations/{location} {project} should be a project number. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create state
state = provider.batch_api.State {
    parent = "value"  # Required. Format: projects/{project}/locations/{location} {project} should be a project number.
}

```

---


### Job

Create a Job.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. When the Job was created. |
| `uid` | String |  | Output only. A system generated unique ID for the Job. |
| `task_groups` | Vec<String> |  | Required. TaskGroups in the Job. Only one TaskGroup is supported now. |
| `priority` | String |  | Priority of the Job. The valid value range is [0, 100). Default value is 0. Higher value indicates higher priority. A job with higher priority value is more likely to run earlier if all other requirements are satisfied. |
| `notifications` | Vec<String> |  | Notification configurations. |
| `name` | String |  | Output only. Job name. For example: "projects/123456/locations/us-central1/jobs/job01". |
| `logs_policy` | String |  | Log preservation policy for the Job. |
| `allocation_policy` | String |  | Compute resource allocation for all TaskGroups in the Job. |
| `status` | String |  | Output only. Job status. It is read only for users. |
| `update_time` | String |  | Output only. The last time the Job was updated. |
| `labels` | HashMap<String, String> |  | Custom labels to apply to the job and any Cloud Logging [LogEntry](https://cloud.google.com/logging/docs/reference/v2/rest/v2/LogEntry) that it generates. Use labels to group and describe the resources they are applied to. Batch automatically applies predefined labels and supports multiple `labels` fields for each job, which each let you apply custom labels to various resources. Label names that start with "goog-" or "google-" are reserved for predefined labels. For more information about labels with Batch, see [Organize resources using labels](https://cloud.google.com/batch/docs/organize-resources-using-labels). |
| `parent` | String | ✅ | Required. The parent resource name where the Job will be created. Pattern: "projects/{project}/locations/{location}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. When the Job was created. |
| `uid` | String | Output only. A system generated unique ID for the Job. |
| `task_groups` | Vec<String> | Required. TaskGroups in the Job. Only one TaskGroup is supported now. |
| `priority` | String | Priority of the Job. The valid value range is [0, 100). Default value is 0. Higher value indicates higher priority. A job with higher priority value is more likely to run earlier if all other requirements are satisfied. |
| `notifications` | Vec<String> | Notification configurations. |
| `name` | String | Output only. Job name. For example: "projects/123456/locations/us-central1/jobs/job01". |
| `logs_policy` | String | Log preservation policy for the Job. |
| `allocation_policy` | String | Compute resource allocation for all TaskGroups in the Job. |
| `status` | String | Output only. Job status. It is read only for users. |
| `update_time` | String | Output only. The last time the Job was updated. |
| `labels` | HashMap<String, String> | Custom labels to apply to the job and any Cloud Logging [LogEntry](https://cloud.google.com/logging/docs/reference/v2/rest/v2/LogEntry) that it generates. Use labels to group and describe the resources they are applied to. Batch automatically applies predefined labels and supports multiple `labels` fields for each job, which each let you apply custom labels to various resources. Label names that start with "goog-" or "google-" are reserved for predefined labels. For more information about labels with Batch, see [Organize resources using labels](https://cloud.google.com/batch/docs/organize-resources-using-labels). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.batch_api.Job {
    parent = "value"  # Required. The parent resource name where the Job will be created. Pattern: "projects/{project}/locations/{location}"
}

# Access job outputs
job_id = job.id
job_create_time = job.create_time
job_uid = job.uid
job_task_groups = job.task_groups
job_priority = job.priority
job_notifications = job.notifications
job_name = job.name
job_logs_policy = job.logs_policy
job_allocation_policy = job.allocation_policy
job_status = job.status
job_update_time = job.update_time
job_labels = job.labels
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple task resources
task_0 = provider.batch_api.Task {
}
task_1 = provider.batch_api.Task {
}
task_2 = provider.batch_api.Task {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    task = provider.batch_api.Task {
    }
```

---

## Related Documentation

- [GCP Batch_api Documentation](https://cloud.google.com/batch_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
