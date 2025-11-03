# File_api Service



**Resources**: 11

---

## Overview

The file_api service provides access to 11 resource types:

- [Backup](#backup) [CRUD]
- [Snapshot](#snapshot) [CRUD]
- [Operation](#operation) [CRD]
- [Instance](#instance) [CRUD]
- [Location](#location) [R]
- [Snapshot](#snapshot) [CRUD]
- [Share](#share) [CRUD]
- [Backup](#backup) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [Instance](#instance) [CRUD]

---

## Resources


### Backup

Creates a backup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `create_time` | String |  | Output only. The time when the backup was created. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `source_instance` | String |  | The resource name of the source Filestore instance, in the format `projects/{project_number}/locations/{location_id}/instances/{instance_id}`, used to create this backup. |
| `name` | String |  | Output only. The resource name of the backup, in the format `projects/{project_number}/locations/{location_id}/backups/{backup_id}`. |
| `download_bytes` | String |  | Output only. Amount of bytes that will be downloaded if the backup is restored. This may be different than storage bytes, since sequential backups of the same disk will share storage. |
| `source_instance_tier` | String |  | Output only. The service tier of the source Filestore instance that this backup is created from. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `kms_key` | String |  | Immutable. KMS key name used for data encryption. |
| `file_system_protocol` | String |  | Output only. The file system protocol of the source Filestore instance that this backup is created from. |
| `capacity_gb` | String |  | Output only. Capacity of the source file share when the backup was created. |
| `source_file_share` | String |  | Name of the file share in the source Filestore instance that the backup is created from. |
| `state` | String |  | Output only. The backup state. |
| `storage_bytes` | String |  | Output only. The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion. |
| `parent` | String | ✅ | Required. The backup's project and location, in the format `projects/{project_number}/locations/{location}`. In Filestore, backup locations map to Google Cloud regions, for example **us-west1**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `create_time` | String | Output only. The time when the backup was created. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `source_instance` | String | The resource name of the source Filestore instance, in the format `projects/{project_number}/locations/{location_id}/instances/{instance_id}`, used to create this backup. |
| `name` | String | Output only. The resource name of the backup, in the format `projects/{project_number}/locations/{location_id}/backups/{backup_id}`. |
| `download_bytes` | String | Output only. Amount of bytes that will be downloaded if the backup is restored. This may be different than storage bytes, since sequential backups of the same disk will share storage. |
| `source_instance_tier` | String | Output only. The service tier of the source Filestore instance that this backup is created from. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `kms_key` | String | Immutable. KMS key name used for data encryption. |
| `file_system_protocol` | String | Output only. The file system protocol of the source Filestore instance that this backup is created from. |
| `capacity_gb` | String | Output only. Capacity of the source file share when the backup was created. |
| `source_file_share` | String | Name of the file share in the source Filestore instance that the backup is created from. |
| `state` | String | Output only. The backup state. |
| `storage_bytes` | String | Output only. The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion. |


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
backup_description = backup.description
backup_create_time = backup.create_time
backup_satisfies_pzi = backup.satisfies_pzi
backup_tags = backup.tags
backup_labels = backup.labels
backup_source_instance = backup.source_instance
backup_name = backup.name
backup_download_bytes = backup.download_bytes
backup_source_instance_tier = backup.source_instance_tier
backup_satisfies_pzs = backup.satisfies_pzs
backup_kms_key = backup.kms_key
backup_file_system_protocol = backup.file_system_protocol
backup_capacity_gb = backup.capacity_gb
backup_source_file_share = backup.source_file_share
backup_state = backup.state
backup_storage_bytes = backup.storage_bytes
```

---


### Snapshot

Creates a snapshot.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `state` | String |  | Output only. The snapshot state. |
| `name` | String |  | Output only. The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`. |
| `filesystem_used_bytes` | String |  | Output only. The amount of bytes needed to allocate a full copy of the snapshot content |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `create_time` | String |  | Output only. The time when the snapshot was created. |
| `description` | String |  | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `parent` | String | ✅ | Required. The Filestore Instance to create the snapshots of, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `state` | String | Output only. The snapshot state. |
| `name` | String | Output only. The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`. |
| `filesystem_used_bytes` | String | Output only. The amount of bytes needed to allocate a full copy of the snapshot content |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `create_time` | String | Output only. The time when the snapshot was created. |
| `description` | String | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |


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
snapshot_labels = snapshot.labels
snapshot_state = snapshot.state
snapshot_name = snapshot.name
snapshot_filesystem_used_bytes = snapshot.filesystem_used_bytes
snapshot_tags = snapshot.tags
snapshot_create_time = snapshot.create_time
snapshot_description = snapshot.description
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.file_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_error = operation.error
operation_done = operation.done
operation_response = operation.response
operation_metadata = operation.metadata
```

---


### Instance

Creates an instance. When creating from a backup, the capacity of the new instance needs to be equal to or larger than the capacity of the backup (and also equal to or larger than the minimum capacity of the tier).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the instance was created. |
| `deletion_protection_enabled` | bool |  | Optional. Indicates whether the instance is protected against deletion. |
| `performance_config` | String |  | Optional. Used to configure performance. |
| `deletion_protection_reason` | String |  | Optional. The reason for enabling deletion protection. |
| `min_capacity_gb` | String |  | Output only. The min capacity of the instance in GB. |
| `state` | String |  | Output only. The instance state. |
| `performance_limits` | String |  | Output only. Used for getting performance limits. |
| `protocol` | String |  | Immutable. The protocol indicates the access protocol for all shares in the instance. This field is immutable and it cannot be changed after the instance has been created. Default value: `NFS_V3`. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `custom_performance_supported` | bool |  | Output only. Indicates whether this instance supports configuring its performance. If true, the user can configure the instance's performance by using the 'performance_config' field. |
| `max_capacity_gb` | String |  | Output only. The max capacity of the instance in GB. |
| `status_message` | String |  | Output only. Additional information about the instance state, if available. |
| `directory_services` | String |  | Optional. Directory Services configuration for Kerberos-based authentication. Should only be set if protocol is "NFS_V4_1". |
| `tier` | String |  | The service tier of the instance. |
| `file_shares` | Vec<String> |  | File system shares on the instance. For this version, only a single file share is supported. |
| `kms_key_name` | String |  | KMS key name used for data encryption. |
| `etag` | String |  | Server-specified ETag for the instance resource to prevent simultaneous updates from overwriting each other. |
| `capacity_step_size_gb` | String |  | Output only. The increase/decrease capacity step size in GB. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `name` | String |  | Output only. The resource name of the instance, in the format `projects/{project}/locations/{location}/instances/{instance}`. |
| `replication` | String |  | Optional. Replication configuration. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `networks` | Vec<String> |  | VPC networks to which the instance is connected. For this version, only a single network is supported. |
| `description` | String |  | The description of the instance (2048 characters or less). |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `suspension_reasons` | Vec<String> |  | Output only. Field indicates all the reasons the instance is in "SUSPENDED" state. |
| `parent` | String | ✅ | Required. The instance's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, locations map to Google Cloud zones, for example **us-west1-b**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the instance was created. |
| `deletion_protection_enabled` | bool | Optional. Indicates whether the instance is protected against deletion. |
| `performance_config` | String | Optional. Used to configure performance. |
| `deletion_protection_reason` | String | Optional. The reason for enabling deletion protection. |
| `min_capacity_gb` | String | Output only. The min capacity of the instance in GB. |
| `state` | String | Output only. The instance state. |
| `performance_limits` | String | Output only. Used for getting performance limits. |
| `protocol` | String | Immutable. The protocol indicates the access protocol for all shares in the instance. This field is immutable and it cannot be changed after the instance has been created. Default value: `NFS_V3`. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `custom_performance_supported` | bool | Output only. Indicates whether this instance supports configuring its performance. If true, the user can configure the instance's performance by using the 'performance_config' field. |
| `max_capacity_gb` | String | Output only. The max capacity of the instance in GB. |
| `status_message` | String | Output only. Additional information about the instance state, if available. |
| `directory_services` | String | Optional. Directory Services configuration for Kerberos-based authentication. Should only be set if protocol is "NFS_V4_1". |
| `tier` | String | The service tier of the instance. |
| `file_shares` | Vec<String> | File system shares on the instance. For this version, only a single file share is supported. |
| `kms_key_name` | String | KMS key name used for data encryption. |
| `etag` | String | Server-specified ETag for the instance resource to prevent simultaneous updates from overwriting each other. |
| `capacity_step_size_gb` | String | Output only. The increase/decrease capacity step size in GB. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `name` | String | Output only. The resource name of the instance, in the format `projects/{project}/locations/{location}/instances/{instance}`. |
| `replication` | String | Optional. Replication configuration. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `networks` | Vec<String> | VPC networks to which the instance is connected. For this version, only a single network is supported. |
| `description` | String | The description of the instance (2048 characters or less). |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `suspension_reasons` | Vec<String> | Output only. Field indicates all the reasons the instance is in "SUSPENDED" state. |


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
instance_create_time = instance.create_time
instance_deletion_protection_enabled = instance.deletion_protection_enabled
instance_performance_config = instance.performance_config
instance_deletion_protection_reason = instance.deletion_protection_reason
instance_min_capacity_gb = instance.min_capacity_gb
instance_state = instance.state
instance_performance_limits = instance.performance_limits
instance_protocol = instance.protocol
instance_satisfies_pzi = instance.satisfies_pzi
instance_custom_performance_supported = instance.custom_performance_supported
instance_max_capacity_gb = instance.max_capacity_gb
instance_status_message = instance.status_message
instance_directory_services = instance.directory_services
instance_tier = instance.tier
instance_file_shares = instance.file_shares
instance_kms_key_name = instance.kms_key_name
instance_etag = instance.etag
instance_capacity_step_size_gb = instance.capacity_step_size_gb
instance_labels = instance.labels
instance_name = instance.name
instance_replication = instance.replication
instance_satisfies_pzs = instance.satisfies_pzs
instance_networks = instance.networks
instance_description = instance.description
instance_tags = instance.tags
instance_suspension_reasons = instance.suspension_reasons
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |


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
location_location_id = location.location_id
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
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
| `name` | String |  | Output only. The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`. |
| `state` | String |  | Output only. The snapshot state. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `description` | String |  | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `filesystem_used_bytes` | String |  | Output only. The amount of bytes needed to allocate a full copy of the snapshot content |
| `parent` | String | ✅ | Required. The Filestore Instance to create the snapshots of, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `create_time` | String | Output only. The time when the snapshot was created. |
| `name` | String | Output only. The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`. |
| `state` | String | Output only. The snapshot state. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `description` | String | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `filesystem_used_bytes` | String | Output only. The amount of bytes needed to allocate a full copy of the snapshot content |


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
snapshot_name = snapshot.name
snapshot_state = snapshot.state
snapshot_labels = snapshot.labels
snapshot_description = snapshot.description
snapshot_filesystem_used_bytes = snapshot.filesystem_used_bytes
```

---


### Share

Creates a share.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the share was created. |
| `backup` | String |  | Immutable. Full name of the Cloud Filestore Backup resource that this Share is restored from, in the format of projects/{project_id}/locations/{location_id}/backups/{backup_id}. Empty, if the Share is created from scratch and not restored from a backup. |
| `mount_name` | String |  | The mount name of the share. Must be 63 characters or less and consist of uppercase or lowercase letters, numbers, and underscores. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `nfs_export_options` | Vec<String> |  | Nfs Export Options. There is a limit of 10 export options per file share. |
| `description` | String |  | A description of the share with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `state` | String |  | Output only. The share state. |
| `name` | String |  | Output only. The resource name of the share, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/shares/{share_id}`. |
| `capacity_gb` | String |  | File share capacity in gigabytes (GB). Filestore defines 1 GB as 1024^3 bytes. Must be greater than 0. |
| `parent` | String | ✅ | Required. The Filestore Instance to create the share for, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the share was created. |
| `backup` | String | Immutable. Full name of the Cloud Filestore Backup resource that this Share is restored from, in the format of projects/{project_id}/locations/{location_id}/backups/{backup_id}. Empty, if the Share is created from scratch and not restored from a backup. |
| `mount_name` | String | The mount name of the share. Must be 63 characters or less and consist of uppercase or lowercase letters, numbers, and underscores. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `nfs_export_options` | Vec<String> | Nfs Export Options. There is a limit of 10 export options per file share. |
| `description` | String | A description of the share with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `state` | String | Output only. The share state. |
| `name` | String | Output only. The resource name of the share, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/shares/{share_id}`. |
| `capacity_gb` | String | File share capacity in gigabytes (GB). Filestore defines 1 GB as 1024^3 bytes. Must be greater than 0. |


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
share_create_time = share.create_time
share_backup = share.backup
share_mount_name = share.mount_name
share_labels = share.labels
share_nfs_export_options = share.nfs_export_options
share_description = share.description
share_state = share.state
share_name = share.name
share_capacity_gb = share.capacity_gb
```

---


### Backup

Creates a backup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the backup was created. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `name` | String |  | Output only. The resource name of the backup, in the format `projects/{project_id}/locations/{location_id}/backups/{backup_id}`. |
| `source_instance` | String |  | The resource name of the source Filestore instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`, used to create this backup. |
| `state` | String |  | Output only. The backup state. |
| `source_instance_tier` | String |  | Output only. The service tier of the source Filestore instance that this backup is created from. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `source_file_share` | String |  | Name of the file share in the source Filestore instance that the backup is created from. |
| `download_bytes` | String |  | Output only. Amount of bytes that will be downloaded if the backup is restored |
| `description` | String |  | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `kms_key_name` | String |  | Immutable. KMS key name used for data encryption. |
| `storage_bytes` | String |  | Output only. The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion. |
| `capacity_gb` | String |  | Output only. Capacity of the source file share when the backup was created. |
| `file_system_protocol` | String |  | Output only. The file system protocol of the source Filestore instance that this backup is created from. |
| `parent` | String | ✅ | Required. The backup's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, backup locations map to Google Cloud regions, for example **us-west1**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the backup was created. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `name` | String | Output only. The resource name of the backup, in the format `projects/{project_id}/locations/{location_id}/backups/{backup_id}`. |
| `source_instance` | String | The resource name of the source Filestore instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`, used to create this backup. |
| `state` | String | Output only. The backup state. |
| `source_instance_tier` | String | Output only. The service tier of the source Filestore instance that this backup is created from. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `source_file_share` | String | Name of the file share in the source Filestore instance that the backup is created from. |
| `download_bytes` | String | Output only. Amount of bytes that will be downloaded if the backup is restored |
| `description` | String | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `kms_key_name` | String | Immutable. KMS key name used for data encryption. |
| `storage_bytes` | String | Output only. The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion. |
| `capacity_gb` | String | Output only. Capacity of the source file share when the backup was created. |
| `file_system_protocol` | String | Output only. The file system protocol of the source Filestore instance that this backup is created from. |


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
backup_create_time = backup.create_time
backup_satisfies_pzi = backup.satisfies_pzi
backup_name = backup.name
backup_source_instance = backup.source_instance
backup_state = backup.state
backup_source_instance_tier = backup.source_instance_tier
backup_tags = backup.tags
backup_satisfies_pzs = backup.satisfies_pzs
backup_labels = backup.labels
backup_source_file_share = backup.source_file_share
backup_download_bytes = backup.download_bytes
backup_description = backup.description
backup_kms_key_name = backup.kms_key_name
backup_storage_bytes = backup.storage_bytes
backup_capacity_gb = backup.capacity_gb
backup_file_system_protocol = backup.file_system_protocol
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_location_id = location.location_id
location_labels = location.labels
location_display_name = location.display_name
location_name = location.name
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.file_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
```

---


### Instance

Creates an instance. When creating from a backup, the capacity of the new instance needs to be equal to or larger than the capacity of the backup (and also equal to or larger than the minimum capacity of the tier).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `networks` | Vec<String> |  | VPC networks to which the instance is connected. For this version, only a single network is supported. |
| `tier` | String |  | The service tier of the instance. |
| `capacity_gb` | String |  | The storage capacity of the instance in gigabytes (GB = 1024^3 bytes). This capacity can be increased up to `max_capacity_gb` GB in multipliers of `capacity_step_size_gb` GB. |
| `backend_type` | String |  | Optional. Immutable. Designates the backend type of this instance. Intended to be used by internal tests and allowed customers. |
| `replication` | String |  | Optional. Replication configuration. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `max_capacity_gb` | String |  | Output only. The max capacity of the instance. |
| `kms_key_name` | String |  | KMS key name used for data encryption. |
| `performance_config` | String |  | Optional. Used to configure performance. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `name` | String |  | Output only. The resource name of the instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`. |
| `create_time` | String |  | Output only. The time when the instance was created. |
| `status_message` | String |  | Output only. Additional information about the instance state, if available. |
| `file_shares` | Vec<String> |  | File system shares on the instance. For this version, only a single file share is supported. |
| `etag` | String |  | Server-specified ETag for the instance resource to prevent simultaneous updates from overwriting each other. |
| `max_share_count` | String |  | The max number of shares allowed. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `directory_services` | String |  | Optional. Directory Services configuration. Should only be set if protocol is "NFS_V4_1". |
| `multi_share_enabled` | bool |  | Indicates whether this instance uses a multi-share configuration with which it can have more than one file-share or none at all. File-shares are added, updated and removed through the separate file-share APIs. |
| `suspension_reasons` | Vec<String> |  | Output only. Field indicates all the reasons the instance is in "SUSPENDED" state. |
| `capacity_step_size_gb` | String |  | Output only. The increase/decrease capacity step size. |
| `min_capacity_gb` | String |  | Output only. The min capacity of the instance. |
| `performance_limits` | String |  | Output only. Used for getting performance limits. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `deletion_protection_reason` | String |  | Optional. The reason for enabling deletion protection. |
| `state` | String |  | Output only. The instance state. |
| `custom_performance_supported` | bool |  | Output only. Indicates whether this instance supports configuring its performance. If true, the user can configure the instance's performance by using the 'performance_config' field. |
| `deletion_protection_enabled` | bool |  | Optional. Indicates whether the instance is protected against deletion. |
| `protocol` | String |  | Immutable. The protocol indicates the access protocol for all shares in the instance. This field is immutable and it cannot be changed after the instance has been created. Default value: `NFS_V3`. |
| `description` | String |  | The description of the instance (2048 characters or less). |
| `parent` | String | ✅ | Required. The instance's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, locations map to Google Cloud zones, for example **us-west1-b**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `networks` | Vec<String> | VPC networks to which the instance is connected. For this version, only a single network is supported. |
| `tier` | String | The service tier of the instance. |
| `capacity_gb` | String | The storage capacity of the instance in gigabytes (GB = 1024^3 bytes). This capacity can be increased up to `max_capacity_gb` GB in multipliers of `capacity_step_size_gb` GB. |
| `backend_type` | String | Optional. Immutable. Designates the backend type of this instance. Intended to be used by internal tests and allowed customers. |
| `replication` | String | Optional. Replication configuration. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `max_capacity_gb` | String | Output only. The max capacity of the instance. |
| `kms_key_name` | String | KMS key name used for data encryption. |
| `performance_config` | String | Optional. Used to configure performance. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `name` | String | Output only. The resource name of the instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`. |
| `create_time` | String | Output only. The time when the instance was created. |
| `status_message` | String | Output only. Additional information about the instance state, if available. |
| `file_shares` | Vec<String> | File system shares on the instance. For this version, only a single file share is supported. |
| `etag` | String | Server-specified ETag for the instance resource to prevent simultaneous updates from overwriting each other. |
| `max_share_count` | String | The max number of shares allowed. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag key-value pairs bound to this resource. Each key must be a namespaced name and each value a short name. Example: "123456789012/environment" : "production", "123456789013/costCenter" : "marketing" See the documentation for more information: - Namespaced name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_key - Short name: https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing#retrieving_tag_value |
| `directory_services` | String | Optional. Directory Services configuration. Should only be set if protocol is "NFS_V4_1". |
| `multi_share_enabled` | bool | Indicates whether this instance uses a multi-share configuration with which it can have more than one file-share or none at all. File-shares are added, updated and removed through the separate file-share APIs. |
| `suspension_reasons` | Vec<String> | Output only. Field indicates all the reasons the instance is in "SUSPENDED" state. |
| `capacity_step_size_gb` | String | Output only. The increase/decrease capacity step size. |
| `min_capacity_gb` | String | Output only. The min capacity of the instance. |
| `performance_limits` | String | Output only. Used for getting performance limits. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `deletion_protection_reason` | String | Optional. The reason for enabling deletion protection. |
| `state` | String | Output only. The instance state. |
| `custom_performance_supported` | bool | Output only. Indicates whether this instance supports configuring its performance. If true, the user can configure the instance's performance by using the 'performance_config' field. |
| `deletion_protection_enabled` | bool | Optional. Indicates whether the instance is protected against deletion. |
| `protocol` | String | Immutable. The protocol indicates the access protocol for all shares in the instance. This field is immutable and it cannot be changed after the instance has been created. Default value: `NFS_V3`. |
| `description` | String | The description of the instance (2048 characters or less). |


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
instance_networks = instance.networks
instance_tier = instance.tier
instance_capacity_gb = instance.capacity_gb
instance_backend_type = instance.backend_type
instance_replication = instance.replication
instance_labels = instance.labels
instance_max_capacity_gb = instance.max_capacity_gb
instance_kms_key_name = instance.kms_key_name
instance_performance_config = instance.performance_config
instance_satisfies_pzs = instance.satisfies_pzs
instance_name = instance.name
instance_create_time = instance.create_time
instance_status_message = instance.status_message
instance_file_shares = instance.file_shares
instance_etag = instance.etag
instance_max_share_count = instance.max_share_count
instance_tags = instance.tags
instance_directory_services = instance.directory_services
instance_multi_share_enabled = instance.multi_share_enabled
instance_suspension_reasons = instance.suspension_reasons
instance_capacity_step_size_gb = instance.capacity_step_size_gb
instance_min_capacity_gb = instance.min_capacity_gb
instance_performance_limits = instance.performance_limits
instance_satisfies_pzi = instance.satisfies_pzi
instance_deletion_protection_reason = instance.deletion_protection_reason
instance_state = instance.state
instance_custom_performance_supported = instance.custom_performance_supported
instance_deletion_protection_enabled = instance.deletion_protection_enabled
instance_protocol = instance.protocol
instance_description = instance.description
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
backup_0 = provider.file_api.Backup {
    parent = "value-0"
}
backup_1 = provider.file_api.Backup {
    parent = "value-1"
}
backup_2 = provider.file_api.Backup {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    backup = provider.file_api.Backup {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP File_api Documentation](https://cloud.google.com/file_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
