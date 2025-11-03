# Parallelstore_api Service



**Resources**: 6

---

## Overview

The parallelstore_api service provides access to 6 resource types:

- [Instance](#instance) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [Instance](#instance) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]

---

## Resources


### Instance

Creates a Parallelstore instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. The description of the instance. 2048 characters or less. |
| `name` | String |  | Identifier. The resource name of the instance, in the format `projects/{project}/locations/{location}/instances/{instance_id}`. |
| `network` | String |  | Optional. Immutable. The name of the Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. |
| `deployment_type` | String |  | Optional. Immutable. The deployment type of the instance. Allowed values are: * `SCRATCH`: the instance is a scratch instance. * `PERSISTENT`: the instance is a persistent instance. |
| `update_time` | String |  | Output only. The time when the instance was updated. |
| `reserved_ip_range` | String |  | Optional. Immutable. The ID of the IP address range being used by the instance's VPC network. See [Configure a VPC network](https://cloud.google.com/parallelstore/docs/vpc#create_and_configure_the_vpc). If no ID is provided, all ranges are considered. |
| `file_stripe_level` | String |  | Optional. Immutable. Stripe level for files. Allowed values are: * `FILE_STRIPE_LEVEL_MIN`: offers the best performance for small size files. * `FILE_STRIPE_LEVEL_BALANCED`: balances performance for workloads involving a mix of small and large files. * `FILE_STRIPE_LEVEL_MAX`: higher throughput performance for larger files. |
| `create_time` | String |  | Output only. The time when the instance was created. |
| `directory_stripe_level` | String |  | Optional. Immutable. Stripe level for directories. Allowed values are: * `DIRECTORY_STRIPE_LEVEL_MIN`: recommended when directories contain a small number of files. * `DIRECTORY_STRIPE_LEVEL_BALANCED`: balances performance for workloads involving a mix of small and large directories. * `DIRECTORY_STRIPE_LEVEL_MAX`: recommended for directories with a large number of files. |
| `capacity_gib` | String |  | Required. Immutable. The instance's storage capacity in Gibibytes (GiB). Allowed values are between 12000 and 100000, in multiples of 4000; e.g., 12000, 16000, 20000, ... |
| `effective_reserved_ip_range` | String |  | Output only. Immutable. The ID of the IP address range being used by the instance's VPC network. This field is populated by the service and contains the value currently used by the service. |
| `labels` | HashMap<String, String> |  | Optional. Cloud Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. See https://cloud.google.com/resource-manager/docs/labels-overview for details. |
| `access_points` | Vec<String> |  | Output only. A list of IPv4 addresses used for client side configuration. |
| `daos_version` | String |  | Output only. Deprecated: The version of DAOS software running in the instance. |
| `state` | String |  | Output only. The instance state. |
| `parent` | String | ✅ | Required. The instance's project and location, in the format `projects/{project}/locations/{location}`. Locations map to Google Cloud zones; for example, `us-west1-b`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. The description of the instance. 2048 characters or less. |
| `name` | String | Identifier. The resource name of the instance, in the format `projects/{project}/locations/{location}/instances/{instance_id}`. |
| `network` | String | Optional. Immutable. The name of the Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. |
| `deployment_type` | String | Optional. Immutable. The deployment type of the instance. Allowed values are: * `SCRATCH`: the instance is a scratch instance. * `PERSISTENT`: the instance is a persistent instance. |
| `update_time` | String | Output only. The time when the instance was updated. |
| `reserved_ip_range` | String | Optional. Immutable. The ID of the IP address range being used by the instance's VPC network. See [Configure a VPC network](https://cloud.google.com/parallelstore/docs/vpc#create_and_configure_the_vpc). If no ID is provided, all ranges are considered. |
| `file_stripe_level` | String | Optional. Immutable. Stripe level for files. Allowed values are: * `FILE_STRIPE_LEVEL_MIN`: offers the best performance for small size files. * `FILE_STRIPE_LEVEL_BALANCED`: balances performance for workloads involving a mix of small and large files. * `FILE_STRIPE_LEVEL_MAX`: higher throughput performance for larger files. |
| `create_time` | String | Output only. The time when the instance was created. |
| `directory_stripe_level` | String | Optional. Immutable. Stripe level for directories. Allowed values are: * `DIRECTORY_STRIPE_LEVEL_MIN`: recommended when directories contain a small number of files. * `DIRECTORY_STRIPE_LEVEL_BALANCED`: balances performance for workloads involving a mix of small and large directories. * `DIRECTORY_STRIPE_LEVEL_MAX`: recommended for directories with a large number of files. |
| `capacity_gib` | String | Required. Immutable. The instance's storage capacity in Gibibytes (GiB). Allowed values are between 12000 and 100000, in multiples of 4000; e.g., 12000, 16000, 20000, ... |
| `effective_reserved_ip_range` | String | Output only. Immutable. The ID of the IP address range being used by the instance's VPC network. This field is populated by the service and contains the value currently used by the service. |
| `labels` | HashMap<String, String> | Optional. Cloud Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. See https://cloud.google.com/resource-manager/docs/labels-overview for details. |
| `access_points` | Vec<String> | Output only. A list of IPv4 addresses used for client side configuration. |
| `daos_version` | String | Output only. Deprecated: The version of DAOS software running in the instance. |
| `state` | String | Output only. The instance state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.parallelstore_api.Instance {
    parent = "value"  # Required. The instance's project and location, in the format `projects/{project}/locations/{location}`. Locations map to Google Cloud zones; for example, `us-west1-b`.
}

# Access instance outputs
instance_id = instance.id
instance_description = instance.description
instance_name = instance.name
instance_network = instance.network
instance_deployment_type = instance.deployment_type
instance_update_time = instance.update_time
instance_reserved_ip_range = instance.reserved_ip_range
instance_file_stripe_level = instance.file_stripe_level
instance_create_time = instance.create_time
instance_directory_stripe_level = instance.directory_stripe_level
instance_capacity_gib = instance.capacity_gib
instance_effective_reserved_ip_range = instance.effective_reserved_ip_range
instance_labels = instance.labels
instance_access_points = instance.access_points
instance_daos_version = instance.daos_version
instance_state = instance.state
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
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
operation = provider.parallelstore_api.Operation {
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


### Instance

Creates a Parallelstore instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the instance was created. |
| `labels` | HashMap<String, String> |  | Optional. Cloud Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. See https://cloud.google.com/resource-manager/docs/labels-overview for details. |
| `deployment_type` | String |  | Optional. Immutable. The deployment type of the instance. Allowed values are: * `SCRATCH`: the instance is a scratch instance. * `PERSISTENT`: the instance is a persistent instance. |
| `name` | String |  | Identifier. The resource name of the instance, in the format `projects/{project}/locations/{location}/instances/{instance_id}`. |
| `network` | String |  | Optional. Immutable. The name of the Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. |
| `state` | String |  | Output only. The instance state. |
| `reserved_ip_range` | String |  | Optional. Immutable. The ID of the IP address range being used by the instance's VPC network. See [Configure a VPC network](https://cloud.google.com/parallelstore/docs/vpc#create_and_configure_the_vpc). If no ID is provided, all ranges are considered. |
| `access_points` | Vec<String> |  | Output only. A list of IPv4 addresses used for client side configuration. |
| `description` | String |  | Optional. The description of the instance. 2048 characters or less. |
| `effective_reserved_ip_range` | String |  | Output only. Immutable. The ID of the IP address range being used by the instance's VPC network. This field is populated by the service and contains the value currently used by the service. |
| `update_time` | String |  | Output only. The time when the instance was updated. |
| `file_stripe_level` | String |  | Optional. Immutable. Stripe level for files. Allowed values are: * `FILE_STRIPE_LEVEL_MIN`: offers the best performance for small size files. * `FILE_STRIPE_LEVEL_BALANCED`: balances performance for workloads involving a mix of small and large files. * `FILE_STRIPE_LEVEL_MAX`: higher throughput performance for larger files. |
| `directory_stripe_level` | String |  | Optional. Immutable. Stripe level for directories. Allowed values are: * `DIRECTORY_STRIPE_LEVEL_MIN`: recommended when directories contain a small number of files. * `DIRECTORY_STRIPE_LEVEL_BALANCED`: balances performance for workloads involving a mix of small and large directories. * `DIRECTORY_STRIPE_LEVEL_MAX`: recommended for directories with a large number of files. |
| `capacity_gib` | String |  | Required. Immutable. The instance's storage capacity in Gibibytes (GiB). Allowed values are between 12000 and 100000, in multiples of 4000; e.g., 12000, 16000, 20000, ... |
| `daos_version` | String |  | Output only. Deprecated: The version of DAOS software running in the instance. |
| `parent` | String | ✅ | Required. The instance's project and location, in the format `projects/{project}/locations/{location}`. Locations map to Google Cloud zones; for example, `us-west1-b`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the instance was created. |
| `labels` | HashMap<String, String> | Optional. Cloud Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. See https://cloud.google.com/resource-manager/docs/labels-overview for details. |
| `deployment_type` | String | Optional. Immutable. The deployment type of the instance. Allowed values are: * `SCRATCH`: the instance is a scratch instance. * `PERSISTENT`: the instance is a persistent instance. |
| `name` | String | Identifier. The resource name of the instance, in the format `projects/{project}/locations/{location}/instances/{instance_id}`. |
| `network` | String | Optional. Immutable. The name of the Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. |
| `state` | String | Output only. The instance state. |
| `reserved_ip_range` | String | Optional. Immutable. The ID of the IP address range being used by the instance's VPC network. See [Configure a VPC network](https://cloud.google.com/parallelstore/docs/vpc#create_and_configure_the_vpc). If no ID is provided, all ranges are considered. |
| `access_points` | Vec<String> | Output only. A list of IPv4 addresses used for client side configuration. |
| `description` | String | Optional. The description of the instance. 2048 characters or less. |
| `effective_reserved_ip_range` | String | Output only. Immutable. The ID of the IP address range being used by the instance's VPC network. This field is populated by the service and contains the value currently used by the service. |
| `update_time` | String | Output only. The time when the instance was updated. |
| `file_stripe_level` | String | Optional. Immutable. Stripe level for files. Allowed values are: * `FILE_STRIPE_LEVEL_MIN`: offers the best performance for small size files. * `FILE_STRIPE_LEVEL_BALANCED`: balances performance for workloads involving a mix of small and large files. * `FILE_STRIPE_LEVEL_MAX`: higher throughput performance for larger files. |
| `directory_stripe_level` | String | Optional. Immutable. Stripe level for directories. Allowed values are: * `DIRECTORY_STRIPE_LEVEL_MIN`: recommended when directories contain a small number of files. * `DIRECTORY_STRIPE_LEVEL_BALANCED`: balances performance for workloads involving a mix of small and large directories. * `DIRECTORY_STRIPE_LEVEL_MAX`: recommended for directories with a large number of files. |
| `capacity_gib` | String | Required. Immutable. The instance's storage capacity in Gibibytes (GiB). Allowed values are between 12000 and 100000, in multiples of 4000; e.g., 12000, 16000, 20000, ... |
| `daos_version` | String | Output only. Deprecated: The version of DAOS software running in the instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.parallelstore_api.Instance {
    parent = "value"  # Required. The instance's project and location, in the format `projects/{project}/locations/{location}`. Locations map to Google Cloud zones; for example, `us-west1-b`.
}

# Access instance outputs
instance_id = instance.id
instance_create_time = instance.create_time
instance_labels = instance.labels
instance_deployment_type = instance.deployment_type
instance_name = instance.name
instance_network = instance.network
instance_state = instance.state
instance_reserved_ip_range = instance.reserved_ip_range
instance_access_points = instance.access_points
instance_description = instance.description
instance_effective_reserved_ip_range = instance.effective_reserved_ip_range
instance_update_time = instance.update_time
instance_file_stripe_level = instance.file_stripe_level
instance_directory_stripe_level = instance.directory_stripe_level
instance_capacity_gib = instance.capacity_gib
instance_daos_version = instance.daos_version
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
operation = provider.parallelstore_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_location_id = location.location_id
location_labels = location.labels
location_metadata = location.metadata
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple instance resources
instance_0 = provider.parallelstore_api.Instance {
    parent = "value-0"
}
instance_1 = provider.parallelstore_api.Instance {
    parent = "value-1"
}
instance_2 = provider.parallelstore_api.Instance {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    instance = provider.parallelstore_api.Instance {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Parallelstore_api Documentation](https://cloud.google.com/parallelstore_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
