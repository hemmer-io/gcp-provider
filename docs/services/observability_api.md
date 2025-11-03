# Observability_api Service



**Resources**: 4

---

## Overview

The observability_api service provides access to 4 resource types:

- [Trace_scope](#trace_scope) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [Scope](#scope) [RU]

---

## Resources


### Trace_scope

Create a new TraceScope.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. Describes this trace scope. The maximum length of the description is 8000 characters. |
| `update_time` | String |  | Output only. The last update timestamp of the trace scope. |
| `resource_names` | Vec<String> |  | Required. Names of the projects that are included in this trace scope. * `projects/[PROJECT_ID]` A trace scope can include a maximum of 20 projects. |
| `create_time` | String |  | Output only. The creation timestamp of the trace scope. |
| `name` | String |  | Identifier. The resource name of the trace scope. For example: projects/my-project/locations/global/traceScopes/my-trace-scope |
| `parent` | String | ✅ | Required. The full resource name of the location where the trace scope should be created projects/[PROJECT_ID]/locations/[LOCATION_ID] For example: projects/my-project/locations/global |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. Describes this trace scope. The maximum length of the description is 8000 characters. |
| `update_time` | String | Output only. The last update timestamp of the trace scope. |
| `resource_names` | Vec<String> | Required. Names of the projects that are included in this trace scope. * `projects/[PROJECT_ID]` A trace scope can include a maximum of 20 projects. |
| `create_time` | String | Output only. The creation timestamp of the trace scope. |
| `name` | String | Identifier. The resource name of the trace scope. For example: projects/my-project/locations/global/traceScopes/my-trace-scope |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trace_scope
trace_scope = provider.observability_api.Trace_scope {
    parent = "value"  # Required. The full resource name of the location where the trace scope should be created projects/[PROJECT_ID]/locations/[LOCATION_ID] For example: projects/my-project/locations/global
}

# Access trace_scope outputs
trace_scope_id = trace_scope.id
trace_scope_description = trace_scope.description
trace_scope_update_time = trace_scope.update_time
trace_scope_resource_names = trace_scope.resource_names
trace_scope_create_time = trace_scope.create_time
trace_scope_name = trace_scope.name
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_labels = location.labels
location_location_id = location.location_id
location_display_name = location.display_name
location_metadata = location.metadata
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.observability_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
```

---


### Scope

Gets details of a single Scope.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Name of the resource. The format is: projects/{project}/locations/{location}/scopes/{scope} The `{location}` field must be set to `global`. The `{scope}` field must be set to `_Default`. |
| `log_scope` | String |  | Required. The full resource name of the `LogScope`. For example: //logging.googleapis.com/projects/myproject/locations/global/logScopes/my-log-scope |
| `trace_scope` | String |  | Required. The resource name of the `TraceScope`. For example: projects/myproject/locations/global/traceScopes/my-trace-scope |
| `update_time` | String |  | Output only. Update timestamp. Note: The Update timestamp for the default scope is initially unset. |
| `name` | String | ✅ | Identifier. Name of the resource. The format is: projects/{project}/locations/{location}/scopes/{scope} The `{location}` field must be set to `global`. The `{scope}` field must be set to `_Default`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Name of the resource. The format is: projects/{project}/locations/{location}/scopes/{scope} The `{location}` field must be set to `global`. The `{scope}` field must be set to `_Default`. |
| `log_scope` | String | Required. The full resource name of the `LogScope`. For example: //logging.googleapis.com/projects/myproject/locations/global/logScopes/my-log-scope |
| `trace_scope` | String | Required. The resource name of the `TraceScope`. For example: projects/myproject/locations/global/traceScopes/my-trace-scope |
| `update_time` | String | Output only. Update timestamp. Note: The Update timestamp for the default scope is initially unset. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access scope outputs
scope_id = scope.id
scope_name = scope.name
scope_log_scope = scope.log_scope
scope_trace_scope = scope.trace_scope
scope_update_time = scope.update_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple trace_scope resources
trace_scope_0 = provider.observability_api.Trace_scope {
    parent = "value-0"
}
trace_scope_1 = provider.observability_api.Trace_scope {
    parent = "value-1"
}
trace_scope_2 = provider.observability_api.Trace_scope {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    trace_scope = provider.observability_api.Trace_scope {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Observability_api Documentation](https://cloud.google.com/observability_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
