# Datalineage_api Service



**Resources**: 5

---

## Overview

The datalineage_api service provides access to 5 resource types:

- [Run](#run) [CRUD]
- [Lineage_event](#lineage_event) [CRD]
- [Operation](#operation) [CRD]
- [Processe](#processe) [CRUD]
- [Location](#location) [C]

---

## Resources


### Run

Creates a new run.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Optional. The timestamp of the end of the run. |
| `display_name` | String |  | Optional. A human-readable name you can set to display in a user interface. Must be not longer than 1024 characters and only contain UTF-8 letters or numbers, spaces or characters like `_-:&.` |
| `name` | String |  | Immutable. The resource name of the run. Format: `projects/{project}/locations/{location}/processes/{process}/runs/{run}`. Can be specified or auto-assigned. {run} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `start_time` | String |  | Required. The timestamp of the start of the run. |
| `state` | String |  | Required. The state of the run. |
| `attributes` | HashMap<String, String> |  | Optional. The attributes of the run. Should only be used for the purpose of non-semantic management (classifying, describing or labeling the run). Up to 100 attributes are allowed. |
| `parent` | String | ✅ | Required. The name of the process that should own the run. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Optional. The timestamp of the end of the run. |
| `display_name` | String | Optional. A human-readable name you can set to display in a user interface. Must be not longer than 1024 characters and only contain UTF-8 letters or numbers, spaces or characters like `_-:&.` |
| `name` | String | Immutable. The resource name of the run. Format: `projects/{project}/locations/{location}/processes/{process}/runs/{run}`. Can be specified or auto-assigned. {run} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `start_time` | String | Required. The timestamp of the start of the run. |
| `state` | String | Required. The state of the run. |
| `attributes` | HashMap<String, String> | Optional. The attributes of the run. Should only be used for the purpose of non-semantic management (classifying, describing or labeling the run). Up to 100 attributes are allowed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create run
run = provider.datalineage_api.Run {
    parent = "value"  # Required. The name of the process that should own the run.
}

# Access run outputs
run_id = run.id
run_end_time = run.end_time
run_display_name = run.display_name
run_name = run.name
run_start_time = run.start_time
run_state = run.state
run_attributes = run.attributes
```

---


### Lineage_event

Creates a new lineage event.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Optional. The end of the transformation which resulted in this lineage event. For streaming scenarios, it should be the end of the period from which the lineage is being reported. |
| `name` | String |  | Immutable. The resource name of the lineage event. Format: `projects/{project}/locations/{location}/processes/{process}/runs/{run}/lineageEvents/{lineage_event}`. Can be specified or auto-assigned. {lineage_event} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `start_time` | String |  | Required. The beginning of the transformation which resulted in this lineage event. For streaming scenarios, it should be the beginning of the period from which the lineage is being reported. |
| `links` | Vec<String> |  | Optional. List of source-target pairs. Can't contain more than 100 tuples. |
| `parent` | String | ✅ | Required. The name of the run that should own the lineage event. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Optional. The end of the transformation which resulted in this lineage event. For streaming scenarios, it should be the end of the period from which the lineage is being reported. |
| `name` | String | Immutable. The resource name of the lineage event. Format: `projects/{project}/locations/{location}/processes/{process}/runs/{run}/lineageEvents/{lineage_event}`. Can be specified or auto-assigned. {lineage_event} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `start_time` | String | Required. The beginning of the transformation which resulted in this lineage event. For streaming scenarios, it should be the beginning of the period from which the lineage is being reported. |
| `links` | Vec<String> | Optional. List of source-target pairs. Can't contain more than 100 tuples. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lineage_event
lineage_event = provider.datalineage_api.Lineage_event {
    parent = "value"  # Required. The name of the run that should own the lineage event.
}

# Access lineage_event outputs
lineage_event_id = lineage_event.id
lineage_event_end_time = lineage_event.end_time
lineage_event_name = lineage_event.name
lineage_event_start_time = lineage_event.start_time
lineage_event_links = lineage_event.links
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.datalineage_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_name = operation.name
operation_done = operation.done
operation_response = operation.response
operation_metadata = operation.metadata
```

---


### Processe

Creates a new process.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `origin` | String |  | Optional. The origin of this process and its runs and lineage events. |
| `attributes` | HashMap<String, String> |  | Optional. The attributes of the process. Should only be used for the purpose of non-semantic management (classifying, describing or labeling the process). Up to 100 attributes are allowed. |
| `display_name` | String |  | Optional. A human-readable name you can set to display in a user interface. Must be not longer than 200 characters and only contain UTF-8 letters or numbers, spaces or characters like `_-:&.` |
| `name` | String |  | Immutable. The resource name of the lineage process. Format: `projects/{project}/locations/{location}/processes/{process}`. Can be specified or auto-assigned. {process} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `parent` | String | ✅ | Required. The name of the project and its location that should own the process. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `origin` | String | Optional. The origin of this process and its runs and lineage events. |
| `attributes` | HashMap<String, String> | Optional. The attributes of the process. Should only be used for the purpose of non-semantic management (classifying, describing or labeling the process). Up to 100 attributes are allowed. |
| `display_name` | String | Optional. A human-readable name you can set to display in a user interface. Must be not longer than 200 characters and only contain UTF-8 letters or numbers, spaces or characters like `_-:&.` |
| `name` | String | Immutable. The resource name of the lineage process. Format: `projects/{project}/locations/{location}/processes/{process}`. Can be specified or auto-assigned. {process} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create processe
processe = provider.datalineage_api.Processe {
    parent = "value"  # Required. The name of the project and its location that should own the process.
}

# Access processe outputs
processe_id = processe.id
processe_origin = processe.origin
processe_attributes = processe.attributes
processe_display_name = processe.display_name
processe_name = processe.name
```

---


### Location

Retrieve information about LineageProcesses associated with specific links. LineageProcesses are transformation pipelines that result in data flowing from **source** to **target** assets. Links between assets represent this operation. If you have specific link names, you can use this method to verify which LineageProcesses contribute to creating those links. See the SearchLinks method for more information on how to retrieve link name. You can retrieve the LineageProcess information in every project where you have the `datalineage.events.get` permission. The project provided in the URL is used for Billing and Quota.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `links` | Vec<String> |  | Required. An array of links to check for their associated LineageProcesses. The maximum number of items in this array is 100. If the request contains more than 100 links, it returns the `INVALID_ARGUMENT` error. Format: `projects/{project}/locations/{location}/links/{link}`. |
| `page_size` | i64 |  | The maximum number of processes to return in a single page of the response. A page may contain fewer results than this value. |
| `page_token` | String |  | The page token received from a previous `BatchSearchLinkProcesses` call. Use it to get the next page. When requesting subsequent pages of a response, remember that all parameters must match the values you provided in the original request. |
| `parent` | String | ✅ | Required. The project and location where you want to search. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.datalineage_api.Location {
    parent = "value"  # Required. The project and location where you want to search.
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

# Create multiple run resources
run_0 = provider.datalineage_api.Run {
    parent = "value-0"
}
run_1 = provider.datalineage_api.Run {
    parent = "value-1"
}
run_2 = provider.datalineage_api.Run {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    run = provider.datalineage_api.Run {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Datalineage_api Documentation](https://cloud.google.com/datalineage_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
