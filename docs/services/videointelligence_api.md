# Videointelligence_api Service



**Resources**: 6

---

## Overview

The videointelligence_api service provides access to 6 resource types:

- [Operation](#operation) [CRD]
- [Video](#video) [C]
- [Video](#video) [C]
- [Video](#video) [C]
- [Video](#video) [C]
- [Video](#video) [C]

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
operation = provider.videointelligence_api.Operation {
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


### Video

Performs asynchronous video annotation. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `AnnotateVideoProgress` (progress). `Operation.response` contains `AnnotateVideoResponse` (results).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `input_content` | String |  | The video data bytes. If unset, the input video(s) should be specified via the `input_uri`. If set, `input_uri` must be unset. |
| `video_context` | String |  | Additional video context and/or feature-specific parameters. |
| `input_uri` | String |  | Input video location. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. URIs must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). To identify multiple videos, a video URI may include wildcards in the `object-id`. Supported wildcards: '*' to match 0 or more characters; '?' to match 1 character. If unset, the input video should be embedded in the request as `input_content`. If set, `input_content` must be unset. |
| `features` | Vec<String> |  | Required. Requested video annotation features. |
| `location_id` | String |  | Optional. Cloud region where annotation should take place. Supported cloud regions are: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region is specified, the region will be determined based on video file location. |
| `output_uri` | String |  | Optional. Location where the output (in JSON format) should be stored. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. These must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create video
video = provider.videointelligence_api.Video {
}

```

---


### Video

Performs asynchronous video annotation. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `AnnotateVideoProgress` (progress). `Operation.response` contains `AnnotateVideoResponse` (results).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location_id` | String |  | Optional. Cloud region where annotation should take place. Supported cloud regions are: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region is specified, the region will be determined based on video file location. |
| `video_context` | String |  | Additional video context and/or feature-specific parameters. |
| `input_uri` | String |  | Input video location. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. URIs must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). To identify multiple videos, a video URI may include wildcards in the `object-id`. Supported wildcards: '*' to match 0 or more characters; '?' to match 1 character. If unset, the input video should be embedded in the request as `input_content`. If set, `input_content` must be unset. |
| `output_uri` | String |  | Optional. Location where the output (in JSON format) should be stored. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. These must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). |
| `features` | Vec<String> |  | Required. Requested video annotation features. |
| `input_content` | String |  | The video data bytes. If unset, the input video(s) should be specified via the `input_uri`. If set, `input_uri` must be unset. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create video
video = provider.videointelligence_api.Video {
}

```

---


### Video

Performs asynchronous video annotation. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `AnnotateVideoProgress` (progress). `Operation.response` contains `AnnotateVideoResponse` (results).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `features` | Vec<String> |  | Required. Requested video annotation features. |
| `input_content` | String |  | The video data bytes. If unset, the input video(s) should be specified via the `input_uri`. If set, `input_uri` must be unset. |
| `output_uri` | String |  | Optional. Location where the output (in JSON format) should be stored. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. These must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). |
| `input_uri` | String |  | Input video location. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. URIs must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). To identify multiple videos, a video URI may include wildcards in the `object-id`. Supported wildcards: '*' to match 0 or more characters; '?' to match 1 character. If unset, the input video should be embedded in the request as `input_content`. If set, `input_content` must be unset. |
| `video_context` | String |  | Additional video context and/or feature-specific parameters. |
| `location_id` | String |  | Optional. Cloud region where annotation should take place. Supported cloud regions are: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region is specified, the region will be determined based on video file location. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create video
video = provider.videointelligence_api.Video {
}

```

---


### Video

Performs asynchronous video annotation. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `AnnotateVideoProgress` (progress). `Operation.response` contains `AnnotateVideoResponse` (results).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `input_uri` | String |  | Input video location. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. URIs must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). To identify multiple videos, a video URI may include wildcards in the `object-id`. Supported wildcards: '*' to match 0 or more characters; '?' to match 1 character. If unset, the input video should be embedded in the request as `input_content`. If set, `input_content` must be unset. |
| `output_uri` | String |  | Optional. Location where the output (in JSON format) should be stored. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. These must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). |
| `features` | Vec<String> |  | Required. Requested video annotation features. |
| `input_content` | String |  | The video data bytes. If unset, the input video(s) should be specified via the `input_uri`. If set, `input_uri` must be unset. |
| `location_id` | String |  | Optional. Cloud region where annotation should take place. Supported cloud regions are: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region is specified, the region will be determined based on video file location. |
| `video_context` | String |  | Additional video context and/or feature-specific parameters. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create video
video = provider.videointelligence_api.Video {
}

```

---


### Video

Performs asynchronous video annotation. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `AnnotateVideoProgress` (progress). `Operation.response` contains `AnnotateVideoResponse` (results).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `features` | Vec<String> |  | Required. Requested video annotation features. |
| `input_uri` | String |  | Input video location. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. URIs must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). To identify multiple videos, a video URI may include wildcards in the `object-id`. Supported wildcards: '*' to match 0 or more characters; '?' to match 1 character. If unset, the input video should be embedded in the request as `input_content`. If set, `input_content` must be unset. |
| `input_content` | String |  | The video data bytes. If unset, the input video(s) should be specified via the `input_uri`. If set, `input_uri` must be unset. |
| `location_id` | String |  | Optional. Cloud region where annotation should take place. Supported cloud regions are: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region is specified, the region will be determined based on video file location. |
| `video_context` | String |  | Additional video context and/or feature-specific parameters. |
| `output_uri` | String |  | Optional. Location where the output (in JSON format) should be stored. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. These must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create video
video = provider.videointelligence_api.Video {
}

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
operation_0 = provider.videointelligence_api.Operation {
    name = "value-0"
}
operation_1 = provider.videointelligence_api.Operation {
    name = "value-1"
}
operation_2 = provider.videointelligence_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.videointelligence_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Videointelligence_api Documentation](https://cloud.google.com/videointelligence_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
