# Networkconnectivity_api Service



**Resources**: 22

---

## Overview

The networkconnectivity_api service provides access to 22 resource types:

- [Operation](#operation) [CRD]
- [Multicloud_data_transfer_config](#multicloud_data_transfer_config) [CRUD]
- [Internal_range](#internal_range) [CRUD]
- [Regional_endpoint](#regional_endpoint) [CRD]
- [Service_connection_token](#service_connection_token) [CRD]
- [Service_connection_policie](#service_connection_policie) [CRUD]
- [Route_table](#route_table) [R]
- [Service_connection_map](#service_connection_map) [CRUD]
- [Hub](#hub) [CRUD]
- [Policy_based_route](#policy_based_route) [CRD]
- [Group](#group) [CRU]
- [Location](#location) [CR]
- [Route](#route) [R]
- [Service_classe](#service_classe) [CRUD]
- [Destination](#destination) [CRUD]
- [Multicloud_data_transfer_supported_service](#multicloud_data_transfer_supported_service) [R]
- [Spoke](#spoke) [CRUD]
- [Internal_range](#internal_range) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Hub](#hub) [CRUD]
- [Spoke](#spoke) [CRUD]

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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.networkconnectivity_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
```

---


### Multicloud_data_transfer_config

Creates a `MulticloudDataTransferConfig` resource in a specified project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The name of the `MulticloudDataTransferConfig` resource. Format: `projects/{project}/locations/{location}/multicloudDataTransferConfigs/{multicloud_data_transfer_config}`. |
| `description` | String |  | Optional. A description of this resource. |
| `destinations_count` | i64 |  | Output only. The number of `Destination` resources configured for the `MulticloudDataTransferConfig` resource. |
| `create_time` | String |  | Output only. Time when the `MulticloudDataTransferConfig` resource was created. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels. |
| `uid` | String |  | Output only. The Google-generated unique ID for the `MulticloudDataTransferConfig` resource. This value is unique across all `MulticloudDataTransferConfig` resources. If a resource is deleted and another with the same name is created, the new resource is assigned a different and unique ID. |
| `destinations_active_count` | i64 |  | Output only. The number of `Destination` resources in use with the `MulticloudDataTransferConfig` resource. |
| `update_time` | String |  | Output only. Time when the `MulticloudDataTransferConfig` resource was updated. |
| `services` | HashMap<String, String> |  | Optional. Maps services to their current or planned states. Service names are keys, and the associated values describe the state of the service. If a state change is expected, the value is either `ADDING` or `DELETING`, depending on the actions taken. Sample output: "services": { "big-query": { "states": [ { "effectiveTime": "2024-12-12T08:00:00Z" "state": "ADDING", }, ] }, "cloud-storage": { "states": [ { "state": "ACTIVE", } ] } } |
| `etag` | String |  | The etag is computed by the server, and might be sent with update and delete requests so that the client has an up-to-date value before proceeding. |
| `parent` | String | ✅ | Required. The name of the parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The name of the `MulticloudDataTransferConfig` resource. Format: `projects/{project}/locations/{location}/multicloudDataTransferConfigs/{multicloud_data_transfer_config}`. |
| `description` | String | Optional. A description of this resource. |
| `destinations_count` | i64 | Output only. The number of `Destination` resources configured for the `MulticloudDataTransferConfig` resource. |
| `create_time` | String | Output only. Time when the `MulticloudDataTransferConfig` resource was created. |
| `labels` | HashMap<String, String> | Optional. User-defined labels. |
| `uid` | String | Output only. The Google-generated unique ID for the `MulticloudDataTransferConfig` resource. This value is unique across all `MulticloudDataTransferConfig` resources. If a resource is deleted and another with the same name is created, the new resource is assigned a different and unique ID. |
| `destinations_active_count` | i64 | Output only. The number of `Destination` resources in use with the `MulticloudDataTransferConfig` resource. |
| `update_time` | String | Output only. Time when the `MulticloudDataTransferConfig` resource was updated. |
| `services` | HashMap<String, String> | Optional. Maps services to their current or planned states. Service names are keys, and the associated values describe the state of the service. If a state change is expected, the value is either `ADDING` or `DELETING`, depending on the actions taken. Sample output: "services": { "big-query": { "states": [ { "effectiveTime": "2024-12-12T08:00:00Z" "state": "ADDING", }, ] }, "cloud-storage": { "states": [ { "state": "ACTIVE", } ] } } |
| `etag` | String | The etag is computed by the server, and might be sent with update and delete requests so that the client has an up-to-date value before proceeding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multicloud_data_transfer_config
multicloud_data_transfer_config = provider.networkconnectivity_api.Multicloud_data_transfer_config {
    parent = "value"  # Required. The name of the parent resource.
}

# Access multicloud_data_transfer_config outputs
multicloud_data_transfer_config_id = multicloud_data_transfer_config.id
multicloud_data_transfer_config_name = multicloud_data_transfer_config.name
multicloud_data_transfer_config_description = multicloud_data_transfer_config.description
multicloud_data_transfer_config_destinations_count = multicloud_data_transfer_config.destinations_count
multicloud_data_transfer_config_create_time = multicloud_data_transfer_config.create_time
multicloud_data_transfer_config_labels = multicloud_data_transfer_config.labels
multicloud_data_transfer_config_uid = multicloud_data_transfer_config.uid
multicloud_data_transfer_config_destinations_active_count = multicloud_data_transfer_config.destinations_active_count
multicloud_data_transfer_config_update_time = multicloud_data_transfer_config.update_time
multicloud_data_transfer_config_services = multicloud_data_transfer_config.services
multicloud_data_transfer_config_etag = multicloud_data_transfer_config.etag
```

---


### Internal_range

Creates a new internal range in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network` | String |  | Immutable. The URL or resource ID of the network in which to reserve the internal range. The network cannot be deleted if there are any reserved internal ranges referring to it. Legacy networks are not supported. For example: https://www.googleapis.com/compute/v1/projects/{project}/locations/global/networks/{network} projects/{project}/locations/global/networks/{network} {network} |
| `allocation_options` | String |  | Optional. Range auto-allocation options, may be set only when auto-allocation is selected by not setting ip_cidr_range (and setting prefix_length). |
| `create_time` | String |  | Time when the internal range was created. |
| `description` | String |  | Optional. A description of this resource. |
| `prefix_length` | i64 |  | Optional. An alternate to ip_cidr_range. Can be set when trying to create an IPv4 reservation that automatically finds a free range of the given size. If both ip_cidr_range and prefix_length are set, there is an error if the range sizes do not match. Can also be used during updates to change the range size. NOTE: For IPv6 this field only works if ip_cidr_range is set as well, and both fields must match. In other words, with IPv6 this field only works as a redundant parameter. |
| `target_cidr_range` | Vec<String> |  | Optional. Can be set to narrow down or pick a different address space while searching for a free range. If not set, defaults to the ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16"] address space (for auto-mode networks, the "10.0.0.0/9" range is used instead of "10.0.0.0/8"). This can be used to target the search in other rfc-1918 address spaces like "172.16.0.0/12" and "192.168.0.0/16" or non-rfc-1918 address spaces used in the VPC. |
| `users` | Vec<String> |  | Output only. The list of resources that refer to this internal range. Resources that use the internal range for their range allocation are referred to as users of the range. Other resources mark themselves as users while doing so by creating a reference to this internal range. Having a user, based on this reference, prevents deletion of the internal range referred to. Can be empty. |
| `immutable` | bool |  | Optional. Immutable ranges cannot have their fields modified, except for labels and description. |
| `name` | String |  | Identifier. The name of an internal range. Format: projects/{project}/locations/{location}/internalRanges/{internal_range} See: https://google.aip.dev/122#fields-representing-resource-names |
| `usage` | String |  | Optional. The type of usage set for this InternalRange. |
| `exclude_cidr_ranges` | Vec<String> |  | Optional. ExcludeCidrRanges flag. Specifies a set of CIDR blocks that allows exclusion of particular CIDR ranges from the auto-allocation process, without having to reserve these blocks |
| `migration` | String |  | Optional. Must be present if usage is set to FOR_MIGRATION. |
| `update_time` | String |  | Time when the internal range was updated. |
| `labels` | HashMap<String, String> |  | User-defined labels. |
| `overlaps` | Vec<String> |  | Optional. Types of resources that are allowed to overlap with the current internal range. |
| `ip_cidr_range` | String |  | Optional. The IP range that this internal range defines. NOTE: IPv6 ranges are limited to usage=EXTERNAL_TO_VPC and peering=FOR_SELF. NOTE: For IPv6 Ranges this field is compulsory, i.e. the address range must be specified explicitly. |
| `peering` | String |  | Optional. The type of peering set for this internal range. |
| `parent` | String | ✅ | Required. The parent resource's name of the internal range. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network` | String | Immutable. The URL or resource ID of the network in which to reserve the internal range. The network cannot be deleted if there are any reserved internal ranges referring to it. Legacy networks are not supported. For example: https://www.googleapis.com/compute/v1/projects/{project}/locations/global/networks/{network} projects/{project}/locations/global/networks/{network} {network} |
| `allocation_options` | String | Optional. Range auto-allocation options, may be set only when auto-allocation is selected by not setting ip_cidr_range (and setting prefix_length). |
| `create_time` | String | Time when the internal range was created. |
| `description` | String | Optional. A description of this resource. |
| `prefix_length` | i64 | Optional. An alternate to ip_cidr_range. Can be set when trying to create an IPv4 reservation that automatically finds a free range of the given size. If both ip_cidr_range and prefix_length are set, there is an error if the range sizes do not match. Can also be used during updates to change the range size. NOTE: For IPv6 this field only works if ip_cidr_range is set as well, and both fields must match. In other words, with IPv6 this field only works as a redundant parameter. |
| `target_cidr_range` | Vec<String> | Optional. Can be set to narrow down or pick a different address space while searching for a free range. If not set, defaults to the ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16"] address space (for auto-mode networks, the "10.0.0.0/9" range is used instead of "10.0.0.0/8"). This can be used to target the search in other rfc-1918 address spaces like "172.16.0.0/12" and "192.168.0.0/16" or non-rfc-1918 address spaces used in the VPC. |
| `users` | Vec<String> | Output only. The list of resources that refer to this internal range. Resources that use the internal range for their range allocation are referred to as users of the range. Other resources mark themselves as users while doing so by creating a reference to this internal range. Having a user, based on this reference, prevents deletion of the internal range referred to. Can be empty. |
| `immutable` | bool | Optional. Immutable ranges cannot have their fields modified, except for labels and description. |
| `name` | String | Identifier. The name of an internal range. Format: projects/{project}/locations/{location}/internalRanges/{internal_range} See: https://google.aip.dev/122#fields-representing-resource-names |
| `usage` | String | Optional. The type of usage set for this InternalRange. |
| `exclude_cidr_ranges` | Vec<String> | Optional. ExcludeCidrRanges flag. Specifies a set of CIDR blocks that allows exclusion of particular CIDR ranges from the auto-allocation process, without having to reserve these blocks |
| `migration` | String | Optional. Must be present if usage is set to FOR_MIGRATION. |
| `update_time` | String | Time when the internal range was updated. |
| `labels` | HashMap<String, String> | User-defined labels. |
| `overlaps` | Vec<String> | Optional. Types of resources that are allowed to overlap with the current internal range. |
| `ip_cidr_range` | String | Optional. The IP range that this internal range defines. NOTE: IPv6 ranges are limited to usage=EXTERNAL_TO_VPC and peering=FOR_SELF. NOTE: For IPv6 Ranges this field is compulsory, i.e. the address range must be specified explicitly. |
| `peering` | String | Optional. The type of peering set for this internal range. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create internal_range
internal_range = provider.networkconnectivity_api.Internal_range {
    parent = "value"  # Required. The parent resource's name of the internal range.
}

# Access internal_range outputs
internal_range_id = internal_range.id
internal_range_network = internal_range.network
internal_range_allocation_options = internal_range.allocation_options
internal_range_create_time = internal_range.create_time
internal_range_description = internal_range.description
internal_range_prefix_length = internal_range.prefix_length
internal_range_target_cidr_range = internal_range.target_cidr_range
internal_range_users = internal_range.users
internal_range_immutable = internal_range.immutable
internal_range_name = internal_range.name
internal_range_usage = internal_range.usage
internal_range_exclude_cidr_ranges = internal_range.exclude_cidr_ranges
internal_range_migration = internal_range.migration
internal_range_update_time = internal_range.update_time
internal_range_labels = internal_range.labels
internal_range_overlaps = internal_range.overlaps
internal_range_ip_cidr_range = internal_range.ip_cidr_range
internal_range_peering = internal_range.peering
```

---


### Regional_endpoint

Creates a new RegionalEndpoint in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when the RegionalEndpoint was created. |
| `name` | String |  | Output only. The name of a RegionalEndpoint. Pattern: `projects/{project}/locations/{location}/regionalEndpoints/^[-a-z0-9](?:[-a-z0-9]{0,44})[a-z0-9]$`. |
| `psc_forwarding_rule` | String |  | Output only. The resource reference of the PSC Forwarding Rule created on behalf of the customer. Format: `//compute.googleapis.com/projects/{project}/regions/{region}/forwardingRules/{forwarding_rule_name}` |
| `target_google_api` | String |  | Required. The service endpoint this private regional endpoint connects to. Format: `{apiname}.{region}.p.rep.googleapis.com` Example: "cloudkms.us-central1.p.rep.googleapis.com". |
| `access_type` | String |  | Required. The access type of this regional endpoint. This field is reflected in the PSC Forwarding Rule configuration to enable global access. |
| `description` | String |  | Optional. A description of this resource. |
| `ip_address` | String |  | Output only. The literal IP address of the PSC Forwarding Rule created on behalf of the customer. This field is deprecated. Use address instead. |
| `network` | String |  | Optional. The name of the VPC network for this private regional endpoint. Format: `projects/{project}/global/networks/{network}` |
| `subnetwork` | String |  | Optional. The name of the subnetwork from which the IP address will be allocated. Format: `projects/{project}/regions/{region}/subnetworks/{subnetwork}` |
| `address` | String |  | Optional. The IP Address of the Regional Endpoint. When no address is provided, an IP from the subnetwork is allocated. Use one of the following formats: * IPv4 address as in `10.0.0.1` * Address resource URI as in `projects/{project}/regions/{region}/addresses/{address_name}` for an IPv4 or IPv6 address. |
| `labels` | HashMap<String, String> |  | User-defined labels. |
| `update_time` | String |  | Output only. Time when the RegionalEndpoint was updated. |
| `parent` | String | ✅ | Required. The parent resource's name of the RegionalEndpoint. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time when the RegionalEndpoint was created. |
| `name` | String | Output only. The name of a RegionalEndpoint. Pattern: `projects/{project}/locations/{location}/regionalEndpoints/^[-a-z0-9](?:[-a-z0-9]{0,44})[a-z0-9]$`. |
| `psc_forwarding_rule` | String | Output only. The resource reference of the PSC Forwarding Rule created on behalf of the customer. Format: `//compute.googleapis.com/projects/{project}/regions/{region}/forwardingRules/{forwarding_rule_name}` |
| `target_google_api` | String | Required. The service endpoint this private regional endpoint connects to. Format: `{apiname}.{region}.p.rep.googleapis.com` Example: "cloudkms.us-central1.p.rep.googleapis.com". |
| `access_type` | String | Required. The access type of this regional endpoint. This field is reflected in the PSC Forwarding Rule configuration to enable global access. |
| `description` | String | Optional. A description of this resource. |
| `ip_address` | String | Output only. The literal IP address of the PSC Forwarding Rule created on behalf of the customer. This field is deprecated. Use address instead. |
| `network` | String | Optional. The name of the VPC network for this private regional endpoint. Format: `projects/{project}/global/networks/{network}` |
| `subnetwork` | String | Optional. The name of the subnetwork from which the IP address will be allocated. Format: `projects/{project}/regions/{region}/subnetworks/{subnetwork}` |
| `address` | String | Optional. The IP Address of the Regional Endpoint. When no address is provided, an IP from the subnetwork is allocated. Use one of the following formats: * IPv4 address as in `10.0.0.1` * Address resource URI as in `projects/{project}/regions/{region}/addresses/{address_name}` for an IPv4 or IPv6 address. |
| `labels` | HashMap<String, String> | User-defined labels. |
| `update_time` | String | Output only. Time when the RegionalEndpoint was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create regional_endpoint
regional_endpoint = provider.networkconnectivity_api.Regional_endpoint {
    parent = "value"  # Required. The parent resource's name of the RegionalEndpoint.
}

# Access regional_endpoint outputs
regional_endpoint_id = regional_endpoint.id
regional_endpoint_create_time = regional_endpoint.create_time
regional_endpoint_name = regional_endpoint.name
regional_endpoint_psc_forwarding_rule = regional_endpoint.psc_forwarding_rule
regional_endpoint_target_google_api = regional_endpoint.target_google_api
regional_endpoint_access_type = regional_endpoint.access_type
regional_endpoint_description = regional_endpoint.description
regional_endpoint_ip_address = regional_endpoint.ip_address
regional_endpoint_network = regional_endpoint.network
regional_endpoint_subnetwork = regional_endpoint.subnetwork
regional_endpoint_address = regional_endpoint.address
regional_endpoint_labels = regional_endpoint.labels
regional_endpoint_update_time = regional_endpoint.update_time
```

---


### Service_connection_token

Creates a new ServiceConnectionToken in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The name of a ServiceConnectionToken. Format: projects/{project}/locations/{location}/ServiceConnectionTokens/{service_connection_token} See: https://google.aip.dev/122#fields-representing-resource-names |
| `description` | String |  | A description of this resource. |
| `etag` | String |  | Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `create_time` | String |  | Output only. Time when the ServiceConnectionToken was created. |
| `token` | String |  | Output only. The token generated by Automation. |
| `labels` | HashMap<String, String> |  | User-defined labels. |
| `update_time` | String |  | Output only. Time when the ServiceConnectionToken was updated. |
| `expire_time` | String |  | Output only. The time to which this token is valid. |
| `network` | String |  | The resource path of the network associated with this token. Example: projects/{projectNumOrId}/global/networks/{resourceId}. |
| `parent` | String | ✅ | Required. The parent resource's name of the ServiceConnectionToken. ex. projects/123/locations/us-east1 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The name of a ServiceConnectionToken. Format: projects/{project}/locations/{location}/ServiceConnectionTokens/{service_connection_token} See: https://google.aip.dev/122#fields-representing-resource-names |
| `description` | String | A description of this resource. |
| `etag` | String | Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `create_time` | String | Output only. Time when the ServiceConnectionToken was created. |
| `token` | String | Output only. The token generated by Automation. |
| `labels` | HashMap<String, String> | User-defined labels. |
| `update_time` | String | Output only. Time when the ServiceConnectionToken was updated. |
| `expire_time` | String | Output only. The time to which this token is valid. |
| `network` | String | The resource path of the network associated with this token. Example: projects/{projectNumOrId}/global/networks/{resourceId}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_connection_token
service_connection_token = provider.networkconnectivity_api.Service_connection_token {
    parent = "value"  # Required. The parent resource's name of the ServiceConnectionToken. ex. projects/123/locations/us-east1
}

# Access service_connection_token outputs
service_connection_token_id = service_connection_token.id
service_connection_token_name = service_connection_token.name
service_connection_token_description = service_connection_token.description
service_connection_token_etag = service_connection_token.etag
service_connection_token_create_time = service_connection_token.create_time
service_connection_token_token = service_connection_token.token
service_connection_token_labels = service_connection_token.labels
service_connection_token_update_time = service_connection_token.update_time
service_connection_token_expire_time = service_connection_token.expire_time
service_connection_token_network = service_connection_token.network
```

---


### Service_connection_policie

Creates a new ServiceConnectionPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `psc_connections` | Vec<String> |  | Output only. [Output only] Information about each Private Service Connect connection. |
| `auto_created_subnet_info` | String |  | Output only. Information for the automatically created subnetwork and its associated IR. |
| `labels` | HashMap<String, String> |  | User-defined labels. |
| `create_time` | String |  | Output only. Time when the ServiceConnectionPolicy was created. |
| `network` | String |  | The resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}. |
| `update_time` | String |  | Output only. Time when the ServiceConnectionPolicy was updated. |
| `name` | String |  | Immutable. The name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names |
| `service_class` | String |  | The service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass. It is provided by the Service Producer. Google services have a prefix of gcp or google-cloud. For example, gcp-memorystore-redis or google-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx. |
| `description` | String |  | A description of this resource. |
| `etag` | String |  | Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `infrastructure` | String |  | Output only. The type of underlying resources used to create the connection. |
| `psc_config` | String |  | Configuration used for Private Service Connect connections. Used when Infrastructure is PSC. |
| `parent` | String | ✅ | Required. The parent resource's name of the ServiceConnectionPolicy. ex. projects/123/locations/us-east1 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `psc_connections` | Vec<String> | Output only. [Output only] Information about each Private Service Connect connection. |
| `auto_created_subnet_info` | String | Output only. Information for the automatically created subnetwork and its associated IR. |
| `labels` | HashMap<String, String> | User-defined labels. |
| `create_time` | String | Output only. Time when the ServiceConnectionPolicy was created. |
| `network` | String | The resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}. |
| `update_time` | String | Output only. Time when the ServiceConnectionPolicy was updated. |
| `name` | String | Immutable. The name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names |
| `service_class` | String | The service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass. It is provided by the Service Producer. Google services have a prefix of gcp or google-cloud. For example, gcp-memorystore-redis or google-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx. |
| `description` | String | A description of this resource. |
| `etag` | String | Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `infrastructure` | String | Output only. The type of underlying resources used to create the connection. |
| `psc_config` | String | Configuration used for Private Service Connect connections. Used when Infrastructure is PSC. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_connection_policie
service_connection_policie = provider.networkconnectivity_api.Service_connection_policie {
    parent = "value"  # Required. The parent resource's name of the ServiceConnectionPolicy. ex. projects/123/locations/us-east1
}

# Access service_connection_policie outputs
service_connection_policie_id = service_connection_policie.id
service_connection_policie_psc_connections = service_connection_policie.psc_connections
service_connection_policie_auto_created_subnet_info = service_connection_policie.auto_created_subnet_info
service_connection_policie_labels = service_connection_policie.labels
service_connection_policie_create_time = service_connection_policie.create_time
service_connection_policie_network = service_connection_policie.network
service_connection_policie_update_time = service_connection_policie.update_time
service_connection_policie_name = service_connection_policie.name
service_connection_policie_service_class = service_connection_policie.service_class
service_connection_policie_description = service_connection_policie.description
service_connection_policie_etag = service_connection_policie.etag
service_connection_policie_infrastructure = service_connection_policie.infrastructure
service_connection_policie_psc_config = service_connection_policie.psc_config
```

---


### Route_table

Gets details about a Network Connectivity Center route table.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time the route table was last updated. |
| `create_time` | String | Output only. The time the route table was created. |
| `state` | String | Output only. The current lifecycle state of this route table. |
| `labels` | HashMap<String, String> | Optional labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements). |
| `name` | String | Immutable. The name of the route table. Route table names must be unique. They use the following form: `projects/{project_number}/locations/global/hubs/{hub}/routeTables/{route_table_id}` |
| `uid` | String | Output only. The Google-generated UUID for the route table. This value is unique across all route table resources. If a route table is deleted and another with the same name is created, the new route table is assigned a different `uid`. |
| `description` | String | An optional description of the route table. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access route_table outputs
route_table_id = route_table.id
route_table_update_time = route_table.update_time
route_table_create_time = route_table.create_time
route_table_state = route_table.state
route_table_labels = route_table.labels
route_table_name = route_table.name
route_table_uid = route_table.uid
route_table_description = route_table.description
```

---


### Service_connection_map

Creates a new ServiceConnectionMap in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when the ServiceConnectionMap was created. |
| `service_class_uri` | String |  | Output only. The service class uri this ServiceConnectionMap is for. |
| `infrastructure` | String |  | Output only. The infrastructure used for connections between consumers/producers. |
| `producer_psc_configs` | Vec<String> |  | The PSC configurations on producer side. |
| `service_class` | String |  | The service class identifier this ServiceConnectionMap is for. The user of ServiceConnectionMap create API needs to have networkconnectivity.serviceClasses.use IAM permission for the service class. |
| `token` | String |  | The token provided by the consumer. This token authenticates that the consumer can create a connection within the specified project and network. |
| `etag` | String |  | Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `description` | String |  | A description of this resource. |
| `name` | String |  | Immutable. The name of a ServiceConnectionMap. Format: projects/{project}/locations/{location}/serviceConnectionMaps/{service_connection_map} See: https://google.aip.dev/122#fields-representing-resource-names |
| `consumer_psc_configs` | Vec<String> |  | The PSC configurations on consumer side. |
| `labels` | HashMap<String, String> |  | User-defined labels. |
| `consumer_psc_connections` | Vec<String> |  | Output only. PSC connection details on consumer side. |
| `update_time` | String |  | Output only. Time when the ServiceConnectionMap was updated. |
| `parent` | String | ✅ | Required. The parent resource's name of the ServiceConnectionMap. ex. projects/123/locations/us-east1 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time when the ServiceConnectionMap was created. |
| `service_class_uri` | String | Output only. The service class uri this ServiceConnectionMap is for. |
| `infrastructure` | String | Output only. The infrastructure used for connections between consumers/producers. |
| `producer_psc_configs` | Vec<String> | The PSC configurations on producer side. |
| `service_class` | String | The service class identifier this ServiceConnectionMap is for. The user of ServiceConnectionMap create API needs to have networkconnectivity.serviceClasses.use IAM permission for the service class. |
| `token` | String | The token provided by the consumer. This token authenticates that the consumer can create a connection within the specified project and network. |
| `etag` | String | Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `description` | String | A description of this resource. |
| `name` | String | Immutable. The name of a ServiceConnectionMap. Format: projects/{project}/locations/{location}/serviceConnectionMaps/{service_connection_map} See: https://google.aip.dev/122#fields-representing-resource-names |
| `consumer_psc_configs` | Vec<String> | The PSC configurations on consumer side. |
| `labels` | HashMap<String, String> | User-defined labels. |
| `consumer_psc_connections` | Vec<String> | Output only. PSC connection details on consumer side. |
| `update_time` | String | Output only. Time when the ServiceConnectionMap was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_connection_map
service_connection_map = provider.networkconnectivity_api.Service_connection_map {
    parent = "value"  # Required. The parent resource's name of the ServiceConnectionMap. ex. projects/123/locations/us-east1
}

# Access service_connection_map outputs
service_connection_map_id = service_connection_map.id
service_connection_map_create_time = service_connection_map.create_time
service_connection_map_service_class_uri = service_connection_map.service_class_uri
service_connection_map_infrastructure = service_connection_map.infrastructure
service_connection_map_producer_psc_configs = service_connection_map.producer_psc_configs
service_connection_map_service_class = service_connection_map.service_class
service_connection_map_token = service_connection_map.token
service_connection_map_etag = service_connection_map.etag
service_connection_map_description = service_connection_map.description
service_connection_map_name = service_connection_map.name
service_connection_map_consumer_psc_configs = service_connection_map.consumer_psc_configs
service_connection_map_labels = service_connection_map.labels
service_connection_map_consumer_psc_connections = service_connection_map.consumer_psc_connections
service_connection_map_update_time = service_connection_map.update_time
```

---


### Hub

Creates a new Network Connectivity Center hub in the specified project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The time the hub was last updated. |
| `create_time` | String |  | Output only. The time the hub was created. |
| `export_psc` | bool |  | Optional. Whether Private Service Connect connection propagation is enabled for the hub. If true, Private Service Connect endpoints in VPC spokes attached to the hub are made accessible to other VPC spokes attached to the hub. The default value is false. |
| `labels` | HashMap<String, String> |  | Optional labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements). |
| `policy_mode` | String |  | Optional. The policy mode of this hub. This field can be either PRESET or CUSTOM. If unspecified, the policy_mode defaults to PRESET. |
| `routing_vpcs` | Vec<String> |  | Output only. The VPC networks associated with this hub's spokes. This field is read-only. Network Connectivity Center automatically populates it based on the set of spokes attached to the hub. |
| `spoke_summary` | String |  | Output only. A summary of the spokes associated with a hub. The summary includes a count of spokes according to type and according to state. If any spokes are inactive, the summary also lists the reasons they are inactive, including a count for each reason. |
| `description` | String |  | Optional. An optional description of the hub. |
| `preset_topology` | String |  | Optional. The topology implemented in this hub. Currently, this field is only used when policy_mode = PRESET. The available preset topologies are MESH and STAR. If preset_topology is unspecified and policy_mode = PRESET, the preset_topology defaults to MESH. When policy_mode = CUSTOM, the preset_topology is set to PRESET_TOPOLOGY_UNSPECIFIED. |
| `route_tables` | Vec<String> |  | Output only. The route tables that belong to this hub. They use the following form: `projects/{project_number}/locations/global/hubs/{hub_id}/routeTables/{route_table_id}` This field is read-only. Network Connectivity Center automatically populates it based on the route tables nested under the hub. |
| `state` | String |  | Output only. The current lifecycle state of this hub. |
| `name` | String |  | Immutable. The name of the hub. Hub names must be unique. They use the following form: `projects/{project_number}/locations/global/hubs/{hub_id}` |
| `unique_id` | String |  | Output only. The Google-generated UUID for the hub. This value is unique across all hub resources. If a hub is deleted and another with the same name is created, the new hub is assigned a different unique_id. |
| `parent` | String | ✅ | Required. The parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time the hub was last updated. |
| `create_time` | String | Output only. The time the hub was created. |
| `export_psc` | bool | Optional. Whether Private Service Connect connection propagation is enabled for the hub. If true, Private Service Connect endpoints in VPC spokes attached to the hub are made accessible to other VPC spokes attached to the hub. The default value is false. |
| `labels` | HashMap<String, String> | Optional labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements). |
| `policy_mode` | String | Optional. The policy mode of this hub. This field can be either PRESET or CUSTOM. If unspecified, the policy_mode defaults to PRESET. |
| `routing_vpcs` | Vec<String> | Output only. The VPC networks associated with this hub's spokes. This field is read-only. Network Connectivity Center automatically populates it based on the set of spokes attached to the hub. |
| `spoke_summary` | String | Output only. A summary of the spokes associated with a hub. The summary includes a count of spokes according to type and according to state. If any spokes are inactive, the summary also lists the reasons they are inactive, including a count for each reason. |
| `description` | String | Optional. An optional description of the hub. |
| `preset_topology` | String | Optional. The topology implemented in this hub. Currently, this field is only used when policy_mode = PRESET. The available preset topologies are MESH and STAR. If preset_topology is unspecified and policy_mode = PRESET, the preset_topology defaults to MESH. When policy_mode = CUSTOM, the preset_topology is set to PRESET_TOPOLOGY_UNSPECIFIED. |
| `route_tables` | Vec<String> | Output only. The route tables that belong to this hub. They use the following form: `projects/{project_number}/locations/global/hubs/{hub_id}/routeTables/{route_table_id}` This field is read-only. Network Connectivity Center automatically populates it based on the route tables nested under the hub. |
| `state` | String | Output only. The current lifecycle state of this hub. |
| `name` | String | Immutable. The name of the hub. Hub names must be unique. They use the following form: `projects/{project_number}/locations/global/hubs/{hub_id}` |
| `unique_id` | String | Output only. The Google-generated UUID for the hub. This value is unique across all hub resources. If a hub is deleted and another with the same name is created, the new hub is assigned a different unique_id. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hub
hub = provider.networkconnectivity_api.Hub {
    parent = "value"  # Required. The parent resource.
}

# Access hub outputs
hub_id = hub.id
hub_update_time = hub.update_time
hub_create_time = hub.create_time
hub_export_psc = hub.export_psc
hub_labels = hub.labels
hub_policy_mode = hub.policy_mode
hub_routing_vpcs = hub.routing_vpcs
hub_spoke_summary = hub.spoke_summary
hub_description = hub.description
hub_preset_topology = hub.preset_topology
hub_route_tables = hub.route_tables
hub_state = hub.state
hub_name = hub.name
hub_unique_id = hub.unique_id
```

---


### Policy_based_route

Creates a new policy-based route in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | User-defined labels. |
| `interconnect_attachment` | String |  | Optional. The interconnect attachments that this policy-based route applies to. |
| `next_hop_other_routes` | String |  | Optional. Other routes that will be referenced to determine the next hop of the packet. |
| `self_link` | String |  | Output only. Server-defined fully-qualified URL for this resource. |
| `priority` | i64 |  | Optional. The priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive. |
| `create_time` | String |  | Output only. Time when the policy-based route was created. |
| `next_hop_ilb_ip` | String |  | Optional. The IP address of a global-access-enabled L4 ILB that is the next hop for matching packets. For this version, only nextHopIlbIp is supported. |
| `name` | String |  | Immutable. A unique name of the resource in the form of `projects/{project_number}/locations/global/PolicyBasedRoutes/{policy_based_route_id}` |
| `virtual_machine` | String |  | Optional. VM instances that this policy-based route applies to. |
| `kind` | String |  | Output only. Type of this resource. Always networkconnectivity#policyBasedRoute for policy-based Route resources. |
| `network` | String |  | Required. Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network. |
| `description` | String |  | Optional. An optional description of this resource. Provide this field when you create the resource. |
| `filter` | String |  | Required. The filter to match L4 traffic. |
| `update_time` | String |  | Output only. Time when the policy-based route was updated. |
| `warnings` | Vec<String> |  | Output only. If potential misconfigurations are detected for this route, this field will be populated with warning messages. |
| `parent` | String | ✅ | Required. The parent resource's name of the PolicyBasedRoute. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | User-defined labels. |
| `interconnect_attachment` | String | Optional. The interconnect attachments that this policy-based route applies to. |
| `next_hop_other_routes` | String | Optional. Other routes that will be referenced to determine the next hop of the packet. |
| `self_link` | String | Output only. Server-defined fully-qualified URL for this resource. |
| `priority` | i64 | Optional. The priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive. |
| `create_time` | String | Output only. Time when the policy-based route was created. |
| `next_hop_ilb_ip` | String | Optional. The IP address of a global-access-enabled L4 ILB that is the next hop for matching packets. For this version, only nextHopIlbIp is supported. |
| `name` | String | Immutable. A unique name of the resource in the form of `projects/{project_number}/locations/global/PolicyBasedRoutes/{policy_based_route_id}` |
| `virtual_machine` | String | Optional. VM instances that this policy-based route applies to. |
| `kind` | String | Output only. Type of this resource. Always networkconnectivity#policyBasedRoute for policy-based Route resources. |
| `network` | String | Required. Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network. |
| `description` | String | Optional. An optional description of this resource. Provide this field when you create the resource. |
| `filter` | String | Required. The filter to match L4 traffic. |
| `update_time` | String | Output only. Time when the policy-based route was updated. |
| `warnings` | Vec<String> | Output only. If potential misconfigurations are detected for this route, this field will be populated with warning messages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policy_based_route
policy_based_route = provider.networkconnectivity_api.Policy_based_route {
    parent = "value"  # Required. The parent resource's name of the PolicyBasedRoute.
}

# Access policy_based_route outputs
policy_based_route_id = policy_based_route.id
policy_based_route_labels = policy_based_route.labels
policy_based_route_interconnect_attachment = policy_based_route.interconnect_attachment
policy_based_route_next_hop_other_routes = policy_based_route.next_hop_other_routes
policy_based_route_self_link = policy_based_route.self_link
policy_based_route_priority = policy_based_route.priority
policy_based_route_create_time = policy_based_route.create_time
policy_based_route_next_hop_ilb_ip = policy_based_route.next_hop_ilb_ip
policy_based_route_name = policy_based_route.name
policy_based_route_virtual_machine = policy_based_route.virtual_machine
policy_based_route_kind = policy_based_route.kind
policy_based_route_network = policy_based_route.network
policy_based_route_description = policy_based_route.description
policy_based_route_filter = policy_based_route.filter
policy_based_route_update_time = policy_based_route.update_time
policy_based_route_warnings = policy_based_route.warnings
```

---


### Group

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_accept` | String | Optional. The auto-accept setting for this group. |
| `labels` | HashMap<String, String> | Optional. Labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements). |
| `state` | String | Output only. The current lifecycle state of this group. |
| `name` | String | Immutable. The name of the group. Group names must be unique. They use the following form: `projects/{project_number}/locations/global/hubs/{hub}/groups/{group_id}` |
| `description` | String | Optional. The description of the group. |
| `route_table` | String | Output only. The name of the route table that corresponds to this group. They use the following form: `projects/{project_number}/locations/global/hubs/{hub_id}/routeTables/{route_table_id}` |
| `create_time` | String | Output only. The time the group was created. |
| `update_time` | String | Output only. The time the group was last updated. |
| `uid` | String | Output only. The Google-generated UUID for the group. This value is unique across all group resources. If a group is deleted and another with the same name is created, the new route table is assigned a different unique_id. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.networkconnectivity_api.Group {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access group outputs
group_id = group.id
group_auto_accept = group.auto_accept
group_labels = group.labels
group_state = group.state
group_name = group.name
group_description = group.description
group_route_table = group.route_table
group_create_time = group.create_time
group_update_time = group.update_time
group_uid = group.uid
```

---


### Location

CheckConsumerConfig validates the consumer network and project for potential PSC connection creation. This method performs several checks, including: - Validating the existence and permissions of the service class. - Ensuring the consumer network exists and is accessible. - Verifying XPN relationships if applicable. - Checking for compatible IP versions between the consumer network and the requested version. This method performs a dynamic IAM check for the `networkconnectivity.serviceClasses.use` permission on the service class resource in the Prepare phase.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consumer_network` | String |  | Required. Full resource name of the consumer network. Example: - projects/{project}/global/networks/{network}. |
| `requested_ip_version` | String |  | The requested IP Version |
| `service_class` | String |  | Required. The service class identifier of the producer. |
| `endpoint_project` | String |  | The project number or ID where the PSC endpoint is to be created. |
| `location` | String | ✅ | Required. The location resource path. Example: - projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.networkconnectivity_api.Location {
    location = "value"  # Required. The location resource path. Example: - projects/{project}/locations/{location}
}

# Access location outputs
location_id = location.id
location_metadata = location.metadata
location_display_name = location.display_name
location_location_id = location.location_id
location_name = location.name
location_labels = location.labels
```

---


### Route

Gets details about the specified route.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The name of the route. Route names must be unique. Route names use the following form: `projects/{project_number}/locations/global/hubs/{hub}/routeTables/{route_table_id}/routes/{route_id}` |
| `ip_cidr_range` | String | The destination IP address range. |
| `update_time` | String | Output only. The time the route was last updated. |
| `uid` | String | Output only. The Google-generated UUID for the route. This value is unique across all Network Connectivity Center route resources. If a route is deleted and another with the same name is created, the new route is assigned a different `uid`. |
| `next_hop_vpn_tunnel` | String | Immutable. The next-hop VPN tunnel for packets on this route. |
| `priority` | String | Output only. The priority of this route. Priority is used to break ties in cases where a destination matches more than one route. In these cases the route with the lowest-numbered priority value wins. |
| `type` | String | Output only. The route's type. Its type is determined by the properties of its IP address range. |
| `next_hop_vpc_network` | String | Immutable. The destination VPC network for packets on this route. |
| `next_hop_interconnect_attachment` | String | Immutable. The next-hop VLAN attachment for packets on this route. |
| `state` | String | Output only. The current lifecycle state of the route. |
| `next_hop_spoke` | String | Immutable. The next-hop spoke for packets on this route. |
| `labels` | HashMap<String, String> | Optional labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements). |
| `spoke` | String | Immutable. The spoke that this route leads to. Example: projects/12345/locations/global/spokes/SPOKE |
| `next_hop_router_appliance_instance` | String | Immutable. The next-hop Router appliance instance for packets on this route. |
| `description` | String | An optional description of the route. |
| `location` | String | Output only. The origin location of the route. Uses the following form: "projects/{project}/locations/{location}" Example: projects/1234/locations/us-central1 |
| `create_time` | String | Output only. The time the route was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access route outputs
route_id = route.id
route_name = route.name
route_ip_cidr_range = route.ip_cidr_range
route_update_time = route.update_time
route_uid = route.uid
route_next_hop_vpn_tunnel = route.next_hop_vpn_tunnel
route_priority = route.priority
route_type = route.type
route_next_hop_vpc_network = route.next_hop_vpc_network
route_next_hop_interconnect_attachment = route.next_hop_interconnect_attachment
route_state = route.state
route_next_hop_spoke = route.next_hop_spoke
route_labels = route.labels
route_spoke = route.spoke
route_next_hop_router_appliance_instance = route.next_hop_router_appliance_instance
route_description = route.description
route_location = route.location
route_create_time = route.create_time
```

---


### Service_classe

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The name of a ServiceClass resource. Format: projects/{project}/locations/{location}/serviceClasses/{service_class} See: https://google.aip.dev/122#fields-representing-resource-names |
| `description` | String | A description of this resource. |
| `create_time` | String | Output only. Time when the ServiceClass was created. |
| `etag` | String | Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `service_class` | String | Output only. The generated service class name. Use this name to refer to the Service class in Service Connection Maps and Service Connection Policies. |
| `labels` | HashMap<String, String> | User-defined labels. |
| `update_time` | String | Output only. Time when the ServiceClass was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_classe
service_classe = provider.networkconnectivity_api.Service_classe {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access service_classe outputs
service_classe_id = service_classe.id
service_classe_name = service_classe.name
service_classe_description = service_classe.description
service_classe_create_time = service_classe.create_time
service_classe_etag = service_classe.etag
service_classe_service_class = service_classe.service_class
service_classe_labels = service_classe.labels
service_classe_update_time = service_classe.update_time
```

---


### Destination

Creates a `Destination` resource in a specified project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. The Google-generated unique ID for the `Destination` resource. This value is unique across all `Destination` resources. If a resource is deleted and another with the same name is created, the new resource is assigned a different and unique ID. |
| `create_time` | String |  | Output only. Time when the `Destination` resource was created. |
| `name` | String |  | Identifier. The name of the `Destination` resource. Format: `projects/{project}/locations/{location}/multicloudDataTransferConfigs/{multicloud_data_transfer_config}/destinations/{destination}`. |
| `state_timeline` | String |  | Output only. The timeline of the expected `Destination` states or the current rest state. If a state change is expected, the value is `ADDING`, `DELETING` or `SUSPENDING`, depending on the action specified. Example: "state_timeline": { "states": [ { // The time when the `Destination` resource will be activated. "effectiveTime": "2024-12-01T08:00:00Z", "state": "ADDING" }, { // The time when the `Destination` resource will be suspended. "effectiveTime": "2024-12-01T20:00:00Z", "state": "SUSPENDING" } ] } |
| `description` | String |  | Optional. A description of this resource. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels. |
| `update_time` | String |  | Output only. Time when the `Destination` resource was updated. |
| `endpoints` | Vec<String> |  | Required. Unordered list. The list of `DestinationEndpoint` resources configured for the IP prefix. |
| `ip_prefix` | String |  | Required. Immutable. The IP prefix that represents your workload on another CSP. |
| `etag` | String |  | The etag is computed by the server, and might be sent with update and delete requests so that the client has an up-to-date value before proceeding. |
| `parent` | String | ✅ | Required. The name of the parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. The Google-generated unique ID for the `Destination` resource. This value is unique across all `Destination` resources. If a resource is deleted and another with the same name is created, the new resource is assigned a different and unique ID. |
| `create_time` | String | Output only. Time when the `Destination` resource was created. |
| `name` | String | Identifier. The name of the `Destination` resource. Format: `projects/{project}/locations/{location}/multicloudDataTransferConfigs/{multicloud_data_transfer_config}/destinations/{destination}`. |
| `state_timeline` | String | Output only. The timeline of the expected `Destination` states or the current rest state. If a state change is expected, the value is `ADDING`, `DELETING` or `SUSPENDING`, depending on the action specified. Example: "state_timeline": { "states": [ { // The time when the `Destination` resource will be activated. "effectiveTime": "2024-12-01T08:00:00Z", "state": "ADDING" }, { // The time when the `Destination` resource will be suspended. "effectiveTime": "2024-12-01T20:00:00Z", "state": "SUSPENDING" } ] } |
| `description` | String | Optional. A description of this resource. |
| `labels` | HashMap<String, String> | Optional. User-defined labels. |
| `update_time` | String | Output only. Time when the `Destination` resource was updated. |
| `endpoints` | Vec<String> | Required. Unordered list. The list of `DestinationEndpoint` resources configured for the IP prefix. |
| `ip_prefix` | String | Required. Immutable. The IP prefix that represents your workload on another CSP. |
| `etag` | String | The etag is computed by the server, and might be sent with update and delete requests so that the client has an up-to-date value before proceeding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create destination
destination = provider.networkconnectivity_api.Destination {
    parent = "value"  # Required. The name of the parent resource.
}

# Access destination outputs
destination_id = destination.id
destination_uid = destination.uid
destination_create_time = destination.create_time
destination_name = destination.name
destination_state_timeline = destination.state_timeline
destination_description = destination.description
destination_labels = destination.labels
destination_update_time = destination.update_time
destination_endpoints = destination.endpoints
destination_ip_prefix = destination.ip_prefix
destination_etag = destination.etag
```

---


### Multicloud_data_transfer_supported_service

Gets the details of a service that is supported for Data Transfer Essentials.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_configs` | Vec<String> | Output only. The network service tier or regional endpoint supported for the service. |
| `name` | String | Identifier. The name of the service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access multicloud_data_transfer_supported_service outputs
multicloud_data_transfer_supported_service_id = multicloud_data_transfer_supported_service.id
multicloud_data_transfer_supported_service_service_configs = multicloud_data_transfer_supported_service.service_configs
multicloud_data_transfer_supported_service_name = multicloud_data_transfer_supported_service.name
```

---


### Spoke

Creates a Network Connectivity Center spoke.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `unique_id` | String |  | Output only. The Google-generated UUID for the spoke. This value is unique across all spoke resources. If a spoke is deleted and another with the same name is created, the new spoke is assigned a different `unique_id`. |
| `linked_interconnect_attachments` | String |  | Optional. VLAN attachments that are associated with the spoke. |
| `update_time` | String |  | Output only. The time the spoke was last updated. |
| `linked_vpc_network` | String |  | Optional. VPC network that is associated with the spoke. |
| `linked_vpn_tunnels` | String |  | Optional. VPN tunnels that are associated with the spoke. |
| `labels` | HashMap<String, String> |  | Optional labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements). |
| `group` | String |  | Optional. The name of the group that this spoke is associated with. |
| `name` | String |  | Immutable. The name of the spoke. Spoke names must be unique. They use the following form: `projects/{project_number}/locations/{region}/spokes/{spoke_id}` |
| `description` | String |  | Optional. An optional description of the spoke. |
| `hub` | String |  | Immutable. The name of the hub that this spoke is attached to. |
| `reasons` | Vec<String> |  | Output only. The reasons for current state of the spoke. |
| `state` | String |  | Output only. The current lifecycle state of this spoke. |
| `field_paths_pending_update` | Vec<String> |  | Optional. The list of fields waiting for hub administration's approval. |
| `etag` | String |  | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `linked_router_appliance_instances` | String |  | Optional. Router appliance instances that are associated with the spoke. |
| `create_time` | String |  | Output only. The time the spoke was created. |
| `linked_producer_vpc_network` | String |  | Optional. The linked producer VPC that is associated with the spoke. |
| `spoke_type` | String |  | Output only. The type of resource associated with the spoke. |
| `parent` | String | ✅ | Required. The parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unique_id` | String | Output only. The Google-generated UUID for the spoke. This value is unique across all spoke resources. If a spoke is deleted and another with the same name is created, the new spoke is assigned a different `unique_id`. |
| `linked_interconnect_attachments` | String | Optional. VLAN attachments that are associated with the spoke. |
| `update_time` | String | Output only. The time the spoke was last updated. |
| `linked_vpc_network` | String | Optional. VPC network that is associated with the spoke. |
| `linked_vpn_tunnels` | String | Optional. VPN tunnels that are associated with the spoke. |
| `labels` | HashMap<String, String> | Optional labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements). |
| `group` | String | Optional. The name of the group that this spoke is associated with. |
| `name` | String | Immutable. The name of the spoke. Spoke names must be unique. They use the following form: `projects/{project_number}/locations/{region}/spokes/{spoke_id}` |
| `description` | String | Optional. An optional description of the spoke. |
| `hub` | String | Immutable. The name of the hub that this spoke is attached to. |
| `reasons` | Vec<String> | Output only. The reasons for current state of the spoke. |
| `state` | String | Output only. The current lifecycle state of this spoke. |
| `field_paths_pending_update` | Vec<String> | Optional. The list of fields waiting for hub administration's approval. |
| `etag` | String | Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `linked_router_appliance_instances` | String | Optional. Router appliance instances that are associated with the spoke. |
| `create_time` | String | Output only. The time the spoke was created. |
| `linked_producer_vpc_network` | String | Optional. The linked producer VPC that is associated with the spoke. |
| `spoke_type` | String | Output only. The type of resource associated with the spoke. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create spoke
spoke = provider.networkconnectivity_api.Spoke {
    parent = "value"  # Required. The parent resource.
}

# Access spoke outputs
spoke_id = spoke.id
spoke_unique_id = spoke.unique_id
spoke_linked_interconnect_attachments = spoke.linked_interconnect_attachments
spoke_update_time = spoke.update_time
spoke_linked_vpc_network = spoke.linked_vpc_network
spoke_linked_vpn_tunnels = spoke.linked_vpn_tunnels
spoke_labels = spoke.labels
spoke_group = spoke.group
spoke_name = spoke.name
spoke_description = spoke.description
spoke_hub = spoke.hub
spoke_reasons = spoke.reasons
spoke_state = spoke.state
spoke_field_paths_pending_update = spoke.field_paths_pending_update
spoke_etag = spoke.etag
spoke_linked_router_appliance_instances = spoke.linked_router_appliance_instances
spoke_create_time = spoke.create_time
spoke_linked_producer_vpc_network = spoke.linked_producer_vpc_network
spoke_spoke_type = spoke.spoke_type
```

---


### Internal_range

Creates a new internal range in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | A description of this resource. |
| `network` | String |  | The URL or resource ID of the network in which to reserve the internal range. The network cannot be deleted if there are any reserved internal ranges referring to it. Legacy networks are not supported. For example: https://www.googleapis.com/compute/v1/projects/{project}/locations/global/networks/{network} projects/{project}/locations/global/networks/{network} {network} |
| `ip_cidr_range` | String |  | IP range that this internal range defines. NOTE: IPv6 ranges are limited to usage=EXTERNAL_TO_VPC and peering=FOR_SELF. NOTE: For IPv6 Ranges this field is compulsory, i.e. the address range must be specified explicitly. |
| `update_time` | String |  | Time when the internal range was updated. |
| `name` | String |  | Immutable. The name of an internal range. Format: projects/{project}/locations/{location}/internalRanges/{internal_range} See: https://google.aip.dev/122#fields-representing-resource-names |
| `immutable` | bool |  | Optional. Immutable ranges cannot have their fields modified, except for labels and description. |
| `overlaps` | Vec<String> |  | Optional. Types of resources that are allowed to overlap with the current internal range. |
| `labels` | HashMap<String, String> |  | User-defined labels. |
| `migration` | String |  | Optional. Must be present if usage is set to FOR_MIGRATION. |
| `create_time` | String |  | Time when the internal range was created. |
| `exclude_cidr_ranges` | Vec<String> |  | Optional. ExcludeCidrRanges flag. Specifies a set of CIDR blocks that allows exclusion of particular CIDR ranges from the auto-allocation process, without having to reserve these blocks |
| `usage` | String |  | The type of usage set for this internal range. |
| `allocation_options` | String |  | Optional. Range auto-allocation options, may be set only when auto-allocation is selected by not setting ip_cidr_range (and setting prefix_length). |
| `target_cidr_range` | Vec<String> |  | Optional. Can be set to narrow down or pick a different address space while searching for a free range. If not set, defaults to the ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16"] address space (for auto-mode networks, the "10.0.0.0/9" range is used instead of "10.0.0.0/8"). This can be used to target the search in other rfc-1918 address spaces like "172.16.0.0/12" and "192.168.0.0/16" or non-rfc-1918 address spaces used in the VPC. |
| `peering` | String |  | The type of peering set for this internal range. |
| `users` | Vec<String> |  | Output only. The list of resources that refer to this internal range. Resources that use the internal range for their range allocation are referred to as users of the range. Other resources mark themselves as users while doing so by creating a reference to this internal range. Having a user, based on this reference, prevents deletion of the internal range that is referred to. Can be empty. |
| `prefix_length` | i64 |  | An alternative to ip_cidr_range. Can be set when trying to create an IPv4 reservation that automatically finds a free range of the given size. If both ip_cidr_range and prefix_length are set, there is an error if the range sizes do not match. Can also be used during updates to change the range size. NOTE: For IPv6 this field only works if ip_cidr_range is set as well, and both fields must match. In other words, with IPv6 this field only works as a redundant parameter. |
| `parent` | String | ✅ | Required. The parent resource's name of the InternalRange. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | A description of this resource. |
| `network` | String | The URL or resource ID of the network in which to reserve the internal range. The network cannot be deleted if there are any reserved internal ranges referring to it. Legacy networks are not supported. For example: https://www.googleapis.com/compute/v1/projects/{project}/locations/global/networks/{network} projects/{project}/locations/global/networks/{network} {network} |
| `ip_cidr_range` | String | IP range that this internal range defines. NOTE: IPv6 ranges are limited to usage=EXTERNAL_TO_VPC and peering=FOR_SELF. NOTE: For IPv6 Ranges this field is compulsory, i.e. the address range must be specified explicitly. |
| `update_time` | String | Time when the internal range was updated. |
| `name` | String | Immutable. The name of an internal range. Format: projects/{project}/locations/{location}/internalRanges/{internal_range} See: https://google.aip.dev/122#fields-representing-resource-names |
| `immutable` | bool | Optional. Immutable ranges cannot have their fields modified, except for labels and description. |
| `overlaps` | Vec<String> | Optional. Types of resources that are allowed to overlap with the current internal range. |
| `labels` | HashMap<String, String> | User-defined labels. |
| `migration` | String | Optional. Must be present if usage is set to FOR_MIGRATION. |
| `create_time` | String | Time when the internal range was created. |
| `exclude_cidr_ranges` | Vec<String> | Optional. ExcludeCidrRanges flag. Specifies a set of CIDR blocks that allows exclusion of particular CIDR ranges from the auto-allocation process, without having to reserve these blocks |
| `usage` | String | The type of usage set for this internal range. |
| `allocation_options` | String | Optional. Range auto-allocation options, may be set only when auto-allocation is selected by not setting ip_cidr_range (and setting prefix_length). |
| `target_cidr_range` | Vec<String> | Optional. Can be set to narrow down or pick a different address space while searching for a free range. If not set, defaults to the ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16"] address space (for auto-mode networks, the "10.0.0.0/9" range is used instead of "10.0.0.0/8"). This can be used to target the search in other rfc-1918 address spaces like "172.16.0.0/12" and "192.168.0.0/16" or non-rfc-1918 address spaces used in the VPC. |
| `peering` | String | The type of peering set for this internal range. |
| `users` | Vec<String> | Output only. The list of resources that refer to this internal range. Resources that use the internal range for their range allocation are referred to as users of the range. Other resources mark themselves as users while doing so by creating a reference to this internal range. Having a user, based on this reference, prevents deletion of the internal range that is referred to. Can be empty. |
| `prefix_length` | i64 | An alternative to ip_cidr_range. Can be set when trying to create an IPv4 reservation that automatically finds a free range of the given size. If both ip_cidr_range and prefix_length are set, there is an error if the range sizes do not match. Can also be used during updates to change the range size. NOTE: For IPv6 this field only works if ip_cidr_range is set as well, and both fields must match. In other words, with IPv6 this field only works as a redundant parameter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create internal_range
internal_range = provider.networkconnectivity_api.Internal_range {
    parent = "value"  # Required. The parent resource's name of the InternalRange.
}

# Access internal_range outputs
internal_range_id = internal_range.id
internal_range_description = internal_range.description
internal_range_network = internal_range.network
internal_range_ip_cidr_range = internal_range.ip_cidr_range
internal_range_update_time = internal_range.update_time
internal_range_name = internal_range.name
internal_range_immutable = internal_range.immutable
internal_range_overlaps = internal_range.overlaps
internal_range_labels = internal_range.labels
internal_range_migration = internal_range.migration
internal_range_create_time = internal_range.create_time
internal_range_exclude_cidr_ranges = internal_range.exclude_cidr_ranges
internal_range_usage = internal_range.usage
internal_range_allocation_options = internal_range.allocation_options
internal_range_target_cidr_range = internal_range.target_cidr_range
internal_range_peering = internal_range.peering
internal_range_users = internal_range.users
internal_range_prefix_length = internal_range.prefix_length
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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

# Create operation
operation = provider.networkconnectivity_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_metadata = operation.metadata
operation_done = operation.done
operation_response = operation.response
operation_error = operation.error
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_metadata = location.metadata
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
location_name = location.name
```

---


### Hub

Creates a new Network Connectivity Center hub in the specified project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Time when the Hub was created. |
| `labels` | HashMap<String, String> |  | User-defined labels. |
| `spokes` | Vec<String> |  | Output only. A list of the URIs of all attached spokes. This field is deprecated and will not be included in future API versions. Call ListSpokes on each region instead. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Hub resources. If a Hub resource is deleted and another with the same name is created, it gets a different unique_id. |
| `update_time` | String |  | Time when the Hub was updated. |
| `state` | String |  | Output only. The current lifecycle state of this Hub. |
| `description` | String |  | Short description of the hub resource. |
| `name` | String |  | Immutable. The name of a Hub resource. |
| `parent` | String | ✅ | Required. The parent resource's name of the Hub. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Time when the Hub was created. |
| `labels` | HashMap<String, String> | User-defined labels. |
| `spokes` | Vec<String> | Output only. A list of the URIs of all attached spokes. This field is deprecated and will not be included in future API versions. Call ListSpokes on each region instead. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Hub resources. If a Hub resource is deleted and another with the same name is created, it gets a different unique_id. |
| `update_time` | String | Time when the Hub was updated. |
| `state` | String | Output only. The current lifecycle state of this Hub. |
| `description` | String | Short description of the hub resource. |
| `name` | String | Immutable. The name of a Hub resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hub
hub = provider.networkconnectivity_api.Hub {
    parent = "value"  # Required. The parent resource's name of the Hub.
}

# Access hub outputs
hub_id = hub.id
hub_create_time = hub.create_time
hub_labels = hub.labels
hub_spokes = hub.spokes
hub_unique_id = hub.unique_id
hub_update_time = hub.update_time
hub_state = hub.state
hub_description = hub.description
hub_name = hub.name
```

---


### Spoke

Creates a Network Connectivity Center spoke.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The current lifecycle state of this Hub. |
| `hub` | String |  | The resource URL of the hub resource that the spoke is attached to |
| `description` | String |  | Short description of the spoke resource |
| `linked_vpn_tunnels` | Vec<String> |  | The URIs of linked VPN tunnel resources |
| `linked_router_appliance_instances` | Vec<String> |  | The URIs of linked Router appliance resources |
| `labels` | HashMap<String, String> |  | User-defined labels. |
| `linked_interconnect_attachments` | Vec<String> |  | The URIs of linked interconnect attachment resources |
| `create_time` | String |  | The time when the Spoke was created. |
| `name` | String |  | Immutable. The name of a Spoke resource. |
| `update_time` | String |  | The time when the Spoke was updated. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Spoke resources. If a Spoke resource is deleted and another with the same name is created, it gets a different unique_id. |
| `parent` | String | ✅ | Required. The parent's resource name of the Spoke. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current lifecycle state of this Hub. |
| `hub` | String | The resource URL of the hub resource that the spoke is attached to |
| `description` | String | Short description of the spoke resource |
| `linked_vpn_tunnels` | Vec<String> | The URIs of linked VPN tunnel resources |
| `linked_router_appliance_instances` | Vec<String> | The URIs of linked Router appliance resources |
| `labels` | HashMap<String, String> | User-defined labels. |
| `linked_interconnect_attachments` | Vec<String> | The URIs of linked interconnect attachment resources |
| `create_time` | String | The time when the Spoke was created. |
| `name` | String | Immutable. The name of a Spoke resource. |
| `update_time` | String | The time when the Spoke was updated. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Spoke resources. If a Spoke resource is deleted and another with the same name is created, it gets a different unique_id. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create spoke
spoke = provider.networkconnectivity_api.Spoke {
    parent = "value"  # Required. The parent's resource name of the Spoke.
}

# Access spoke outputs
spoke_id = spoke.id
spoke_state = spoke.state
spoke_hub = spoke.hub
spoke_description = spoke.description
spoke_linked_vpn_tunnels = spoke.linked_vpn_tunnels
spoke_linked_router_appliance_instances = spoke.linked_router_appliance_instances
spoke_labels = spoke.labels
spoke_linked_interconnect_attachments = spoke.linked_interconnect_attachments
spoke_create_time = spoke.create_time
spoke_name = spoke.name
spoke_update_time = spoke.update_time
spoke_unique_id = spoke.unique_id
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
operation_0 = provider.networkconnectivity_api.Operation {
    name = "value-0"
}
operation_1 = provider.networkconnectivity_api.Operation {
    name = "value-1"
}
operation_2 = provider.networkconnectivity_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.networkconnectivity_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Networkconnectivity_api Documentation](https://cloud.google.com/networkconnectivity_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
