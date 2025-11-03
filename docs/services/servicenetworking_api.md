# Servicenetworking_api Service



**Resources**: 11

---

## Overview

The servicenetworking_api service provides access to 11 resource types:

- [Peered_dns_domain](#peered_dns_domain) [CRD]
- [Network](#network) [RU]
- [Connection](#connection) [CRU]
- [Service](#service) [CU]
- [Dns_record_set](#dns_record_set) [CRU]
- [Role](#role) [C]
- [Dns_zone](#dns_zone) [CR]
- [Operation](#operation) [CRD]
- [Service](#service) [CU]
- [Connection](#connection) [CR]
- [Operation](#operation) [R]

---

## Resources


### Peered_dns_domain

Creates a peered DNS domain which sends requests for records in given namespace originating in the service producer VPC network to the consumer VPC network to be resolved.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dns_suffix` | String |  | The DNS domain name suffix e.g. `example.com.`. Cloud DNS requires that a DNS suffix ends with a trailing dot. |
| `name` | String |  | Required. User assigned name for this resource. Must be unique within the consumer network. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `parent` | String | ✅ | Required. Parent resource identifying the connection for which the peered DNS domain will be created in the format: `services/{service}/projects/{project}/global/networks/{network}` {service} is the peering service that is managing connectivity for the service producer's organization. For Google services that support this functionality, this value is `servicenetworking.googleapis.com`. {project} is the number of the project that contains the service consumer's VPC network e.g. `12345`. {network} is the name of the service consumer's VPC network. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `peered_dns_domains` | Vec<String> | The list of peered DNS domains. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create peered_dns_domain
peered_dns_domain = provider.servicenetworking_api.Peered_dns_domain {
    parent = "value"  # Required. Parent resource identifying the connection for which the peered DNS domain will be created in the format: `services/{service}/projects/{project}/global/networks/{network}` {service} is the peering service that is managing connectivity for the service producer's organization. For Google services that support this functionality, this value is `servicenetworking.googleapis.com`. {project} is the number of the project that contains the service consumer's VPC network e.g. `12345`. {network} is the name of the service consumer's VPC network.
}

# Access peered_dns_domain outputs
peered_dns_domain_id = peered_dns_domain.id
peered_dns_domain_peered_dns_domains = peered_dns_domain.peered_dns_domains
```

---


### Network

Service producers use this method to get the configuration of their connection including the import/export of custom routes and subnetwork routes with public IP.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consumer_config` | String |  | Required. The updated peering config. |
| `parent` | String | ✅ | Required. Parent resource identifying the connection for which the consumer config is being updated in the format: `services/{service}/projects/{project}/global/networks/{network}` {service} is the peering service that is managing connectivity for the service producer's organization. For Google services that support this functionality, this value is `servicenetworking.googleapis.com`. {project} is the number of the project that contains the service consumer's VPC network e.g. `12345`. {network} is the name of the service consumer's VPC network. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_ranges` | Vec<String> | Output only. The reserved ranges associated with this private service access connection. |
| `consumer_import_subnet_routes_with_public_ip` | bool | Import subnet routes with public ip flag value for peering from consumer to producer. |
| `cloudsql_configs` | Vec<String> | Represents one or multiple Cloud SQL configurations. |
| `consumer_export_subnet_routes_with_public_ip` | bool | Export subnet routes with public ip flag value for peering from consumer to producer. |
| `producer_export_subnet_routes_with_public_ip` | bool | Export subnet routes with public ip flag value for peering from producer to consumer. |
| `producer_network` | String | Output only. The VPC host network that is used to host managed service instances. In the format, projects/{project}/global/networks/{network} where {project} is the project number e.g. '12345' and {network} is the network name. |
| `vpc_sc_reference_architecture_enabled` | bool | Output only. Indicates whether the VPC Service Controls reference architecture is configured for the producer VPC host network. |
| `producer_export_custom_routes` | bool | Export custom routes flag value for peering from producer to consumer. |
| `used_ip_ranges` | Vec<String> | Output only. The IP ranges already in use by consumer or producer |
| `consumer_import_custom_routes` | bool | Import custom routes flag value for peering from consumer to producer. |
| `consumer_export_custom_routes` | bool | Export custom routes flag value for peering from consumer to producer. |
| `producer_import_custom_routes` | bool | Import custom routes flag value for peering from producer to consumer. |
| `producer_import_subnet_routes_with_public_ip` | bool | Import subnet routes with public ip flag value for peering from producer to consumer. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access network outputs
network_id = network.id
network_reserved_ranges = network.reserved_ranges
network_consumer_import_subnet_routes_with_public_ip = network.consumer_import_subnet_routes_with_public_ip
network_cloudsql_configs = network.cloudsql_configs
network_consumer_export_subnet_routes_with_public_ip = network.consumer_export_subnet_routes_with_public_ip
network_producer_export_subnet_routes_with_public_ip = network.producer_export_subnet_routes_with_public_ip
network_producer_network = network.producer_network
network_vpc_sc_reference_architecture_enabled = network.vpc_sc_reference_architecture_enabled
network_producer_export_custom_routes = network.producer_export_custom_routes
network_used_ip_ranges = network.used_ip_ranges
network_consumer_import_custom_routes = network.consumer_import_custom_routes
network_consumer_export_custom_routes = network.consumer_export_custom_routes
network_producer_import_custom_routes = network.producer_import_custom_routes
network_producer_import_subnet_routes_with_public_ip = network.producer_import_subnet_routes_with_public_ip
```

---


### Connection

Creates a private connection that establishes a VPC Network Peering connection to a VPC network in the service producer's organization. The administrator of the service consumer's VPC network invokes this method. The administrator must assign one or more allocated IP ranges for provisioning subnetworks in the service producer's VPC network. This connection is used for all supported services in the service producer's organization, so it only needs to be invoked once.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network` | String |  | Required. The name of service consumer's VPC network that's connected with service producer network, in the following format: `projects/{project}/global/networks/{network}`. `{project}` is a project number, such as in `12345` that includes the VPC service consumer's VPC network. `{network}` is the name of the service consumer's VPC network. |
| `peering` | String |  | Output only. The name of the VPC Network Peering connection that was created by the service producer. |
| `service` | String |  | Output only. The name of the peering service that's associated with this connection, in the following format: `services/{service name}`. |
| `reserved_peering_ranges` | Vec<String> |  | The name of one or more allocated IP address ranges for this service producer of type `PEERING`. Note that invoking CreateConnection method with a different range when connection is already established will not modify already provisioned service producer subnetworks. If CreateConnection method is invoked repeatedly to reconnect when peering connection had been disconnected on the consumer side, leaving this field empty will restore previously allocated IP ranges. |
| `parent` | String | ✅ | Required. The service that is managing peering connectivity for a service producer's organization. For Google services that support this functionality, this value is `services/servicenetworking.googleapis.com`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connections` | Vec<String> | The list of Connections. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection
connection = provider.servicenetworking_api.Connection {
    parent = "value"  # Required. The service that is managing peering connectivity for a service producer's organization. For Google services that support this functionality, this value is `services/servicenetworking.googleapis.com`.
}

# Access connection outputs
connection_id = connection.id
connection_connections = connection.connections
```

---


### Service

Service producers can use this method to find a currently unused range within consumer allocated ranges. This returned range is not reserved, and not guaranteed to remain unused. It will validate previously provided allocated ranges, find non-conflicting sub-range of requested size (expressed in number of leading bits of ipv4 network mask, as in CIDR range notation).

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ip_prefix_length` | i64 |  | Required. The prefix length of the IP range. Use usual CIDR range notation. For example, '30' to find unused x.x.x.x/30 CIDR range. Actual range will be determined using allocated range for the consumer peered network and returned in the result. |
| `network` | String |  | Required. Network name in the consumer project. This network must have been already peered with a shared VPC network using CreateConnection method. Must be in a form 'projects/{project}/global/networks/{network}'. {project} is a project number, as in '12345' {network} is network name. |
| `parent` | String | ✅ | Required. This is in a form services/{service}. {service} the name of the private access management service, for example 'service-peering.example.com'. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.servicenetworking_api.Service {
    parent = "value"  # Required. This is in a form services/{service}. {service} the name of the private access management service, for example 'service-peering.example.com'.
}

```

---


### Dns_record_set

Service producers can use this method to add DNS record sets to private DNS zones in the shared producer host project.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consumer_network` | String |  | Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is the project number, as in '12345' {network} is the network name. |
| `dns_record_set` | String |  | Required. The DNS record set to add. |
| `zone` | String |  | Required. The name of the private DNS zone in the shared producer host project to which the record set will be added. |
| `parent` | String | ✅ | Required. The service that is managing peering connectivity for a service producer's organization. For Google services that support this functionality, this value is `services/servicenetworking.googleapis.com`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Required. The identifier of a supported record type. |
| `ttl` | String | Required. The period of time for which this RecordSet can be cached by resolvers. |
| `domain` | String | Required. The DNS or domain name of the record set, e.g. `test.example.com`. Cloud DNS requires that a DNS suffix ends with a trailing dot. |
| `data` | Vec<String> | Required. As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) for examples see https://cloud.google.com/dns/records/json-record. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dns_record_set
dns_record_set = provider.servicenetworking_api.Dns_record_set {
    parent = "value"  # Required. The service that is managing peering connectivity for a service producer's organization. For Google services that support this functionality, this value is `services/servicenetworking.googleapis.com`.
}

# Access dns_record_set outputs
dns_record_set_id = dns_record_set.id
dns_record_set_type = dns_record_set.type
dns_record_set_ttl = dns_record_set.ttl
dns_record_set_domain = dns_record_set.domain
dns_record_set_data = dns_record_set.data
```

---


### Role

Service producers can use this method to add roles in the shared VPC host project. Each role is bound to the provided member. Each role must be selected from within an allowlisted set of roles. Each role is applied at only the granularity specified in the allowlist.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consumer_network` | String |  | Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is a project number, as in '12345' {network} is a network name. |
| `policy_binding` | Vec<String> |  | Required. List of policy bindings to add to shared VPC host project. |
| `parent` | String | ✅ | Required. This is in a form services/{service} where {service} is the name of the private access management service. For example 'service-peering.example.com'. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create role
role = provider.servicenetworking_api.Role {
    parent = "value"  # Required. This is in a form services/{service} where {service} is the name of the private access management service. For example 'service-peering.example.com'.
}

```

---


### Dns_zone

Service producers can use this method to add private DNS zones in the shared producer host project and matching peering zones in the consumer project.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The name for both the private zone in the shared producer host project and the peering zone in the consumer project. Must be unique within both projects. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `consumer_network` | String |  | Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is the project number, as in '12345' {network} is the network name. |
| `dns_suffix` | String |  | Required. The DNS name suffix for the zones e.g. `example.com.`. Cloud DNS requires that a DNS suffix ends with a trailing dot. |
| `parent` | String | ✅ | Required. The service that is managing peering connectivity for a service producer's organization. For Google services that support this functionality, this value is `services/servicenetworking.googleapis.com`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `producer_private_zone` | String | The private DNS zone created in the shared producer host project. |
| `consumer_peering_zone` | String | The DNS peering zone created in the consumer project. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dns_zone
dns_zone = provider.servicenetworking_api.Dns_zone {
    parent = "value"  # Required. The service that is managing peering connectivity for a service producer's organization. For Google services that support this functionality, this value is `services/servicenetworking.googleapis.com`.
}

# Access dns_zone outputs
dns_zone_id = dns_zone.id
dns_zone_producer_private_zone = dns_zone.producer_private_zone
dns_zone_consumer_peering_zone = dns_zone.consumer_peering_zone
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.servicenetworking_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_response = operation.response
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
```

---


### Service

Service producers can use this method to find a currently unused range within consumer allocated ranges. This returned range is not reserved, and not guaranteed to remain unused. It will validate previously provided allocated ranges, find non-conflicting sub-range of requested size (expressed in number of leading bits of ipv4 network mask, as in CIDR range notation). Operation

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network` | String |  | Network name in the consumer project. This network must have been already peered with a shared VPC network using CreateConnection method. Must be in a form 'projects/{project}/global/networks/{network}'. {project} is a project number, as in '12345' {network} is network name. |
| `ip_prefix_length` | i64 |  | Required. The prefix length of the IP range. Use usual CIDR range notation. For example, '30' to find unused x.x.x.x/30 CIDR range. Actual range will be determined using allocated range for the consumer peered network and returned in the result. |
| `parent` | String | ✅ | Required. This is in a form services/{service}. {service} the name of the private access management service, for example 'service-peering.example.com'. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.servicenetworking_api.Service {
    parent = "value"  # Required. This is in a form services/{service}. {service} the name of the private access management service, for example 'service-peering.example.com'.
}

```

---


### Connection

Creates a private connection that establishes a VPC Network Peering connection to a VPC network in the service producer's organization. The administrator of the service consumer's VPC network invokes this method. The administrator must assign one or more allocated IP ranges for provisioning subnetworks in the service producer's VPC network. This connection is used for all supported services in the service producer's organization, so it only needs to be invoked once. The response from the `get` operation will be of type `Connection` if the operation successfully completes.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `peering` | String |  | Output only. The name of the VPC Network Peering connection that was created by the service producer. |
| `service` | String |  | Output only. The name of the peering service that's associated with this connection, in the following format: `services/{service name}`. |
| `network` | String |  | The name of service consumer's VPC network that's connected with service producer network, in the following format: `projects/{project}/global/networks/{network}`. `{project}` is a project number, such as in `12345` that includes the VPC service consumer's VPC network. `{network}` is the name of the service consumer's VPC network. |
| `reserved_peering_ranges` | Vec<String> |  | The name of one or more allocated IP address ranges for this service producer of type `PEERING`. Note that invoking this method with a different range when connection is already established will not modify already provisioned service producer subnetworks. |
| `parent` | String | ✅ | The service that is managing peering connectivity for a service producer's organization. For Google services that support this functionality, this value is `services/servicenetworking.googleapis.com`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connections` | Vec<String> | The list of Connections. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection
connection = provider.servicenetworking_api.Connection {
    parent = "value"  # The service that is managing peering connectivity for a service producer's organization. For Google services that support this functionality, this value is `services/servicenetworking.googleapis.com`.
}

# Access connection outputs
connection_id = connection.id
connection_connections = connection.connections
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation_name = operation.name
operation_error = operation.error
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple peered_dns_domain resources
peered_dns_domain_0 = provider.servicenetworking_api.Peered_dns_domain {
    parent = "value-0"
}
peered_dns_domain_1 = provider.servicenetworking_api.Peered_dns_domain {
    parent = "value-1"
}
peered_dns_domain_2 = provider.servicenetworking_api.Peered_dns_domain {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    peered_dns_domain = provider.servicenetworking_api.Peered_dns_domain {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Servicenetworking_api Documentation](https://cloud.google.com/servicenetworking_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
