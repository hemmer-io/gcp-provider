# Vmwareengine_api Service



**Resources**: 20

---

## Overview

The vmwareengine_api service provides access to 20 resource types:

- [Private_connection](#private_connection) [CRUD]
- [Private_cloud](#private_cloud) [CRUD]
- [Cluster](#cluster) [CRUD]
- [External_access_rule](#external_access_rule) [CRUD]
- [Dns_bind_permission](#dns_bind_permission) [C]
- [Location](#location) [R]
- [Node](#node) [R]
- [Announcement](#announcement) [R]
- [Node_type](#node_type) [R]
- [Upgrade](#upgrade) [RU]
- [Network_peering](#network_peering) [CRUD]
- [Hcx_activation_key](#hcx_activation_key) [CR]
- [Management_dns_zone_binding](#management_dns_zone_binding) [CRUD]
- [Network_policie](#network_policie) [CRUD]
- [Logging_server](#logging_server) [CRUD]
- [Peering_route](#peering_route) [R]
- [External_addresse](#external_addresse) [CRUD]
- [Vmware_engine_network](#vmware_engine_network) [CRUD]
- [Operation](#operation) [RD]
- [Subnet](#subnet) [RU]

---

## Resources


### Private_connection

Creates a new private connection that can be used for accessing private Clouds.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `peering_id` | String |  | Output only. VPC network peering id between given network VPC and VMwareEngineNetwork. |
| `peering_state` | String |  | Output only. Peering state between service network and VMware Engine network. |
| `update_time` | String |  | Output only. Last update time of this resource. |
| `type` | String |  | Required. Private connection type. |
| `vmware_engine_network` | String |  | Required. The relative resource name of Legacy VMware Engine network. Specify the name in the following form: `projects/{project}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}` where `{project}`, `{location}` will be same as specified in private connection resource name and `{vmware_engine_network_id}` will be in the form of `{location}`-default e.g. projects/project/locations/us-central1/vmwareEngineNetworks/us-central1-default. |
| `state` | String |  | Output only. State of the private connection. |
| `name` | String |  | Output only. The resource name of the private connection. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/privateConnections/my-connection` |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `create_time` | String |  | Output only. Creation time of this resource. |
| `service_network` | String |  | Required. Service network to create private connection. Specify the name in the following form: `projects/{project}/global/networks/{network_id}` For type = PRIVATE_SERVICE_ACCESS, this field represents servicenetworking VPC, e.g. projects/project-tp/global/networks/servicenetworking. For type = NETAPP_CLOUD_VOLUME, this field represents NetApp service VPC, e.g. projects/project-tp/global/networks/netapp-tenant-vpc. For type = DELL_POWERSCALE, this field represent Dell service VPC, e.g. projects/project-tp/global/networks/dell-tenant-vpc. For type= THIRD_PARTY_SERVICE, this field could represent a consumer VPC or any other producer VPC to which the VMware Engine Network needs to be connected, e.g. projects/project/global/networks/vpc. |
| `description` | String |  | Optional. User-provided description for this private connection. |
| `routing_mode` | String |  | Optional. Routing Mode. Default value is set to GLOBAL. For type = PRIVATE_SERVICE_ACCESS, this field can be set to GLOBAL or REGIONAL, for other types only GLOBAL is supported. |
| `vmware_engine_network_canonical` | String |  | Output only. The canonical name of the VMware Engine network in the form: `projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}` |
| `parent` | String | ✅ | Required. The resource name of the location to create the new private connection in. Private connection is a regional resource. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `peering_id` | String | Output only. VPC network peering id between given network VPC and VMwareEngineNetwork. |
| `peering_state` | String | Output only. Peering state between service network and VMware Engine network. |
| `update_time` | String | Output only. Last update time of this resource. |
| `type` | String | Required. Private connection type. |
| `vmware_engine_network` | String | Required. The relative resource name of Legacy VMware Engine network. Specify the name in the following form: `projects/{project}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}` where `{project}`, `{location}` will be same as specified in private connection resource name and `{vmware_engine_network_id}` will be in the form of `{location}`-default e.g. projects/project/locations/us-central1/vmwareEngineNetworks/us-central1-default. |
| `state` | String | Output only. State of the private connection. |
| `name` | String | Output only. The resource name of the private connection. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/privateConnections/my-connection` |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `create_time` | String | Output only. Creation time of this resource. |
| `service_network` | String | Required. Service network to create private connection. Specify the name in the following form: `projects/{project}/global/networks/{network_id}` For type = PRIVATE_SERVICE_ACCESS, this field represents servicenetworking VPC, e.g. projects/project-tp/global/networks/servicenetworking. For type = NETAPP_CLOUD_VOLUME, this field represents NetApp service VPC, e.g. projects/project-tp/global/networks/netapp-tenant-vpc. For type = DELL_POWERSCALE, this field represent Dell service VPC, e.g. projects/project-tp/global/networks/dell-tenant-vpc. For type= THIRD_PARTY_SERVICE, this field could represent a consumer VPC or any other producer VPC to which the VMware Engine Network needs to be connected, e.g. projects/project/global/networks/vpc. |
| `description` | String | Optional. User-provided description for this private connection. |
| `routing_mode` | String | Optional. Routing Mode. Default value is set to GLOBAL. For type = PRIVATE_SERVICE_ACCESS, this field can be set to GLOBAL or REGIONAL, for other types only GLOBAL is supported. |
| `vmware_engine_network_canonical` | String | Output only. The canonical name of the VMware Engine network in the form: `projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create private_connection
private_connection = provider.vmwareengine_api.Private_connection {
    parent = "value"  # Required. The resource name of the location to create the new private connection in. Private connection is a regional resource. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1`
}

# Access private_connection outputs
private_connection_id = private_connection.id
private_connection_peering_id = private_connection.peering_id
private_connection_peering_state = private_connection.peering_state
private_connection_update_time = private_connection.update_time
private_connection_type = private_connection.type
private_connection_vmware_engine_network = private_connection.vmware_engine_network
private_connection_state = private_connection.state
private_connection_name = private_connection.name
private_connection_uid = private_connection.uid
private_connection_create_time = private_connection.create_time
private_connection_service_network = private_connection.service_network
private_connection_description = private_connection.description
private_connection_routing_mode = private_connection.routing_mode
private_connection_vmware_engine_network_canonical = private_connection.vmware_engine_network_canonical
```

---


### Private_cloud

Creates a new `PrivateCloud` resource in a given project and location. Private clouds of type `STANDARD` and `TIME_LIMITED` are zonal resources, `STRETCHED` private clouds are regional. Creating a private cloud also creates a [management cluster](https://cloud.google.com/vmware-engine/docs/concepts-vmware-components) for that private cloud.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Identifier. The resource name of this private cloud. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud` |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `state` | String |  | Output only. State of the resource. New values may be added to this enum when appropriate. |
| `vcenter` | String |  | Output only. Vcenter appliance. |
| `expire_time` | String |  | Output only. Time when the resource will be irreversibly deleted. |
| `type` | String |  | Optional. Type of the private cloud. Defaults to STANDARD. |
| `delete_time` | String |  | Output only. Time when the resource was scheduled for deletion. |
| `update_time` | String |  | Output only. Last update time of this resource. |
| `hcx` | String |  | Output only. HCX appliance. |
| `description` | String |  | User-provided description for this private cloud. |
| `network_config` | String |  | Required. Network configuration of the private cloud. |
| `create_time` | String |  | Output only. Creation time of this resource. |
| `management_cluster` | String |  | Required. Input only. The management cluster for this private cloud. This field is required during creation of the private cloud to provide details for the default cluster. The following fields can't be changed after private cloud creation: `ManagementCluster.clusterId`, `ManagementCluster.nodeTypeId`. |
| `nsx` | String |  | Output only. NSX appliance. |
| `parent` | String | ✅ | Required. The resource name of the location to create the new private cloud in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Identifier. The resource name of this private cloud. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud` |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `state` | String | Output only. State of the resource. New values may be added to this enum when appropriate. |
| `vcenter` | String | Output only. Vcenter appliance. |
| `expire_time` | String | Output only. Time when the resource will be irreversibly deleted. |
| `type` | String | Optional. Type of the private cloud. Defaults to STANDARD. |
| `delete_time` | String | Output only. Time when the resource was scheduled for deletion. |
| `update_time` | String | Output only. Last update time of this resource. |
| `hcx` | String | Output only. HCX appliance. |
| `description` | String | User-provided description for this private cloud. |
| `network_config` | String | Required. Network configuration of the private cloud. |
| `create_time` | String | Output only. Creation time of this resource. |
| `management_cluster` | String | Required. Input only. The management cluster for this private cloud. This field is required during creation of the private cloud to provide details for the default cluster. The following fields can't be changed after private cloud creation: `ManagementCluster.clusterId`, `ManagementCluster.nodeTypeId`. |
| `nsx` | String | Output only. NSX appliance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create private_cloud
private_cloud = provider.vmwareengine_api.Private_cloud {
    parent = "value"  # Required. The resource name of the location to create the new private cloud in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a`
}

# Access private_cloud outputs
private_cloud_id = private_cloud.id
private_cloud_name = private_cloud.name
private_cloud_uid = private_cloud.uid
private_cloud_state = private_cloud.state
private_cloud_vcenter = private_cloud.vcenter
private_cloud_expire_time = private_cloud.expire_time
private_cloud_type = private_cloud.type
private_cloud_delete_time = private_cloud.delete_time
private_cloud_update_time = private_cloud.update_time
private_cloud_hcx = private_cloud.hcx
private_cloud_description = private_cloud.description
private_cloud_network_config = private_cloud.network_config
private_cloud_create_time = private_cloud.create_time
private_cloud_management_cluster = private_cloud.management_cluster
private_cloud_nsx = private_cloud.nsx
```

---


### Cluster

Creates a new cluster in a given private cloud. Creating a new cluster provides additional nodes for use in the parent private cloud and requires sufficient [node quota](https://cloud.google.com/vmware-engine/quotas).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Last update time of this resource. |
| `state` | String |  | Output only. State of the resource. |
| `management` | bool |  | Output only. True if the cluster is a management cluster; false otherwise. There can only be one management cluster in a private cloud and it has to be the first one. |
| `name` | String |  | Output only. Identifier. The resource name of this cluster. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/clusters/my-cluster` |
| `create_time` | String |  | Output only. Creation time of this resource. |
| `autoscaling_settings` | String |  | Optional. Configuration of the autoscaling applied to this cluster. |
| `node_type_configs` | HashMap<String, String> |  | Required. The map of cluster node types in this cluster, where the key is canonical identifier of the node type (corresponds to the `NodeType`). |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `stretched_cluster_config` | String |  | Optional. Configuration of a stretched cluster. Required for clusters that belong to a STRETCHED private cloud. |
| `parent` | String | ✅ | Required. The resource name of the private cloud to create a new cluster in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Last update time of this resource. |
| `state` | String | Output only. State of the resource. |
| `management` | bool | Output only. True if the cluster is a management cluster; false otherwise. There can only be one management cluster in a private cloud and it has to be the first one. |
| `name` | String | Output only. Identifier. The resource name of this cluster. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/clusters/my-cluster` |
| `create_time` | String | Output only. Creation time of this resource. |
| `autoscaling_settings` | String | Optional. Configuration of the autoscaling applied to this cluster. |
| `node_type_configs` | HashMap<String, String> | Required. The map of cluster node types in this cluster, where the key is canonical identifier of the node type (corresponds to the `NodeType`). |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `stretched_cluster_config` | String | Optional. Configuration of a stretched cluster. Required for clusters that belong to a STRETCHED private cloud. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.vmwareengine_api.Cluster {
    parent = "value"  # Required. The resource name of the private cloud to create a new cluster in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
}

# Access cluster outputs
cluster_id = cluster.id
cluster_update_time = cluster.update_time
cluster_state = cluster.state
cluster_management = cluster.management
cluster_name = cluster.name
cluster_create_time = cluster.create_time
cluster_autoscaling_settings = cluster.autoscaling_settings
cluster_node_type_configs = cluster.node_type_configs
cluster_uid = cluster.uid
cluster_stretched_cluster_config = cluster.stretched_cluster_config
```

---


### External_access_rule

Creates a new external access rule in a given network policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_ports` | Vec<String> |  | A list of destination ports to which the external access rule applies. This field is only applicable for the UDP or TCP protocol. Each entry must be either an integer or a range. For example: `["22"]`, `["80","443"]`, or `["12345-12349"]`. To match all destination ports, specify `["0-65535"]`. |
| `description` | String |  | User-provided description for this external access rule. |
| `ip_protocol` | String |  | The IP protocol to which the external access rule applies. This value can be one of the following three protocol strings (not case-sensitive): `tcp`, `udp`, or `icmp`. |
| `name` | String |  | Output only. The resource name of this external access rule. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/networkPolicies/my-policy/externalAccessRules/my-rule` |
| `create_time` | String |  | Output only. Creation time of this resource. |
| `priority` | i64 |  | External access rule priority, which determines the external access rule to use when multiple rules apply. If multiple rules have the same priority, their ordering is non-deterministic. If specific ordering is required, assign unique priorities to enforce such ordering. The external access rule priority is an integer from 100 to 4096, both inclusive. Lower integers indicate higher precedence. For example, a rule with priority `100` has higher precedence than a rule with priority `101`. |
| `state` | String |  | Output only. The state of the resource. |
| `destination_ip_ranges` | Vec<String> |  | If destination ranges are specified, the external access rule applies only to the traffic that has a destination IP address in these ranges. The specified IP addresses must have reserved external IP addresses in the scope of the parent network policy. To match all external IP addresses in the scope of the parent network policy, specify `0.0.0.0/0`. To match a specific external IP address, specify it using the `IpRange.external_address` property. |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `update_time` | String |  | Output only. Last update time of this resource. |
| `source_ip_ranges` | Vec<String> |  | If source ranges are specified, the external access rule applies only to traffic that has a source IP address in these ranges. These ranges can either be expressed in the CIDR format or as an IP address. As only inbound rules are supported, `ExternalAddress` resources cannot be the source IP addresses of an external access rule. To match all source addresses, specify `0.0.0.0/0`. |
| `source_ports` | Vec<String> |  | A list of source ports to which the external access rule applies. This field is only applicable for the UDP or TCP protocol. Each entry must be either an integer or a range. For example: `["22"]`, `["80","443"]`, or `["12345-12349"]`. To match all source ports, specify `["0-65535"]`. |
| `action` | String |  | The action that the external access rule performs. |
| `parent` | String | ✅ | Required. The resource name of the network policy to create a new external access firewall rule in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/networkPolicies/my-policy` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination_ports` | Vec<String> | A list of destination ports to which the external access rule applies. This field is only applicable for the UDP or TCP protocol. Each entry must be either an integer or a range. For example: `["22"]`, `["80","443"]`, or `["12345-12349"]`. To match all destination ports, specify `["0-65535"]`. |
| `description` | String | User-provided description for this external access rule. |
| `ip_protocol` | String | The IP protocol to which the external access rule applies. This value can be one of the following three protocol strings (not case-sensitive): `tcp`, `udp`, or `icmp`. |
| `name` | String | Output only. The resource name of this external access rule. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/networkPolicies/my-policy/externalAccessRules/my-rule` |
| `create_time` | String | Output only. Creation time of this resource. |
| `priority` | i64 | External access rule priority, which determines the external access rule to use when multiple rules apply. If multiple rules have the same priority, their ordering is non-deterministic. If specific ordering is required, assign unique priorities to enforce such ordering. The external access rule priority is an integer from 100 to 4096, both inclusive. Lower integers indicate higher precedence. For example, a rule with priority `100` has higher precedence than a rule with priority `101`. |
| `state` | String | Output only. The state of the resource. |
| `destination_ip_ranges` | Vec<String> | If destination ranges are specified, the external access rule applies only to the traffic that has a destination IP address in these ranges. The specified IP addresses must have reserved external IP addresses in the scope of the parent network policy. To match all external IP addresses in the scope of the parent network policy, specify `0.0.0.0/0`. To match a specific external IP address, specify it using the `IpRange.external_address` property. |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `update_time` | String | Output only. Last update time of this resource. |
| `source_ip_ranges` | Vec<String> | If source ranges are specified, the external access rule applies only to traffic that has a source IP address in these ranges. These ranges can either be expressed in the CIDR format or as an IP address. As only inbound rules are supported, `ExternalAddress` resources cannot be the source IP addresses of an external access rule. To match all source addresses, specify `0.0.0.0/0`. |
| `source_ports` | Vec<String> | A list of source ports to which the external access rule applies. This field is only applicable for the UDP or TCP protocol. Each entry must be either an integer or a range. For example: `["22"]`, `["80","443"]`, or `["12345-12349"]`. To match all source ports, specify `["0-65535"]`. |
| `action` | String | The action that the external access rule performs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create external_access_rule
external_access_rule = provider.vmwareengine_api.External_access_rule {
    parent = "value"  # Required. The resource name of the network policy to create a new external access firewall rule in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/networkPolicies/my-policy`
}

# Access external_access_rule outputs
external_access_rule_id = external_access_rule.id
external_access_rule_destination_ports = external_access_rule.destination_ports
external_access_rule_description = external_access_rule.description
external_access_rule_ip_protocol = external_access_rule.ip_protocol
external_access_rule_name = external_access_rule.name
external_access_rule_create_time = external_access_rule.create_time
external_access_rule_priority = external_access_rule.priority
external_access_rule_state = external_access_rule.state
external_access_rule_destination_ip_ranges = external_access_rule.destination_ip_ranges
external_access_rule_uid = external_access_rule.uid
external_access_rule_update_time = external_access_rule.update_time
external_access_rule_source_ip_ranges = external_access_rule.source_ip_ranges
external_access_rule_source_ports = external_access_rule.source_ports
external_access_rule_action = external_access_rule.action
```

---


### Dns_bind_permission

Grants the bind permission to the customer provided principal(user / service account) to bind their DNS zone with the intranet VPC associated with the project. DnsBindPermission is a global resource and location can only be global.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `principal` | String |  | Required. The consumer provided user/service account which needs to be granted permission to bind with the intranet VPC corresponding to the consumer project. |
| `request_id` | String |  | Optional. A request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server guarantees that a request doesn't result in creation of duplicate commitments for at least 60 minutes. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000). |
| `name` | String | ✅ | Required. The name of the resource which stores the users/service accounts having the permission to bind to the corresponding intranet VPC of the consumer project. DnsBindPermission is a global resource. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/global/dnsBindPermission` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dns_bind_permission
dns_bind_permission = provider.vmwareengine_api.Dns_bind_permission {
    name = "value"  # Required. The name of the resource which stores the users/service accounts having the permission to bind to the corresponding intranet VPC of the consumer project. DnsBindPermission is a global resource. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/global/dnsBindPermission`
}

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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


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
location_name = location.name
location_display_name = location.display_name
location_location_id = location.location_id
location_labels = location.labels
```

---


### Node

Gets details of a single node.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fqdn` | String | Output only. Fully qualified domain name of the node. |
| `name` | String | Output only. The resource name of this node. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: projects/my-project/locations/us-central1-a/privateClouds/my-cloud/clusters/my-cluster/nodes/my-node |
| `state` | String | Output only. The state of the appliance. |
| `version` | String | Output only. The version number of the VMware ESXi management component in this cluster. |
| `custom_core_count` | String | Output only. Customized number of cores |
| `node_type_id` | String | Output only. The canonical identifier of the node type (corresponds to the `NodeType`). For example: standard-72. |
| `internal_ip` | String | Output only. Internal IP address of the node. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access node outputs
node_id = node.id
node_fqdn = node.fqdn
node_name = node.name
node_state = node.state
node_version = node.version
node_custom_core_count = node.custom_core_count
node_node_type_id = node.node_type_id
node_internal_ip = node.internal_ip
```

---


### Announcement

Retrieves a `Announcement` by its resource name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the resource. New values may be added to this enum when appropriate. |
| `update_time` | String | Output only. Last update time of this resource. |
| `private_cloud` | String | A Private Cloud resource name. |
| `activity_type` | String | Optional. Activity type of the announcement There can be only one active announcement for a given activity type and target resource. |
| `name` | String | Output only. The resource name of the announcement. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-west1-a/announcements/my-announcement-id` |
| `code` | String | Required. Code of the announcement. Indicates the presence of a VMware Engine related announcement and corresponds to a related message in the `description` field. |
| `metadata` | HashMap<String, String> | Output only. Additional structured details about this announcement. |
| `cluster` | String | A Cluster resource name. |
| `target_resource_type` | String | Output only. Target Resource Type defines the type of the target for the announcement |
| `description` | String | Output only. Description of the announcement. |
| `create_time` | String | Output only. Creation time of this resource. It also serves as start time of notification. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access announcement outputs
announcement_id = announcement.id
announcement_state = announcement.state
announcement_update_time = announcement.update_time
announcement_private_cloud = announcement.private_cloud
announcement_activity_type = announcement.activity_type
announcement_name = announcement.name
announcement_code = announcement.code
announcement_metadata = announcement.metadata
announcement_cluster = announcement.cluster
announcement_target_resource_type = announcement.target_resource_type
announcement_description = announcement.description
announcement_create_time = announcement.create_time
```

---


### Node_type

Gets details of a single `NodeType`.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `virtual_cpu_count` | i64 | Output only. The total number of virtual CPUs in a single node. |
| `memory_gb` | i64 | Output only. The amount of physical memory available, defined in GB. |
| `families` | Vec<String> | Output only. Families of the node type. For node types to be in the same cluster they must share at least one element in the `families`. |
| `total_core_count` | i64 | Output only. The total number of CPU cores in a single node. |
| `kind` | String | Output only. The type of the resource. |
| `disk_size_gb` | i64 | Output only. The amount of storage available, defined in GB. |
| `name` | String | Output only. The resource name of this node type. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-proj/locations/us-central1-a/nodeTypes/standard-72` |
| `capabilities` | Vec<String> | Output only. Capabilities of this node type. |
| `node_type_id` | String | Output only. The canonical identifier of the node type (corresponds to the `NodeType`). For example: standard-72. |
| `available_custom_core_counts` | Vec<i64> | Output only. List of possible values of custom core count. |
| `display_name` | String | Output only. The friendly name for this node type. For example: ve1-standard-72 |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access node_type outputs
node_type_id = node_type.id
node_type_virtual_cpu_count = node_type.virtual_cpu_count
node_type_memory_gb = node_type.memory_gb
node_type_families = node_type.families
node_type_total_core_count = node_type.total_core_count
node_type_kind = node_type.kind
node_type_disk_size_gb = node_type.disk_size_gb
node_type_name = node_type.name
node_type_capabilities = node_type.capabilities
node_type_node_type_id = node_type.node_type_id
node_type_available_custom_core_counts = node_type.available_custom_core_counts
node_type_display_name = node_type.display_name
```

---


### Upgrade

Retrieves a private cloud `Upgrade` resource by its resource name.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Output Only. Creation time of this resource. |
| `version` | String |  | Output only.  |
| `etag` | String |  | The etag for the upgrade resource. If this is provided on update, it must match the server's etag. |
| `end_time` | String |  | Output only. Output Only. End time of the upgrade. |
| `component_upgrades` | Vec<String> |  | Output only. Output Only. The list of component upgrades. |
| `start_version` | String |  | Output only. Output Only. The start version |
| `state` | String |  | Output only. The current state of the upgrade. |
| `name` | String |  | Output only. Identifier. The resource name of the private cloud `Upgrade`. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-west1-a/privateClouds/my-cloud/upgrades/my-upgrade` |
| `schedule` | String |  | Schedule details for the upgrade. |
| `target_version` | String |  | Output only. Output Only. The target version |
| `update_time` | String |  | Output only. Output Only. Last update time of this resource. |
| `estimated_duration` | String |  | Output only. Output Only. The estimated total duration of the upgrade. This information can be used to plan or schedule upgrades to minimize disruptions. Please note that the estimated duration is only an estimate. The actual upgrade duration may vary. |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `description` | String |  | Output only. Output Only. The description of the upgrade. This is used to provide additional information about the private cloud upgrade, such as the upgrade's purpose, the changes included in the upgrade, or any other relevant information about the upgrade. |
| `type` | String |  | Output only. Output Only. The type of upgrade. |
| `name` | String | ✅ | Output only. Identifier. The resource name of the private cloud `Upgrade`. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-west1-a/privateClouds/my-cloud/upgrades/my-upgrade` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Output Only. Creation time of this resource. |
| `version` | String | Output only.  |
| `etag` | String | The etag for the upgrade resource. If this is provided on update, it must match the server's etag. |
| `end_time` | String | Output only. Output Only. End time of the upgrade. |
| `component_upgrades` | Vec<String> | Output only. Output Only. The list of component upgrades. |
| `start_version` | String | Output only. Output Only. The start version |
| `state` | String | Output only. The current state of the upgrade. |
| `name` | String | Output only. Identifier. The resource name of the private cloud `Upgrade`. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-west1-a/privateClouds/my-cloud/upgrades/my-upgrade` |
| `schedule` | String | Schedule details for the upgrade. |
| `target_version` | String | Output only. Output Only. The target version |
| `update_time` | String | Output only. Output Only. Last update time of this resource. |
| `estimated_duration` | String | Output only. Output Only. The estimated total duration of the upgrade. This information can be used to plan or schedule upgrades to minimize disruptions. Please note that the estimated duration is only an estimate. The actual upgrade duration may vary. |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `description` | String | Output only. Output Only. The description of the upgrade. This is used to provide additional information about the private cloud upgrade, such as the upgrade's purpose, the changes included in the upgrade, or any other relevant information about the upgrade. |
| `type` | String | Output only. Output Only. The type of upgrade. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access upgrade outputs
upgrade_id = upgrade.id
upgrade_create_time = upgrade.create_time
upgrade_version = upgrade.version
upgrade_etag = upgrade.etag
upgrade_end_time = upgrade.end_time
upgrade_component_upgrades = upgrade.component_upgrades
upgrade_start_version = upgrade.start_version
upgrade_state = upgrade.state
upgrade_name = upgrade.name
upgrade_schedule = upgrade.schedule
upgrade_target_version = upgrade.target_version
upgrade_update_time = upgrade.update_time
upgrade_estimated_duration = upgrade.estimated_duration
upgrade_uid = upgrade.uid
upgrade_description = upgrade.description
upgrade_type = upgrade.type
```

---


### Network_peering

Creates a new network peering between the peer network and VMware Engine network provided in a `NetworkPeering` resource. NetworkPeering is a global resource and location can only be global.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. User-provided description for this network peering. |
| `import_custom_routes` | bool |  | Optional. True if custom routes are imported from the peered network; false otherwise. The default value is true. |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `export_custom_routes` | bool |  | Optional. True if custom routes are exported to the peered network; false otherwise. The default value is true. |
| `name` | String |  | Output only. Identifier. The resource name of the network peering. NetworkPeering is a global resource and location can only be global. Resource names are scheme-less URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/global/networkPeerings/my-peering` |
| `state` | String |  | Output only. State of the network peering. This field has a value of 'ACTIVE' when there's a matching configuration in the peer network. New values may be added to this enum when appropriate. |
| `peer_mtu` | i64 |  | Optional. Maximum transmission unit (MTU) in bytes. The default value is `1500`. If a value of `0` is provided for this field, VMware Engine uses the default value instead. |
| `import_custom_routes_with_public_ip` | bool |  | Optional. True if all subnet routes with public IP address range are imported; false otherwise. The default value is true. IPv4 special-use ranges (https://en.wikipedia.org/wiki/IPv4#Special_addresses) are always imported to peers and are not controlled by this field. |
| `vmware_engine_network` | String |  | Required. The relative resource name of the VMware Engine network. Specify the name in the following form: `projects/{project}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}` where `{project}` can either be a project number or a project ID. |
| `peer_network_type` | String |  | Required. The type of the network to peer with the VMware Engine network. |
| `create_time` | String |  | Output only. Creation time of this resource. |
| `peer_network` | String |  | Required. The relative resource name of the network to peer with a standard VMware Engine network. The provided network can be a consumer VPC network or another standard VMware Engine network. If the `peer_network_type` is VMWARE_ENGINE_NETWORK, specify the name in the form: `projects/{project}/locations/global/vmwareEngineNetworks/{vmware_engine_network_id}`. Otherwise specify the name in the form: `projects/{project}/global/networks/{network_id}`, where `{project}` can either be a project number or a project ID. |
| `export_custom_routes_with_public_ip` | bool |  | Optional. True if all subnet routes with a public IP address range are exported; false otherwise. The default value is true. IPv4 special-use ranges (https://en.wikipedia.org/wiki/IPv4#Special_addresses) are always exported to peers and are not controlled by this field. |
| `state_details` | String |  | Output only. Output Only. Details about the current state of the network peering. |
| `exchange_subnet_routes` | bool |  | Optional. True if full mesh connectivity is created and managed automatically between peered networks; false otherwise. Currently this field is always true because Google Compute Engine automatically creates and manages subnetwork routes between two VPC networks when peering state is 'ACTIVE'. |
| `update_time` | String |  | Output only. Last update time of this resource. |
| `parent` | String | ✅ | Required. The resource name of the location to create the new network peering in. This value is always `global`, because `NetworkPeering` is a global resource. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. User-provided description for this network peering. |
| `import_custom_routes` | bool | Optional. True if custom routes are imported from the peered network; false otherwise. The default value is true. |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `export_custom_routes` | bool | Optional. True if custom routes are exported to the peered network; false otherwise. The default value is true. |
| `name` | String | Output only. Identifier. The resource name of the network peering. NetworkPeering is a global resource and location can only be global. Resource names are scheme-less URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/global/networkPeerings/my-peering` |
| `state` | String | Output only. State of the network peering. This field has a value of 'ACTIVE' when there's a matching configuration in the peer network. New values may be added to this enum when appropriate. |
| `peer_mtu` | i64 | Optional. Maximum transmission unit (MTU) in bytes. The default value is `1500`. If a value of `0` is provided for this field, VMware Engine uses the default value instead. |
| `import_custom_routes_with_public_ip` | bool | Optional. True if all subnet routes with public IP address range are imported; false otherwise. The default value is true. IPv4 special-use ranges (https://en.wikipedia.org/wiki/IPv4#Special_addresses) are always imported to peers and are not controlled by this field. |
| `vmware_engine_network` | String | Required. The relative resource name of the VMware Engine network. Specify the name in the following form: `projects/{project}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}` where `{project}` can either be a project number or a project ID. |
| `peer_network_type` | String | Required. The type of the network to peer with the VMware Engine network. |
| `create_time` | String | Output only. Creation time of this resource. |
| `peer_network` | String | Required. The relative resource name of the network to peer with a standard VMware Engine network. The provided network can be a consumer VPC network or another standard VMware Engine network. If the `peer_network_type` is VMWARE_ENGINE_NETWORK, specify the name in the form: `projects/{project}/locations/global/vmwareEngineNetworks/{vmware_engine_network_id}`. Otherwise specify the name in the form: `projects/{project}/global/networks/{network_id}`, where `{project}` can either be a project number or a project ID. |
| `export_custom_routes_with_public_ip` | bool | Optional. True if all subnet routes with a public IP address range are exported; false otherwise. The default value is true. IPv4 special-use ranges (https://en.wikipedia.org/wiki/IPv4#Special_addresses) are always exported to peers and are not controlled by this field. |
| `state_details` | String | Output only. Output Only. Details about the current state of the network peering. |
| `exchange_subnet_routes` | bool | Optional. True if full mesh connectivity is created and managed automatically between peered networks; false otherwise. Currently this field is always true because Google Compute Engine automatically creates and manages subnetwork routes between two VPC networks when peering state is 'ACTIVE'. |
| `update_time` | String | Output only. Last update time of this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create network_peering
network_peering = provider.vmwareengine_api.Network_peering {
    parent = "value"  # Required. The resource name of the location to create the new network peering in. This value is always `global`, because `NetworkPeering` is a global resource. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/global`
}

# Access network_peering outputs
network_peering_id = network_peering.id
network_peering_description = network_peering.description
network_peering_import_custom_routes = network_peering.import_custom_routes
network_peering_uid = network_peering.uid
network_peering_export_custom_routes = network_peering.export_custom_routes
network_peering_name = network_peering.name
network_peering_state = network_peering.state
network_peering_peer_mtu = network_peering.peer_mtu
network_peering_import_custom_routes_with_public_ip = network_peering.import_custom_routes_with_public_ip
network_peering_vmware_engine_network = network_peering.vmware_engine_network
network_peering_peer_network_type = network_peering.peer_network_type
network_peering_create_time = network_peering.create_time
network_peering_peer_network = network_peering.peer_network
network_peering_export_custom_routes_with_public_ip = network_peering.export_custom_routes_with_public_ip
network_peering_state_details = network_peering.state_details
network_peering_exchange_subnet_routes = network_peering.exchange_subnet_routes
network_peering_update_time = network_peering.update_time
```

---


### Hcx_activation_key

Creates a new HCX activation key in a given private cloud.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `activation_key` | String |  | Output only. HCX activation key. |
| `create_time` | String |  | Output only. Creation time of HCX activation key. |
| `state` | String |  | Output only. State of HCX activation key. |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `name` | String |  | Output only. The resource name of this HcxActivationKey. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/privateClouds/my-cloud/hcxActivationKeys/my-key` |
| `parent` | String | ✅ | Required. The resource name of the private cloud to create the key for. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/privateClouds/my-cloud` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `activation_key` | String | Output only. HCX activation key. |
| `create_time` | String | Output only. Creation time of HCX activation key. |
| `state` | String | Output only. State of HCX activation key. |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `name` | String | Output only. The resource name of this HcxActivationKey. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/privateClouds/my-cloud/hcxActivationKeys/my-key` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hcx_activation_key
hcx_activation_key = provider.vmwareengine_api.Hcx_activation_key {
    parent = "value"  # Required. The resource name of the private cloud to create the key for. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/privateClouds/my-cloud`
}

# Access hcx_activation_key outputs
hcx_activation_key_id = hcx_activation_key.id
hcx_activation_key_activation_key = hcx_activation_key.activation_key
hcx_activation_key_create_time = hcx_activation_key.create_time
hcx_activation_key_state = hcx_activation_key.state
hcx_activation_key_uid = hcx_activation_key.uid
hcx_activation_key_name = hcx_activation_key.name
```

---


### Management_dns_zone_binding

Creates a new `ManagementDnsZoneBinding` resource in a private cloud. This RPC creates the DNS binding and the resource that represents the DNS binding of the consumer VPC network to the management DNS zone. A management DNS zone is the Cloud DNS cross-project binding zone that VMware Engine creates for each private cloud. It contains FQDNs and corresponding IP addresses for the private cloud's ESXi hosts and management VM appliances like vCenter and NSX Manager.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | User-provided description for this resource. |
| `state` | String |  | Output only. The state of the resource. |
| `vpc_network` | String |  | Network to bind is a standard consumer VPC. Specify the name in the following form for consumer VPC network: `projects/{project}/global/networks/{network_id}`. `{project}` can either be a project number or a project ID. |
| `name` | String |  | Output only. The resource name of this binding. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/managementDnsZoneBindings/my-management-dns-zone-binding` |
| `update_time` | String |  | Output only. Last update time of this resource. |
| `vmware_engine_network` | String |  | Network to bind is a VMware Engine network. Specify the name in the following form for VMware engine network: `projects/{project}/locations/global/vmwareEngineNetworks/{vmware_engine_network_id}`. `{project}` can either be a project number or a project ID. |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `create_time` | String |  | Output only. Creation time of this resource. |
| `parent` | String | ✅ | Required. The resource name of the private cloud to create a new management DNS zone binding for. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | User-provided description for this resource. |
| `state` | String | Output only. The state of the resource. |
| `vpc_network` | String | Network to bind is a standard consumer VPC. Specify the name in the following form for consumer VPC network: `projects/{project}/global/networks/{network_id}`. `{project}` can either be a project number or a project ID. |
| `name` | String | Output only. The resource name of this binding. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/managementDnsZoneBindings/my-management-dns-zone-binding` |
| `update_time` | String | Output only. Last update time of this resource. |
| `vmware_engine_network` | String | Network to bind is a VMware Engine network. Specify the name in the following form for VMware engine network: `projects/{project}/locations/global/vmwareEngineNetworks/{vmware_engine_network_id}`. `{project}` can either be a project number or a project ID. |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `create_time` | String | Output only. Creation time of this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create management_dns_zone_binding
management_dns_zone_binding = provider.vmwareengine_api.Management_dns_zone_binding {
    parent = "value"  # Required. The resource name of the private cloud to create a new management DNS zone binding for. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
}

# Access management_dns_zone_binding outputs
management_dns_zone_binding_id = management_dns_zone_binding.id
management_dns_zone_binding_description = management_dns_zone_binding.description
management_dns_zone_binding_state = management_dns_zone_binding.state
management_dns_zone_binding_vpc_network = management_dns_zone_binding.vpc_network
management_dns_zone_binding_name = management_dns_zone_binding.name
management_dns_zone_binding_update_time = management_dns_zone_binding.update_time
management_dns_zone_binding_vmware_engine_network = management_dns_zone_binding.vmware_engine_network
management_dns_zone_binding_uid = management_dns_zone_binding.uid
management_dns_zone_binding_create_time = management_dns_zone_binding.create_time
```

---


### Network_policie

Creates a new network policy in a given VMware Engine network of a project and location (region). A new network policy cannot be created if another network policy already exists in the same scope.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Identifier. The resource name of this network policy. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/networkPolicies/my-network-policy` |
| `vmware_engine_network` | String |  | Optional. The relative resource name of the VMware Engine network. Specify the name in the following form: `projects/{project}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}` where `{project}` can either be a project number or a project ID. |
| `external_ip` | String |  | Network service that allows External IP addresses to be assigned to VMware workloads. This service can only be enabled when `internet_access` is also enabled. |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `vmware_engine_network_canonical` | String |  | Output only. The canonical name of the VMware Engine network in the form: `projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}` |
| `description` | String |  | Optional. User-provided description for this network policy. |
| `edge_services_cidr` | String |  | Required. IP address range in CIDR notation used to create internet access and external IP access. An RFC 1918 CIDR block, with a "/26" prefix, is required. The range cannot overlap with any prefixes either in the consumer VPC network or in use by the private clouds attached to that VPC network. |
| `create_time` | String |  | Output only. Creation time of this resource. |
| `internet_access` | String |  | Network service that allows VMware workloads to access the internet. |
| `update_time` | String |  | Output only. Last update time of this resource. |
| `parent` | String | ✅ | Required. The resource name of the location (region) to create the new network policy in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Identifier. The resource name of this network policy. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1/networkPolicies/my-network-policy` |
| `vmware_engine_network` | String | Optional. The relative resource name of the VMware Engine network. Specify the name in the following form: `projects/{project}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}` where `{project}` can either be a project number or a project ID. |
| `external_ip` | String | Network service that allows External IP addresses to be assigned to VMware workloads. This service can only be enabled when `internet_access` is also enabled. |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `vmware_engine_network_canonical` | String | Output only. The canonical name of the VMware Engine network in the form: `projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}` |
| `description` | String | Optional. User-provided description for this network policy. |
| `edge_services_cidr` | String | Required. IP address range in CIDR notation used to create internet access and external IP access. An RFC 1918 CIDR block, with a "/26" prefix, is required. The range cannot overlap with any prefixes either in the consumer VPC network or in use by the private clouds attached to that VPC network. |
| `create_time` | String | Output only. Creation time of this resource. |
| `internet_access` | String | Network service that allows VMware workloads to access the internet. |
| `update_time` | String | Output only. Last update time of this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create network_policie
network_policie = provider.vmwareengine_api.Network_policie {
    parent = "value"  # Required. The resource name of the location (region) to create the new network policy in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1`
}

# Access network_policie outputs
network_policie_id = network_policie.id
network_policie_name = network_policie.name
network_policie_vmware_engine_network = network_policie.vmware_engine_network
network_policie_external_ip = network_policie.external_ip
network_policie_uid = network_policie.uid
network_policie_vmware_engine_network_canonical = network_policie.vmware_engine_network_canonical
network_policie_description = network_policie.description
network_policie_edge_services_cidr = network_policie.edge_services_cidr
network_policie_create_time = network_policie.create_time
network_policie_internet_access = network_policie.internet_access
network_policie_update_time = network_policie.update_time
```

---


### Logging_server

Create a new logging server for a given private cloud.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_type` | String |  | Required. The type of component that produces logs that will be forwarded to this logging server. |
| `hostname` | String |  | Required. Fully-qualified domain name (FQDN) or IP Address of the logging server. |
| `create_time` | String |  | Output only. Creation time of this resource. |
| `update_time` | String |  | Output only. Last update time of this resource. |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `name` | String |  | Output only. The resource name of this logging server. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/loggingServers/my-logging-server` |
| `port` | i64 |  | Required. Port number at which the logging server receives logs. |
| `protocol` | String |  | Required. Protocol used by vCenter to send logs to a logging server. |
| `parent` | String | ✅ | Required. The resource name of the private cloud to create a new Logging Server in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_type` | String | Required. The type of component that produces logs that will be forwarded to this logging server. |
| `hostname` | String | Required. Fully-qualified domain name (FQDN) or IP Address of the logging server. |
| `create_time` | String | Output only. Creation time of this resource. |
| `update_time` | String | Output only. Last update time of this resource. |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `name` | String | Output only. The resource name of this logging server. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/loggingServers/my-logging-server` |
| `port` | i64 | Required. Port number at which the logging server receives logs. |
| `protocol` | String | Required. Protocol used by vCenter to send logs to a logging server. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create logging_server
logging_server = provider.vmwareengine_api.Logging_server {
    parent = "value"  # Required. The resource name of the private cloud to create a new Logging Server in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
}

# Access logging_server outputs
logging_server_id = logging_server.id
logging_server_source_type = logging_server.source_type
logging_server_hostname = logging_server.hostname
logging_server_create_time = logging_server.create_time
logging_server_update_time = logging_server.update_time
logging_server_uid = logging_server.uid
logging_server_name = logging_server.name
logging_server_port = logging_server.port
logging_server_protocol = logging_server.protocol
```

---


### Peering_route

Lists the network peering routes exchanged over a peering connection. NetworkPeering is a global resource and location can only be global.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `peering_routes` | Vec<String> | A list of peering routes. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access peering_route outputs
peering_route_id = peering_route.id
peering_route_peering_routes = peering_route.peering_routes
peering_route_next_page_token = peering_route.next_page_token
```

---


### External_addresse

Creates a new `ExternalAddress` resource in a given private cloud. The network policy that corresponds to the private cloud must have the external IP address network service enabled (`NetworkPolicy.external_ip`).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `external_ip` | String |  | Output only. The external IP address of a workload VM. |
| `description` | String |  | User-provided description for this resource. |
| `create_time` | String |  | Output only. Creation time of this resource. |
| `internal_ip` | String |  | The internal IP address of a workload VM. |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `name` | String |  | Output only. Identifier. The resource name of this external IP address. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/externalAddresses/my-address` |
| `update_time` | String |  | Output only. Last update time of this resource. |
| `state` | String |  | Output only. The state of the resource. |
| `parent` | String | ✅ | Required. The resource name of the private cloud to create a new external IP address in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `external_ip` | String | Output only. The external IP address of a workload VM. |
| `description` | String | User-provided description for this resource. |
| `create_time` | String | Output only. Creation time of this resource. |
| `internal_ip` | String | The internal IP address of a workload VM. |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `name` | String | Output only. Identifier. The resource name of this external IP address. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/externalAddresses/my-address` |
| `update_time` | String | Output only. Last update time of this resource. |
| `state` | String | Output only. The state of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create external_addresse
external_addresse = provider.vmwareengine_api.External_addresse {
    parent = "value"  # Required. The resource name of the private cloud to create a new external IP address in. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
}

# Access external_addresse outputs
external_addresse_id = external_addresse.id
external_addresse_external_ip = external_addresse.external_ip
external_addresse_description = external_addresse.description
external_addresse_create_time = external_addresse.create_time
external_addresse_internal_ip = external_addresse.internal_ip
external_addresse_uid = external_addresse.uid
external_addresse_name = external_addresse.name
external_addresse_update_time = external_addresse.update_time
external_addresse_state = external_addresse.state
```

---


### Vmware_engine_network

Creates a new VMware Engine network that can be used by a private cloud.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Creation time of this resource. |
| `vpc_networks` | Vec<String> |  | Output only. VMware Engine service VPC networks that provide connectivity from a private cloud to customer projects, the internet, and other Google Cloud services. |
| `etag` | String |  | Checksum that may be sent on update and delete requests to ensure that the user-provided value is up to date before the server processes a request. The server computes checksums based on the value of other fields in the request. |
| `type` | String |  | Required. VMware Engine network type. |
| `update_time` | String |  | Output only. Last update time of this resource. |
| `uid` | String |  | Output only. System-generated unique identifier for the resource. |
| `name` | String |  | Output only. Identifier. The resource name of the VMware Engine network. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/global/vmwareEngineNetworks/my-network` |
| `state` | String |  | Output only. State of the VMware Engine network. |
| `description` | String |  | User-provided description for this VMware Engine network. |
| `parent` | String | ✅ | Required. The resource name of the location to create the new VMware Engine network in. A VMware Engine network of type `LEGACY` is a regional resource, and a VMware Engine network of type `STANDARD` is a global resource. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Creation time of this resource. |
| `vpc_networks` | Vec<String> | Output only. VMware Engine service VPC networks that provide connectivity from a private cloud to customer projects, the internet, and other Google Cloud services. |
| `etag` | String | Checksum that may be sent on update and delete requests to ensure that the user-provided value is up to date before the server processes a request. The server computes checksums based on the value of other fields in the request. |
| `type` | String | Required. VMware Engine network type. |
| `update_time` | String | Output only. Last update time of this resource. |
| `uid` | String | Output only. System-generated unique identifier for the resource. |
| `name` | String | Output only. Identifier. The resource name of the VMware Engine network. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/global/vmwareEngineNetworks/my-network` |
| `state` | String | Output only. State of the VMware Engine network. |
| `description` | String | User-provided description for this VMware Engine network. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create vmware_engine_network
vmware_engine_network = provider.vmwareengine_api.Vmware_engine_network {
    parent = "value"  # Required. The resource name of the location to create the new VMware Engine network in. A VMware Engine network of type `LEGACY` is a regional resource, and a VMware Engine network of type `STANDARD` is a global resource. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/global`
}

# Access vmware_engine_network outputs
vmware_engine_network_id = vmware_engine_network.id
vmware_engine_network_create_time = vmware_engine_network.create_time
vmware_engine_network_vpc_networks = vmware_engine_network.vpc_networks
vmware_engine_network_etag = vmware_engine_network.etag
vmware_engine_network_type = vmware_engine_network.type
vmware_engine_network_update_time = vmware_engine_network.update_time
vmware_engine_network_uid = vmware_engine_network.uid
vmware_engine_network_name = vmware_engine_network.name
vmware_engine_network_state = vmware_engine_network.state
vmware_engine_network_description = vmware_engine_network.description
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |


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
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
```

---


### Subnet

Gets details of a single subnet.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The state of the resource. |
| `gateway_ip` | String |  | The IP address of the gateway of this subnet. Must fall within the IP prefix defined above. |
| `vlan_id` | i64 |  | Output only. VLAN ID of the VLAN on which the subnet is configured |
| `ip_cidr_range` | String |  | The IP address range of the subnet in CIDR format '10.0.0.0/24'. |
| `name` | String |  | Output only. Identifier. The resource name of this subnet. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/subnets/my-subnet` |
| `type` | String |  | Output only. The type of the subnet. For example "management" or "userDefined". |
| `name` | String | ✅ | Output only. Identifier. The resource name of this subnet. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/subnets/my-subnet` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The state of the resource. |
| `gateway_ip` | String | The IP address of the gateway of this subnet. Must fall within the IP prefix defined above. |
| `vlan_id` | i64 | Output only. VLAN ID of the VLAN on which the subnet is configured |
| `ip_cidr_range` | String | The IP address range of the subnet in CIDR format '10.0.0.0/24'. |
| `name` | String | Output only. Identifier. The resource name of this subnet. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/subnets/my-subnet` |
| `type` | String | Output only. The type of the subnet. For example "management" or "userDefined". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access subnet outputs
subnet_id = subnet.id
subnet_state = subnet.state
subnet_gateway_ip = subnet.gateway_ip
subnet_vlan_id = subnet.vlan_id
subnet_ip_cidr_range = subnet.ip_cidr_range
subnet_name = subnet.name
subnet_type = subnet.type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple private_connection resources
private_connection_0 = provider.vmwareengine_api.Private_connection {
    parent = "value-0"
}
private_connection_1 = provider.vmwareengine_api.Private_connection {
    parent = "value-1"
}
private_connection_2 = provider.vmwareengine_api.Private_connection {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    private_connection = provider.vmwareengine_api.Private_connection {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Vmwareengine_api Documentation](https://cloud.google.com/vmwareengine_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
