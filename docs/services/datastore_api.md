# Datastore_api Service



**Resources**: 5

---

## Overview

The datastore_api service provides access to 5 resource types:

- [Project](#project) [C]
- [Project](#project) [C]
- [Operation](#operation) [CRD]
- [Indexe](#indexe) [CRD]
- [Project](#project) [C]

---

## Resources


### Project

Queries for entities.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query` | String |  | The query to run. |
| `partition_id` | String |  | Entities are partitioned into subsets, identified by a partition ID. Queries are scoped to a single partition. This partition ID is normalized with the standard default context partition ID. |
| `explain_options` | String |  | Optional. Explain options for the query. If set, additional query statistics will be returned. If not, only query results will be returned. |
| `property_mask` | String |  | The properties to return. This field must not be set for a projection query. See LookupRequest.property_mask. |
| `gql_query` | String |  | The GQL query to run. This query must be a non-aggregation query. |
| `read_options` | String |  | The options for this query. |
| `project_id` | String | ✅ | Required. The ID of the project against which to make the request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.datastore_api.Project {
    project_id = "value"  # Required. The ID of the project against which to make the request.
}

```

---


### Project

Imports entities into Google Cloud Datastore. Existing entities with the same key are overwritten. The import occurs in the background and its progress can be monitored and managed via the Operation resource that is created. If an ImportEntities operation is cancelled, it is possible that a subset of the data has already been imported to Cloud Datastore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `input_url` | String |  | Required. The full resource URL of the external storage location. Currently, only Google Cloud Storage is supported. So input_url should be of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]/OVERALL_EXPORT_METADATA_FILE`, where `BUCKET_NAME` is the name of the Cloud Storage bucket, `NAMESPACE_PATH` is an optional Cloud Storage namespace path (this is not a Cloud Datastore namespace), and `OVERALL_EXPORT_METADATA_FILE` is the metadata file written by the ExportEntities operation. For more information about Cloud Storage namespace paths, see [Object name considerations](https://cloud.google.com/storage/docs/naming#object-considerations). For more information, see google.datastore.admin.v1.ExportEntitiesResponse.output_url. |
| `labels` | HashMap<String, String> |  | Client-assigned labels. |
| `entity_filter` | String |  | Optionally specify which kinds/namespaces are to be imported. If provided, the list must be a subset of the EntityFilter used in creating the export, otherwise a FAILED_PRECONDITION error will be returned. If no filter is specified then all entities from the export are imported. |
| `project_id` | String | ✅ | Required. Project ID against which to make the request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.datastore_api.Project {
    project_id = "value"  # Required. Project ID against which to make the request.
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.datastore_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
```

---


### Indexe

Creates the specified index. A newly created index's initial state is `CREATING`. On completion of the returned google.longrunning.Operation, the state will be `READY`. If the index already exists, the call will return an `ALREADY_EXISTS` status. During index creation, the process could result in an error, in which case the index will move to the `ERROR` state. The process can be recovered by fixing the data that caused the error, removing the index with delete, then re-creating the index with create. Indexes with a single property cannot be created.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ancestor` | String |  | Required. The index's ancestor mode. Must not be ANCESTOR_MODE_UNSPECIFIED. |
| `index_id` | String |  | Output only. The resource ID of the index. |
| `project_id` | String |  | Output only. Project ID. |
| `state` | String |  | Output only. The state of the index. |
| `kind` | String |  | Required. The entity kind to which this index applies. |
| `properties` | Vec<String> |  | Required. An ordered sequence of property names and their index attributes. Requires: * A maximum of 100 properties. |
| `project_id` | String | ✅ | Project ID against which to make the request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ancestor` | String | Required. The index's ancestor mode. Must not be ANCESTOR_MODE_UNSPECIFIED. |
| `index_id` | String | Output only. The resource ID of the index. |
| `project_id` | String | Output only. Project ID. |
| `state` | String | Output only. The state of the index. |
| `kind` | String | Required. The entity kind to which this index applies. |
| `properties` | Vec<String> | Required. An ordered sequence of property names and their index attributes. Requires: * A maximum of 100 properties. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create indexe
indexe = provider.datastore_api.Indexe {
    project_id = "value"  # Project ID against which to make the request.
}

# Access indexe outputs
indexe_id = indexe.id
indexe_ancestor = indexe.ancestor
indexe_index_id = indexe.index_id
indexe_project_id = indexe.project_id
indexe_state = indexe.state
indexe_kind = indexe.kind
indexe_properties = indexe.properties
```

---


### Project

Exports a copy of all or a subset of entities from Google Cloud Datastore to another storage system, such as Google Cloud Storage. Recent updates to entities may not be reflected in the export. The export occurs in the background and its progress can be monitored and managed via the Operation resource that is created. The output of an export may only be used once the associated operation is done. If an export operation is cancelled before completion it may leave partial data behind in Google Cloud Storage.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Client-assigned labels. |
| `output_url_prefix` | String |  | Location for the export metadata and data files. The full resource URL of the external storage location. Currently, only Google Cloud Storage is supported. So output_url_prefix should be of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the name of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud Storage namespace path (this is not a Cloud Datastore namespace). For more information about Cloud Storage namespace paths, see [Object name considerations](https://cloud.google.com/storage/docs/naming#object-considerations). The resulting files will be nested deeper than the specified URL prefix. The final output URL will be provided in the google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url field. That value should be used for subsequent ImportEntities operations. By nesting the data files deeper, the same Cloud Storage bucket can be used in multiple ExportEntities operations without conflict. |
| `entity_filter` | String |  | Description of what data from the project is included in the export. |
| `project_id` | String | ✅ | Project ID against which to make the request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.datastore_api.Project {
    project_id = "value"  # Project ID against which to make the request.
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

# Create multiple project resources
project_0 = provider.datastore_api.Project {
    project_id = "value-0"
}
project_1 = provider.datastore_api.Project {
    project_id = "value-1"
}
project_2 = provider.datastore_api.Project {
    project_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    project = provider.datastore_api.Project {
        project_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Datastore_api Documentation](https://cloud.google.com/datastore_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
