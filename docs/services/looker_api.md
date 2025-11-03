# Looker_api Service



**Resources**: 4

---

## Overview

The looker_api service provides access to 4 resource types:

- [Backup](#backup) [CRD]
- [Instance](#instance) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]

---

## Resources


### Backup

Backup Looker instance.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expire_time` | String |  | Output only. The time when the backup will be deleted. |
| `name` | String |  | Immutable. The relative resource name of the backup, in the following form: `projects/{project_number}/locations/{location_id}/instances/{instance_id}/backups/{backup}` |
| `create_time` | String |  | Output only. The time when the backup was started. |
| `encryption_config` | String |  | Output only. Current status of the CMEK encryption |
| `state` | String |  | Output only. The current state of the backup. |
| `parent` | String | ✅ | Required. Format: projects/{project}/locations/{location}/instances/{instance} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expire_time` | String | Output only. The time when the backup will be deleted. |
| `name` | String | Immutable. The relative resource name of the backup, in the following form: `projects/{project_number}/locations/{location_id}/instances/{instance_id}/backups/{backup}` |
| `create_time` | String | Output only. The time when the backup was started. |
| `encryption_config` | String | Output only. Current status of the CMEK encryption |
| `state` | String | Output only. The current state of the backup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.looker_api.Backup {
    parent = "value"  # Required. Format: projects/{project}/locations/{location}/instances/{instance}
}

# Access backup outputs
backup_id = backup.id
backup_expire_time = backup.expire_time
backup_name = backup.name
backup_create_time = backup.create_time
backup_encryption_config = backup.encryption_config
backup_state = backup.state
```

---


### Instance

Creates a new Instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_domain` | String |  | Custom domain configuration for the instance. |
| `ingress_public_ip` | String |  | Output only. Public Ingress IP (IPv4). |
| `ingress_private_ip` | String |  | Output only. Private Ingress IP (IPv4). |
| `deny_maintenance_period` | String |  | Maintenance denial period for this instance. |
| `maintenance_window` | String |  | Maintenance window for this instance. |
| `egress_public_ip` | String |  | Output only. Public Egress IP (IPv4). |
| `public_ip_enabled` | bool |  | Whether public IP is enabled on the Looker instance. |
| `admin_settings` | String |  | Looker Instance Admin settings. |
| `platform_edition` | String |  | Platform edition. |
| `controlled_egress_enabled` | bool |  | Optional. Whether controlled egress is enabled on the Looker instance. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `controlled_egress_config` | String |  | Optional. Controlled egress configuration. |
| `last_deny_maintenance_period` | String |  | Output only. Last computed maintenance denial period for this instance. |
| `state` | String |  | Output only. The state of the instance. |
| `private_ip_enabled` | bool |  | Whether private IP is enabled on the Looker instance. |
| `consumer_network` | String |  | Network name in the consumer project. Format: `projects/{project}/global/networks/{network}`. Note that the consumer network may be in a different GCP project than the consumer project that is hosting the Looker Instance. |
| `create_time` | String |  | Output only. The time when the Looker instance provisioning was first requested. |
| `psc_enabled` | bool |  | Optional. Whether to use Private Service Connect (PSC) for private IP connectivity. If true, neither `public_ip_enabled` nor `private_ip_enabled` can be true. |
| `maintenance_schedule` | String |  | Maintenance schedule for this instance. |
| `looker_version` | String |  | Output only. The Looker version that the instance is using. |
| `class_type` | String |  | Optional. Storage class of the instance. |
| `fips_enabled` | bool |  | Optional. Whether FIPS is enabled on the Looker instance. |
| `update_time` | String |  | Output only. The time when the Looker instance was last updated. |
| `oauth_config` | String |  | Looker instance OAuth login settings. |
| `gemini_enabled` | bool |  | Optional. Whether Gemini feature is enabled on the Looker instance or not. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `psc_config` | String |  | Optional. PSC configuration. Used when `psc_enabled` is true. |
| `encryption_config` | String |  | Encryption configuration (CMEK). Only set if CMEK has been enabled on the instance. |
| `linked_lsp_project_number` | String |  | Optional. Linked Google Cloud Project Number for Looker Studio Pro. |
| `looker_uri` | String |  | Output only. Looker instance URI which can be used to access the Looker Instance UI. |
| `name` | String |  | Output only. Format: `projects/{project}/locations/{location}/instances/{instance}`. |
| `reserved_range` | String |  | Name of a reserved IP address range within the Instance.consumer_network, to be used for private services access connection. May or may not be specified in a create request. |
| `user_metadata` | String |  | Optional. User metadata. |
| `parent` | String | ✅ | Required. Format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_domain` | String | Custom domain configuration for the instance. |
| `ingress_public_ip` | String | Output only. Public Ingress IP (IPv4). |
| `ingress_private_ip` | String | Output only. Private Ingress IP (IPv4). |
| `deny_maintenance_period` | String | Maintenance denial period for this instance. |
| `maintenance_window` | String | Maintenance window for this instance. |
| `egress_public_ip` | String | Output only. Public Egress IP (IPv4). |
| `public_ip_enabled` | bool | Whether public IP is enabled on the Looker instance. |
| `admin_settings` | String | Looker Instance Admin settings. |
| `platform_edition` | String | Platform edition. |
| `controlled_egress_enabled` | bool | Optional. Whether controlled egress is enabled on the Looker instance. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `controlled_egress_config` | String | Optional. Controlled egress configuration. |
| `last_deny_maintenance_period` | String | Output only. Last computed maintenance denial period for this instance. |
| `state` | String | Output only. The state of the instance. |
| `private_ip_enabled` | bool | Whether private IP is enabled on the Looker instance. |
| `consumer_network` | String | Network name in the consumer project. Format: `projects/{project}/global/networks/{network}`. Note that the consumer network may be in a different GCP project than the consumer project that is hosting the Looker Instance. |
| `create_time` | String | Output only. The time when the Looker instance provisioning was first requested. |
| `psc_enabled` | bool | Optional. Whether to use Private Service Connect (PSC) for private IP connectivity. If true, neither `public_ip_enabled` nor `private_ip_enabled` can be true. |
| `maintenance_schedule` | String | Maintenance schedule for this instance. |
| `looker_version` | String | Output only. The Looker version that the instance is using. |
| `class_type` | String | Optional. Storage class of the instance. |
| `fips_enabled` | bool | Optional. Whether FIPS is enabled on the Looker instance. |
| `update_time` | String | Output only. The time when the Looker instance was last updated. |
| `oauth_config` | String | Looker instance OAuth login settings. |
| `gemini_enabled` | bool | Optional. Whether Gemini feature is enabled on the Looker instance or not. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `psc_config` | String | Optional. PSC configuration. Used when `psc_enabled` is true. |
| `encryption_config` | String | Encryption configuration (CMEK). Only set if CMEK has been enabled on the instance. |
| `linked_lsp_project_number` | String | Optional. Linked Google Cloud Project Number for Looker Studio Pro. |
| `looker_uri` | String | Output only. Looker instance URI which can be used to access the Looker Instance UI. |
| `name` | String | Output only. Format: `projects/{project}/locations/{location}/instances/{instance}`. |
| `reserved_range` | String | Name of a reserved IP address range within the Instance.consumer_network, to be used for private services access connection. May or may not be specified in a create request. |
| `user_metadata` | String | Optional. User metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.looker_api.Instance {
    parent = "value"  # Required. Format: `projects/{project}/locations/{location}`.
}

# Access instance outputs
instance_id = instance.id
instance_custom_domain = instance.custom_domain
instance_ingress_public_ip = instance.ingress_public_ip
instance_ingress_private_ip = instance.ingress_private_ip
instance_deny_maintenance_period = instance.deny_maintenance_period
instance_maintenance_window = instance.maintenance_window
instance_egress_public_ip = instance.egress_public_ip
instance_public_ip_enabled = instance.public_ip_enabled
instance_admin_settings = instance.admin_settings
instance_platform_edition = instance.platform_edition
instance_controlled_egress_enabled = instance.controlled_egress_enabled
instance_satisfies_pzs = instance.satisfies_pzs
instance_controlled_egress_config = instance.controlled_egress_config
instance_last_deny_maintenance_period = instance.last_deny_maintenance_period
instance_state = instance.state
instance_private_ip_enabled = instance.private_ip_enabled
instance_consumer_network = instance.consumer_network
instance_create_time = instance.create_time
instance_psc_enabled = instance.psc_enabled
instance_maintenance_schedule = instance.maintenance_schedule
instance_looker_version = instance.looker_version
instance_class_type = instance.class_type
instance_fips_enabled = instance.fips_enabled
instance_update_time = instance.update_time
instance_oauth_config = instance.oauth_config
instance_gemini_enabled = instance.gemini_enabled
instance_satisfies_pzi = instance.satisfies_pzi
instance_psc_config = instance.psc_config
instance_encryption_config = instance.encryption_config
instance_linked_lsp_project_number = instance.linked_lsp_project_number
instance_looker_uri = instance.looker_uri
instance_name = instance.name
instance_reserved_range = instance.reserved_range
instance_user_metadata = instance.user_metadata
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_name = location.name
location_location_id = location.location_id
location_display_name = location.display_name
location_labels = location.labels
location_metadata = location.metadata
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.looker_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_name = operation.name
operation_metadata = operation.metadata
operation_response = operation.response
operation_done = operation.done
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple backup resources
backup_0 = provider.looker_api.Backup {
    parent = "value-0"
}
backup_1 = provider.looker_api.Backup {
    parent = "value-1"
}
backup_2 = provider.looker_api.Backup {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    backup = provider.looker_api.Backup {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Looker_api Documentation](https://cloud.google.com/looker_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
