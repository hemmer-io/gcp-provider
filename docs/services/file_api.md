# File_api Service



**Resources**: 11

---

## Overview

The file_api service provides access to 11 resource types:

- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Instance](#instance) [CRUD]
- [Backup](#backup) [CRUD]
- [Snapshot](#snapshot) [CRUD]
- [Snapshot](#snapshot) [CRUD]
- [Location](#location) [R]
- [Backup](#backup) [CRUD]
- [Share](#share) [CRUD]
- [Instance](#instance) [CRUD]
- [Operation](#operation) [CRD]

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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.file_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_response = operation.response
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_labels = location.labels
location_location_id = location.location_id
location_name = location.name
location_metadata = location.metadata
```

---


### Instance

Creates an instance. When creating from a backup, the capacity of the new instance needs to be equal to or larger than the capacity of the backup (and also equal to or larger than the minimum capacity of the tier).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tier` | String |  | The service tier of the instance. |
| `capacity_step_size_gb` | String |  | Output only. The increase/decrease capacity step size in GB. |
| `status_message` | String |  | Output only. Additional information about the instance state, if available. |
| `performance_config` | String |  | Optional. Used to configure performance. |
| `create_time` | String |  | Output only. The time when the instance was created. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `protocol` | String |  | Immutable. The protocol indicates the access protocol for all shares in the instance. This field is immutable and it cannot be changed after the instance has been created. Default value: `NFS_V3`. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `max_capacity_gb` | String |  | Output only. The max capacity of the instance in GB. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `suspension_reasons` | Vec<String> |  | Output only. Field indicates all the reasons the instance is in "SUSPENDED" state. |
| `description` | String |  | The description of the instance (2048 characters or less). |
| `min_capacity_gb` | String |  | Output only. The min capacity of the instance in GB. |
| `file_shares` | Vec<String> |  | File system shares on the instance. For this version, only a single file share is supported. |
| `replication` | String |  | Optional. Replication configuration. |
| `networks` | Vec<String> |  | VPC networks to which the instance is connected. For this version, only a single network is supported. |
| `etag` | String |  | Server-specified ETag for the instance resource to prevent simultaneous updates from overwriting each other. |
| `custom_performance_supported` | bool |  | Output only. Indicates whether this instance supports configuring its performance. If true, the user can configure the instance's performance by using the 'performance_config' field. |
| `kms_key_name` | String |  | KMS key name used for data encryption. |
| `name` | String |  | Output only. The resource name of the instance, in the format `projects/{project}/locations/{location}/instances/{instance}`. |
| `performance_limits` | String |  | Output only. Used for getting performance limits. |
| `state` | String |  | Output only. The instance state. |
| `deletion_protection_enabled` | bool |  | Optional. Indicates whether the instance is protected against deletion. |
| `directory_services` | String |  | Optional. Directory Services configuration for Kerberos-based authentication. Should only be set if protocol is "NFS_V4_1". |
| `deletion_protection_reason` | String |  | Optional. The reason for enabling deletion protection. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The instance's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, locations map to Google Cloud zones, for example **us-west1-b**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tier` | String | The service tier of the instance. |
| `capacity_step_size_gb` | String | Output only. The increase/decrease capacity step size in GB. |
| `status_message` | String | Output only. Additional information about the instance state, if available. |
| `performance_config` | String | Optional. Used to configure performance. |
| `create_time` | String | Output only. The time when the instance was created. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `protocol` | String | Immutable. The protocol indicates the access protocol for all shares in the instance. This field is immutable and it cannot be changed after the instance has been created. Default value: `NFS_V3`. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `max_capacity_gb` | String | Output only. The max capacity of the instance in GB. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `suspension_reasons` | Vec<String> | Output only. Field indicates all the reasons the instance is in "SUSPENDED" state. |
| `description` | String | The description of the instance (2048 characters or less). |
| `min_capacity_gb` | String | Output only. The min capacity of the instance in GB. |
| `file_shares` | Vec<String> | File system shares on the instance. For this version, only a single file share is supported. |
| `replication` | String | Optional. Replication configuration. |
| `networks` | Vec<String> | VPC networks to which the instance is connected. For this version, only a single network is supported. |
| `etag` | String | Server-specified ETag for the instance resource to prevent simultaneous updates from overwriting each other. |
| `custom_performance_supported` | bool | Output only. Indicates whether this instance supports configuring its performance. If true, the user can configure the instance's performance by using the 'performance_config' field. |
| `kms_key_name` | String | KMS key name used for data encryption. |
| `name` | String | Output only. The resource name of the instance, in the format `projects/{project}/locations/{location}/instances/{instance}`. |
| `performance_limits` | String | Output only. Used for getting performance limits. |
| `state` | String | Output only. The instance state. |
| `deletion_protection_enabled` | bool | Optional. Indicates whether the instance is protected against deletion. |
| `directory_services` | String | Optional. Directory Services configuration for Kerberos-based authentication. Should only be set if protocol is "NFS_V4_1". |
| `deletion_protection_reason` | String | Optional. The reason for enabling deletion protection. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.file_api.Instance {
    parent = "value"  # Required. The instance's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, locations map to Google Cloud zones, for example **us-west1-b**.
}

# Access instance outputs
instance_id = instance.id
instance_tier = instance.tier
instance_capacity_step_size_gb = instance.capacity_step_size_gb
instance_status_message = instance.status_message
instance_performance_config = instance.performance_config
instance_create_time = instance.create_time
instance_labels = instance.labels
instance_protocol = instance.protocol
instance_tags = instance.tags
instance_max_capacity_gb = instance.max_capacity_gb
instance_satisfies_pzs = instance.satisfies_pzs
instance_suspension_reasons = instance.suspension_reasons
instance_description = instance.description
instance_min_capacity_gb = instance.min_capacity_gb
instance_file_shares = instance.file_shares
instance_replication = instance.replication
instance_networks = instance.networks
instance_etag = instance.etag
instance_custom_performance_supported = instance.custom_performance_supported
instance_kms_key_name = instance.kms_key_name
instance_name = instance.name
instance_performance_limits = instance.performance_limits
instance_state = instance.state
instance_deletion_protection_enabled = instance.deletion_protection_enabled
instance_directory_services = instance.directory_services
instance_deletion_protection_reason = instance.deletion_protection_reason
instance_satisfies_pzi = instance.satisfies_pzi
```

---


### Backup

Creates a backup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_instance_tier` | String |  | Output only. The service tier of the source Filestore instance that this backup is created from. |
| `file_system_protocol` | String |  | Output only. The file system protocol of the source Filestore instance that this backup is created from. |
| `capacity_gb` | String |  | Output only. Capacity of the source file share when the backup was created. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `source_file_share` | String |  | Name of the file share in the source Filestore instance that the backup is created from. |
| `download_bytes` | String |  | Output only. Amount of bytes that will be downloaded if the backup is restored. This may be different than storage bytes, since sequential backups of the same disk will share storage. |
| `source_instance` | String |  | The resource name of the source Filestore instance, in the format `projects/{project_number}/locations/{location_id}/instances/{instance_id}`, used to create this backup. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `name` | String |  | Output only. The resource name of the backup, in the format `projects/{project_number}/locations/{location_id}/backups/{backup_id}`. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `storage_bytes` | String |  | Output only. The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion. |
| `description` | String |  | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `kms_key` | String |  | Immutable. KMS key name used for data encryption. |
| `state` | String |  | Output only. The backup state. |
| `create_time` | String |  | Output only. The time when the backup was created. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The backup's project and location, in the format `projects/{project_number}/locations/{location}`. In Filestore, backup locations map to Google Cloud regions, for example **us-west1**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_instance_tier` | String | Output only. The service tier of the source Filestore instance that this backup is created from. |
| `file_system_protocol` | String | Output only. The file system protocol of the source Filestore instance that this backup is created from. |
| `capacity_gb` | String | Output only. Capacity of the source file share when the backup was created. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `source_file_share` | String | Name of the file share in the source Filestore instance that the backup is created from. |
| `download_bytes` | String | Output only. Amount of bytes that will be downloaded if the backup is restored. This may be different than storage bytes, since sequential backups of the same disk will share storage. |
| `source_instance` | String | The resource name of the source Filestore instance, in the format `projects/{project_number}/locations/{location_id}/instances/{instance_id}`, used to create this backup. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `name` | String | Output only. The resource name of the backup, in the format `projects/{project_number}/locations/{location_id}/backups/{backup_id}`. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `storage_bytes` | String | Output only. The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion. |
| `description` | String | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `kms_key` | String | Immutable. KMS key name used for data encryption. |
| `state` | String | Output only. The backup state. |
| `create_time` | String | Output only. The time when the backup was created. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.file_api.Backup {
    parent = "value"  # Required. The backup's project and location, in the format `projects/{project_number}/locations/{location}`. In Filestore, backup locations map to Google Cloud regions, for example **us-west1**.
}

# Access backup outputs
backup_id = backup.id
backup_source_instance_tier = backup.source_instance_tier
backup_file_system_protocol = backup.file_system_protocol
backup_capacity_gb = backup.capacity_gb
backup_tags = backup.tags
backup_source_file_share = backup.source_file_share
backup_download_bytes = backup.download_bytes
backup_source_instance = backup.source_instance
backup_labels = backup.labels
backup_name = backup.name
backup_satisfies_pzi = backup.satisfies_pzi
backup_storage_bytes = backup.storage_bytes
backup_description = backup.description
backup_kms_key = backup.kms_key
backup_state = backup.state
backup_create_time = backup.create_time
backup_satisfies_pzs = backup.satisfies_pzs
```

---


### Snapshot

Creates a snapshot.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the snapshot was created. |
| `filesystem_used_bytes` | String |  | Output only. The amount of bytes needed to allocate a full copy of the snapshot content |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `state` | String |  | Output only. The snapshot state. |
| `description` | String |  | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `name` | String |  | Output only. The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`. |
| `parent` | String | ✅ | Required. The Filestore Instance to create the snapshots of, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the snapshot was created. |
| `filesystem_used_bytes` | String | Output only. The amount of bytes needed to allocate a full copy of the snapshot content |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `state` | String | Output only. The snapshot state. |
| `description` | String | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `name` | String | Output only. The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create snapshot
snapshot = provider.file_api.Snapshot {
    parent = "value"  # Required. The Filestore Instance to create the snapshots of, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}`
}

# Access snapshot outputs
snapshot_id = snapshot.id
snapshot_create_time = snapshot.create_time
snapshot_filesystem_used_bytes = snapshot.filesystem_used_bytes
snapshot_labels = snapshot.labels
snapshot_state = snapshot.state
snapshot_description = snapshot.description
snapshot_tags = snapshot.tags
snapshot_name = snapshot.name
```

---


### Snapshot

Creates a snapshot.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `create_time` | String |  | Output only. The time when the snapshot was created. |
| `description` | String |  | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `filesystem_used_bytes` | String |  | Output only. The amount of bytes needed to allocate a full copy of the snapshot content |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `name` | String |  | Output only. The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`. |
| `state` | String |  | Output only. The snapshot state. |
| `parent` | String | ✅ | Required. The Filestore Instance to create the snapshots of, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `create_time` | String | Output only. The time when the snapshot was created. |
| `description` | String | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `filesystem_used_bytes` | String | Output only. The amount of bytes needed to allocate a full copy of the snapshot content |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `name` | String | Output only. The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`. |
| `state` | String | Output only. The snapshot state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create snapshot
snapshot = provider.file_api.Snapshot {
    parent = "value"  # Required. The Filestore Instance to create the snapshots of, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}`
}

# Access snapshot outputs
snapshot_id = snapshot.id
snapshot_tags = snapshot.tags
snapshot_create_time = snapshot.create_time
snapshot_description = snapshot.description
snapshot_filesystem_used_bytes = snapshot.filesystem_used_bytes
snapshot_labels = snapshot.labels
snapshot_name = snapshot.name
snapshot_state = snapshot.state
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_display_name = location.display_name
location_labels = location.labels
location_metadata = location.metadata
location_location_id = location.location_id
location_name = location.name
```

---


### Backup

Creates a backup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_instance_tier` | String |  | Output only. The service tier of the source Filestore instance that this backup is created from. |
| `kms_key_name` | String |  | Immutable. KMS key name used for data encryption. |
| `source_instance` | String |  | The resource name of the source Filestore instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`, used to create this backup. |
| `download_bytes` | String |  | Output only. Amount of bytes that will be downloaded if the backup is restored |
| `source_file_share` | String |  | Name of the file share in the source Filestore instance that the backup is created from. |
| `name` | String |  | Output only. The resource name of the backup, in the format `projects/{project_id}/locations/{location_id}/backups/{backup_id}`. |
| `storage_bytes` | String |  | Output only. The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion. |
| `state` | String |  | Output only. The backup state. |
| `capacity_gb` | String |  | Output only. Capacity of the source file share when the backup was created. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `file_system_protocol` | String |  | Output only. The file system protocol of the source Filestore instance that this backup is created from. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `description` | String |  | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `create_time` | String |  | Output only. The time when the backup was created. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `parent` | String | ✅ | Required. The backup's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, backup locations map to Google Cloud regions, for example **us-west1**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_instance_tier` | String | Output only. The service tier of the source Filestore instance that this backup is created from. |
| `kms_key_name` | String | Immutable. KMS key name used for data encryption. |
| `source_instance` | String | The resource name of the source Filestore instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`, used to create this backup. |
| `download_bytes` | String | Output only. Amount of bytes that will be downloaded if the backup is restored |
| `source_file_share` | String | Name of the file share in the source Filestore instance that the backup is created from. |
| `name` | String | Output only. The resource name of the backup, in the format `projects/{project_id}/locations/{location_id}/backups/{backup_id}`. |
| `storage_bytes` | String | Output only. The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion. |
| `state` | String | Output only. The backup state. |
| `capacity_gb` | String | Output only. Capacity of the source file share when the backup was created. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `file_system_protocol` | String | Output only. The file system protocol of the source Filestore instance that this backup is created from. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `description` | String | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `create_time` | String | Output only. The time when the backup was created. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.file_api.Backup {
    parent = "value"  # Required. The backup's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, backup locations map to Google Cloud regions, for example **us-west1**.
}

# Access backup outputs
backup_id = backup.id
backup_source_instance_tier = backup.source_instance_tier
backup_kms_key_name = backup.kms_key_name
backup_source_instance = backup.source_instance
backup_download_bytes = backup.download_bytes
backup_source_file_share = backup.source_file_share
backup_name = backup.name
backup_storage_bytes = backup.storage_bytes
backup_state = backup.state
backup_capacity_gb = backup.capacity_gb
backup_satisfies_pzi = backup.satisfies_pzi
backup_file_system_protocol = backup.file_system_protocol
backup_labels = backup.labels
backup_satisfies_pzs = backup.satisfies_pzs
backup_description = backup.description
backup_create_time = backup.create_time
backup_tags = backup.tags
```

---


### Share

Creates a share.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `nfs_export_options` | Vec<String> |  | Nfs Export Options. There is a limit of 10 export options per file share. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `create_time` | String |  | Output only. The time when the share was created. |
| `state` | String |  | Output only. The share state. |
| `backup` | String |  | Immutable. Full name of the Cloud Filestore Backup resource that this Share is restored from, in the format of projects/{project_id}/locations/{location_id}/backups/{backup_id}. Empty, if the Share is created from scratch and not restored from a backup. |
| `capacity_gb` | String |  | File share capacity in gigabytes (GB). Filestore defines 1 GB as 1024^3 bytes. Must be greater than 0. |
| `mount_name` | String |  | The mount name of the share. Must be 63 characters or less and consist of uppercase or lowercase letters, numbers, and underscores. |
| `description` | String |  | A description of the share with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `name` | String |  | Output only. The resource name of the share, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/shares/{share_id}`. |
| `parent` | String | ✅ | Required. The Filestore Instance to create the share for, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nfs_export_options` | Vec<String> | Nfs Export Options. There is a limit of 10 export options per file share. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `create_time` | String | Output only. The time when the share was created. |
| `state` | String | Output only. The share state. |
| `backup` | String | Immutable. Full name of the Cloud Filestore Backup resource that this Share is restored from, in the format of projects/{project_id}/locations/{location_id}/backups/{backup_id}. Empty, if the Share is created from scratch and not restored from a backup. |
| `capacity_gb` | String | File share capacity in gigabytes (GB). Filestore defines 1 GB as 1024^3 bytes. Must be greater than 0. |
| `mount_name` | String | The mount name of the share. Must be 63 characters or less and consist of uppercase or lowercase letters, numbers, and underscores. |
| `description` | String | A description of the share with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `name` | String | Output only. The resource name of the share, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/shares/{share_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create share
share = provider.file_api.Share {
    parent = "value"  # Required. The Filestore Instance to create the share for, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}`
}

# Access share outputs
share_id = share.id
share_nfs_export_options = share.nfs_export_options
share_labels = share.labels
share_create_time = share.create_time
share_state = share.state
share_backup = share.backup
share_capacity_gb = share.capacity_gb
share_mount_name = share.mount_name
share_description = share.description
share_name = share.name
```

---


### Instance

Creates an instance. When creating from a backup, the capacity of the new instance needs to be equal to or larger than the capacity of the backup (and also equal to or larger than the minimum capacity of the tier).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status_message` | String |  | Output only. Additional information about the instance state, if available. |
| `kms_key_name` | String |  | KMS key name used for data encryption. |
| `performance_limits` | String |  | Output only. Used for getting performance limits. |
| `tier` | String |  | The service tier of the instance. |
| `protocol` | String |  | Immutable. The protocol indicates the access protocol for all shares in the instance. This field is immutable and it cannot be changed after the instance has been created. Default value: `NFS_V3`. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `suspension_reasons` | Vec<String> |  | Output only. Field indicates all the reasons the instance is in "SUSPENDED" state. |
| `backend_type` | String |  | Optional. Immutable. Designates the backend type of this instance. Intended to be used by internal tests and allowed customers. |
| `capacity_gb` | String |  | The storage capacity of the instance in gigabytes (GB = 1024^3 bytes). This capacity can be increased up to `max_capacity_gb` GB in multipliers of `capacity_step_size_gb` GB. |
| `state` | String |  | Output only. The instance state. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `performance_config` | String |  | Optional. Used to configure performance. |
| `max_share_count` | String |  | The max number of shares allowed. |
| `capacity_step_size_gb` | String |  | Output only. The increase/decrease capacity step size. |
| `min_capacity_gb` | String |  | Output only. The min capacity of the instance. |
| `multi_share_enabled` | bool |  | Indicates whether this instance uses a multi-share configuration with which it can have more than one file-share or none at all. File-shares are added, updated and removed through the separate file-share APIs. |
| `max_capacity_gb` | String |  | Output only. The max capacity of the instance. |
| `custom_performance_supported` | bool |  | Output only. Indicates whether this instance supports configuring its performance. If true, the user can configure the instance's performance by using the 'performance_config' field. |
| `directory_services` | String |  | Optional. Directory Services configuration. Should only be set if protocol is "NFS_V4_1". |
| `deletion_protection_enabled` | bool |  | Optional. Indicates whether the instance is protected against deletion. |
| `etag` | String |  | Server-specified ETag for the instance resource to prevent simultaneous updates from overwriting each other. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `networks` | Vec<String> |  | VPC networks to which the instance is connected. For this version, only a single network is supported. |
| `description` | String |  | The description of the instance (2048 characters or less). |
| `file_shares` | Vec<String> |  | File system shares on the instance. For this version, only a single file share is supported. |
| `deletion_protection_reason` | String |  | Optional. The reason for enabling deletion protection. |
| `create_time` | String |  | Output only. The time when the instance was created. |
| `name` | String |  | Output only. The resource name of the instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`. |
| `replication` | String |  | Optional. Replication configuration. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `parent` | String | ✅ | Required. The instance's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, locations map to Google Cloud zones, for example **us-west1-b**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_message` | String | Output only. Additional information about the instance state, if available. |
| `kms_key_name` | String | KMS key name used for data encryption. |
| `performance_limits` | String | Output only. Used for getting performance limits. |
| `tier` | String | The service tier of the instance. |
| `protocol` | String | Immutable. The protocol indicates the access protocol for all shares in the instance. This field is immutable and it cannot be changed after the instance has been created. Default value: `NFS_V3`. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `suspension_reasons` | Vec<String> | Output only. Field indicates all the reasons the instance is in "SUSPENDED" state. |
| `backend_type` | String | Optional. Immutable. Designates the backend type of this instance. Intended to be used by internal tests and allowed customers. |
| `capacity_gb` | String | The storage capacity of the instance in gigabytes (GB = 1024^3 bytes). This capacity can be increased up to `max_capacity_gb` GB in multipliers of `capacity_step_size_gb` GB. |
| `state` | String | Output only. The instance state. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `performance_config` | String | Optional. Used to configure performance. |
| `max_share_count` | String | The max number of shares allowed. |
| `capacity_step_size_gb` | String | Output only. The increase/decrease capacity step size. |
| `min_capacity_gb` | String | Output only. The min capacity of the instance. |
| `multi_share_enabled` | bool | Indicates whether this instance uses a multi-share configuration with which it can have more than one file-share or none at all. File-shares are added, updated and removed through the separate file-share APIs. |
| `max_capacity_gb` | String | Output only. The max capacity of the instance. |
| `custom_performance_supported` | bool | Output only. Indicates whether this instance supports configuring its performance. If true, the user can configure the instance's performance by using the 'performance_config' field. |
| `directory_services` | String | Optional. Directory Services configuration. Should only be set if protocol is "NFS_V4_1". |
| `deletion_protection_enabled` | bool | Optional. Indicates whether the instance is protected against deletion. |
| `etag` | String | Server-specified ETag for the instance resource to prevent simultaneous updates from overwriting each other. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `networks` | Vec<String> | VPC networks to which the instance is connected. For this version, only a single network is supported. |
| `description` | String | The description of the instance (2048 characters or less). |
| `file_shares` | Vec<String> | File system shares on the instance. For this version, only a single file share is supported. |
| `deletion_protection_reason` | String | Optional. The reason for enabling deletion protection. |
| `create_time` | String | Output only. The time when the instance was created. |
| `name` | String | Output only. The resource name of the instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`. |
| `replication` | String | Optional. Replication configuration. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.file_api.Instance {
    parent = "value"  # Required. The instance's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, locations map to Google Cloud zones, for example **us-west1-b**.
}

# Access instance outputs
instance_id = instance.id
instance_status_message = instance.status_message
instance_kms_key_name = instance.kms_key_name
instance_performance_limits = instance.performance_limits
instance_tier = instance.tier
instance_protocol = instance.protocol
instance_labels = instance.labels
instance_suspension_reasons = instance.suspension_reasons
instance_backend_type = instance.backend_type
instance_capacity_gb = instance.capacity_gb
instance_state = instance.state
instance_satisfies_pzi = instance.satisfies_pzi
instance_performance_config = instance.performance_config
instance_max_share_count = instance.max_share_count
instance_capacity_step_size_gb = instance.capacity_step_size_gb
instance_min_capacity_gb = instance.min_capacity_gb
instance_multi_share_enabled = instance.multi_share_enabled
instance_max_capacity_gb = instance.max_capacity_gb
instance_custom_performance_supported = instance.custom_performance_supported
instance_directory_services = instance.directory_services
instance_deletion_protection_enabled = instance.deletion_protection_enabled
instance_etag = instance.etag
instance_satisfies_pzs = instance.satisfies_pzs
instance_networks = instance.networks
instance_description = instance.description
instance_file_shares = instance.file_shares
instance_deletion_protection_reason = instance.deletion_protection_reason
instance_create_time = instance.create_time
instance_name = instance.name
instance_replication = instance.replication
instance_tags = instance.tags
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.file_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_response = operation.response
operation_error = operation.error
operation_name = operation.name
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

# Create multiple operation resources
operation_0 = provider.file_api.Operation {
    name = "value-0"
}
operation_1 = provider.file_api.Operation {
    name = "value-1"
}
operation_2 = provider.file_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.file_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP File_api Documentation](https://cloud.google.com/file_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
