# Firebaseml_api Service



**Resources**: 4

---

## Overview

The firebaseml_api service provides access to 4 resource types:

- [Operation](#operation) [CRD]
- [Model](#model) [CRUD]
- [Operation](#operation) [R]
- [Model](#model) [C]

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
| `operations` | Vec<String> | A list of operations that matches the specified filter in the request. |
| `unreachable` | Vec<String> | Unordered list. Unreachable resources. Populated when the request sets `ListOperationsRequest.return_partial_success` and reads across collections e.g. when attempting to list all resources across all supported locations. |
| `next_page_token` | String | The standard List next-page token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.firebaseml_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_operations = operation.operations
operation_unreachable = operation.unreachable
operation_next_page_token = operation.next_page_token
```

---


### Model

Creates a model in Firebase ML. The longrunning operation will eventually return a Model

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tflite_model` | String |  | A TFLite Model |
| `create_time` | String |  | Output only. Timestamp when this model was created in Firebase ML. |
| `tags` | Vec<String> |  | User defined tags which can be used to group/filter models during listing |
| `etag` | String |  | Output only. See RFC7232 https://tools.ietf.org/html/rfc7232#section-2.3 |
| `active_operations` | Vec<String> |  | Output only. Lists operation ids associated with this model whose status is NOT done. |
| `update_time` | String |  | Output only. Timestamp when this model was updated in Firebase ML. |
| `display_name` | String |  | Required. The name of the model to create. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores(_) and ASCII digits 0-9. It must start with a letter. |
| `model_hash` | String |  | Output only. The model_hash will change if a new file is available for download. |
| `state` | String |  | State common to all model types. Includes publishing and validation information. |
| `name` | String |  | The resource name of the Model. Model names have the form `projects/{project_id}/models/{model_id}` The name is ignored when creating a model. |
| `parent` | String | ✅ | Required. The parent project resource where the model is to be created. The parent must have the form `projects/{project_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tflite_model` | String | A TFLite Model |
| `create_time` | String | Output only. Timestamp when this model was created in Firebase ML. |
| `tags` | Vec<String> | User defined tags which can be used to group/filter models during listing |
| `etag` | String | Output only. See RFC7232 https://tools.ietf.org/html/rfc7232#section-2.3 |
| `active_operations` | Vec<String> | Output only. Lists operation ids associated with this model whose status is NOT done. |
| `update_time` | String | Output only. Timestamp when this model was updated in Firebase ML. |
| `display_name` | String | Required. The name of the model to create. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores(_) and ASCII digits 0-9. It must start with a letter. |
| `model_hash` | String | Output only. The model_hash will change if a new file is available for download. |
| `state` | String | State common to all model types. Includes publishing and validation information. |
| `name` | String | The resource name of the Model. Model names have the form `projects/{project_id}/models/{model_id}` The name is ignored when creating a model. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model
model = provider.firebaseml_api.Model {
    parent = "value"  # Required. The parent project resource where the model is to be created. The parent must have the form `projects/{project_id}`
}

# Access model outputs
model_id = model.id
model_tflite_model = model.tflite_model
model_create_time = model.create_time
model_tags = model.tags
model_etag = model.etag
model_active_operations = model.active_operations
model_update_time = model.update_time
model_display_name = model.display_name
model_model_hash = model.model_hash
model_state = model.state
model_name = model.name
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
```

---


### Model

Generate content with multimodal inputs.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. It is used for billing and reporting only. Label keys and values can be no longer than 63 characters (Unicode codepoints) and can only contain lowercase letters, numeric characters, underscores, and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. |
| `cached_content` | String |  | Optional. The name of the cached content used as context to serve the prediction. Note: only used in explicit caching, where users can have control over caching (e.g. what content to cache) and enjoy guaranteed cost savings. Format: `projects/{project}/locations/{location}/cachedContents/{cachedContent}` |
| `contents` | Vec<String> |  | Required. The content of the current conversation with the model. For single-turn queries, this is a single instance. For multi-turn queries, this is a repeated field that contains conversation history + latest request. |
| `generation_config` | String |  | Optional. Generation config. |
| `system_instruction` | String |  | Optional. The user provided system instructions for the model. Note: only text should be used in parts and content in each part will be in a separate paragraph. |
| `model_armor_config` | String |  | Optional. Settings for prompt and response sanitization using the Model Armor service. If supplied, safety_settings must not be supplied. |
| `tool_config` | String |  | Optional. Tool config. This config is shared for all tools provided in the request. |
| `safety_settings` | Vec<String> |  | Optional. Per request settings for blocking unsafe content. Enforced on GenerateContentResponse.candidates. |
| `tools` | Vec<String> |  | Optional. A list of `Tools` the model may use to generate the next response. A `Tool` is a piece of code that enables the system to interact with external systems to perform an action, or set of actions, outside of knowledge and scope of the model. |
| `model` | String | ✅ | Required. The fully qualified name of the publisher model or tuned model endpoint to use. Publisher model format: `projects/{project}/locations/{location}/publishers/*/models/*` Tuned model endpoint format: `projects/{project}/locations/{location}/endpoints/{endpoint}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model
model = provider.firebaseml_api.Model {
    model = "value"  # Required. The fully qualified name of the publisher model or tuned model endpoint to use. Publisher model format: `projects/{project}/locations/{location}/publishers/*/models/*` Tuned model endpoint format: `projects/{project}/locations/{location}/endpoints/{endpoint}`
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
operation_0 = provider.firebaseml_api.Operation {
    name = "value-0"
}
operation_1 = provider.firebaseml_api.Operation {
    name = "value-1"
}
operation_2 = provider.firebaseml_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.firebaseml_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebaseml_api Documentation](https://cloud.google.com/firebaseml_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
