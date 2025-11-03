# Prod_tt_sasportal_api Service



**Resources**: 6

---

## Overview

The prod_tt_sasportal_api service provides access to 6 resource types:

- [Device](#device) [CRUD]
- [Policie](#policie) [CR]
- [Deployment](#deployment) [CRUD]
- [Customer](#customer) [CRU]
- [Node](#node) [CRUD]
- [Installer](#installer) [C]

---

## Resources


### Device

Creates a device under a node or customer.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource path name. |
| `serial_number` | String |  | A serial number assigned to the device by the device manufacturer. |
| `grants` | Vec<String> |  | Output only. Grants held by the device. |
| `preloaded_config` | String |  | Configuration of the device, as specified via SAS Portal API. |
| `current_channels` | Vec<String> |  | Output only. Current channels with scores. |
| `state` | String |  | Output only. Device state. |
| `device_metadata` | String |  | Device parameters that can be overridden by both SAS Portal and SAS registration requests. |
| `grant_range_allowlists` | Vec<String> |  | Only ranges that are within the allowlists are available for new grants. |
| `active_config` | String |  | Output only. Current configuration of the device as registered to the SAS. |
| `display_name` | String |  | Device display name. |
| `fcc_id` | String |  | The FCC identifier of the device. Refer to https://www.fcc.gov/oet/ea/fccid for FccID format. Accept underscores and periods because some test-SAS customers use them. |
| `parent` | String | ✅ | Required. The name of the parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource path name. |
| `serial_number` | String | A serial number assigned to the device by the device manufacturer. |
| `grants` | Vec<String> | Output only. Grants held by the device. |
| `preloaded_config` | String | Configuration of the device, as specified via SAS Portal API. |
| `current_channels` | Vec<String> | Output only. Current channels with scores. |
| `state` | String | Output only. Device state. |
| `device_metadata` | String | Device parameters that can be overridden by both SAS Portal and SAS registration requests. |
| `grant_range_allowlists` | Vec<String> | Only ranges that are within the allowlists are available for new grants. |
| `active_config` | String | Output only. Current configuration of the device as registered to the SAS. |
| `display_name` | String | Device display name. |
| `fcc_id` | String | The FCC identifier of the device. Refer to https://www.fcc.gov/oet/ea/fccid for FccID format. Accept underscores and periods because some test-SAS customers use them. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device
device = provider.prod_tt_sasportal_api.Device {
    parent = "value"  # Required. The name of the parent resource.
}

# Access device outputs
device_id = device.id
device_name = device.name
device_serial_number = device.serial_number
device_grants = device.grants
device_preloaded_config = device.preloaded_config
device_current_channels = device.current_channels
device_state = device.state
device_device_metadata = device.device_metadata
device_grant_range_allowlists = device.grant_range_allowlists
device_active_config = device.active_config
device_display_name = device.display_name
device_fcc_id = device.fcc_id
```

---


### Policie

Returns permissions that a caller has on the specified resource.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource` | String |  | Required. The resource for which the permissions are being requested. |
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assignments` | Vec<String> | List of assignments |
| `etag` | String | The etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to GetPolicy, and systems are expected to put that etag in the request to SetPolicy to ensure that their change will be applied to the same version of the policy. If no etag is provided in the call to GetPolicy, then the existing policy is overwritten blindly. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.prod_tt_sasportal_api.Policie {
}

# Access policie outputs
policie_id = policie.id
policie_assignments = policie.assignments
policie_etag = policie.etag
```

---


### Deployment

Creates a new deployment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name. |
| `display_name` | String |  | The deployment's display name. |
| `frns` | Vec<String> |  | Output only. The FCC Registration Numbers (FRNs) copied from its direct parent. |
| `sas_user_ids` | Vec<String> |  | User ID used by the devices belonging to this deployment. Each deployment should be associated with one unique user ID. |
| `parent` | String | ✅ | Required. The parent resource name where the deployment is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name. |
| `display_name` | String | The deployment's display name. |
| `frns` | Vec<String> | Output only. The FCC Registration Numbers (FRNs) copied from its direct parent. |
| `sas_user_ids` | Vec<String> | User ID used by the devices belonging to this deployment. Each deployment should be associated with one unique user ID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment
deployment = provider.prod_tt_sasportal_api.Deployment {
    parent = "value"  # Required. The parent resource name where the deployment is to be created.
}

# Access deployment outputs
deployment_id = deployment.id
deployment_name = deployment.name
deployment_display_name = deployment.display_name
deployment_frns = deployment.frns
deployment_sas_user_ids = deployment.sas_user_ids
```

---


### Customer

Creates a new SAS deployment through the GCP workflow. Creates a SAS organization if an organization match is not found.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_id` | String |  | Optional. If this field is set then a new deployment will be created under the organization specified by this id. |
| `new_deployment_display_name` | String |  | Optional. If this field is set, and a new SAS Portal Deployment needs to be created, its display name will be set to the value of this field. |
| `new_organization_display_name` | String |  | Optional. If this field is set, and a new SAS Portal Organization needs to be created, its display name will be set to the value of this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sas_user_ids` | Vec<String> | User IDs used by the devices belonging to this customer. |
| `name` | String | Output only. Resource name of the customer. |
| `display_name` | String | Required. Name of the organization that the customer entity represents. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create customer
customer = provider.prod_tt_sasportal_api.Customer {
}

# Access customer outputs
customer_id = customer.id
customer_sas_user_ids = customer.sas_user_ids
customer_name = customer.name
customer_display_name = customer.display_name
```

---


### Node

Creates a new node.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sas_user_ids` | Vec<String> |  | User ids used by the devices belonging to this node. |
| `name` | String |  | Output only. Resource name. |
| `display_name` | String |  | The node's display name. |
| `parent` | String | ✅ | Required. The parent resource name where the node is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sas_user_ids` | Vec<String> | User ids used by the devices belonging to this node. |
| `name` | String | Output only. Resource name. |
| `display_name` | String | The node's display name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create node
node = provider.prod_tt_sasportal_api.Node {
    parent = "value"  # Required. The parent resource name where the node is to be created.
}

# Access node outputs
node_id = node.id
node_sas_user_ids = node.sas_user_ids
node_name = node.name
node_display_name = node.display_name
```

---


### Installer

Generates a secret to be used with the ValidateInstaller.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create installer
installer = provider.prod_tt_sasportal_api.Installer {
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple device resources
device_0 = provider.prod_tt_sasportal_api.Device {
    parent = "value-0"
}
device_1 = provider.prod_tt_sasportal_api.Device {
    parent = "value-1"
}
device_2 = provider.prod_tt_sasportal_api.Device {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    device = provider.prod_tt_sasportal_api.Device {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Prod_tt_sasportal_api Documentation](https://cloud.google.com/prod_tt_sasportal_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
