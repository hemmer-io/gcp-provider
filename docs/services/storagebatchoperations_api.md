# Storagebatchoperations_api Service



**Resources**: 3

---

## Overview

The storagebatchoperations_api service provides access to 3 resource types:

- [Operation](#operation) [CRD]
- [Job](#job) [CRD]
- [Location](#location) [R]

---

## Resources


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

# Create operation
operation = provider.storagebatchoperations_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
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


### Job

Creates a batch job.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bucket_list` | String |  | Specifies a list of buckets and their objects to be transformed. |
| `create_time` | String |  | Output only. The time that the job was created. |
| `put_object_hold` | String |  | Changes object hold status. |
| `error_summaries` | Vec<String> |  | Output only. Summarizes errors encountered with sample error log entries. |
| `complete_time` | String |  | Output only. The time that the job was completed. |
| `delete_object` | String |  | Delete objects. |
| `name` | String |  | Identifier. The resource name of the Job. job_id is unique within the project, that is either set by the customer or defined by the service. Format: projects/{project}/locations/global/jobs/{job_id} . For example: "projects/123456/locations/global/jobs/job01". |
| `put_metadata` | String |  | Updates object metadata. Allows updating fixed-key and custom metadata and fixed-key metadata i.e. Cache-Control, Content-Disposition, Content-Encoding, Content-Language, Content-Type, Custom-Time. |
| `state` | String |  | Output only. State of the job. |
| `rewrite_object` | String |  | Rewrite the object and updates metadata like KMS key. |
| `logging_config` | String |  | Optional. Logging configuration. |
| `schedule_time` | String |  | Output only. The time that the job was scheduled. |
| `counters` | String |  | Output only. Information about the progress of the job. |
| `description` | String |  | Optional. A description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bucket_list` | String | Specifies a list of buckets and their objects to be transformed. |
| `create_time` | String | Output only. The time that the job was created. |
| `put_object_hold` | String | Changes object hold status. |
| `error_summaries` | Vec<String> | Output only. Summarizes errors encountered with sample error log entries. |
| `complete_time` | String | Output only. The time that the job was completed. |
| `delete_object` | String | Delete objects. |
| `name` | String | Identifier. The resource name of the Job. job_id is unique within the project, that is either set by the customer or defined by the service. Format: projects/{project}/locations/global/jobs/{job_id} . For example: "projects/123456/locations/global/jobs/job01". |
| `put_metadata` | String | Updates object metadata. Allows updating fixed-key and custom metadata and fixed-key metadata i.e. Cache-Control, Content-Disposition, Content-Encoding, Content-Language, Content-Type, Custom-Time. |
| `state` | String | Output only. State of the job. |
| `rewrite_object` | String | Rewrite the object and updates metadata like KMS key. |
| `logging_config` | String | Optional. Logging configuration. |
| `schedule_time` | String | Output only. The time that the job was scheduled. |
| `counters` | String | Output only. Information about the progress of the job. |
| `description` | String | Optional. A description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.storagebatchoperations_api.Job {
    parent = "value"  # Required. Value for parent.
}

# Access job outputs
job_id = job.id
job_bucket_list = job.bucket_list
job_create_time = job.create_time
job_put_object_hold = job.put_object_hold
job_error_summaries = job.error_summaries
job_complete_time = job.complete_time
job_delete_object = job.delete_object
job_name = job.name
job_put_metadata = job.put_metadata
job_state = job.state
job_rewrite_object = job.rewrite_object
job_logging_config = job.logging_config
job_schedule_time = job.schedule_time
job_counters = job.counters
job_description = job.description
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_name = location.name
location_labels = location.labels
location_metadata = location.metadata
location_display_name = location.display_name
location_location_id = location.location_id
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
operation_0 = provider.storagebatchoperations_api.Operation {
    name = "value-0"
}
operation_1 = provider.storagebatchoperations_api.Operation {
    name = "value-1"
}
operation_2 = provider.storagebatchoperations_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.storagebatchoperations_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Storagebatchoperations_api Documentation](https://cloud.google.com/storagebatchoperations_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
