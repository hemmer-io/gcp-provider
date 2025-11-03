# Servicenetworking_api Service



**Resources**: 11

---

## Overview

The servicenetworking_api service provides access to 11 resource types:

- [Dns_record_set](#dns_record_set) [CRU]
- [Service](#service) [CU]
- [Network](#network) [RU]
- [Role](#role) [C]
- [Dns_zone](#dns_zone) [CR]
- [Peered_dns_domain](#peered_dns_domain) [CRD]
- [Connection](#connection) [CRU]
- [Operation](#operation) [CRD]
- [Connection](#connection) [CR]
- [Service](#service) [CU]
- [Operation](#operation) [R]

---

## Resources


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
| `domain` | String | Required. The DNS or domain name of the record set, e.g. `test.example.com`. Cloud DNS requires that a DNS suffix ends with a trailing dot. |
| `data` | Vec<String> | Required. As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) for examples see https://cloud.google.com/dns/records/json-record. |
| `ttl` | String | Required. The period of time for which this RecordSet can be cached by resolvers. |


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
dns_record_set_domain = dns_record_set.domain
dns_record_set_data = dns_record_set.data
dns_record_set_ttl = dns_record_set.ttl
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
| `consumer_import_subnet_routes_with_public_ip` | bool | Import subnet routes with public ip flag value for peering from consumer to producer. |
| `consumer_export_custom_routes` | bool | Export custom routes flag value for peering from consumer to producer. |
| `producer_export_subnet_routes_with_public_ip` | bool | Export subnet routes with public ip flag value for peering from producer to consumer. |
| `consumer_export_subnet_routes_with_public_ip` | bool | Export subnet routes with public ip flag value for peering from consumer to producer. |
| `producer_export_custom_routes` | bool | Export custom routes flag value for peering from producer to consumer. |
| `consumer_import_custom_routes` | bool | Import custom routes flag value for peering from consumer to producer. |
| `reserved_ranges` | Vec<String> | Output only. The reserved ranges associated with this private service access connection. |
| `cloudsql_configs` | Vec<String> | Represents one or multiple Cloud SQL configurations. |
| `producer_import_subnet_routes_with_public_ip` | bool | Import subnet routes with public ip flag value for peering from producer to consumer. |
| `used_ip_ranges` | Vec<String> | Output only. The IP ranges already in use by consumer or producer |
| `producer_import_custom_routes` | bool | Import custom routes flag value for peering from producer to consumer. |
| `producer_network` | String | Output only. The VPC host network that is used to host managed service instances. In the format, projects/{project}/global/networks/{network} where {project} is the project number e.g. '12345' and {network} is the network name. |
| `vpc_sc_reference_architecture_enabled` | bool | Output only. Indicates whether the VPC Service Controls reference architecture is configured for the producer VPC host network. |


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
network_consumer_import_subnet_routes_with_public_ip = network.consumer_import_subnet_routes_with_public_ip
network_consumer_export_custom_routes = network.consumer_export_custom_routes
network_producer_export_subnet_routes_with_public_ip = network.producer_export_subnet_routes_with_public_ip
network_consumer_export_subnet_routes_with_public_ip = network.consumer_export_subnet_routes_with_public_ip
network_producer_export_custom_routes = network.producer_export_custom_routes
network_consumer_import_custom_routes = network.consumer_import_custom_routes
network_reserved_ranges = network.reserved_ranges
network_cloudsql_configs = network.cloudsql_configs
network_producer_import_subnet_routes_with_public_ip = network.producer_import_subnet_routes_with_public_ip
network_used_ip_ranges = network.used_ip_ranges
network_producer_import_custom_routes = network.producer_import_custom_routes
network_producer_network = network.producer_network
network_vpc_sc_reference_architecture_enabled = network.vpc_sc_reference_architecture_enabled
```

---


### Role

Service producers can use this method to add roles in the shared VPC host project. Each role is bound to the provided member. Each role must be selected from within an allowlisted set of roles. Each role is applied at only the granularity specified in the allowlist.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_binding` | Vec<String> |  | Required. List of policy bindings to add to shared VPC host project. |
| `consumer_network` | String |  | Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is a project number, as in '12345' {network} is a network name. |
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

Service producers can use this method to remove private DNS zones in the shared producer host project and matching peering zones in the consumer project.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The name for both the private zone in the shared producer host project and the peering zone in the consumer project. |
| `consumer_network` | String |  | Required. The network that the consumer is using to connect with services. Must be in the form of projects/{project}/global/networks/{network} {project} is the project number, as in '12345' {network} is the network name. |
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


### Peered_dns_domain

Creates a peered DNS domain which sends requests for records in given namespace originating in the service producer VPC network to the consumer VPC network to be resolved.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. User assigned name for this resource. Must be unique within the consumer network. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `dns_suffix` | String |  | The DNS domain name suffix e.g. `example.com.`. Cloud DNS requires that a DNS suffix ends with a trailing dot. |
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


### Connection

Creates a private connection that establishes a VPC Network Peering connection to a VPC network in the service producer's organization. The administrator of the service consumer's VPC network invokes this method. The administrator must assign one or more allocated IP ranges for provisioning subnetworks in the service producer's VPC network. This connection is used for all supported services in the service producer's organization, so it only needs to be invoked once.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `peering` | String |  | Output only. The name of the VPC Network Peering connection that was created by the service producer. |
| `reserved_peering_ranges` | Vec<String> |  | The name of one or more allocated IP address ranges for this service producer of type `PEERING`. Note that invoking CreateConnection method with a different range when connection is already established will not modify already provisioned service producer subnetworks. If CreateConnection method is invoked repeatedly to reconnect when peering connection had been disconnected on the consumer side, leaving this field empty will restore previously allocated IP ranges. |
| `network` | String |  | Required. The name of service consumer's VPC network that's connected with service producer network, in the following format: `projects/{project}/global/networks/{network}`. `{project}` is a project number, such as in `12345` that includes the VPC service consumer's VPC network. `{network}` is the name of the service consumer's VPC network. |
| `service` | String |  | Output only. The name of the peering service that's associated with this connection, in the following format: `services/{service name}`. |
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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

# Create operation
operation = provider.servicenetworking_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_metadata = operation.metadata
operation_response = operation.response
```

---


### Connection

Creates a private connection that establishes a VPC Network Peering connection to a VPC network in the service producer's organization. The administrator of the service consumer's VPC network invokes this method. The administrator must assign one or more allocated IP ranges for provisioning subnetworks in the service producer's VPC network. This connection is used for all supported services in the service producer's organization, so it only needs to be invoked once. The response from the `get` operation will be of type `Connection` if the operation successfully completes.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reserved_peering_ranges` | Vec<String> |  | The name of one or more allocated IP address ranges for this service producer of type `PEERING`. Note that invoking this method with a different range when connection is already established will not modify already provisioned service producer subnetworks. |
| `peering` | String |  | Output only. The name of the VPC Network Peering connection that was created by the service producer. |
| `network` | String |  | The name of service consumer's VPC network that's connected with service producer network, in the following format: `projects/{project}/global/networks/{network}`. `{project}` is a project number, such as in `12345` that includes the VPC service consumer's VPC network. `{network}` is the name of the service consumer's VPC network. |
| `service` | String |  | Output only. The name of the peering service that's associated with this connection, in the following format: `services/{service name}`. |
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


### Service

For service producers, provisions a new subnet in a peered service's shared VPC network in the requested region and with the requested size that's expressed as a CIDR range (number of leading bits of ipV4 network mask). The method checks against the assigned allocated ranges to find a non-conflicting IP address range. The method will reuse a subnet if subsequent calls contain the same subnet name, region, and prefix length. This method will make producer's tenant project to be a shared VPC service project as needed. The response from the `get` operation will be of type `Subnetwork` if the operation successfully completes.

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consumer` | String |  | Required. A resource that represents the service consumer, such as `projects/123456`. The project number can be different from the value in the consumer network parameter. For example, the network might be part of a Shared VPC network. In those cases, Service Networking validates that this resource belongs to that Shared VPC. |
| `region` | String |  | Required. The name of a [region](/compute/docs/regions-zones) for the subnet, such `europe-west1`. |
| `ip_prefix_length` | i64 |  | Required. The prefix length of the subnet's IP address range. Use CIDR range notation, such as `30` to provision a subnet with an `x.x.x.x/30` CIDR range. The IP address range is drawn from a pool of available ranges in the service consumer's allocated range. |
| `subnetwork_users` | Vec<String> |  | A list of members that are granted the `compute.networkUser` role on the subnet. |
| `requested_address` | String |  | Optional. The starting address of a range. The address must be a valid IPv4 address in the x.x.x.x format. This value combined with the IP prefix range is the CIDR range for the subnet. The range must be within the allocated range that is assigned to the private connection. If the CIDR range isn't available, the call fails. |
| `consumer_network` | String |  | Required. The name of the service consumer's VPC network. The network must have an existing private connection that was provisioned through the connections.create method. The name must be in the following format: `projects/{project}/global/networks/{network}`, where {project} is a project number, such as `12345`. {network} is the name of a VPC network in the project. |
| `subnetwork` | String |  | Required. A name for the new subnet. For information about the naming requirements, see [subnetwork](/compute/docs/reference/rest/v1/subnetworks) in the Compute API documentation. |
| `description` | String |  | An optional description of the subnet. |
| `parent` | String | ✅ | Required. A tenant project in the service producer organization, in the following format: services/{service}/{collection-id}/{resource-id}. {collection-id} is the cloud resource collection type that represents the tenant project. Only `projects` are supported. {resource-id} is the tenant project numeric id, such as `123456`. {service} the name of the peering service, such as `service-peering.example.com`. This service must already be enabled in the service consumer's project. |



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
    parent = "value"  # Required. A tenant project in the service producer organization, in the following format: services/{service}/{collection-id}/{resource-id}. {collection-id} is the cloud resource collection type that represents the tenant project. Only `projects` are supported. {resource-id} is the tenant project numeric id, such as `123456`. {service} the name of the peering service, such as `service-peering.example.com`. This service must already be enabled in the service consumer's project.
}

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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


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
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple dns_record_set resources
dns_record_set_0 = provider.servicenetworking_api.Dns_record_set {
    parent = "value-0"
}
dns_record_set_1 = provider.servicenetworking_api.Dns_record_set {
    parent = "value-1"
}
dns_record_set_2 = provider.servicenetworking_api.Dns_record_set {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    dns_record_set = provider.servicenetworking_api.Dns_record_set {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Servicenetworking_api Documentation](https://cloud.google.com/servicenetworking_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
