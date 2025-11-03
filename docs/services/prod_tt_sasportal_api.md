# Prod_tt_sasportal_api Service



**Resources**: 6

---

## Overview

The prod_tt_sasportal_api service provides access to 6 resource types:

- [Deployment](#deployment) [CRUD]
- [Policie](#policie) [CR]
- [Installer](#installer) [C]
- [Customer](#customer) [CRU]
- [Node](#node) [CRUD]
- [Device](#device) [CRUD]

---

## Resources


### Deployment

Creates a new deployment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sas_user_ids` | Vec<String> |  | User ID used by the devices belonging to this deployment. Each deployment should be associated with one unique user ID. |
| `name` | String |  | Output only. Resource name. |
| `frns` | Vec<String> |  | Output only. The FCC Registration Numbers (FRNs) copied from its direct parent. |
| `display_name` | String |  | The deployment's display name. |
| `parent` | String | ✅ | Required. The parent resource name where the deployment is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sas_user_ids` | Vec<String> | User ID used by the devices belonging to this deployment. Each deployment should be associated with one unique user ID. |
| `name` | String | Output only. Resource name. |
| `frns` | Vec<String> | Output only. The FCC Registration Numbers (FRNs) copied from its direct parent. |
| `display_name` | String | The deployment's display name. |


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
deployment_sas_user_ids = deployment.sas_user_ids
deployment_name = deployment.name
deployment_frns = deployment.frns
deployment_display_name = deployment.display_name
```

---


### Policie

Returns permissions that a caller has on the specified resource.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. |
| `resource` | String |  | Required. The resource for which the permissions are being requested. |


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


### Installer

Validates the identity of a Certified Professional Installer (CPI).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encoded_secret` | String |  | Required. JSON Web Token signed using a CPI private key. Payload must include a "secret" claim whose value is the secret. |
| `installer_id` | String |  | Required. Unique installer id (CPI ID) from the Certified Professional Installers database. |
| `secret` | String |  | Required. Secret returned by the GenerateSecret. |



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


### Customer

Setups the a GCP Project to receive SAS Analytics messages via GCP Pub/Sub with a subscription to BigQuery. All the Pub/Sub topics and BigQuery tables are created automatically as part of this service.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String |  | Optional. User id to setup analytics for, if not provided the user id associated with the project is used. optional |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. Name of the organization that the customer entity represents. |
| `name` | String | Output only. Resource name of the customer. |
| `sas_user_ids` | Vec<String> | User IDs used by the devices belonging to this customer. |


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
customer_display_name = customer.display_name
customer_name = customer.name
customer_sas_user_ids = customer.sas_user_ids
```

---


### Node

Creates a new node.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The node's display name. |
| `name` | String |  | Output only. Resource name. |
| `sas_user_ids` | Vec<String> |  | User ids used by the devices belonging to this node. |
| `parent` | String | ✅ | Required. The parent resource name where the node is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The node's display name. |
| `name` | String | Output only. Resource name. |
| `sas_user_ids` | Vec<String> | User ids used by the devices belonging to this node. |


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
node_display_name = node.display_name
node_name = node.name
node_sas_user_ids = node.sas_user_ids
```

---


### Device

Creates a device under a node or customer.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `preloaded_config` | String |  | Configuration of the device, as specified via SAS Portal API. |
| `state` | String |  | Output only. Device state. |
| `serial_number` | String |  | A serial number assigned to the device by the device manufacturer. |
| `grants` | Vec<String> |  | Output only. Grants held by the device. |
| `active_config` | String |  | Output only. Current configuration of the device as registered to the SAS. |
| `fcc_id` | String |  | The FCC identifier of the device. Refer to https://www.fcc.gov/oet/ea/fccid for FccID format. Accept underscores and periods because some test-SAS customers use them. |
| `grant_range_allowlists` | Vec<String> |  | Only ranges that are within the allowlists are available for new grants. |
| `current_channels` | Vec<String> |  | Output only. Current channels with scores. |
| `name` | String |  | Output only. The resource path name. |
| `device_metadata` | String |  | Device parameters that can be overridden by both SAS Portal and SAS registration requests. |
| `display_name` | String |  | Device display name. |
| `parent` | String | ✅ | Required. The name of the parent resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `preloaded_config` | String | Configuration of the device, as specified via SAS Portal API. |
| `state` | String | Output only. Device state. |
| `serial_number` | String | A serial number assigned to the device by the device manufacturer. |
| `grants` | Vec<String> | Output only. Grants held by the device. |
| `active_config` | String | Output only. Current configuration of the device as registered to the SAS. |
| `fcc_id` | String | The FCC identifier of the device. Refer to https://www.fcc.gov/oet/ea/fccid for FccID format. Accept underscores and periods because some test-SAS customers use them. |
| `grant_range_allowlists` | Vec<String> | Only ranges that are within the allowlists are available for new grants. |
| `current_channels` | Vec<String> | Output only. Current channels with scores. |
| `name` | String | Output only. The resource path name. |
| `device_metadata` | String | Device parameters that can be overridden by both SAS Portal and SAS registration requests. |
| `display_name` | String | Device display name. |


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
device_preloaded_config = device.preloaded_config
device_state = device.state
device_serial_number = device.serial_number
device_grants = device.grants
device_active_config = device.active_config
device_fcc_id = device.fcc_id
device_grant_range_allowlists = device.grant_range_allowlists
device_current_channels = device.current_channels
device_name = device.name
device_device_metadata = device.device_metadata
device_display_name = device.display_name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple deployment resources
deployment_0 = provider.prod_tt_sasportal_api.Deployment {
    parent = "value-0"
}
deployment_1 = provider.prod_tt_sasportal_api.Deployment {
    parent = "value-1"
}
deployment_2 = provider.prod_tt_sasportal_api.Deployment {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    deployment = provider.prod_tt_sasportal_api.Deployment {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Prod_tt_sasportal_api Documentation](https://cloud.google.com/prod_tt_sasportal_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
