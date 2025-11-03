# Tpu_api Service



**Resources**: 23

---

## Overview

The tpu_api service provides access to 23 resource types:

- [Tensorflow_version](#tensorflow_version) [R]
- [Node](#node) [CRD]
- [Operation](#operation) [CRD]
- [Accelerator_type](#accelerator_type) [R]
- [Location](#location) [R]
- [Accelerator_type](#accelerator_type) [R]
- [Node](#node) [CRUD]
- [Operation](#operation) [CRD]
- [Queued_resource](#queued_resource) [CRD]
- [Location](#location) [CR]
- [Runtime_version](#runtime_version) [R]
- [Node](#node) [CRD]
- [Location](#location) [R]
- [Accelerator_type](#accelerator_type) [R]
- [Tensorflow_version](#tensorflow_version) [R]
- [Operation](#operation) [CRD]
- [Node](#node) [CRUD]
- [Reservation](#reservation) [R]
- [Operation](#operation) [CRD]
- [Location](#location) [CR]
- [Accelerator_type](#accelerator_type) [R]
- [Queued_resource](#queued_resource) [CRD]
- [Runtime_version](#runtime_version) [R]

---

## Resources


### Tensorflow_version

Gets TensorFlow Version.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name. |
| `version` | String | the tensorflow version. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access tensorflow_version outputs
tensorflow_version_id = tensorflow_version.id
tensorflow_version_name = tensorflow_version.name
tensorflow_version_version = tensorflow_version.version
```

---


### Node

Creates a node.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network` | String |  | The name of a network they wish to peer the TPU node to. It must be a preexisting Compute Engine network inside of the project on which this API has been activated. If none is provided, "default" will be used. |
| `symptoms` | Vec<String> |  | Output only. The Symptoms that have occurred to the TPU Node. |
| `description` | String |  | The user-supplied description of the TPU. Maximum of 512 characters. |
| `use_service_networking` | bool |  | Whether the VPC peering for the node is set up through Service Networking API. The VPC Peering should be set up before provisioning the node. If this field is set, cidr_block field should not be specified. If the network, that you want to peer the TPU Node to, is Shared VPC networks, the node must be created with this this field enabled. |
| `scheduling_config` | String |  | The scheduling options for this node. |
| `port` | String |  | Output only. DEPRECATED! Use network_endpoints instead. The network port for the TPU Node as visible to Compute Engine instances. |
| `health_description` | String |  | Output only. If this field is populated, it contains a description of why the TPU Node is unhealthy. |
| `network_endpoints` | Vec<String> |  | Output only. The network endpoints where TPU workers can be accessed and sent work. It is recommended that Tensorflow clients of the node reach out to the 0th entry in this map first. |
| `state` | String |  | Output only. The current state for the TPU Node. |
| `health` | String |  | The health status of the TPU node. |
| `cidr_block` | String |  | The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger block would be wasteful (a node can only consume one IP address). Errors will occur if the CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts with any subnetworks in the user's provided network, or the provided network is peered with another network that is using that CIDR block. |
| `tensorflow_version` | String |  | Required. The version of Tensorflow running in the Node. |
| `ip_address` | String |  | Output only. DEPRECATED! Use network_endpoints instead. The network address for the TPU Node as visible to Compute Engine instances. |
| `name` | String |  | Output only. Immutable. The name of the TPU |
| `api_version` | String |  | Output only. The API version that created this Node. |
| `accelerator_type` | String |  | Required. The type of hardware accelerators associated with this node. |
| `service_account` | String |  | Output only. The service account used to run the tensor flow services within the node. To share resources, including Google Cloud Storage data, with the Tensorflow job running in the Node, this account must have permissions to that data. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user-provided metadata. |
| `create_time` | String |  | Output only. The time when the node was created. |
| `parent` | String | ✅ | Required. The parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network` | String | The name of a network they wish to peer the TPU node to. It must be a preexisting Compute Engine network inside of the project on which this API has been activated. If none is provided, "default" will be used. |
| `symptoms` | Vec<String> | Output only. The Symptoms that have occurred to the TPU Node. |
| `description` | String | The user-supplied description of the TPU. Maximum of 512 characters. |
| `use_service_networking` | bool | Whether the VPC peering for the node is set up through Service Networking API. The VPC Peering should be set up before provisioning the node. If this field is set, cidr_block field should not be specified. If the network, that you want to peer the TPU Node to, is Shared VPC networks, the node must be created with this this field enabled. |
| `scheduling_config` | String | The scheduling options for this node. |
| `port` | String | Output only. DEPRECATED! Use network_endpoints instead. The network port for the TPU Node as visible to Compute Engine instances. |
| `health_description` | String | Output only. If this field is populated, it contains a description of why the TPU Node is unhealthy. |
| `network_endpoints` | Vec<String> | Output only. The network endpoints where TPU workers can be accessed and sent work. It is recommended that Tensorflow clients of the node reach out to the 0th entry in this map first. |
| `state` | String | Output only. The current state for the TPU Node. |
| `health` | String | The health status of the TPU node. |
| `cidr_block` | String | The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger block would be wasteful (a node can only consume one IP address). Errors will occur if the CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts with any subnetworks in the user's provided network, or the provided network is peered with another network that is using that CIDR block. |
| `tensorflow_version` | String | Required. The version of Tensorflow running in the Node. |
| `ip_address` | String | Output only. DEPRECATED! Use network_endpoints instead. The network address for the TPU Node as visible to Compute Engine instances. |
| `name` | String | Output only. Immutable. The name of the TPU |
| `api_version` | String | Output only. The API version that created this Node. |
| `accelerator_type` | String | Required. The type of hardware accelerators associated with this node. |
| `service_account` | String | Output only. The service account used to run the tensor flow services within the node. To share resources, including Google Cloud Storage data, with the Tensorflow job running in the Node, this account must have permissions to that data. |
| `labels` | HashMap<String, String> | Resource labels to represent user-provided metadata. |
| `create_time` | String | Output only. The time when the node was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create node
node = provider.tpu_api.Node {
    parent = "value"  # Required. The parent resource name.
}

# Access node outputs
node_id = node.id
node_network = node.network
node_symptoms = node.symptoms
node_description = node.description
node_use_service_networking = node.use_service_networking
node_scheduling_config = node.scheduling_config
node_port = node.port
node_health_description = node.health_description
node_network_endpoints = node.network_endpoints
node_state = node.state
node_health = node.health
node_cidr_block = node.cidr_block
node_tensorflow_version = node.tensorflow_version
node_ip_address = node.ip_address
node_name = node.name
node_api_version = node.api_version
node_accelerator_type = node.accelerator_type
node_service_account = node.service_account
node_labels = node.labels
node_create_time = node.create_time
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.tpu_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_response = operation.response
```

---


### Accelerator_type

Gets AcceleratorType.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name. |
| `type` | String | the accelerator type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access accelerator_type outputs
accelerator_type_id = accelerator_type.id
accelerator_type_name = accelerator_type.name
accelerator_type_type = accelerator_type.type
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_labels = location.labels
location_display_name = location.display_name
location_name = location.name
location_location_id = location.location_id
location_metadata = location.metadata
```

---


### Accelerator_type

Gets AcceleratorType.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `accelerator_configs` | Vec<String> | The accelerator config. |
| `type` | String | The accelerator type. |
| `name` | String | The resource name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access accelerator_type outputs
accelerator_type_id = accelerator_type.id
accelerator_type_accelerator_configs = accelerator_type.accelerator_configs
accelerator_type_type = accelerator_type.type
accelerator_type_name = accelerator_type.name
```

---


### Node

Creates a node.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `symptoms` | Vec<String> |  | Output only. The Symptoms that have occurred to the TPU Node. |
| `tags` | Vec<String> |  | Tags to apply to the TPU Node. Tags are used to identify valid sources or targets for network firewalls. |
| `description` | String |  | The user-supplied description of the TPU. Maximum of 512 characters. |
| `boot_disk_config` | String |  | Optional. Boot disk configuration. |
| `data_disks` | Vec<String> |  | The additional data disks for the Node. |
| `health_description` | String |  | Output only. If this field is populated, it contains a description of why the TPU Node is unhealthy. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user-provided metadata. |
| `multislice_node` | bool |  | Output only. Whether the Node belongs to a Multislice group. |
| `network_config` | String |  | Network configurations for the TPU node. network_config and network_configs are mutually exclusive, you can only specify one of them. If both are specified, an error will be returned. |
| `queued_resource` | String |  | Output only. The qualified name of the QueuedResource that requested this Node. |
| `shielded_instance_config` | String |  | Shielded Instance options. |
| `accelerator_type` | String |  | Optional. The type of hardware accelerators associated with this node. |
| `api_version` | String |  | Output only. The API version that created this Node. |
| `create_time` | String |  | Output only. The time when the node was created. |
| `health` | String |  | The health status of the TPU node. |
| `network_endpoints` | Vec<String> |  | Output only. The network endpoints where TPU workers can be accessed and sent work. It is recommended that runtime clients of the node reach out to the 0th entry in this map first. |
| `upcoming_maintenance` | String |  | Output only. Upcoming maintenance on this TPU node. |
| `accelerator_config` | String |  | The AccleratorConfig for the TPU Node. |
| `id` | String |  | Output only. The unique identifier for the TPU Node. |
| `cidr_block` | String |  | The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger block would be wasteful (a node can only consume one IP address). Errors will occur if the CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts with any subnetworks in the user's provided network, or the provided network is peered with another network that is using that CIDR block. |
| `name` | String |  | Output only. Immutable. The name of the TPU. |
| `runtime_version` | String |  | Required. The runtime version running in the Node. |
| `scheduling_config` | String |  | The scheduling options for this node. |
| `network_configs` | Vec<String> |  | Optional. Repeated network configurations for the TPU node. This field is used to specify multiple networks configs for the TPU node. network_config and network_configs are mutually exclusive, you can only specify one of them. If both are specified, an error will be returned. |
| `service_account` | String |  | The Google Cloud Platform Service Account to be used by the TPU node VMs. If None is specified, the default compute service account will be used. |
| `state` | String |  | Output only. The current state for the TPU Node. |
| `metadata` | HashMap<String, String> |  | Custom metadata to apply to the TPU Node. Can set startup-script and shutdown-script |
| `parent` | String | ✅ | Required. The parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `symptoms` | Vec<String> | Output only. The Symptoms that have occurred to the TPU Node. |
| `tags` | Vec<String> | Tags to apply to the TPU Node. Tags are used to identify valid sources or targets for network firewalls. |
| `description` | String | The user-supplied description of the TPU. Maximum of 512 characters. |
| `boot_disk_config` | String | Optional. Boot disk configuration. |
| `data_disks` | Vec<String> | The additional data disks for the Node. |
| `health_description` | String | Output only. If this field is populated, it contains a description of why the TPU Node is unhealthy. |
| `labels` | HashMap<String, String> | Resource labels to represent user-provided metadata. |
| `multislice_node` | bool | Output only. Whether the Node belongs to a Multislice group. |
| `network_config` | String | Network configurations for the TPU node. network_config and network_configs are mutually exclusive, you can only specify one of them. If both are specified, an error will be returned. |
| `queued_resource` | String | Output only. The qualified name of the QueuedResource that requested this Node. |
| `shielded_instance_config` | String | Shielded Instance options. |
| `accelerator_type` | String | Optional. The type of hardware accelerators associated with this node. |
| `api_version` | String | Output only. The API version that created this Node. |
| `create_time` | String | Output only. The time when the node was created. |
| `health` | String | The health status of the TPU node. |
| `network_endpoints` | Vec<String> | Output only. The network endpoints where TPU workers can be accessed and sent work. It is recommended that runtime clients of the node reach out to the 0th entry in this map first. |
| `upcoming_maintenance` | String | Output only. Upcoming maintenance on this TPU node. |
| `accelerator_config` | String | The AccleratorConfig for the TPU Node. |
| `id` | String | Output only. The unique identifier for the TPU Node. |
| `cidr_block` | String | The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger block would be wasteful (a node can only consume one IP address). Errors will occur if the CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts with any subnetworks in the user's provided network, or the provided network is peered with another network that is using that CIDR block. |
| `name` | String | Output only. Immutable. The name of the TPU. |
| `runtime_version` | String | Required. The runtime version running in the Node. |
| `scheduling_config` | String | The scheduling options for this node. |
| `network_configs` | Vec<String> | Optional. Repeated network configurations for the TPU node. This field is used to specify multiple networks configs for the TPU node. network_config and network_configs are mutually exclusive, you can only specify one of them. If both are specified, an error will be returned. |
| `service_account` | String | The Google Cloud Platform Service Account to be used by the TPU node VMs. If None is specified, the default compute service account will be used. |
| `state` | String | Output only. The current state for the TPU Node. |
| `metadata` | HashMap<String, String> | Custom metadata to apply to the TPU Node. Can set startup-script and shutdown-script |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create node
node = provider.tpu_api.Node {
    parent = "value"  # Required. The parent resource name.
}

# Access node outputs
node_id = node.id
node_symptoms = node.symptoms
node_tags = node.tags
node_description = node.description
node_boot_disk_config = node.boot_disk_config
node_data_disks = node.data_disks
node_health_description = node.health_description
node_labels = node.labels
node_multislice_node = node.multislice_node
node_network_config = node.network_config
node_queued_resource = node.queued_resource
node_shielded_instance_config = node.shielded_instance_config
node_accelerator_type = node.accelerator_type
node_api_version = node.api_version
node_create_time = node.create_time
node_health = node.health
node_network_endpoints = node.network_endpoints
node_upcoming_maintenance = node.upcoming_maintenance
node_accelerator_config = node.accelerator_config
node_id = node.id
node_cidr_block = node.cidr_block
node_name = node.name
node_runtime_version = node.runtime_version
node_scheduling_config = node.scheduling_config
node_network_configs = node.network_configs
node_service_account = node.service_account
node_state = node.state
node_metadata = node.metadata
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.tpu_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
```

---


### Queued_resource

Creates a QueuedResource TPU instance.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `spot` | String |  | Optional. The Spot tier. |
| `tpu` | String |  | Optional. Defines a TPU resource. |
| `create_time` | String |  | Output only. The time when the QueuedResource was created. |
| `state` | String |  | Output only. State of the QueuedResource request. |
| `queueing_policy` | String |  | Optional. The queueing policy of the QueuedRequest. |
| `reservation_name` | String |  | Optional. Name of the reservation in which the resource should be provisioned. Format: projects/{project}/locations/{zone}/reservations/{reservation} |
| `guaranteed` | String |  | Optional. The Guaranteed tier |
| `name` | String |  | Output only. Immutable. The name of the QueuedResource. |
| `parent` | String | ✅ | Required. The parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spot` | String | Optional. The Spot tier. |
| `tpu` | String | Optional. Defines a TPU resource. |
| `create_time` | String | Output only. The time when the QueuedResource was created. |
| `state` | String | Output only. State of the QueuedResource request. |
| `queueing_policy` | String | Optional. The queueing policy of the QueuedRequest. |
| `reservation_name` | String | Optional. Name of the reservation in which the resource should be provisioned. Format: projects/{project}/locations/{zone}/reservations/{reservation} |
| `guaranteed` | String | Optional. The Guaranteed tier |
| `name` | String | Output only. Immutable. The name of the QueuedResource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create queued_resource
queued_resource = provider.tpu_api.Queued_resource {
    parent = "value"  # Required. The parent resource name.
}

# Access queued_resource outputs
queued_resource_id = queued_resource.id
queued_resource_spot = queued_resource.spot
queued_resource_tpu = queued_resource.tpu
queued_resource_create_time = queued_resource.create_time
queued_resource_state = queued_resource.state
queued_resource_queueing_policy = queued_resource.queueing_policy
queued_resource_reservation_name = queued_resource.reservation_name
queued_resource_guaranteed = queued_resource.guaranteed
queued_resource_name = queued_resource.name
```

---


### Location

Generates the Cloud TPU service identity for the project.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. The parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.tpu_api.Location {
    parent = "value"  # Required. The parent resource name.
}

# Access location outputs
location_id = location.id
location_display_name = location.display_name
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
```

---


### Runtime_version

Gets a runtime version.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name. |
| `version` | String | The runtime version. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime_version outputs
runtime_version_id = runtime_version.id
runtime_version_name = runtime_version.name
runtime_version_version = runtime_version.version
```

---


### Node

Creates a node.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `symptoms` | Vec<String> |  | Output only. The Symptoms that have occurred to the TPU Node. |
| `port` | String |  | Output only. DEPRECATED! Use network_endpoints instead. The network port for the TPU Node as visible to Compute Engine instances. |
| `create_time` | String |  | Output only. The time when the node was created. |
| `api_version` | String |  | Output only. The API version that created this Node. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user-provided metadata. |
| `cidr_block` | String |  | The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger block would be wasteful (a node can only consume one IP address). Errors will occur if the CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts with any subnetworks in the user's provided network, or the provided network is peered with another network that is using that CIDR block. |
| `health_description` | String |  | Output only. If this field is populated, it contains a description of why the TPU Node is unhealthy. |
| `health` | String |  | The health status of the TPU node. |
| `use_service_networking` | bool |  | Whether the VPC peering for the node is set up through Service Networking API. The VPC Peering should be set up before provisioning the node. If this field is set, cidr_block field should not be specified. If the network, that you want to peer the TPU Node to, is Shared VPC networks, the node must be created with this this field enabled. |
| `service_account` | String |  | Output only. The service account used to run the tensor flow services within the node. To share resources, including Google Cloud Storage data, with the Tensorflow job running in the Node, this account must have permissions to that data. |
| `description` | String |  | The user-supplied description of the TPU. Maximum of 512 characters. |
| `network_endpoints` | Vec<String> |  | Output only. The network endpoints where TPU workers can be accessed and sent work. It is recommended that Tensorflow clients of the node reach out to the 0th entry in this map first. |
| `network` | String |  | The name of a network they wish to peer the TPU node to. It must be a preexisting Compute Engine network inside of the project on which this API has been activated. If none is provided, "default" will be used. |
| `accelerator_type` | String |  | Required. The type of hardware accelerators associated with this node. |
| `state` | String |  | Output only. The current state for the TPU Node. |
| `name` | String |  | Output only. Immutable. The name of the TPU |
| `ip_address` | String |  | Output only. DEPRECATED! Use network_endpoints instead. The network address for the TPU Node as visible to Compute Engine instances. |
| `scheduling_config` | String |  | The scheduling options for this node. |
| `tensorflow_version` | String |  | Required. The version of Tensorflow running in the Node. |
| `parent` | String | ✅ | Required. The parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `symptoms` | Vec<String> | Output only. The Symptoms that have occurred to the TPU Node. |
| `port` | String | Output only. DEPRECATED! Use network_endpoints instead. The network port for the TPU Node as visible to Compute Engine instances. |
| `create_time` | String | Output only. The time when the node was created. |
| `api_version` | String | Output only. The API version that created this Node. |
| `labels` | HashMap<String, String> | Resource labels to represent user-provided metadata. |
| `cidr_block` | String | The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger block would be wasteful (a node can only consume one IP address). Errors will occur if the CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts with any subnetworks in the user's provided network, or the provided network is peered with another network that is using that CIDR block. |
| `health_description` | String | Output only. If this field is populated, it contains a description of why the TPU Node is unhealthy. |
| `health` | String | The health status of the TPU node. |
| `use_service_networking` | bool | Whether the VPC peering for the node is set up through Service Networking API. The VPC Peering should be set up before provisioning the node. If this field is set, cidr_block field should not be specified. If the network, that you want to peer the TPU Node to, is Shared VPC networks, the node must be created with this this field enabled. |
| `service_account` | String | Output only. The service account used to run the tensor flow services within the node. To share resources, including Google Cloud Storage data, with the Tensorflow job running in the Node, this account must have permissions to that data. |
| `description` | String | The user-supplied description of the TPU. Maximum of 512 characters. |
| `network_endpoints` | Vec<String> | Output only. The network endpoints where TPU workers can be accessed and sent work. It is recommended that Tensorflow clients of the node reach out to the 0th entry in this map first. |
| `network` | String | The name of a network they wish to peer the TPU node to. It must be a preexisting Compute Engine network inside of the project on which this API has been activated. If none is provided, "default" will be used. |
| `accelerator_type` | String | Required. The type of hardware accelerators associated with this node. |
| `state` | String | Output only. The current state for the TPU Node. |
| `name` | String | Output only. Immutable. The name of the TPU |
| `ip_address` | String | Output only. DEPRECATED! Use network_endpoints instead. The network address for the TPU Node as visible to Compute Engine instances. |
| `scheduling_config` | String | The scheduling options for this node. |
| `tensorflow_version` | String | Required. The version of Tensorflow running in the Node. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create node
node = provider.tpu_api.Node {
    parent = "value"  # Required. The parent resource name.
}

# Access node outputs
node_id = node.id
node_symptoms = node.symptoms
node_port = node.port
node_create_time = node.create_time
node_api_version = node.api_version
node_labels = node.labels
node_cidr_block = node.cidr_block
node_health_description = node.health_description
node_health = node.health
node_use_service_networking = node.use_service_networking
node_service_account = node.service_account
node_description = node.description
node_network_endpoints = node.network_endpoints
node_network = node.network
node_accelerator_type = node.accelerator_type
node_state = node.state
node_name = node.name
node_ip_address = node.ip_address
node_scheduling_config = node.scheduling_config
node_tensorflow_version = node.tensorflow_version
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_display_name = location.display_name
location_name = location.name
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
```

---


### Accelerator_type

Gets AcceleratorType.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | the accelerator type. |
| `name` | String | The resource name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access accelerator_type outputs
accelerator_type_id = accelerator_type.id
accelerator_type_type = accelerator_type.type
accelerator_type_name = accelerator_type.name
```

---


### Tensorflow_version

Gets TensorFlow Version.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | String | the tensorflow version. |
| `name` | String | The resource name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access tensorflow_version outputs
tensorflow_version_id = tensorflow_version.id
tensorflow_version_version = tensorflow_version.version
tensorflow_version_name = tensorflow_version.name
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

# Create operation
operation = provider.tpu_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_response = operation.response
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Node

Creates a node.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cidr_block` | String |  | The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger block would be wasteful (a node can only consume one IP address). Errors will occur if the CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts with any subnetworks in the user's provided network, or the provided network is peered with another network that is using that CIDR block. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user-provided metadata. |
| `accelerator_type` | String |  | The type of hardware accelerators associated with this node. |
| `api_version` | String |  | Output only. The API version that created this Node. |
| `description` | String |  | The user-supplied description of the TPU. Maximum of 512 characters. |
| `health_description` | String |  | Output only. If this field is populated, it contains a description of why the TPU Node is unhealthy. |
| `health` | String |  | The health status of the TPU node. |
| `metadata` | HashMap<String, String> |  | Custom metadata to apply to the TPU Node. Can set startup-script and shutdown-script |
| `multislice_node` | bool |  | Output only. Whether the Node belongs to a Multislice group. |
| `name` | String |  | Output only. Immutable. The name of the TPU. |
| `network_configs` | Vec<String> |  | Optional. Repeated network configurations for the TPU node. This field is used to specify multiple networks configs for the TPU node. network_config and network_configs are mutually exclusive, you can only specify one of them. If both are specified, an error will be returned. |
| `queued_resource` | String |  | Output only. The qualified name of the QueuedResource that requested this Node. |
| `network_config` | String |  | Network configurations for the TPU node. network_config and network_configs are mutually exclusive, you can only specify one of them. If both are specified, an error will be returned. |
| `id` | String |  | Output only. The unique identifier for the TPU Node. |
| `runtime_version` | String |  | Required. The runtime version running in the Node. |
| `service_account` | String |  | The Google Cloud Platform Service Account to be used by the TPU node VMs. If None is specified, the default compute service account will be used. |
| `accelerator_config` | String |  | The AccleratorConfig for the TPU Node. |
| `network_endpoints` | Vec<String> |  | Output only. The network endpoints where TPU workers can be accessed and sent work. It is recommended that runtime clients of the node reach out to the 0th entry in this map first. |
| `boot_disk_config` | String |  | Optional. Boot disk configuration. |
| `scheduling_config` | String |  | The scheduling options for this node. |
| `data_disks` | Vec<String> |  | The additional data disks for the Node. |
| `create_time` | String |  | Output only. The time when the node was created. |
| `shielded_instance_config` | String |  | Shielded Instance options. |
| `state` | String |  | Output only. The current state for the TPU Node. |
| `symptoms` | Vec<String> |  | Output only. The Symptoms that have occurred to the TPU Node. |
| `tags` | Vec<String> |  | Tags to apply to the TPU Node. Tags are used to identify valid sources or targets for network firewalls. |
| `upcoming_maintenance` | String |  | Output only. Upcoming maintenance on this TPU node. |
| `autocheckpoint_enabled` | bool |  | Optional. Whether Autocheckpoint is enabled. |
| `parent` | String | ✅ | Required. The parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cidr_block` | String | The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger block would be wasteful (a node can only consume one IP address). Errors will occur if the CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts with any subnetworks in the user's provided network, or the provided network is peered with another network that is using that CIDR block. |
| `labels` | HashMap<String, String> | Resource labels to represent user-provided metadata. |
| `accelerator_type` | String | The type of hardware accelerators associated with this node. |
| `api_version` | String | Output only. The API version that created this Node. |
| `description` | String | The user-supplied description of the TPU. Maximum of 512 characters. |
| `health_description` | String | Output only. If this field is populated, it contains a description of why the TPU Node is unhealthy. |
| `health` | String | The health status of the TPU node. |
| `metadata` | HashMap<String, String> | Custom metadata to apply to the TPU Node. Can set startup-script and shutdown-script |
| `multislice_node` | bool | Output only. Whether the Node belongs to a Multislice group. |
| `name` | String | Output only. Immutable. The name of the TPU. |
| `network_configs` | Vec<String> | Optional. Repeated network configurations for the TPU node. This field is used to specify multiple networks configs for the TPU node. network_config and network_configs are mutually exclusive, you can only specify one of them. If both are specified, an error will be returned. |
| `queued_resource` | String | Output only. The qualified name of the QueuedResource that requested this Node. |
| `network_config` | String | Network configurations for the TPU node. network_config and network_configs are mutually exclusive, you can only specify one of them. If both are specified, an error will be returned. |
| `id` | String | Output only. The unique identifier for the TPU Node. |
| `runtime_version` | String | Required. The runtime version running in the Node. |
| `service_account` | String | The Google Cloud Platform Service Account to be used by the TPU node VMs. If None is specified, the default compute service account will be used. |
| `accelerator_config` | String | The AccleratorConfig for the TPU Node. |
| `network_endpoints` | Vec<String> | Output only. The network endpoints where TPU workers can be accessed and sent work. It is recommended that runtime clients of the node reach out to the 0th entry in this map first. |
| `boot_disk_config` | String | Optional. Boot disk configuration. |
| `scheduling_config` | String | The scheduling options for this node. |
| `data_disks` | Vec<String> | The additional data disks for the Node. |
| `create_time` | String | Output only. The time when the node was created. |
| `shielded_instance_config` | String | Shielded Instance options. |
| `state` | String | Output only. The current state for the TPU Node. |
| `symptoms` | Vec<String> | Output only. The Symptoms that have occurred to the TPU Node. |
| `tags` | Vec<String> | Tags to apply to the TPU Node. Tags are used to identify valid sources or targets for network firewalls. |
| `upcoming_maintenance` | String | Output only. Upcoming maintenance on this TPU node. |
| `autocheckpoint_enabled` | bool | Optional. Whether Autocheckpoint is enabled. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create node
node = provider.tpu_api.Node {
    parent = "value"  # Required. The parent resource name.
}

# Access node outputs
node_id = node.id
node_cidr_block = node.cidr_block
node_labels = node.labels
node_accelerator_type = node.accelerator_type
node_api_version = node.api_version
node_description = node.description
node_health_description = node.health_description
node_health = node.health
node_metadata = node.metadata
node_multislice_node = node.multislice_node
node_name = node.name
node_network_configs = node.network_configs
node_queued_resource = node.queued_resource
node_network_config = node.network_config
node_id = node.id
node_runtime_version = node.runtime_version
node_service_account = node.service_account
node_accelerator_config = node.accelerator_config
node_network_endpoints = node.network_endpoints
node_boot_disk_config = node.boot_disk_config
node_scheduling_config = node.scheduling_config
node_data_disks = node.data_disks
node_create_time = node.create_time
node_shielded_instance_config = node.shielded_instance_config
node_state = node.state
node_symptoms = node.symptoms
node_tags = node.tags
node_upcoming_maintenance = node.upcoming_maintenance
node_autocheckpoint_enabled = node.autocheckpoint_enabled
```

---


### Reservation

Retrieves the reservations for the given project in the given location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The next page token or empty if none. |
| `reservations` | Vec<String> | The listed reservations. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access reservation outputs
reservation_id = reservation.id
reservation_next_page_token = reservation.next_page_token
reservation_reservations = reservation.reservations
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.tpu_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
```

---


### Location

Generates the Cloud TPU service identity for the project.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. The parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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

# Create location
location = provider.tpu_api.Location {
    parent = "value"  # Required. The parent resource name.
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
location_display_name = location.display_name
location_name = location.name
```

---


### Accelerator_type

Gets AcceleratorType.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | The accelerator type. |
| `accelerator_configs` | Vec<String> | The accelerator config. |
| `name` | String | The resource name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access accelerator_type outputs
accelerator_type_id = accelerator_type.id
accelerator_type_type = accelerator_type.type
accelerator_type_accelerator_configs = accelerator_type.accelerator_configs
accelerator_type_name = accelerator_type.name
```

---


### Queued_resource

Creates a QueuedResource TPU instance.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Immutable. The name of the QueuedResource. |
| `create_time` | String |  | Output only. The time when the QueuedResource was created. |
| `state` | String |  | Output only. State of the QueuedResource request. |
| `best_effort` | String |  | The BestEffort tier. |
| `spot` | String |  | Optional. The Spot tier. |
| `queueing_policy` | String |  | The queueing policy of the QueuedRequest. |
| `tpu` | String |  | Defines a TPU resource. |
| `reservation_name` | String |  | Name of the reservation in which the resource should be provisioned. Format: projects/{project}/locations/{zone}/reservations/{reservation} |
| `run_duration` | String |  | Optional. The duration of the requested resource. |
| `guaranteed` | String |  | The Guaranteed tier. |
| `provisioning_model` | String |  | Optional. The provisioning model for the resource. |
| `parent` | String | ✅ | Required. The parent resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Immutable. The name of the QueuedResource. |
| `create_time` | String | Output only. The time when the QueuedResource was created. |
| `state` | String | Output only. State of the QueuedResource request. |
| `best_effort` | String | The BestEffort tier. |
| `spot` | String | Optional. The Spot tier. |
| `queueing_policy` | String | The queueing policy of the QueuedRequest. |
| `tpu` | String | Defines a TPU resource. |
| `reservation_name` | String | Name of the reservation in which the resource should be provisioned. Format: projects/{project}/locations/{zone}/reservations/{reservation} |
| `run_duration` | String | Optional. The duration of the requested resource. |
| `guaranteed` | String | The Guaranteed tier. |
| `provisioning_model` | String | Optional. The provisioning model for the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create queued_resource
queued_resource = provider.tpu_api.Queued_resource {
    parent = "value"  # Required. The parent resource name.
}

# Access queued_resource outputs
queued_resource_id = queued_resource.id
queued_resource_name = queued_resource.name
queued_resource_create_time = queued_resource.create_time
queued_resource_state = queued_resource.state
queued_resource_best_effort = queued_resource.best_effort
queued_resource_spot = queued_resource.spot
queued_resource_queueing_policy = queued_resource.queueing_policy
queued_resource_tpu = queued_resource.tpu
queued_resource_reservation_name = queued_resource.reservation_name
queued_resource_run_duration = queued_resource.run_duration
queued_resource_guaranteed = queued_resource.guaranteed
queued_resource_provisioning_model = queued_resource.provisioning_model
```

---


### Runtime_version

Gets a runtime version.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name. |
| `version` | String | The runtime version. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime_version outputs
runtime_version_id = runtime_version.id
runtime_version_name = runtime_version.name
runtime_version_version = runtime_version.version
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple tensorflow_version resources
tensorflow_version_0 = provider.tpu_api.Tensorflow_version {
}
tensorflow_version_1 = provider.tpu_api.Tensorflow_version {
}
tensorflow_version_2 = provider.tpu_api.Tensorflow_version {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    tensorflow_version = provider.tpu_api.Tensorflow_version {
    }
```

---

## Related Documentation

- [GCP Tpu_api Documentation](https://cloud.google.com/tpu_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
