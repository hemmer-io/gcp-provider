# Datalineage_api Service



**Resources**: 5

---

## Overview

The datalineage_api service provides access to 5 resource types:

- [Lineage_event](#lineage_event) [CRD]
- [Run](#run) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [C]
- [Processe](#processe) [CRUD]

---

## Resources


### Lineage_event

Creates a new lineage event.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_time` | String |  | Required. The beginning of the transformation which resulted in this lineage event. For streaming scenarios, it should be the beginning of the period from which the lineage is being reported. |
| `links` | Vec<String> |  | Optional. List of source-target pairs. Can't contain more than 100 tuples. |
| `name` | String |  | Immutable. The resource name of the lineage event. Format: `projects/{project}/locations/{location}/processes/{process}/runs/{run}/lineageEvents/{lineage_event}`. Can be specified or auto-assigned. {lineage_event} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `end_time` | String |  | Optional. The end of the transformation which resulted in this lineage event. For streaming scenarios, it should be the end of the period from which the lineage is being reported. |
| `parent` | String | ✅ | Required. The name of the run that should own the lineage event. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | Required. The beginning of the transformation which resulted in this lineage event. For streaming scenarios, it should be the beginning of the period from which the lineage is being reported. |
| `links` | Vec<String> | Optional. List of source-target pairs. Can't contain more than 100 tuples. |
| `name` | String | Immutable. The resource name of the lineage event. Format: `projects/{project}/locations/{location}/processes/{process}/runs/{run}/lineageEvents/{lineage_event}`. Can be specified or auto-assigned. {lineage_event} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `end_time` | String | Optional. The end of the transformation which resulted in this lineage event. For streaming scenarios, it should be the end of the period from which the lineage is being reported. |


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
lineage_event_start_time = lineage_event.start_time
lineage_event_links = lineage_event.links
lineage_event_name = lineage_event.name
lineage_event_end_time = lineage_event.end_time
```

---


### Run

Creates a new run.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The resource name of the run. Format: `projects/{project}/locations/{location}/processes/{process}/runs/{run}`. Can be specified or auto-assigned. {run} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `attributes` | HashMap<String, String> |  | Optional. The attributes of the run. Should only be used for the purpose of non-semantic management (classifying, describing or labeling the run). Up to 100 attributes are allowed. |
| `end_time` | String |  | Optional. The timestamp of the end of the run. |
| `start_time` | String |  | Required. The timestamp of the start of the run. |
| `display_name` | String |  | Optional. A human-readable name you can set to display in a user interface. Must be not longer than 1024 characters and only contain UTF-8 letters or numbers, spaces or characters like `_-:&.` |
| `state` | String |  | Required. The state of the run. |
| `parent` | String | ✅ | Required. The name of the process that should own the run. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The resource name of the run. Format: `projects/{project}/locations/{location}/processes/{process}/runs/{run}`. Can be specified or auto-assigned. {run} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `attributes` | HashMap<String, String> | Optional. The attributes of the run. Should only be used for the purpose of non-semantic management (classifying, describing or labeling the run). Up to 100 attributes are allowed. |
| `end_time` | String | Optional. The timestamp of the end of the run. |
| `start_time` | String | Required. The timestamp of the start of the run. |
| `display_name` | String | Optional. A human-readable name you can set to display in a user interface. Must be not longer than 1024 characters and only contain UTF-8 letters or numbers, spaces or characters like `_-:&.` |
| `state` | String | Required. The state of the run. |


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
run_name = run.name
run_attributes = run.attributes
run_end_time = run.end_time
run_start_time = run.start_time
run_display_name = run.display_name
run_state = run.state
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
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation_response = operation.response
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Location

Retrieve a list of links connected to a specific asset. Links represent the data flow between **source** (upstream) and **target** (downstream) assets in transformation pipelines. Links are stored in the same project as the Lineage Events that create them. You can retrieve links in every project where you have the `datalineage.events.get` permission. The project provided in the URL is used for Billing and Quota.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source` | String |  | Optional. Send asset information in the **source** field to retrieve all links that lead from the specified asset to downstream assets. |
| `target` | String |  | Optional. Send asset information in the **target** field to retrieve all links that lead from upstream assets to the specified asset. |
| `page_token` | String |  | Optional. The page token received from a previous `SearchLinksRequest` call. Use it to get the next page. When requesting subsequent pages of a response, remember that all parameters must match the values you provided in the original request. |
| `page_size` | i64 |  | Optional. The maximum number of links to return in a single page of the response. A page may contain fewer links than this value. If unspecified, at most 10 links are returned. Maximum value is 100; values greater than 100 are reduced to 100. |
| `parent` | String | ✅ | Required. The project and location you want search in. |



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
    parent = "value"  # Required. The project and location you want search in.
}

```

---


### Processe

Creates a new process.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attributes` | HashMap<String, String> |  | Optional. The attributes of the process. Should only be used for the purpose of non-semantic management (classifying, describing or labeling the process). Up to 100 attributes are allowed. |
| `name` | String |  | Immutable. The resource name of the lineage process. Format: `projects/{project}/locations/{location}/processes/{process}`. Can be specified or auto-assigned. {process} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `origin` | String |  | Optional. The origin of this process and its runs and lineage events. |
| `display_name` | String |  | Optional. A human-readable name you can set to display in a user interface. Must be not longer than 200 characters and only contain UTF-8 letters or numbers, spaces or characters like `_-:&.` |
| `parent` | String | ✅ | Required. The name of the project and its location that should own the process. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | HashMap<String, String> | Optional. The attributes of the process. Should only be used for the purpose of non-semantic management (classifying, describing or labeling the process). Up to 100 attributes are allowed. |
| `name` | String | Immutable. The resource name of the lineage process. Format: `projects/{project}/locations/{location}/processes/{process}`. Can be specified or auto-assigned. {process} must be not longer than 200 characters and only contain characters in a set: `a-zA-Z0-9_-:.` |
| `origin` | String | Optional. The origin of this process and its runs and lineage events. |
| `display_name` | String | Optional. A human-readable name you can set to display in a user interface. Must be not longer than 200 characters and only contain UTF-8 letters or numbers, spaces or characters like `_-:&.` |


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
processe_attributes = processe.attributes
processe_name = processe.name
processe_origin = processe.origin
processe_display_name = processe.display_name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple lineage_event resources
lineage_event_0 = provider.datalineage_api.Lineage_event {
    parent = "value-0"
}
lineage_event_1 = provider.datalineage_api.Lineage_event {
    parent = "value-1"
}
lineage_event_2 = provider.datalineage_api.Lineage_event {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    lineage_event = provider.datalineage_api.Lineage_event {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Datalineage_api Documentation](https://cloud.google.com/datalineage_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
