# Texttospeech_api Service



**Resources**: 8

---

## Overview

The texttospeech_api service provides access to 8 resource types:

- [Voice](#voice) [R]
- [Location](#location) [C]
- [Text](#text) [C]
- [Operation](#operation) [CRD]
- [Text](#text) [C]
- [Operation](#operation) [R]
- [Location](#location) [C]
- [Voice](#voice) [R]

---

## Resources


### Voice

Returns a list of Voice supported for synthesis.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voices` | Vec<String> | The list of voices. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access voice outputs
voice_id = voice.id
voice_voices = voice.voices
```

---


### Location

Synthesizes long form text asynchronously.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `audio_config` | String |  | Required. The configuration of the synthesized audio. |
| `input` | String |  | Required. The Synthesizer requires either plain text or SSML as input. |
| `voice` | String |  | Required. The desired voice of the synthesized audio. |
| `output_gcs_uri` | String |  | Required. Specifies a Cloud Storage URI for the synthesis results. Must be specified in the format: `gs://bucket_name/object_name`, and the bucket must already exist. |
| `parent` | String | ✅ | The resource states of the request in the form of `projects/*/locations/*`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.texttospeech_api.Location {
    parent = "value"  # The resource states of the request in the form of `projects/*/locations/*`.
}

```

---


### Text

Synthesizes speech synchronously: receive results after all text input has been processed.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `voice` | String |  | Required. The desired voice of the synthesized audio. |
| `input` | String |  | Required. The Synthesizer requires either plain text or SSML as input. |
| `advanced_voice_options` | String |  | Advanced voice options. |
| `audio_config` | String |  | Required. The configuration of the synthesized audio. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create text
text = provider.texttospeech_api.Text {
}

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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.texttospeech_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Text

Synthesizes speech synchronously: receive results after all text input has been processed.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_time_pointing` | Vec<String> |  | Whether and what timepoints are returned in the response. |
| `advanced_voice_options` | String |  | Advanced voice options. |
| `input` | String |  | Required. The Synthesizer requires either plain text or SSML as input. |
| `voice` | String |  | Required. The desired voice of the synthesized audio. |
| `audio_config` | String |  | Required. The configuration of the synthesized audio. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create text
text = provider.texttospeech_api.Text {
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


### Location

Synthesizes long form text asynchronously.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `audio_config` | String |  | Required. The configuration of the synthesized audio. |
| `input` | String |  | Required. The Synthesizer requires either plain text or SSML as input. |
| `voice` | String |  | Required. The desired voice of the synthesized audio. |
| `output_gcs_uri` | String |  | Required. Specifies a Cloud Storage URI for the synthesis results. Must be specified in the format: `gs://bucket_name/object_name`, and the bucket must already exist. |
| `parent` | String | ✅ | The resource states of the request in the form of `projects/*/locations/*`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.texttospeech_api.Location {
    parent = "value"  # The resource states of the request in the form of `projects/*/locations/*`.
}

```

---


### Voice

Returns a list of Voice supported for synthesis.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voices` | Vec<String> | The list of voices. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access voice outputs
voice_id = voice.id
voice_voices = voice.voices
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple voice resources
voice_0 = provider.texttospeech_api.Voice {
}
voice_1 = provider.texttospeech_api.Voice {
}
voice_2 = provider.texttospeech_api.Voice {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    voice = provider.texttospeech_api.Voice {
    }
```

---

## Related Documentation

- [GCP Texttospeech_api Documentation](https://cloud.google.com/texttospeech_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
