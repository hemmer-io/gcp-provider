# Ondemandscanning_api Service



**Resources**: 6

---

## Overview

The ondemandscanning_api service provides access to 6 resource types:

- [Vulnerabilitie](#vulnerabilitie) [R]
- [Operation](#operation) [CRD]
- [Scan](#scan) [C]
- [Vulnerabilitie](#vulnerabilitie) [R]
- [Scan](#scan) [C]
- [Operation](#operation) [CRD]

---

## Resources


### Vulnerabilitie

Lists vulnerabilities resulting from a successfully completed scan.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `occurrences` | Vec<String> | The list of Vulnerability Occurrences resulting from a scan. |
| `next_page_token` | String | A page token that can be used in a subsequent call to ListVulnerabilities to continue retrieving results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access vulnerabilitie outputs
vulnerabilitie_id = vulnerabilitie.id
vulnerabilitie_occurrences = vulnerabilitie.occurrences
vulnerabilitie_next_page_token = vulnerabilitie.next_page_token
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.ondemandscanning_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
```

---


### Scan

Initiates an analysis of the provided packages.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `packages` | Vec<String> |  | The packages to analyze. |
| `resource_uri` | String |  | Required. The resource URI of the container image being scanned. |
| `include_osv_data` | bool |  | [DEPRECATED] Whether to include OSV data in the scan. For backwards compatibility reasons, this field can be neither removed nor renamed. |
| `parent` | String | ✅ | Required. The parent of the resource for which analysis is requested. Format: projects/[project_name]/locations/[location] |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create scan
scan = provider.ondemandscanning_api.Scan {
    parent = "value"  # Required. The parent of the resource for which analysis is requested. Format: projects/[project_name]/locations/[location]
}

```

---


### Vulnerabilitie

Lists vulnerabilities resulting from a successfully completed scan.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A page token that can be used in a subsequent call to ListVulnerabilities to continue retrieving results. |
| `occurrences` | Vec<String> | The list of Vulnerability Occurrences resulting from a scan. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access vulnerabilitie outputs
vulnerabilitie_id = vulnerabilitie.id
vulnerabilitie_next_page_token = vulnerabilitie.next_page_token
vulnerabilitie_occurrences = vulnerabilitie.occurrences
```

---


### Scan

Initiates an analysis of the provided packages.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `packages` | Vec<String> |  | The packages to analyze. |
| `resource_uri` | String |  | Required. The resource URI of the container image being scanned. |
| `parent` | String | ✅ | Required. The parent of the resource for which analysis is requested. Format: projects/[project_name]/locations/[location] |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create scan
scan = provider.ondemandscanning_api.Scan {
    parent = "value"  # Required. The parent of the resource for which analysis is requested. Format: projects/[project_name]/locations/[location]
}

```

---


### Operation

Waits until the specified long-running operation is done or reaches at most a specified timeout, returning the latest state. If the operation is already done, the latest state is immediately returned. If the timeout specified is greater than the default HTTP/RPC timeout, the HTTP/RPC timeout is used. If the server does not support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Note that this method is on a best-effort basis. It may return the latest state before the specified timeout (including immediately), meaning even an immediate response is no guarantee that the operation is done.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to wait on. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.ondemandscanning_api.Operation {
    name = "value"  # The name of the operation resource to wait on.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple vulnerabilitie resources
vulnerabilitie_0 = provider.ondemandscanning_api.Vulnerabilitie {
}
vulnerabilitie_1 = provider.ondemandscanning_api.Vulnerabilitie {
}
vulnerabilitie_2 = provider.ondemandscanning_api.Vulnerabilitie {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    vulnerabilitie = provider.ondemandscanning_api.Vulnerabilitie {
    }
```

---

## Related Documentation

- [GCP Ondemandscanning_api Documentation](https://cloud.google.com/ondemandscanning_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
