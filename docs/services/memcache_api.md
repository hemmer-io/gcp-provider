# Memcache_api Service



**Resources**: 6

---

## Overview

The memcache_api service provides access to 6 resource types:

- [Location](#location) [R]
- [Instance](#instance) [CRUD]
- [Operation](#operation) [CRD]
- [Instance](#instance) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]

---

## Resources


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


### Instance

Creates a new Instance in a given location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The state of this Memcached instance. |
| `node_config` | String |  | Required. Configuration for Memcached nodes. |
| `instance_messages` | Vec<String> |  | List of messages that describe the current state of the Memcached instance. |
| `name` | String |  | Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Memcached instances are managed and addressed at the regional level so `location_id` here refers to a Google Cloud region; however, users may choose which zones Memcached nodes should be provisioned in within an instance. Refer to zones field for more details. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `discovery_endpoint` | String |  | Output only. Endpoint for the Discovery API. |
| `memcache_full_version` | String |  | Output only. The full version of memcached server running on this instance. System automatically determines the full memcached version for an instance based on the input MemcacheVersion. The full version format will be "memcached-1.5.16". |
| `node_count` | i64 |  | Required. Number of nodes in the Memcached instance. |
| `reserved_ip_range_id` | Vec<String> |  | Optional. Contains the id of allocated IP address ranges associated with the private service access connection for example, "test-default" associated with IP range 10.0.0.0/29. |
| `maintenance_schedule` | String |  | Output only. Published maintenance schedule. |
| `satisfies_pzi` | bool |  | Optional. Output only. Reserved for future use. |
| `satisfies_pzs` | bool |  | Optional. Output only. Reserved for future use. |
| `memcache_version` | String |  | The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest supported major version is `MEMCACHE_1_5`. The minor version will be automatically determined by our system based on the latest supported minor version. |
| `memcache_nodes` | Vec<String> |  | Output only. List of Memcached nodes. Refer to Node message for more details. |
| `zones` | Vec<String> |  | Zones in which Memcached nodes should be provisioned. Memcached nodes will be equally distributed across these zones. If not provided, the service will by default create nodes in all zones in the region for the instance. |
| `parameters` | String |  | User defined parameters to apply to the memcached process on each node. |
| `display_name` | String |  | User provided name for the instance, which is only used for display purposes. Cannot be more than 80 characters. |
| `maintenance_policy` | String |  | The maintenance policy for the instance. If not provided, the maintenance event will be performed based on Memorystore internal rollout schedule. |
| `authorized_network` | String |  | The full name of the Google Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. If left unspecified, the `default` network will be used. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `update_time` | String |  | Output only. The time the instance was updated. |
| `parent` | String | ✅ | Required. The resource name of the instance location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a GCP region |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The state of this Memcached instance. |
| `node_config` | String | Required. Configuration for Memcached nodes. |
| `instance_messages` | Vec<String> | List of messages that describe the current state of the Memcached instance. |
| `name` | String | Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Memcached instances are managed and addressed at the regional level so `location_id` here refers to a Google Cloud region; however, users may choose which zones Memcached nodes should be provisioned in within an instance. Refer to zones field for more details. |
| `labels` | HashMap<String, String> | Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `discovery_endpoint` | String | Output only. Endpoint for the Discovery API. |
| `memcache_full_version` | String | Output only. The full version of memcached server running on this instance. System automatically determines the full memcached version for an instance based on the input MemcacheVersion. The full version format will be "memcached-1.5.16". |
| `node_count` | i64 | Required. Number of nodes in the Memcached instance. |
| `reserved_ip_range_id` | Vec<String> | Optional. Contains the id of allocated IP address ranges associated with the private service access connection for example, "test-default" associated with IP range 10.0.0.0/29. |
| `maintenance_schedule` | String | Output only. Published maintenance schedule. |
| `satisfies_pzi` | bool | Optional. Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Optional. Output only. Reserved for future use. |
| `memcache_version` | String | The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest supported major version is `MEMCACHE_1_5`. The minor version will be automatically determined by our system based on the latest supported minor version. |
| `memcache_nodes` | Vec<String> | Output only. List of Memcached nodes. Refer to Node message for more details. |
| `zones` | Vec<String> | Zones in which Memcached nodes should be provisioned. Memcached nodes will be equally distributed across these zones. If not provided, the service will by default create nodes in all zones in the region for the instance. |
| `parameters` | String | User defined parameters to apply to the memcached process on each node. |
| `display_name` | String | User provided name for the instance, which is only used for display purposes. Cannot be more than 80 characters. |
| `maintenance_policy` | String | The maintenance policy for the instance. If not provided, the maintenance event will be performed based on Memorystore internal rollout schedule. |
| `authorized_network` | String | The full name of the Google Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. If left unspecified, the `default` network will be used. |
| `create_time` | String | Output only. The time the instance was created. |
| `update_time` | String | Output only. The time the instance was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.memcache_api.Instance {
    parent = "value"  # Required. The resource name of the instance location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a GCP region
}

# Access instance outputs
instance_id = instance.id
instance_state = instance.state
instance_node_config = instance.node_config
instance_instance_messages = instance.instance_messages
instance_name = instance.name
instance_labels = instance.labels
instance_discovery_endpoint = instance.discovery_endpoint
instance_memcache_full_version = instance.memcache_full_version
instance_node_count = instance.node_count
instance_reserved_ip_range_id = instance.reserved_ip_range_id
instance_maintenance_schedule = instance.maintenance_schedule
instance_satisfies_pzi = instance.satisfies_pzi
instance_satisfies_pzs = instance.satisfies_pzs
instance_memcache_version = instance.memcache_version
instance_memcache_nodes = instance.memcache_nodes
instance_zones = instance.zones
instance_parameters = instance.parameters
instance_display_name = instance.display_name
instance_maintenance_policy = instance.maintenance_policy
instance_authorized_network = instance.authorized_network
instance_create_time = instance.create_time
instance_update_time = instance.update_time
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.memcache_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_error = operation.error
```

---


### Instance

Creates a new Instance in a given location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `node_config` | String |  | Required. Configuration for Memcached nodes. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `name` | String |  | Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Memcached instances are managed and addressed at the regional level so `location_id` here refers to a Google Cloud region; however, users may choose which zones Memcached nodes should be provisioned in within an instance. Refer to zones field for more details. |
| `maintenance_policy` | String |  | The maintenance policy for the instance. If not provided, the maintenance event will be performed based on Memorystore internal rollout schedule. |
| `node_count` | i64 |  | Required. Number of nodes in the Memcached instance. |
| `update_available` | bool |  | Output only. Returns true if there is an update waiting to be applied |
| `zones` | Vec<String> |  | Zones in which Memcached nodes should be provisioned. Memcached nodes will be equally distributed across these zones. If not provided, the service will by default create nodes in all zones in the region for the instance. |
| `discovery_endpoint` | String |  | Output only. Endpoint for the Discovery API. |
| `display_name` | String |  | User provided name for the instance, which is only used for display purposes. Cannot be more than 80 characters. |
| `instance_messages` | Vec<String> |  | List of messages that describe the current state of the Memcached instance. |
| `maintenance_schedule` | String |  | Output only. Published maintenance schedule. |
| `parameters` | String |  | User defined parameters to apply to the memcached process on each node. |
| `authorized_network` | String |  | The full name of the Google Compute Engine [network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. If left unspecified, the `default` network will be used. |
| `state` | String |  | Output only. The state of this Memcached instance. |
| `memcache_nodes` | Vec<String> |  | Output only. List of Memcached nodes. Refer to Node message for more details. |
| `reserved_ip_range_id` | Vec<String> |  | Optional. Contains the id of allocated IP address ranges associated with the private service access connection for example, "test-default" associated with IP range 10.0.0.0/29. |
| `satisfies_pzs` | bool |  | Optional. Output only. Reserved for future use. |
| `update_time` | String |  | Output only. The time the instance was updated. |
| `memcache_full_version` | String |  | Output only. The full version of memcached server running on this instance. System automatically determines the full memcached version for an instance based on the input MemcacheVersion. The full version format will be "memcached-1.5.16". |
| `memcache_version` | String |  | The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest supported major version is `MEMCACHE_1_5`. The minor version will be automatically determined by our system based on the latest supported minor version. |
| `satisfies_pzi` | bool |  | Optional. Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The resource name of the instance location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a GCP region |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `node_config` | String | Required. Configuration for Memcached nodes. |
| `create_time` | String | Output only. The time the instance was created. |
| `name` | String | Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Memcached instances are managed and addressed at the regional level so `location_id` here refers to a Google Cloud region; however, users may choose which zones Memcached nodes should be provisioned in within an instance. Refer to zones field for more details. |
| `maintenance_policy` | String | The maintenance policy for the instance. If not provided, the maintenance event will be performed based on Memorystore internal rollout schedule. |
| `node_count` | i64 | Required. Number of nodes in the Memcached instance. |
| `update_available` | bool | Output only. Returns true if there is an update waiting to be applied |
| `zones` | Vec<String> | Zones in which Memcached nodes should be provisioned. Memcached nodes will be equally distributed across these zones. If not provided, the service will by default create nodes in all zones in the region for the instance. |
| `discovery_endpoint` | String | Output only. Endpoint for the Discovery API. |
| `display_name` | String | User provided name for the instance, which is only used for display purposes. Cannot be more than 80 characters. |
| `instance_messages` | Vec<String> | List of messages that describe the current state of the Memcached instance. |
| `maintenance_schedule` | String | Output only. Published maintenance schedule. |
| `parameters` | String | User defined parameters to apply to the memcached process on each node. |
| `authorized_network` | String | The full name of the Google Compute Engine [network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. If left unspecified, the `default` network will be used. |
| `state` | String | Output only. The state of this Memcached instance. |
| `memcache_nodes` | Vec<String> | Output only. List of Memcached nodes. Refer to Node message for more details. |
| `reserved_ip_range_id` | Vec<String> | Optional. Contains the id of allocated IP address ranges associated with the private service access connection for example, "test-default" associated with IP range 10.0.0.0/29. |
| `satisfies_pzs` | bool | Optional. Output only. Reserved for future use. |
| `update_time` | String | Output only. The time the instance was updated. |
| `memcache_full_version` | String | Output only. The full version of memcached server running on this instance. System automatically determines the full memcached version for an instance based on the input MemcacheVersion. The full version format will be "memcached-1.5.16". |
| `memcache_version` | String | The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest supported major version is `MEMCACHE_1_5`. The minor version will be automatically determined by our system based on the latest supported minor version. |
| `satisfies_pzi` | bool | Optional. Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.memcache_api.Instance {
    parent = "value"  # Required. The resource name of the instance location using the form: `projects/{project_id}/locations/{location_id}` where `location_id` refers to a GCP region
}

# Access instance outputs
instance_id = instance.id
instance_labels = instance.labels
instance_node_config = instance.node_config
instance_create_time = instance.create_time
instance_name = instance.name
instance_maintenance_policy = instance.maintenance_policy
instance_node_count = instance.node_count
instance_update_available = instance.update_available
instance_zones = instance.zones
instance_discovery_endpoint = instance.discovery_endpoint
instance_display_name = instance.display_name
instance_instance_messages = instance.instance_messages
instance_maintenance_schedule = instance.maintenance_schedule
instance_parameters = instance.parameters
instance_authorized_network = instance.authorized_network
instance_state = instance.state
instance_memcache_nodes = instance.memcache_nodes
instance_reserved_ip_range_id = instance.reserved_ip_range_id
instance_satisfies_pzs = instance.satisfies_pzs
instance_update_time = instance.update_time
instance_memcache_full_version = instance.memcache_full_version
instance_memcache_version = instance.memcache_version
instance_satisfies_pzi = instance.satisfies_pzi
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.memcache_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_name = operation.name
operation_error = operation.error
operation_done = operation.done
operation_response = operation.response
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_display_name = location.display_name
location_labels = location.labels
location_metadata = location.metadata
location_location_id = location.location_id
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

# Create multiple location resources
location_0 = provider.memcache_api.Location {
}
location_1 = provider.memcache_api.Location {
}
location_2 = provider.memcache_api.Location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.memcache_api.Location {
    }
```

---

## Related Documentation

- [GCP Memcache_api Documentation](https://cloud.google.com/memcache_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
