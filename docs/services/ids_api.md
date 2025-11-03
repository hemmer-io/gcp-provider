# Ids_api Service



**Resources**: 3

---

## Overview

The ids_api service provides access to 3 resource types:

- [Endpoint](#endpoint) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]

---

## Resources


### Endpoint

Creates a new Endpoint in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `endpoint_forwarding_rule` | String |  | Output only. The fully qualified URL of the endpoint's ILB Forwarding Rule. |
| `description` | String |  | User-provided description of the endpoint |
| `severity` | String |  | Required. Lowest threat severity that this endpoint will alert on. |
| `labels` | HashMap<String, String> |  | The labels of the endpoint. |
| `name` | String |  | Output only. The name of the endpoint. |
| `satisfies_pzi` | bool |  | Output only. [Output Only] Reserved for future use. |
| `create_time` | String |  | Output only. The create time timestamp. |
| `threat_exceptions` | Vec<String> |  | List of threat IDs to be excepted from generating alerts. |
| `satisfies_pzs` | bool |  | Output only. [Output Only] Reserved for future use. |
| `endpoint_ip` | String |  | Output only. The IP address of the IDS Endpoint's ILB. |
| `traffic_logs` | bool |  | Whether the endpoint should report traffic logs in addition to threat logs. |
| `update_time` | String |  | Output only. The update time timestamp. |
| `state` | String |  | Output only. Current state of the endpoint. |
| `network` | String |  | Required. The fully qualified URL of the network to which the IDS Endpoint is attached. |
| `parent` | String | ✅ | Required. The endpoint's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_forwarding_rule` | String | Output only. The fully qualified URL of the endpoint's ILB Forwarding Rule. |
| `description` | String | User-provided description of the endpoint |
| `severity` | String | Required. Lowest threat severity that this endpoint will alert on. |
| `labels` | HashMap<String, String> | The labels of the endpoint. |
| `name` | String | Output only. The name of the endpoint. |
| `satisfies_pzi` | bool | Output only. [Output Only] Reserved for future use. |
| `create_time` | String | Output only. The create time timestamp. |
| `threat_exceptions` | Vec<String> | List of threat IDs to be excepted from generating alerts. |
| `satisfies_pzs` | bool | Output only. [Output Only] Reserved for future use. |
| `endpoint_ip` | String | Output only. The IP address of the IDS Endpoint's ILB. |
| `traffic_logs` | bool | Whether the endpoint should report traffic logs in addition to threat logs. |
| `update_time` | String | Output only. The update time timestamp. |
| `state` | String | Output only. Current state of the endpoint. |
| `network` | String | Required. The fully qualified URL of the network to which the IDS Endpoint is attached. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create endpoint
endpoint = provider.ids_api.Endpoint {
    parent = "value"  # Required. The endpoint's parent.
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_endpoint_forwarding_rule = endpoint.endpoint_forwarding_rule
endpoint_description = endpoint.description
endpoint_severity = endpoint.severity
endpoint_labels = endpoint.labels
endpoint_name = endpoint.name
endpoint_satisfies_pzi = endpoint.satisfies_pzi
endpoint_create_time = endpoint.create_time
endpoint_threat_exceptions = endpoint.threat_exceptions
endpoint_satisfies_pzs = endpoint.satisfies_pzs
endpoint_endpoint_ip = endpoint.endpoint_ip
endpoint_traffic_logs = endpoint.traffic_logs
endpoint_update_time = endpoint.update_time
endpoint_state = endpoint.state
endpoint_network = endpoint.network
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.ids_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_metadata = location.metadata
location_location_id = location.location_id
location_display_name = location.display_name
location_name = location.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple endpoint resources
endpoint_0 = provider.ids_api.Endpoint {
    parent = "value-0"
}
endpoint_1 = provider.ids_api.Endpoint {
    parent = "value-1"
}
endpoint_2 = provider.ids_api.Endpoint {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    endpoint = provider.ids_api.Endpoint {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Ids_api Documentation](https://cloud.google.com/ids_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
