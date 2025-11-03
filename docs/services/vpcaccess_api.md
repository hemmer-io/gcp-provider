# Vpcaccess_api Service



**Resources**: 6

---

## Overview

The vpcaccess_api service provides access to 6 resource types:

- [Location](#location) [R]
- [Operation](#operation) [R]
- [Connector](#connector) [CRUD]
- [Location](#location) [R]
- [Connector](#connector) [CRUD]
- [Operation](#operation) [R]

---

## Resources


### Location

Lists information about the supported locations for this service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `locations` | Vec<String> | A list of locations that matches the specified filter in the request. |
| `next_page_token` | String | The standard List next-page token. |


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
location_locations = location.locations
location_next_page_token = location.next_page_token
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Connector

Creates a Serverless VPC Access connector, returns an operation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `max_instances` | i64 |  | Maximum value of instances in autoscaling group underlying the connector. |
| `machine_type` | String |  | Machine type of VM Instance underlying connector. Default is e2-micro |
| `name` | String |  | The resource name in the format `projects/*/locations/*/connectors/*`. |
| `subnet` | String |  | Optional. The subnet in which to house the VPC Access Connector. |
| `state` | String |  | Output only. State of the VPC access connector. |
| `min_throughput` | i64 |  | Minimum throughput of the connector in Mbps. Refers to the expected throughput when using an `e2-micro` machine type. Value must be a multiple of 100 from 200 through 900. Must be lower than the value specified by --max-throughput. If both min-throughput and min-instances are provided, min-instances takes precedence over min-throughput. The use of `min-throughput` is discouraged in favor of `min-instances`. |
| `connected_projects` | Vec<String> |  | Output only. List of projects using the connector. |
| `network` | String |  | Optional. Name of a VPC network. |
| `max_throughput` | i64 |  | Maximum throughput of the connector in Mbps. Refers to the expected throughput when using an `e2-micro` machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by --min-throughput. If both max-throughput and max-instances are provided, max-instances takes precedence over max-throughput. The use of `max-throughput` is discouraged in favor of `max-instances`. |
| `min_instances` | i64 |  | Minimum value of instances in autoscaling group underlying the connector. |
| `ip_cidr_range` | String |  | Optional. The range of internal addresses that follows RFC 4632 notation. Example: `10.132.0.0/28`. |
| `parent` | String | ✅ | Required. The project ID and location in which the configuration should be created, specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `max_instances` | i64 | Maximum value of instances in autoscaling group underlying the connector. |
| `machine_type` | String | Machine type of VM Instance underlying connector. Default is e2-micro |
| `name` | String | The resource name in the format `projects/*/locations/*/connectors/*`. |
| `subnet` | String | Optional. The subnet in which to house the VPC Access Connector. |
| `state` | String | Output only. State of the VPC access connector. |
| `min_throughput` | i64 | Minimum throughput of the connector in Mbps. Refers to the expected throughput when using an `e2-micro` machine type. Value must be a multiple of 100 from 200 through 900. Must be lower than the value specified by --max-throughput. If both min-throughput and min-instances are provided, min-instances takes precedence over min-throughput. The use of `min-throughput` is discouraged in favor of `min-instances`. |
| `connected_projects` | Vec<String> | Output only. List of projects using the connector. |
| `network` | String | Optional. Name of a VPC network. |
| `max_throughput` | i64 | Maximum throughput of the connector in Mbps. Refers to the expected throughput when using an `e2-micro` machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by --min-throughput. If both max-throughput and max-instances are provided, max-instances takes precedence over max-throughput. The use of `max-throughput` is discouraged in favor of `max-instances`. |
| `min_instances` | i64 | Minimum value of instances in autoscaling group underlying the connector. |
| `ip_cidr_range` | String | Optional. The range of internal addresses that follows RFC 4632 notation. Example: `10.132.0.0/28`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connector
connector = provider.vpcaccess_api.Connector {
    parent = "value"  # Required. The project ID and location in which the configuration should be created, specified in the format `projects/*/locations/*`.
}

# Access connector outputs
connector_id = connector.id
connector_max_instances = connector.max_instances
connector_machine_type = connector.machine_type
connector_name = connector.name
connector_subnet = connector.subnet
connector_state = connector.state
connector_min_throughput = connector.min_throughput
connector_connected_projects = connector.connected_projects
connector_network = connector.network
connector_max_throughput = connector.max_throughput
connector_min_instances = connector.min_instances
connector_ip_cidr_range = connector.ip_cidr_range
```

---


### Location

Lists information about the supported locations for this service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The standard List next-page token. |
| `locations` | Vec<String> | A list of locations that matches the specified filter in the request. |


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
location_next_page_token = location.next_page_token
location_locations = location.locations
```

---


### Connector

Creates a Serverless VPC Access connector, returns an operation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connected_projects` | Vec<String> |  | Output only. List of projects using the connector. |
| `ip_cidr_range` | String |  | Optional. The range of internal addresses that follows RFC 4632 notation. Example: `10.132.0.0/28`. |
| `max_instances` | i64 |  | Maximum value of instances in autoscaling group underlying the connector. |
| `network` | String |  | Optional. Name of a VPC network. |
| `state` | String |  | Output only. State of the VPC access connector. |
| `machine_type` | String |  | Machine type of VM Instance underlying connector. Default is e2-micro |
| `create_time` | String |  | Output only. The creation time of the connector. |
| `max_throughput` | i64 |  | Maximum throughput of the connector in Mbps. Refers to the expected throughput when using an `e2-micro` machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by --min-throughput. If both max-throughput and max-instances are provided, max-instances takes precedence over max-throughput. The use of `max-throughput` is discouraged in favor of `max-instances`. |
| `min_throughput` | i64 |  | Minimum throughput of the connector in Mbps. Refers to the expected throughput when using an `e2-micro` machine type. Value must be a multiple of 100 from 200 through 900. Must be lower than the value specified by --max-throughput. If both min-throughput and min-instances are provided, min-instances takes precedence over min-throughput. The use of `min-throughput` is discouraged in favor of `min-instances`. |
| `min_instances` | i64 |  | Minimum value of instances in autoscaling group underlying the connector. |
| `name` | String |  | The resource name in the format `projects/*/locations/*/connectors/*`. |
| `subnet` | String |  | Optional. The subnet in which to house the VPC Access Connector. |
| `last_restart_time` | String |  | Output only. The last restart time of the connector. |
| `parent` | String | ✅ | Required. The project ID and location in which the configuration should be created, specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connected_projects` | Vec<String> | Output only. List of projects using the connector. |
| `ip_cidr_range` | String | Optional. The range of internal addresses that follows RFC 4632 notation. Example: `10.132.0.0/28`. |
| `max_instances` | i64 | Maximum value of instances in autoscaling group underlying the connector. |
| `network` | String | Optional. Name of a VPC network. |
| `state` | String | Output only. State of the VPC access connector. |
| `machine_type` | String | Machine type of VM Instance underlying connector. Default is e2-micro |
| `create_time` | String | Output only. The creation time of the connector. |
| `max_throughput` | i64 | Maximum throughput of the connector in Mbps. Refers to the expected throughput when using an `e2-micro` machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by --min-throughput. If both max-throughput and max-instances are provided, max-instances takes precedence over max-throughput. The use of `max-throughput` is discouraged in favor of `max-instances`. |
| `min_throughput` | i64 | Minimum throughput of the connector in Mbps. Refers to the expected throughput when using an `e2-micro` machine type. Value must be a multiple of 100 from 200 through 900. Must be lower than the value specified by --max-throughput. If both min-throughput and min-instances are provided, min-instances takes precedence over min-throughput. The use of `min-throughput` is discouraged in favor of `min-instances`. |
| `min_instances` | i64 | Minimum value of instances in autoscaling group underlying the connector. |
| `name` | String | The resource name in the format `projects/*/locations/*/connectors/*`. |
| `subnet` | String | Optional. The subnet in which to house the VPC Access Connector. |
| `last_restart_time` | String | Output only. The last restart time of the connector. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connector
connector = provider.vpcaccess_api.Connector {
    parent = "value"  # Required. The project ID and location in which the configuration should be created, specified in the format `projects/*/locations/*`.
}

# Access connector outputs
connector_id = connector.id
connector_connected_projects = connector.connected_projects
connector_ip_cidr_range = connector.ip_cidr_range
connector_max_instances = connector.max_instances
connector_network = connector.network
connector_state = connector.state
connector_machine_type = connector.machine_type
connector_create_time = connector.create_time
connector_max_throughput = connector.max_throughput
connector_min_throughput = connector.min_throughput
connector_min_instances = connector.min_instances
connector_name = connector.name
connector_subnet = connector.subnet
connector_last_restart_time = connector.last_restart_time
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


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
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
operation_metadata = operation.metadata
operation_response = operation.response
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
location_0 = provider.vpcaccess_api.Location {
}
location_1 = provider.vpcaccess_api.Location {
}
location_2 = provider.vpcaccess_api.Location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.vpcaccess_api.Location {
    }
```

---

## Related Documentation

- [GCP Vpcaccess_api Documentation](https://cloud.google.com/vpcaccess_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
