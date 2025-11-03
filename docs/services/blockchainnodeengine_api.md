# Blockchainnodeengine_api Service



**Resources**: 3

---

## Overview

The blockchainnodeengine_api service provides access to 3 resource types:

- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Blockchain_node](#blockchain_node) [CRUD]

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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.blockchainnodeengine_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_metadata = location.metadata
location_labels = location.labels
location_name = location.name
location_display_name = location.display_name
location_location_id = location.location_id
```

---


### Blockchain_node

Creates a new blockchain node in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connection_info` | String |  | Output only. The connection information used to interact with a blockchain node. |
| `create_time` | String |  | Output only. The timestamp at which the blockchain node was first created. |
| `name` | String |  | Output only. The fully qualified name of the blockchain node. e.g. `projects/my-project/locations/us-central1/blockchainNodes/my-node`. |
| `private_service_connect_enabled` | bool |  | Optional. When true, the node is only accessible via Private Service Connect; no public endpoints are exposed. Otherwise, the node is only accessible via public endpoints. Warning: These nodes are deprecated, please use public endpoints instead. |
| `blockchain_type` | String |  | Immutable. The blockchain type of the node. |
| `ethereum_details` | String |  | Ethereum-specific blockchain node details. |
| `update_time` | String |  | Output only. The timestamp at which the blockchain node was last updated. |
| `labels` | HashMap<String, String> |  | User-provided key-value pairs. |
| `state` | String |  | Output only. A status representing the state of the node. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connection_info` | String | Output only. The connection information used to interact with a blockchain node. |
| `create_time` | String | Output only. The timestamp at which the blockchain node was first created. |
| `name` | String | Output only. The fully qualified name of the blockchain node. e.g. `projects/my-project/locations/us-central1/blockchainNodes/my-node`. |
| `private_service_connect_enabled` | bool | Optional. When true, the node is only accessible via Private Service Connect; no public endpoints are exposed. Otherwise, the node is only accessible via public endpoints. Warning: These nodes are deprecated, please use public endpoints instead. |
| `blockchain_type` | String | Immutable. The blockchain type of the node. |
| `ethereum_details` | String | Ethereum-specific blockchain node details. |
| `update_time` | String | Output only. The timestamp at which the blockchain node was last updated. |
| `labels` | HashMap<String, String> | User-provided key-value pairs. |
| `state` | String | Output only. A status representing the state of the node. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create blockchain_node
blockchain_node = provider.blockchainnodeengine_api.Blockchain_node {
    parent = "value"  # Required. Value for parent.
}

# Access blockchain_node outputs
blockchain_node_id = blockchain_node.id
blockchain_node_connection_info = blockchain_node.connection_info
blockchain_node_create_time = blockchain_node.create_time
blockchain_node_name = blockchain_node.name
blockchain_node_private_service_connect_enabled = blockchain_node.private_service_connect_enabled
blockchain_node_blockchain_type = blockchain_node.blockchain_type
blockchain_node_ethereum_details = blockchain_node.ethereum_details
blockchain_node_update_time = blockchain_node.update_time
blockchain_node_labels = blockchain_node.labels
blockchain_node_state = blockchain_node.state
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
operation_0 = provider.blockchainnodeengine_api.Operation {
    name = "value-0"
}
operation_1 = provider.blockchainnodeengine_api.Operation {
    name = "value-1"
}
operation_2 = provider.blockchainnodeengine_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.blockchainnodeengine_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Blockchainnodeengine_api Documentation](https://cloud.google.com/blockchainnodeengine_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
