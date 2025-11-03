# Netapp_api Service



**Resources**: 24

---

## Overview

The netapp_api service provides access to 24 resource types:

- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Volume](#volume) [CRUD]
- [Quota_rule](#quota_rule) [CRUD]
- [Replication](#replication) [CRUD]
- [Backup_vault](#backup_vault) [CRUD]
- [Kms_config](#kms_config) [CRUD]
- [Snapshot](#snapshot) [CRUD]
- [Active_directorie](#active_directorie) [CRUD]
- [Backup](#backup) [CRUD]
- [Backup_policie](#backup_policie) [CRUD]
- [Storage_pool](#storage_pool) [CRUD]
- [Volume](#volume) [CRUD]
- [Replication](#replication) [CRUD]
- [Backup_policie](#backup_policie) [CRUD]
- [Active_directorie](#active_directorie) [CRUD]
- [Backup_vault](#backup_vault) [CRUD]
- [Kms_config](#kms_config) [CRUD]
- [Operation](#operation) [CRD]
- [Backup](#backup) [CRUD]
- [Storage_pool](#storage_pool) [CRUD]
- [Snapshot](#snapshot) [CRUD]
- [Quota_rule](#quota_rule) [CRUD]
- [Location](#location) [R]

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
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.netapp_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
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


### Volume

Creates a new Volume in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mount_options` | Vec<String> |  | Output only. Mount options of this volume |
| `create_time` | String |  | Output only. Create time of the volume |
| `replica_zone` | String |  | Output only. Specifies the replica zone for regional volume. |
| `backup_config` | String |  | BackupConfig of the volume. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `multiple_endpoints` | bool |  | Optional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints. Only the volume with large_capacity will be allowed to have multiple endpoints. |
| `encryption_type` | String |  | Output only. Specified the current volume encryption key source. |
| `description` | String |  | Optional. Description of the volume |
| `protocols` | Vec<String> |  | Required. Protocols required for the volume |
| `share_name` | String |  | Required. Share name of the volume |
| `snap_reserve` | f64 |  | Optional. Snap_reserve specifies percentage of volume storage reserved for snapshot storage. Default is 0 percent. |
| `kerberos_enabled` | bool |  | Optional. Flag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p). |
| `smb_settings` | Vec<String> |  | Optional. SMB share settings for the volume. |
| `unix_permissions` | String |  | Optional. Default unix style permission (e.g. 777) the mount point will be created with. Applicable for NFS protocol types only. |
| `kms_config` | String |  | Output only. Specifies the KMS config to be used for volume encryption. |
| `security_style` | String |  | Optional. Security Style of the Volume |
| `active_directory` | String |  | Output only. Specifies the ActiveDirectory name of a SMB volume. |
| `export_policy` | String |  | Optional. Export policy of the volume |
| `cache_parameters` | String |  | Optional. Cache parameters for the volume. |
| `name` | String |  | Identifier. Name of the volume |
| `restricted_actions` | Vec<String> |  | Optional. List of actions that are restricted on this volume. |
| `storage_pool` | String |  | Required. StoragePool name of the volume |
| `tiering_policy` | String |  | Tiering policy for the volume. |
| `has_replication` | bool |  | Output only. Indicates whether the volume is part of a replication relationship. |
| `zone` | String |  | Output only. Specifies the active zone for regional volume. |
| `ldap_enabled` | bool |  | Output only. Flag indicating if the volume is NFS LDAP enabled or not. |
| `psa_range` | String |  | Output only. This field is not implemented. The values provided in this field are ignored. |
| `hybrid_replication_parameters` | String |  | Optional. The Hybrid Replication parameters for the volume. |
| `snapshot_directory` | bool |  | Optional. Snapshot_directory if enabled (true) the volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. |
| `throughput_mibps` | f64 |  | Optional. Throughput of the volume (in MiB/s) |
| `state` | String |  | Output only. State of the volume |
| `hot_tier_size_used_gib` | String |  | Output only. Total hot tier data rounded down to the nearest GiB used by the Volume. This field is only used for flex Service Level |
| `service_level` | String |  | Output only. Service level of the volume |
| `state_details` | String |  | Output only. State details of the volume |
| `restore_parameters` | String |  | Optional. Specifies the source of the volume to be created from. |
| `capacity_gib` | String |  | Required. Capacity in GIB of the volume |
| `snapshot_policy` | String |  | Optional. SnapshotPolicy for a volume. |
| `used_gib` | String |  | Output only. Used capacity in GIB of the volume. This is computed periodically and it does not represent the realtime usage. |
| `network` | String |  | Output only. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `large_capacity` | bool |  | Optional. Flag indicating if the volume will be a large capacity volume or a regular volume. |
| `cold_tier_size_gib` | String |  | Output only. Size of the volume cold tier data rounded down to the nearest GiB. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mount_options` | Vec<String> | Output only. Mount options of this volume |
| `create_time` | String | Output only. Create time of the volume |
| `replica_zone` | String | Output only. Specifies the replica zone for regional volume. |
| `backup_config` | String | BackupConfig of the volume. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `multiple_endpoints` | bool | Optional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints. Only the volume with large_capacity will be allowed to have multiple endpoints. |
| `encryption_type` | String | Output only. Specified the current volume encryption key source. |
| `description` | String | Optional. Description of the volume |
| `protocols` | Vec<String> | Required. Protocols required for the volume |
| `share_name` | String | Required. Share name of the volume |
| `snap_reserve` | f64 | Optional. Snap_reserve specifies percentage of volume storage reserved for snapshot storage. Default is 0 percent. |
| `kerberos_enabled` | bool | Optional. Flag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p). |
| `smb_settings` | Vec<String> | Optional. SMB share settings for the volume. |
| `unix_permissions` | String | Optional. Default unix style permission (e.g. 777) the mount point will be created with. Applicable for NFS protocol types only. |
| `kms_config` | String | Output only. Specifies the KMS config to be used for volume encryption. |
| `security_style` | String | Optional. Security Style of the Volume |
| `active_directory` | String | Output only. Specifies the ActiveDirectory name of a SMB volume. |
| `export_policy` | String | Optional. Export policy of the volume |
| `cache_parameters` | String | Optional. Cache parameters for the volume. |
| `name` | String | Identifier. Name of the volume |
| `restricted_actions` | Vec<String> | Optional. List of actions that are restricted on this volume. |
| `storage_pool` | String | Required. StoragePool name of the volume |
| `tiering_policy` | String | Tiering policy for the volume. |
| `has_replication` | bool | Output only. Indicates whether the volume is part of a replication relationship. |
| `zone` | String | Output only. Specifies the active zone for regional volume. |
| `ldap_enabled` | bool | Output only. Flag indicating if the volume is NFS LDAP enabled or not. |
| `psa_range` | String | Output only. This field is not implemented. The values provided in this field are ignored. |
| `hybrid_replication_parameters` | String | Optional. The Hybrid Replication parameters for the volume. |
| `snapshot_directory` | bool | Optional. Snapshot_directory if enabled (true) the volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. |
| `throughput_mibps` | f64 | Optional. Throughput of the volume (in MiB/s) |
| `state` | String | Output only. State of the volume |
| `hot_tier_size_used_gib` | String | Output only. Total hot tier data rounded down to the nearest GiB used by the Volume. This field is only used for flex Service Level |
| `service_level` | String | Output only. Service level of the volume |
| `state_details` | String | Output only. State details of the volume |
| `restore_parameters` | String | Optional. Specifies the source of the volume to be created from. |
| `capacity_gib` | String | Required. Capacity in GIB of the volume |
| `snapshot_policy` | String | Optional. SnapshotPolicy for a volume. |
| `used_gib` | String | Output only. Used capacity in GIB of the volume. This is computed periodically and it does not represent the realtime usage. |
| `network` | String | Output only. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `large_capacity` | bool | Optional. Flag indicating if the volume will be a large capacity volume or a regular volume. |
| `cold_tier_size_gib` | String | Output only. Size of the volume cold tier data rounded down to the nearest GiB. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create volume
volume = provider.netapp_api.Volume {
    parent = "value"  # Required. Value for parent.
}

# Access volume outputs
volume_id = volume.id
volume_mount_options = volume.mount_options
volume_create_time = volume.create_time
volume_replica_zone = volume.replica_zone
volume_backup_config = volume.backup_config
volume_labels = volume.labels
volume_multiple_endpoints = volume.multiple_endpoints
volume_encryption_type = volume.encryption_type
volume_description = volume.description
volume_protocols = volume.protocols
volume_share_name = volume.share_name
volume_snap_reserve = volume.snap_reserve
volume_kerberos_enabled = volume.kerberos_enabled
volume_smb_settings = volume.smb_settings
volume_unix_permissions = volume.unix_permissions
volume_kms_config = volume.kms_config
volume_security_style = volume.security_style
volume_active_directory = volume.active_directory
volume_export_policy = volume.export_policy
volume_cache_parameters = volume.cache_parameters
volume_name = volume.name
volume_restricted_actions = volume.restricted_actions
volume_storage_pool = volume.storage_pool
volume_tiering_policy = volume.tiering_policy
volume_has_replication = volume.has_replication
volume_zone = volume.zone
volume_ldap_enabled = volume.ldap_enabled
volume_psa_range = volume.psa_range
volume_hybrid_replication_parameters = volume.hybrid_replication_parameters
volume_snapshot_directory = volume.snapshot_directory
volume_throughput_mibps = volume.throughput_mibps
volume_state = volume.state
volume_hot_tier_size_used_gib = volume.hot_tier_size_used_gib
volume_service_level = volume.service_level
volume_state_details = volume.state_details
volume_restore_parameters = volume.restore_parameters
volume_capacity_gib = volume.capacity_gib
volume_snapshot_policy = volume.snapshot_policy
volume_used_gib = volume.used_gib
volume_network = volume.network
volume_large_capacity = volume.large_capacity
volume_cold_tier_size_gib = volume.cold_tier_size_gib
```

---


### Quota_rule

Creates a new quota rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target` | String |  | Optional. The quota rule applies to the specified user or group, identified by a Unix UID/GID, Windows SID, or null for default. |
| `type` | String |  | Required. The type of quota rule. |
| `description` | String |  | Optional. Description of the quota rule |
| `state_details` | String |  | Output only. State details of the quota rule |
| `disk_limit_mib` | i64 |  | Required. The maximum allowed disk space in MiB. |
| `create_time` | String |  | Output only. Create time of the quota rule |
| `labels` | HashMap<String, String> |  | Optional. Labels of the quota rule |
| `name` | String |  | Identifier. The resource name of the quota rule. Format: `projects/{project_number}/locations/{location_id}/volumes/volumes/{volume_id}/quotaRules/{quota_rule_id}`. |
| `state` | String |  | Output only. State of the quota rule |
| `parent` | String | ✅ | Required. Parent value for CreateQuotaRuleRequest |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target` | String | Optional. The quota rule applies to the specified user or group, identified by a Unix UID/GID, Windows SID, or null for default. |
| `type` | String | Required. The type of quota rule. |
| `description` | String | Optional. Description of the quota rule |
| `state_details` | String | Output only. State details of the quota rule |
| `disk_limit_mib` | i64 | Required. The maximum allowed disk space in MiB. |
| `create_time` | String | Output only. Create time of the quota rule |
| `labels` | HashMap<String, String> | Optional. Labels of the quota rule |
| `name` | String | Identifier. The resource name of the quota rule. Format: `projects/{project_number}/locations/{location_id}/volumes/volumes/{volume_id}/quotaRules/{quota_rule_id}`. |
| `state` | String | Output only. State of the quota rule |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create quota_rule
quota_rule = provider.netapp_api.Quota_rule {
    parent = "value"  # Required. Parent value for CreateQuotaRuleRequest
}

# Access quota_rule outputs
quota_rule_id = quota_rule.id
quota_rule_target = quota_rule.target
quota_rule_type = quota_rule.type
quota_rule_description = quota_rule.description
quota_rule_state_details = quota_rule.state_details
quota_rule_disk_limit_mib = quota_rule.disk_limit_mib
quota_rule_create_time = quota_rule.create_time
quota_rule_labels = quota_rule.labels
quota_rule_name = quota_rule.name
quota_rule_state = quota_rule.state
```

---


### Replication

Create a new replication for a volume.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_details` | String |  | Output only. State details of the replication. |
| `destination_volume` | String |  | Output only. Full name of destination volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `mirror_state` | String |  | Output only. Indicates the state of mirroring. |
| `cluster_location` | String |  | Optional. Location of the user cluster. |
| `role` | String |  | Output only. Indicates whether this points to source or destination. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `replication_schedule` | String |  | Required. Indicates the schedule for replication. |
| `create_time` | String |  | Output only. Replication create time. |
| `hybrid_peering_details` | String |  | Output only. Hybrid peering details. |
| `transfer_stats` | String |  | Output only. Replication transfer statistics. |
| `healthy` | bool |  | Output only. Condition of the relationship. Can be one of the following: - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer. - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer. |
| `destination_volume_parameters` | String |  | Required. Input only. Destination volume parameters |
| `state` | String |  | Output only. State of the replication. |
| `description` | String |  | A description about this replication relationship. |
| `source_volume` | String |  | Output only. Full name of source volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `hybrid_replication_user_commands` | String |  | Output only. Copy pastable snapmirror commands to be executed on onprem cluster by the customer. |
| `name` | String |  | Identifier. The resource name of the Replication. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}`. |
| `hybrid_replication_type` | String |  | Output only. Type of the hybrid replication. |
| `parent` | String | ✅ | Required. The NetApp volume to create the replications of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state_details` | String | Output only. State details of the replication. |
| `destination_volume` | String | Output only. Full name of destination volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `mirror_state` | String | Output only. Indicates the state of mirroring. |
| `cluster_location` | String | Optional. Location of the user cluster. |
| `role` | String | Output only. Indicates whether this points to source or destination. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `replication_schedule` | String | Required. Indicates the schedule for replication. |
| `create_time` | String | Output only. Replication create time. |
| `hybrid_peering_details` | String | Output only. Hybrid peering details. |
| `transfer_stats` | String | Output only. Replication transfer statistics. |
| `healthy` | bool | Output only. Condition of the relationship. Can be one of the following: - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer. - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer. |
| `destination_volume_parameters` | String | Required. Input only. Destination volume parameters |
| `state` | String | Output only. State of the replication. |
| `description` | String | A description about this replication relationship. |
| `source_volume` | String | Output only. Full name of source volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `hybrid_replication_user_commands` | String | Output only. Copy pastable snapmirror commands to be executed on onprem cluster by the customer. |
| `name` | String | Identifier. The resource name of the Replication. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}`. |
| `hybrid_replication_type` | String | Output only. Type of the hybrid replication. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create replication
replication = provider.netapp_api.Replication {
    parent = "value"  # Required. The NetApp volume to create the replications of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}`
}

# Access replication outputs
replication_id = replication.id
replication_state_details = replication.state_details
replication_destination_volume = replication.destination_volume
replication_mirror_state = replication.mirror_state
replication_cluster_location = replication.cluster_location
replication_role = replication.role
replication_labels = replication.labels
replication_replication_schedule = replication.replication_schedule
replication_create_time = replication.create_time
replication_hybrid_peering_details = replication.hybrid_peering_details
replication_transfer_stats = replication.transfer_stats
replication_healthy = replication.healthy
replication_destination_volume_parameters = replication.destination_volume_parameters
replication_state = replication.state
replication_description = replication.description
replication_source_volume = replication.source_volume
replication_hybrid_replication_user_commands = replication.hybrid_replication_user_commands
replication_name = replication.name
replication_hybrid_replication_type = replication.hybrid_replication_type
```

---


### Backup_vault

Creates new backup vault

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the backup vault. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`. |
| `description` | String |  | Description of the backup vault. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `state` | String |  | Output only. The backup vault state. |
| `create_time` | String |  | Output only. Create time of the backup vault. |
| `source_region` | String |  | Output only. Region in which the backup vault is created. Format: `projects/{project_id}/locations/{location}` |
| `backup_vault_type` | String |  | Optional. Type of backup vault to be created. Default is IN_REGION. |
| `source_backup_vault` | String |  | Output only. Name of the Backup vault created in source region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `backup_retention_policy` | String |  | Optional. Backup retention policy defining the retenton of backups. |
| `destination_backup_vault` | String |  | Output only. Name of the Backup vault created in backup region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `backup_region` | String |  | Optional. Region where the backups are stored. Format: `projects/{project_id}/locations/{location}` |
| `parent` | String | ✅ | Required. The location to create the backup vaults, in the format `projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the backup vault. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`. |
| `description` | String | Description of the backup vault. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `state` | String | Output only. The backup vault state. |
| `create_time` | String | Output only. Create time of the backup vault. |
| `source_region` | String | Output only. Region in which the backup vault is created. Format: `projects/{project_id}/locations/{location}` |
| `backup_vault_type` | String | Optional. Type of backup vault to be created. Default is IN_REGION. |
| `source_backup_vault` | String | Output only. Name of the Backup vault created in source region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `backup_retention_policy` | String | Optional. Backup retention policy defining the retenton of backups. |
| `destination_backup_vault` | String | Output only. Name of the Backup vault created in backup region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `backup_region` | String | Optional. Region where the backups are stored. Format: `projects/{project_id}/locations/{location}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_vault
backup_vault = provider.netapp_api.Backup_vault {
    parent = "value"  # Required. The location to create the backup vaults, in the format `projects/{project_id}/locations/{location}`
}

# Access backup_vault outputs
backup_vault_id = backup_vault.id
backup_vault_name = backup_vault.name
backup_vault_description = backup_vault.description
backup_vault_labels = backup_vault.labels
backup_vault_state = backup_vault.state
backup_vault_create_time = backup_vault.create_time
backup_vault_source_region = backup_vault.source_region
backup_vault_backup_vault_type = backup_vault.backup_vault_type
backup_vault_source_backup_vault = backup_vault.source_backup_vault
backup_vault_backup_retention_policy = backup_vault.backup_retention_policy
backup_vault_destination_backup_vault = backup_vault.destination_backup_vault
backup_vault_backup_region = backup_vault.backup_region
```

---


### Kms_config

Creates a new KMS config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the KmsConfig. |
| `service_account` | String |  | Output only. The Service account which will have access to the customer provided encryption key. |
| `crypto_key_name` | String |  | Required. Customer managed crypto key resource full name. Format: projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}. |
| `name` | String |  | Identifier. Name of the KmsConfig. |
| `state_details` | String |  | Output only. State details of the KmsConfig. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `create_time` | String |  | Output only. Create time of the KmsConfig. |
| `instructions` | String |  | Output only. Instructions to provide the access to the customer provided encryption key. |
| `description` | String |  | Description of the KmsConfig. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the KmsConfig. |
| `service_account` | String | Output only. The Service account which will have access to the customer provided encryption key. |
| `crypto_key_name` | String | Required. Customer managed crypto key resource full name. Format: projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}. |
| `name` | String | Identifier. Name of the KmsConfig. |
| `state_details` | String | Output only. State details of the KmsConfig. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `create_time` | String | Output only. Create time of the KmsConfig. |
| `instructions` | String | Output only. Instructions to provide the access to the customer provided encryption key. |
| `description` | String | Description of the KmsConfig. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create kms_config
kms_config = provider.netapp_api.Kms_config {
    parent = "value"  # Required. Value for parent.
}

# Access kms_config outputs
kms_config_id = kms_config.id
kms_config_state = kms_config.state
kms_config_service_account = kms_config.service_account
kms_config_crypto_key_name = kms_config.crypto_key_name
kms_config_name = kms_config.name
kms_config_state_details = kms_config.state_details
kms_config_labels = kms_config.labels
kms_config_create_time = kms_config.create_time
kms_config_instructions = kms_config.instructions
kms_config_description = kms_config.description
```

---


### Snapshot

Create a new snapshot for a volume.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `used_bytes` | f64 |  | Output only. Current storage usage for the snapshot in bytes. |
| `name` | String |  | Identifier. The resource name of the snapshot. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}`. |
| `state` | String |  | Output only. The snapshot state. |
| `description` | String |  | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `create_time` | String |  | Output only. The time when the snapshot was created. |
| `state_details` | String |  | Output only. State details of the storage pool |
| `parent` | String | ✅ | Required. The NetApp volume to create the snapshots of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `used_bytes` | f64 | Output only. Current storage usage for the snapshot in bytes. |
| `name` | String | Identifier. The resource name of the snapshot. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}`. |
| `state` | String | Output only. The snapshot state. |
| `description` | String | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `create_time` | String | Output only. The time when the snapshot was created. |
| `state_details` | String | Output only. State details of the storage pool |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create snapshot
snapshot = provider.netapp_api.Snapshot {
    parent = "value"  # Required. The NetApp volume to create the snapshots of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}`
}

# Access snapshot outputs
snapshot_id = snapshot.id
snapshot_used_bytes = snapshot.used_bytes
snapshot_name = snapshot.name
snapshot_state = snapshot.state
snapshot_description = snapshot.description
snapshot_labels = snapshot.labels
snapshot_create_time = snapshot.create_time
snapshot_state_details = snapshot.state_details
```

---


### Active_directorie

CreateActiveDirectory Creates the active directory specified in the request.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `administrators` | Vec<String> |  | Optional. Users to be added to the Built-in Admininstrators group. |
| `backup_operators` | Vec<String> |  | Optional. Users to be added to the Built-in Backup Operator active directory group. |
| `description` | String |  | Description of the active directory. |
| `create_time` | String |  | Output only. Create time of the active directory. |
| `kdc_hostname` | String |  | Name of the active directory machine. This optional parameter is used only while creating kerberos volume |
| `organizational_unit` | String |  | The Organizational Unit (OU) within the Windows Active Directory the user belongs to. |
| `aes_encryption` | bool |  | If enabled, AES encryption will be enabled for SMB communication. |
| `password` | String |  | Required. Password of the Active Directory domain administrator. |
| `username` | String |  | Required. Username of the Active Directory domain administrator. |
| `kdc_ip` | String |  | KDC server IP address for the active directory machine. |
| `name` | String |  | Identifier. The resource name of the active directory. Format: `projects/{project_number}/locations/{location_id}/activeDirectories/{active_directory_id}`. |
| `security_operators` | Vec<String> |  | Optional. Domain users to be given the SeSecurityPrivilege. |
| `net_bios_prefix` | String |  | Required. NetBIOSPrefix is used as a prefix for SMB server name. |
| `site` | String |  | The Active Directory site the service will limit Domain Controller discovery too. |
| `nfs_users_with_ldap` | bool |  | If enabled, will allow access to local users and LDAP users. If access is needed for only LDAP users, it has to be disabled. |
| `labels` | HashMap<String, String> |  | Labels for the active directory. |
| `encrypt_dc_connections` | bool |  | If enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted. |
| `ldap_signing` | bool |  | Specifies whether or not the LDAP traffic needs to be signed. |
| `state` | String |  | Output only. The state of the AD. |
| `domain` | String |  | Required. Name of the Active Directory domain |
| `dns` | String |  | Required. Comma separated list of DNS server IP addresses for the Active Directory domain. |
| `state_details` | String |  | Output only. The state details of the Active Directory. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `administrators` | Vec<String> | Optional. Users to be added to the Built-in Admininstrators group. |
| `backup_operators` | Vec<String> | Optional. Users to be added to the Built-in Backup Operator active directory group. |
| `description` | String | Description of the active directory. |
| `create_time` | String | Output only. Create time of the active directory. |
| `kdc_hostname` | String | Name of the active directory machine. This optional parameter is used only while creating kerberos volume |
| `organizational_unit` | String | The Organizational Unit (OU) within the Windows Active Directory the user belongs to. |
| `aes_encryption` | bool | If enabled, AES encryption will be enabled for SMB communication. |
| `password` | String | Required. Password of the Active Directory domain administrator. |
| `username` | String | Required. Username of the Active Directory domain administrator. |
| `kdc_ip` | String | KDC server IP address for the active directory machine. |
| `name` | String | Identifier. The resource name of the active directory. Format: `projects/{project_number}/locations/{location_id}/activeDirectories/{active_directory_id}`. |
| `security_operators` | Vec<String> | Optional. Domain users to be given the SeSecurityPrivilege. |
| `net_bios_prefix` | String | Required. NetBIOSPrefix is used as a prefix for SMB server name. |
| `site` | String | The Active Directory site the service will limit Domain Controller discovery too. |
| `nfs_users_with_ldap` | bool | If enabled, will allow access to local users and LDAP users. If access is needed for only LDAP users, it has to be disabled. |
| `labels` | HashMap<String, String> | Labels for the active directory. |
| `encrypt_dc_connections` | bool | If enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted. |
| `ldap_signing` | bool | Specifies whether or not the LDAP traffic needs to be signed. |
| `state` | String | Output only. The state of the AD. |
| `domain` | String | Required. Name of the Active Directory domain |
| `dns` | String | Required. Comma separated list of DNS server IP addresses for the Active Directory domain. |
| `state_details` | String | Output only. The state details of the Active Directory. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create active_directorie
active_directorie = provider.netapp_api.Active_directorie {
    parent = "value"  # Required. Value for parent.
}

# Access active_directorie outputs
active_directorie_id = active_directorie.id
active_directorie_administrators = active_directorie.administrators
active_directorie_backup_operators = active_directorie.backup_operators
active_directorie_description = active_directorie.description
active_directorie_create_time = active_directorie.create_time
active_directorie_kdc_hostname = active_directorie.kdc_hostname
active_directorie_organizational_unit = active_directorie.organizational_unit
active_directorie_aes_encryption = active_directorie.aes_encryption
active_directorie_password = active_directorie.password
active_directorie_username = active_directorie.username
active_directorie_kdc_ip = active_directorie.kdc_ip
active_directorie_name = active_directorie.name
active_directorie_security_operators = active_directorie.security_operators
active_directorie_net_bios_prefix = active_directorie.net_bios_prefix
active_directorie_site = active_directorie.site
active_directorie_nfs_users_with_ldap = active_directorie.nfs_users_with_ldap
active_directorie_labels = active_directorie.labels
active_directorie_encrypt_dc_connections = active_directorie.encrypt_dc_connections
active_directorie_ldap_signing = active_directorie.ldap_signing
active_directorie_state = active_directorie.state
active_directorie_domain = active_directorie.domain
active_directorie_dns = active_directorie.dns
active_directorie_state_details = active_directorie.state_details
```

---


### Backup

Creates a backup from the volume specified in the request The backup can be created from the given snapshot if specified in the request. If no snapshot specified, there'll be a new snapshot taken to initiate the backup creation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `volume_region` | String |  | Output only. Region of the volume from which the backup was created. Format: `projects/{project_id}/locations/{location}` |
| `backup_region` | String |  | Output only. Region in which backup is stored. Format: `projects/{project_id}/locations/{location}` |
| `state` | String |  | Output only. The backup state. |
| `description` | String |  | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `chain_storage_bytes` | String |  | Output only. Total size of all backups in a chain in bytes = baseline backup size + sum(incremental backup size) |
| `create_time` | String |  | Output only. The time when the backup was created. |
| `name` | String |  | Identifier. The resource name of the backup. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`. |
| `source_snapshot` | String |  | If specified, backup will be created from the given snapshot. If not specified, there will be a new snapshot taken to initiate the backup creation. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}` |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `backup_type` | String |  | Output only. Type of backup, manually created or created by a backup policy. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use |
| `source_volume` | String |  | Volume full name of this backup belongs to. Format: `projects/{projects_id}/locations/{location}/volumes/{volume_id}` |
| `enforced_retention_end_time` | String |  | Output only. The time until which the backup is not deletable. |
| `volume_usage_bytes` | String |  | Output only. Size of the file system when the backup was created. When creating a new volume from the backup, the volume capacity will have to be at least as big. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use |
| `parent` | String | ✅ | Required. The NetApp backupVault to create the backups of, in the format `projects/*/locations/*/backupVaults/{backup_vault_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `volume_region` | String | Output only. Region of the volume from which the backup was created. Format: `projects/{project_id}/locations/{location}` |
| `backup_region` | String | Output only. Region in which backup is stored. Format: `projects/{project_id}/locations/{location}` |
| `state` | String | Output only. The backup state. |
| `description` | String | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `chain_storage_bytes` | String | Output only. Total size of all backups in a chain in bytes = baseline backup size + sum(incremental backup size) |
| `create_time` | String | Output only. The time when the backup was created. |
| `name` | String | Identifier. The resource name of the backup. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`. |
| `source_snapshot` | String | If specified, backup will be created from the given snapshot. If not specified, there will be a new snapshot taken to initiate the backup creation. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}` |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `backup_type` | String | Output only. Type of backup, manually created or created by a backup policy. |
| `satisfies_pzs` | bool | Output only. Reserved for future use |
| `source_volume` | String | Volume full name of this backup belongs to. Format: `projects/{projects_id}/locations/{location}/volumes/{volume_id}` |
| `enforced_retention_end_time` | String | Output only. The time until which the backup is not deletable. |
| `volume_usage_bytes` | String | Output only. Size of the file system when the backup was created. When creating a new volume from the backup, the volume capacity will have to be at least as big. |
| `satisfies_pzi` | bool | Output only. Reserved for future use |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.netapp_api.Backup {
    parent = "value"  # Required. The NetApp backupVault to create the backups of, in the format `projects/*/locations/*/backupVaults/{backup_vault_id}`
}

# Access backup outputs
backup_id = backup.id
backup_volume_region = backup.volume_region
backup_backup_region = backup.backup_region
backup_state = backup.state
backup_description = backup.description
backup_chain_storage_bytes = backup.chain_storage_bytes
backup_create_time = backup.create_time
backup_name = backup.name
backup_source_snapshot = backup.source_snapshot
backup_labels = backup.labels
backup_backup_type = backup.backup_type
backup_satisfies_pzs = backup.satisfies_pzs
backup_source_volume = backup.source_volume
backup_enforced_retention_end_time = backup.enforced_retention_end_time
backup_volume_usage_bytes = backup.volume_usage_bytes
backup_satisfies_pzi = backup.satisfies_pzi
```

---


### Backup_policie

Creates new backup policy

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `assigned_volume_count` | i64 |  | Output only. The total number of volumes assigned by this backup policy. |
| `enabled` | bool |  | If enabled, make backups automatically according to the schedules. This will be applied to all volumes that have this policy attached and enforced on volume level. If not specified, default is true. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `monthly_backup_limit` | i64 |  | Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `description` | String |  | Description of the backup policy. |
| `daily_backup_limit` | i64 |  | Number of daily backups to keep. Note that the minimum daily backup limit is 2. |
| `create_time` | String |  | Output only. The time when the backup policy was created. |
| `name` | String |  | Identifier. The resource name of the backup policy. Format: `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`. |
| `weekly_backup_limit` | i64 |  | Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `state` | String |  | Output only. The backup policy state. |
| `parent` | String | ✅ | Required. The location to create the backup policies of, in the format `projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assigned_volume_count` | i64 | Output only. The total number of volumes assigned by this backup policy. |
| `enabled` | bool | If enabled, make backups automatically according to the schedules. This will be applied to all volumes that have this policy attached and enforced on volume level. If not specified, default is true. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `monthly_backup_limit` | i64 | Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `description` | String | Description of the backup policy. |
| `daily_backup_limit` | i64 | Number of daily backups to keep. Note that the minimum daily backup limit is 2. |
| `create_time` | String | Output only. The time when the backup policy was created. |
| `name` | String | Identifier. The resource name of the backup policy. Format: `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`. |
| `weekly_backup_limit` | i64 | Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `state` | String | Output only. The backup policy state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_policie
backup_policie = provider.netapp_api.Backup_policie {
    parent = "value"  # Required. The location to create the backup policies of, in the format `projects/{project_id}/locations/{location}`
}

# Access backup_policie outputs
backup_policie_id = backup_policie.id
backup_policie_assigned_volume_count = backup_policie.assigned_volume_count
backup_policie_enabled = backup_policie.enabled
backup_policie_labels = backup_policie.labels
backup_policie_monthly_backup_limit = backup_policie.monthly_backup_limit
backup_policie_description = backup_policie.description
backup_policie_daily_backup_limit = backup_policie.daily_backup_limit
backup_policie_create_time = backup_policie.create_time
backup_policie_name = backup_policie.name
backup_policie_weekly_backup_limit = backup_policie.weekly_backup_limit
backup_policie_state = backup_policie.state
```

---


### Storage_pool

Creates a new storage pool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `global_access_allowed` | bool |  | Deprecated. Used to allow SO pool to access AD or DNS server from other regions. |
| `description` | String |  | Optional. Description of the storage pool |
| `custom_performance_enabled` | bool |  | Optional. True if using Independent Scaling of capacity and performance (Hyperdisk) By default set to false |
| `capacity_gib` | String |  | Required. Capacity in GIB of the pool |
| `available_throughput_mibps` | f64 |  | Output only. Available throughput of the storage pool (in MiB/s). |
| `enable_hot_tier_auto_resize` | bool |  | Optional. Flag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true. The increment will kick in only if the new size after increment is still less than or equal to storage pool size. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `replica_zone` | String |  | Optional. Specifies the replica zone for regional storagePool. |
| `total_throughput_mibps` | String |  | Optional. Custom Performance Total Throughput of the pool (in MiBps) |
| `volume_count` | i64 |  | Output only. Volume count of the storage pool |
| `psa_range` | String |  | Optional. This field is not implemented. The values provided in this field are ignored. |
| `create_time` | String |  | Output only. Create time of the storage pool |
| `network` | String |  | Required. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `service_level` | String |  | Required. Service level of the storage pool |
| `hot_tier_size_gib` | String |  | Optional. Total hot tier capacity for the Storage Pool. It is applicable only to Flex service level. It should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set. |
| `kms_config` | String |  | Optional. Specifies the KMS config to be used for volume encryption. |
| `encryption_type` | String |  | Output only. Specifies the current pool encryption key source. |
| `active_directory` | String |  | Optional. Specifies the Active Directory to be used for creating a SMB volume. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use |
| `hot_tier_size_used_gib` | String |  | Output only. Total hot tier data rounded down to the nearest GiB used by the storage pool. |
| `name` | String |  | Identifier. Name of the storage pool |
| `volume_capacity_gib` | String |  | Output only. Allocated size of all volumes in GIB in the storage pool |
| `cold_tier_size_used_gib` | String |  | Output only. Total cold tier data rounded down to the nearest GiB used by the storage pool. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use |
| `ldap_enabled` | bool |  | Optional. Flag indicating if the pool is NFS LDAP enabled or not. |
| `total_iops` | String |  | Optional. Custom Performance Total IOPS of the pool if not provided, it will be calculated based on the total_throughput_mibps |
| `state_details` | String |  | Output only. State details of the storage pool |
| `zone` | String |  | Optional. Specifies the active zone for regional storagePool. |
| `qos_type` | String |  | Optional. QoS (Quality of Service) Type of the storage pool |
| `allow_auto_tiering` | bool |  | Optional. True if the storage pool supports Auto Tiering enabled volumes. Default is false. Auto-tiering can be enabled after storage pool creation but it can't be disabled once enabled. |
| `state` | String |  | Output only. State of the storage pool |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `global_access_allowed` | bool | Deprecated. Used to allow SO pool to access AD or DNS server from other regions. |
| `description` | String | Optional. Description of the storage pool |
| `custom_performance_enabled` | bool | Optional. True if using Independent Scaling of capacity and performance (Hyperdisk) By default set to false |
| `capacity_gib` | String | Required. Capacity in GIB of the pool |
| `available_throughput_mibps` | f64 | Output only. Available throughput of the storage pool (in MiB/s). |
| `enable_hot_tier_auto_resize` | bool | Optional. Flag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true. The increment will kick in only if the new size after increment is still less than or equal to storage pool size. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `replica_zone` | String | Optional. Specifies the replica zone for regional storagePool. |
| `total_throughput_mibps` | String | Optional. Custom Performance Total Throughput of the pool (in MiBps) |
| `volume_count` | i64 | Output only. Volume count of the storage pool |
| `psa_range` | String | Optional. This field is not implemented. The values provided in this field are ignored. |
| `create_time` | String | Output only. Create time of the storage pool |
| `network` | String | Required. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `service_level` | String | Required. Service level of the storage pool |
| `hot_tier_size_gib` | String | Optional. Total hot tier capacity for the Storage Pool. It is applicable only to Flex service level. It should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set. |
| `kms_config` | String | Optional. Specifies the KMS config to be used for volume encryption. |
| `encryption_type` | String | Output only. Specifies the current pool encryption key source. |
| `active_directory` | String | Optional. Specifies the Active Directory to be used for creating a SMB volume. |
| `satisfies_pzs` | bool | Output only. Reserved for future use |
| `hot_tier_size_used_gib` | String | Output only. Total hot tier data rounded down to the nearest GiB used by the storage pool. |
| `name` | String | Identifier. Name of the storage pool |
| `volume_capacity_gib` | String | Output only. Allocated size of all volumes in GIB in the storage pool |
| `cold_tier_size_used_gib` | String | Output only. Total cold tier data rounded down to the nearest GiB used by the storage pool. |
| `satisfies_pzi` | bool | Output only. Reserved for future use |
| `ldap_enabled` | bool | Optional. Flag indicating if the pool is NFS LDAP enabled or not. |
| `total_iops` | String | Optional. Custom Performance Total IOPS of the pool if not provided, it will be calculated based on the total_throughput_mibps |
| `state_details` | String | Output only. State details of the storage pool |
| `zone` | String | Optional. Specifies the active zone for regional storagePool. |
| `qos_type` | String | Optional. QoS (Quality of Service) Type of the storage pool |
| `allow_auto_tiering` | bool | Optional. True if the storage pool supports Auto Tiering enabled volumes. Default is false. Auto-tiering can be enabled after storage pool creation but it can't be disabled once enabled. |
| `state` | String | Output only. State of the storage pool |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create storage_pool
storage_pool = provider.netapp_api.Storage_pool {
    parent = "value"  # Required. Value for parent.
}

# Access storage_pool outputs
storage_pool_id = storage_pool.id
storage_pool_global_access_allowed = storage_pool.global_access_allowed
storage_pool_description = storage_pool.description
storage_pool_custom_performance_enabled = storage_pool.custom_performance_enabled
storage_pool_capacity_gib = storage_pool.capacity_gib
storage_pool_available_throughput_mibps = storage_pool.available_throughput_mibps
storage_pool_enable_hot_tier_auto_resize = storage_pool.enable_hot_tier_auto_resize
storage_pool_labels = storage_pool.labels
storage_pool_replica_zone = storage_pool.replica_zone
storage_pool_total_throughput_mibps = storage_pool.total_throughput_mibps
storage_pool_volume_count = storage_pool.volume_count
storage_pool_psa_range = storage_pool.psa_range
storage_pool_create_time = storage_pool.create_time
storage_pool_network = storage_pool.network
storage_pool_service_level = storage_pool.service_level
storage_pool_hot_tier_size_gib = storage_pool.hot_tier_size_gib
storage_pool_kms_config = storage_pool.kms_config
storage_pool_encryption_type = storage_pool.encryption_type
storage_pool_active_directory = storage_pool.active_directory
storage_pool_satisfies_pzs = storage_pool.satisfies_pzs
storage_pool_hot_tier_size_used_gib = storage_pool.hot_tier_size_used_gib
storage_pool_name = storage_pool.name
storage_pool_volume_capacity_gib = storage_pool.volume_capacity_gib
storage_pool_cold_tier_size_used_gib = storage_pool.cold_tier_size_used_gib
storage_pool_satisfies_pzi = storage_pool.satisfies_pzi
storage_pool_ldap_enabled = storage_pool.ldap_enabled
storage_pool_total_iops = storage_pool.total_iops
storage_pool_state_details = storage_pool.state_details
storage_pool_zone = storage_pool.zone
storage_pool_qos_type = storage_pool.qos_type
storage_pool_allow_auto_tiering = storage_pool.allow_auto_tiering
storage_pool_state = storage_pool.state
```

---


### Volume

Creates a new Volume in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `restore_parameters` | String |  | Optional. Specifies the source of the volume to be created from. |
| `service_level` | String |  | Output only. Service level of the volume |
| `backup_config` | String |  | BackupConfig of the volume. |
| `encryption_type` | String |  | Output only. Specified the current volume encryption key source. |
| `has_replication` | bool |  | Output only. Indicates whether the volume is part of a replication relationship. |
| `mount_options` | Vec<String> |  | Output only. Mount options of this volume |
| `export_policy` | String |  | Optional. Export policy of the volume |
| `smb_settings` | Vec<String> |  | Optional. SMB share settings for the volume. |
| `state_details` | String |  | Output only. State details of the volume |
| `psa_range` | String |  | Output only. This field is not implemented. The values provided in this field are ignored. |
| `hot_tier_size_used_gib` | String |  | Output only. Total hot tier data rounded down to the nearest GiB used by the Volume. This field is only used for flex Service Level |
| `replica_zone` | String |  | Output only. Specifies the replica zone for regional volume. |
| `snapshot_directory` | bool |  | Optional. Snapshot_directory if enabled (true) the volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. |
| `unix_permissions` | String |  | Optional. Default unix style permission (e.g. 777) the mount point will be created with. Applicable for NFS protocol types only. |
| `snapshot_policy` | String |  | Optional. SnapshotPolicy for a volume. |
| `multiple_endpoints` | bool |  | Optional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints. Only the volume with large_capacity will be allowed to have multiple endpoints. |
| `name` | String |  | Identifier. Name of the volume |
| `storage_pool` | String |  | Required. StoragePool name of the volume |
| `large_capacity` | bool |  | Optional. Flag indicating if the volume will be a large capacity volume or a regular volume. |
| `hybrid_replication_parameters` | String |  | Optional. The Hybrid Replication parameters for the volume. |
| `kms_config` | String |  | Output only. Specifies the KMS config to be used for volume encryption. |
| `network` | String |  | Output only. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `snap_reserve` | f64 |  | Optional. Snap_reserve specifies percentage of volume storage reserved for snapshot storage. Default is 0 percent. |
| `state` | String |  | Output only. State of the volume |
| `protocols` | Vec<String> |  | Required. Protocols required for the volume |
| `description` | String |  | Optional. Description of the volume |
| `ldap_enabled` | bool |  | Output only. Flag indicating if the volume is NFS LDAP enabled or not. |
| `capacity_gib` | String |  | Required. Capacity in GIB of the volume |
| `create_time` | String |  | Output only. Create time of the volume |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `cold_tier_size_gib` | String |  | Output only. Size of the volume cold tier data rounded down to the nearest GiB. |
| `security_style` | String |  | Optional. Security Style of the Volume |
| `throughput_mibps` | f64 |  | Optional. Throughput of the volume (in MiB/s) |
| `tiering_policy` | String |  | Tiering policy for the volume. |
| `zone` | String |  | Output only. Specifies the active zone for regional volume. |
| `kerberos_enabled` | bool |  | Optional. Flag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p). |
| `active_directory` | String |  | Output only. Specifies the ActiveDirectory name of a SMB volume. |
| `restricted_actions` | Vec<String> |  | Optional. List of actions that are restricted on this volume. |
| `share_name` | String |  | Required. Share name of the volume |
| `used_gib` | String |  | Output only. Used capacity in GIB of the volume. This is computed periodically and it does not represent the realtime usage. |
| `cache_parameters` | String |  | Optional. Cache parameters for the volume. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `restore_parameters` | String | Optional. Specifies the source of the volume to be created from. |
| `service_level` | String | Output only. Service level of the volume |
| `backup_config` | String | BackupConfig of the volume. |
| `encryption_type` | String | Output only. Specified the current volume encryption key source. |
| `has_replication` | bool | Output only. Indicates whether the volume is part of a replication relationship. |
| `mount_options` | Vec<String> | Output only. Mount options of this volume |
| `export_policy` | String | Optional. Export policy of the volume |
| `smb_settings` | Vec<String> | Optional. SMB share settings for the volume. |
| `state_details` | String | Output only. State details of the volume |
| `psa_range` | String | Output only. This field is not implemented. The values provided in this field are ignored. |
| `hot_tier_size_used_gib` | String | Output only. Total hot tier data rounded down to the nearest GiB used by the Volume. This field is only used for flex Service Level |
| `replica_zone` | String | Output only. Specifies the replica zone for regional volume. |
| `snapshot_directory` | bool | Optional. Snapshot_directory if enabled (true) the volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. |
| `unix_permissions` | String | Optional. Default unix style permission (e.g. 777) the mount point will be created with. Applicable for NFS protocol types only. |
| `snapshot_policy` | String | Optional. SnapshotPolicy for a volume. |
| `multiple_endpoints` | bool | Optional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints. Only the volume with large_capacity will be allowed to have multiple endpoints. |
| `name` | String | Identifier. Name of the volume |
| `storage_pool` | String | Required. StoragePool name of the volume |
| `large_capacity` | bool | Optional. Flag indicating if the volume will be a large capacity volume or a regular volume. |
| `hybrid_replication_parameters` | String | Optional. The Hybrid Replication parameters for the volume. |
| `kms_config` | String | Output only. Specifies the KMS config to be used for volume encryption. |
| `network` | String | Output only. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `snap_reserve` | f64 | Optional. Snap_reserve specifies percentage of volume storage reserved for snapshot storage. Default is 0 percent. |
| `state` | String | Output only. State of the volume |
| `protocols` | Vec<String> | Required. Protocols required for the volume |
| `description` | String | Optional. Description of the volume |
| `ldap_enabled` | bool | Output only. Flag indicating if the volume is NFS LDAP enabled or not. |
| `capacity_gib` | String | Required. Capacity in GIB of the volume |
| `create_time` | String | Output only. Create time of the volume |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `cold_tier_size_gib` | String | Output only. Size of the volume cold tier data rounded down to the nearest GiB. |
| `security_style` | String | Optional. Security Style of the Volume |
| `throughput_mibps` | f64 | Optional. Throughput of the volume (in MiB/s) |
| `tiering_policy` | String | Tiering policy for the volume. |
| `zone` | String | Output only. Specifies the active zone for regional volume. |
| `kerberos_enabled` | bool | Optional. Flag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p). |
| `active_directory` | String | Output only. Specifies the ActiveDirectory name of a SMB volume. |
| `restricted_actions` | Vec<String> | Optional. List of actions that are restricted on this volume. |
| `share_name` | String | Required. Share name of the volume |
| `used_gib` | String | Output only. Used capacity in GIB of the volume. This is computed periodically and it does not represent the realtime usage. |
| `cache_parameters` | String | Optional. Cache parameters for the volume. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create volume
volume = provider.netapp_api.Volume {
    parent = "value"  # Required. Value for parent.
}

# Access volume outputs
volume_id = volume.id
volume_restore_parameters = volume.restore_parameters
volume_service_level = volume.service_level
volume_backup_config = volume.backup_config
volume_encryption_type = volume.encryption_type
volume_has_replication = volume.has_replication
volume_mount_options = volume.mount_options
volume_export_policy = volume.export_policy
volume_smb_settings = volume.smb_settings
volume_state_details = volume.state_details
volume_psa_range = volume.psa_range
volume_hot_tier_size_used_gib = volume.hot_tier_size_used_gib
volume_replica_zone = volume.replica_zone
volume_snapshot_directory = volume.snapshot_directory
volume_unix_permissions = volume.unix_permissions
volume_snapshot_policy = volume.snapshot_policy
volume_multiple_endpoints = volume.multiple_endpoints
volume_name = volume.name
volume_storage_pool = volume.storage_pool
volume_large_capacity = volume.large_capacity
volume_hybrid_replication_parameters = volume.hybrid_replication_parameters
volume_kms_config = volume.kms_config
volume_network = volume.network
volume_snap_reserve = volume.snap_reserve
volume_state = volume.state
volume_protocols = volume.protocols
volume_description = volume.description
volume_ldap_enabled = volume.ldap_enabled
volume_capacity_gib = volume.capacity_gib
volume_create_time = volume.create_time
volume_labels = volume.labels
volume_cold_tier_size_gib = volume.cold_tier_size_gib
volume_security_style = volume.security_style
volume_throughput_mibps = volume.throughput_mibps
volume_tiering_policy = volume.tiering_policy
volume_zone = volume.zone
volume_kerberos_enabled = volume.kerberos_enabled
volume_active_directory = volume.active_directory
volume_restricted_actions = volume.restricted_actions
volume_share_name = volume.share_name
volume_used_gib = volume.used_gib
volume_cache_parameters = volume.cache_parameters
```

---


### Replication

Create a new replication for a volume.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hybrid_replication_type` | String |  | Output only. Type of the hybrid replication. |
| `hybrid_peering_details` | String |  | Output only. Hybrid peering details. |
| `create_time` | String |  | Output only. Replication create time. |
| `destination_volume` | String |  | Output only. Full name of destination volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `destination_volume_parameters` | String |  | Required. Input only. Destination volume parameters |
| `hybrid_replication_user_commands` | String |  | Output only. Copy pastable snapmirror commands to be executed on onprem cluster by the customer. |
| `state` | String |  | Output only. State of the replication. |
| `source_volume` | String |  | Output only. Full name of source volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `name` | String |  | Identifier. The resource name of the Replication. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}`. |
| `description` | String |  | A description about this replication relationship. |
| `replication_schedule` | String |  | Required. Indicates the schedule for replication. |
| `mirror_state` | String |  | Output only. Indicates the state of mirroring. |
| `transfer_stats` | String |  | Output only. Replication transfer statistics. |
| `cluster_location` | String |  | Optional. Location of the user cluster. |
| `state_details` | String |  | Output only. State details of the replication. |
| `healthy` | bool |  | Output only. Condition of the relationship. Can be one of the following: - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer. - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer. |
| `role` | String |  | Output only. Indicates whether this points to source or destination. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `parent` | String | ✅ | Required. The NetApp volume to create the replications of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hybrid_replication_type` | String | Output only. Type of the hybrid replication. |
| `hybrid_peering_details` | String | Output only. Hybrid peering details. |
| `create_time` | String | Output only. Replication create time. |
| `destination_volume` | String | Output only. Full name of destination volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `destination_volume_parameters` | String | Required. Input only. Destination volume parameters |
| `hybrid_replication_user_commands` | String | Output only. Copy pastable snapmirror commands to be executed on onprem cluster by the customer. |
| `state` | String | Output only. State of the replication. |
| `source_volume` | String | Output only. Full name of source volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `name` | String | Identifier. The resource name of the Replication. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}`. |
| `description` | String | A description about this replication relationship. |
| `replication_schedule` | String | Required. Indicates the schedule for replication. |
| `mirror_state` | String | Output only. Indicates the state of mirroring. |
| `transfer_stats` | String | Output only. Replication transfer statistics. |
| `cluster_location` | String | Optional. Location of the user cluster. |
| `state_details` | String | Output only. State details of the replication. |
| `healthy` | bool | Output only. Condition of the relationship. Can be one of the following: - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer. - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer. |
| `role` | String | Output only. Indicates whether this points to source or destination. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create replication
replication = provider.netapp_api.Replication {
    parent = "value"  # Required. The NetApp volume to create the replications of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}`
}

# Access replication outputs
replication_id = replication.id
replication_hybrid_replication_type = replication.hybrid_replication_type
replication_hybrid_peering_details = replication.hybrid_peering_details
replication_create_time = replication.create_time
replication_destination_volume = replication.destination_volume
replication_destination_volume_parameters = replication.destination_volume_parameters
replication_hybrid_replication_user_commands = replication.hybrid_replication_user_commands
replication_state = replication.state
replication_source_volume = replication.source_volume
replication_name = replication.name
replication_description = replication.description
replication_replication_schedule = replication.replication_schedule
replication_mirror_state = replication.mirror_state
replication_transfer_stats = replication.transfer_stats
replication_cluster_location = replication.cluster_location
replication_state_details = replication.state_details
replication_healthy = replication.healthy
replication_role = replication.role
replication_labels = replication.labels
```

---


### Backup_policie

Creates new backup policy

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the backup policy was created. |
| `state` | String |  | Output only. The backup policy state. |
| `assigned_volume_count` | i64 |  | Output only. The total number of volumes assigned by this backup policy. |
| `description` | String |  | Description of the backup policy. |
| `daily_backup_limit` | i64 |  | Number of daily backups to keep. Note that the minimum daily backup limit is 2. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `weekly_backup_limit` | i64 |  | Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `enabled` | bool |  | If enabled, make backups automatically according to the schedules. This will be applied to all volumes that have this policy attached and enforced on volume level. If not specified, default is true. |
| `name` | String |  | Identifier. The resource name of the backup policy. Format: `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`. |
| `monthly_backup_limit` | i64 |  | Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `parent` | String | ✅ | Required. The location to create the backup policies of, in the format `projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the backup policy was created. |
| `state` | String | Output only. The backup policy state. |
| `assigned_volume_count` | i64 | Output only. The total number of volumes assigned by this backup policy. |
| `description` | String | Description of the backup policy. |
| `daily_backup_limit` | i64 | Number of daily backups to keep. Note that the minimum daily backup limit is 2. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `weekly_backup_limit` | i64 | Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `enabled` | bool | If enabled, make backups automatically according to the schedules. This will be applied to all volumes that have this policy attached and enforced on volume level. If not specified, default is true. |
| `name` | String | Identifier. The resource name of the backup policy. Format: `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`. |
| `monthly_backup_limit` | i64 | Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_policie
backup_policie = provider.netapp_api.Backup_policie {
    parent = "value"  # Required. The location to create the backup policies of, in the format `projects/{project_id}/locations/{location}`
}

# Access backup_policie outputs
backup_policie_id = backup_policie.id
backup_policie_create_time = backup_policie.create_time
backup_policie_state = backup_policie.state
backup_policie_assigned_volume_count = backup_policie.assigned_volume_count
backup_policie_description = backup_policie.description
backup_policie_daily_backup_limit = backup_policie.daily_backup_limit
backup_policie_labels = backup_policie.labels
backup_policie_weekly_backup_limit = backup_policie.weekly_backup_limit
backup_policie_enabled = backup_policie.enabled
backup_policie_name = backup_policie.name
backup_policie_monthly_backup_limit = backup_policie.monthly_backup_limit
```

---


### Active_directorie

CreateActiveDirectory Creates the active directory specified in the request.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kdc_hostname` | String |  | Name of the active directory machine. This optional parameter is used only while creating kerberos volume |
| `password` | String |  | Required. Password of the Active Directory domain administrator. |
| `labels` | HashMap<String, String> |  | Labels for the active directory. |
| `organizational_unit` | String |  | The Organizational Unit (OU) within the Windows Active Directory the user belongs to. |
| `ldap_signing` | bool |  | Specifies whether or not the LDAP traffic needs to be signed. |
| `description` | String |  | Description of the active directory. |
| `net_bios_prefix` | String |  | Required. NetBIOSPrefix is used as a prefix for SMB server name. |
| `domain` | String |  | Required. Name of the Active Directory domain |
| `create_time` | String |  | Output only. Create time of the active directory. |
| `administrators` | Vec<String> |  | Optional. Users to be added to the Built-in Admininstrators group. |
| `kdc_ip` | String |  | KDC server IP address for the active directory machine. |
| `nfs_users_with_ldap` | bool |  | If enabled, will allow access to local users and LDAP users. If access is needed for only LDAP users, it has to be disabled. |
| `name` | String |  | Identifier. The resource name of the active directory. Format: `projects/{project_number}/locations/{location_id}/activeDirectories/{active_directory_id}`. |
| `security_operators` | Vec<String> |  | Optional. Domain users to be given the SeSecurityPrivilege. |
| `backup_operators` | Vec<String> |  | Optional. Users to be added to the Built-in Backup Operator active directory group. |
| `state` | String |  | Output only. The state of the AD. |
| `username` | String |  | Required. Username of the Active Directory domain administrator. |
| `encrypt_dc_connections` | bool |  | If enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted. |
| `dns` | String |  | Required. Comma separated list of DNS server IP addresses for the Active Directory domain. |
| `state_details` | String |  | Output only. The state details of the Active Directory. |
| `site` | String |  | The Active Directory site the service will limit Domain Controller discovery too. |
| `aes_encryption` | bool |  | If enabled, AES encryption will be enabled for SMB communication. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kdc_hostname` | String | Name of the active directory machine. This optional parameter is used only while creating kerberos volume |
| `password` | String | Required. Password of the Active Directory domain administrator. |
| `labels` | HashMap<String, String> | Labels for the active directory. |
| `organizational_unit` | String | The Organizational Unit (OU) within the Windows Active Directory the user belongs to. |
| `ldap_signing` | bool | Specifies whether or not the LDAP traffic needs to be signed. |
| `description` | String | Description of the active directory. |
| `net_bios_prefix` | String | Required. NetBIOSPrefix is used as a prefix for SMB server name. |
| `domain` | String | Required. Name of the Active Directory domain |
| `create_time` | String | Output only. Create time of the active directory. |
| `administrators` | Vec<String> | Optional. Users to be added to the Built-in Admininstrators group. |
| `kdc_ip` | String | KDC server IP address for the active directory machine. |
| `nfs_users_with_ldap` | bool | If enabled, will allow access to local users and LDAP users. If access is needed for only LDAP users, it has to be disabled. |
| `name` | String | Identifier. The resource name of the active directory. Format: `projects/{project_number}/locations/{location_id}/activeDirectories/{active_directory_id}`. |
| `security_operators` | Vec<String> | Optional. Domain users to be given the SeSecurityPrivilege. |
| `backup_operators` | Vec<String> | Optional. Users to be added to the Built-in Backup Operator active directory group. |
| `state` | String | Output only. The state of the AD. |
| `username` | String | Required. Username of the Active Directory domain administrator. |
| `encrypt_dc_connections` | bool | If enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted. |
| `dns` | String | Required. Comma separated list of DNS server IP addresses for the Active Directory domain. |
| `state_details` | String | Output only. The state details of the Active Directory. |
| `site` | String | The Active Directory site the service will limit Domain Controller discovery too. |
| `aes_encryption` | bool | If enabled, AES encryption will be enabled for SMB communication. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create active_directorie
active_directorie = provider.netapp_api.Active_directorie {
    parent = "value"  # Required. Value for parent.
}

# Access active_directorie outputs
active_directorie_id = active_directorie.id
active_directorie_kdc_hostname = active_directorie.kdc_hostname
active_directorie_password = active_directorie.password
active_directorie_labels = active_directorie.labels
active_directorie_organizational_unit = active_directorie.organizational_unit
active_directorie_ldap_signing = active_directorie.ldap_signing
active_directorie_description = active_directorie.description
active_directorie_net_bios_prefix = active_directorie.net_bios_prefix
active_directorie_domain = active_directorie.domain
active_directorie_create_time = active_directorie.create_time
active_directorie_administrators = active_directorie.administrators
active_directorie_kdc_ip = active_directorie.kdc_ip
active_directorie_nfs_users_with_ldap = active_directorie.nfs_users_with_ldap
active_directorie_name = active_directorie.name
active_directorie_security_operators = active_directorie.security_operators
active_directorie_backup_operators = active_directorie.backup_operators
active_directorie_state = active_directorie.state
active_directorie_username = active_directorie.username
active_directorie_encrypt_dc_connections = active_directorie.encrypt_dc_connections
active_directorie_dns = active_directorie.dns
active_directorie_state_details = active_directorie.state_details
active_directorie_site = active_directorie.site
active_directorie_aes_encryption = active_directorie.aes_encryption
```

---


### Backup_vault

Creates new backup vault

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_vault_type` | String |  | Optional. Type of backup vault to be created. Default is IN_REGION. |
| `source_backup_vault` | String |  | Output only. Name of the Backup vault created in source region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `state` | String |  | Output only. The backup vault state. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `backup_retention_policy` | String |  | Optional. Backup retention policy defining the retenton of backups. |
| `source_region` | String |  | Output only. Region in which the backup vault is created. Format: `projects/{project_id}/locations/{location}` |
| `name` | String |  | Identifier. The resource name of the backup vault. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`. |
| `destination_backup_vault` | String |  | Output only. Name of the Backup vault created in backup region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `backup_region` | String |  | Optional. Region where the backups are stored. Format: `projects/{project_id}/locations/{location}` |
| `description` | String |  | Description of the backup vault. |
| `create_time` | String |  | Output only. Create time of the backup vault. |
| `parent` | String | ✅ | Required. The location to create the backup vaults, in the format `projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_vault_type` | String | Optional. Type of backup vault to be created. Default is IN_REGION. |
| `source_backup_vault` | String | Output only. Name of the Backup vault created in source region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `state` | String | Output only. The backup vault state. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `backup_retention_policy` | String | Optional. Backup retention policy defining the retenton of backups. |
| `source_region` | String | Output only. Region in which the backup vault is created. Format: `projects/{project_id}/locations/{location}` |
| `name` | String | Identifier. The resource name of the backup vault. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`. |
| `destination_backup_vault` | String | Output only. Name of the Backup vault created in backup region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `backup_region` | String | Optional. Region where the backups are stored. Format: `projects/{project_id}/locations/{location}` |
| `description` | String | Description of the backup vault. |
| `create_time` | String | Output only. Create time of the backup vault. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_vault
backup_vault = provider.netapp_api.Backup_vault {
    parent = "value"  # Required. The location to create the backup vaults, in the format `projects/{project_id}/locations/{location}`
}

# Access backup_vault outputs
backup_vault_id = backup_vault.id
backup_vault_backup_vault_type = backup_vault.backup_vault_type
backup_vault_source_backup_vault = backup_vault.source_backup_vault
backup_vault_state = backup_vault.state
backup_vault_labels = backup_vault.labels
backup_vault_backup_retention_policy = backup_vault.backup_retention_policy
backup_vault_source_region = backup_vault.source_region
backup_vault_name = backup_vault.name
backup_vault_destination_backup_vault = backup_vault.destination_backup_vault
backup_vault_backup_region = backup_vault.backup_region
backup_vault_description = backup_vault.description
backup_vault_create_time = backup_vault.create_time
```

---


### Kms_config

Creates a new KMS config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Description of the KmsConfig. |
| `name` | String |  | Identifier. Name of the KmsConfig. |
| `state` | String |  | Output only. State of the KmsConfig. |
| `instructions` | String |  | Output only. Instructions to provide the access to the customer provided encryption key. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `service_account` | String |  | Output only. The Service account which will have access to the customer provided encryption key. |
| `crypto_key_name` | String |  | Required. Customer managed crypto key resource full name. Format: projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}. |
| `state_details` | String |  | Output only. State details of the KmsConfig. |
| `create_time` | String |  | Output only. Create time of the KmsConfig. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Description of the KmsConfig. |
| `name` | String | Identifier. Name of the KmsConfig. |
| `state` | String | Output only. State of the KmsConfig. |
| `instructions` | String | Output only. Instructions to provide the access to the customer provided encryption key. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `service_account` | String | Output only. The Service account which will have access to the customer provided encryption key. |
| `crypto_key_name` | String | Required. Customer managed crypto key resource full name. Format: projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}. |
| `state_details` | String | Output only. State details of the KmsConfig. |
| `create_time` | String | Output only. Create time of the KmsConfig. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create kms_config
kms_config = provider.netapp_api.Kms_config {
    parent = "value"  # Required. Value for parent.
}

# Access kms_config outputs
kms_config_id = kms_config.id
kms_config_description = kms_config.description
kms_config_name = kms_config.name
kms_config_state = kms_config.state
kms_config_instructions = kms_config.instructions
kms_config_labels = kms_config.labels
kms_config_service_account = kms_config.service_account
kms_config_crypto_key_name = kms_config.crypto_key_name
kms_config_state_details = kms_config.state_details
kms_config_create_time = kms_config.create_time
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.netapp_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Backup

Creates a backup from the volume specified in the request The backup can be created from the given snapshot if specified in the request. If no snapshot specified, there'll be a new snapshot taken to initiate the backup creation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzs` | bool |  | Output only. Reserved for future use |
| `source_snapshot` | String |  | If specified, backup will be created from the given snapshot. If not specified, there will be a new snapshot taken to initiate the backup creation. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}` |
| `backup_region` | String |  | Output only. Region in which backup is stored. Format: `projects/{project_id}/locations/{location}` |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use |
| `state` | String |  | Output only. The backup state. |
| `chain_storage_bytes` | String |  | Output only. Total size of all backups in a chain in bytes = baseline backup size + sum(incremental backup size) |
| `source_volume` | String |  | Volume full name of this backup belongs to. Format: `projects/{projects_id}/locations/{location}/volumes/{volume_id}` |
| `description` | String |  | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `backup_type` | String |  | Output only. Type of backup, manually created or created by a backup policy. |
| `volume_usage_bytes` | String |  | Output only. Size of the file system when the backup was created. When creating a new volume from the backup, the volume capacity will have to be at least as big. |
| `enforced_retention_end_time` | String |  | Output only. The time until which the backup is not deletable. |
| `name` | String |  | Identifier. The resource name of the backup. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `volume_region` | String |  | Output only. Region of the volume from which the backup was created. Format: `projects/{project_id}/locations/{location}` |
| `create_time` | String |  | Output only. The time when the backup was created. |
| `parent` | String | ✅ | Required. The NetApp backupVault to create the backups of, in the format `projects/*/locations/*/backupVaults/{backup_vault_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzs` | bool | Output only. Reserved for future use |
| `source_snapshot` | String | If specified, backup will be created from the given snapshot. If not specified, there will be a new snapshot taken to initiate the backup creation. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}` |
| `backup_region` | String | Output only. Region in which backup is stored. Format: `projects/{project_id}/locations/{location}` |
| `satisfies_pzi` | bool | Output only. Reserved for future use |
| `state` | String | Output only. The backup state. |
| `chain_storage_bytes` | String | Output only. Total size of all backups in a chain in bytes = baseline backup size + sum(incremental backup size) |
| `source_volume` | String | Volume full name of this backup belongs to. Format: `projects/{projects_id}/locations/{location}/volumes/{volume_id}` |
| `description` | String | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `backup_type` | String | Output only. Type of backup, manually created or created by a backup policy. |
| `volume_usage_bytes` | String | Output only. Size of the file system when the backup was created. When creating a new volume from the backup, the volume capacity will have to be at least as big. |
| `enforced_retention_end_time` | String | Output only. The time until which the backup is not deletable. |
| `name` | String | Identifier. The resource name of the backup. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `volume_region` | String | Output only. Region of the volume from which the backup was created. Format: `projects/{project_id}/locations/{location}` |
| `create_time` | String | Output only. The time when the backup was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.netapp_api.Backup {
    parent = "value"  # Required. The NetApp backupVault to create the backups of, in the format `projects/*/locations/*/backupVaults/{backup_vault_id}`
}

# Access backup outputs
backup_id = backup.id
backup_satisfies_pzs = backup.satisfies_pzs
backup_source_snapshot = backup.source_snapshot
backup_backup_region = backup.backup_region
backup_satisfies_pzi = backup.satisfies_pzi
backup_state = backup.state
backup_chain_storage_bytes = backup.chain_storage_bytes
backup_source_volume = backup.source_volume
backup_description = backup.description
backup_backup_type = backup.backup_type
backup_volume_usage_bytes = backup.volume_usage_bytes
backup_enforced_retention_end_time = backup.enforced_retention_end_time
backup_name = backup.name
backup_labels = backup.labels
backup_volume_region = backup.volume_region
backup_create_time = backup.create_time
```

---


### Storage_pool

Creates a new storage pool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hot_tier_size_gib` | String |  | Optional. Total hot tier capacity for the Storage Pool. It is applicable only to Flex service level. It should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set. |
| `volume_capacity_gib` | String |  | Output only. Allocated size of all volumes in GIB in the storage pool |
| `psa_range` | String |  | Optional. This field is not implemented. The values provided in this field are ignored. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `state` | String |  | Output only. State of the storage pool |
| `zone` | String |  | Optional. Specifies the active zone for regional storagePool. |
| `network` | String |  | Required. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `encryption_type` | String |  | Output only. Specifies the current pool encryption key source. |
| `replica_zone` | String |  | Optional. Specifies the replica zone for regional storagePool. |
| `custom_performance_enabled` | bool |  | Optional. True if using Independent Scaling of capacity and performance (Hyperdisk) By default set to false |
| `active_directory` | String |  | Optional. Specifies the Active Directory to be used for creating a SMB volume. |
| `ldap_enabled` | bool |  | Optional. Flag indicating if the pool is NFS LDAP enabled or not. |
| `service_level` | String |  | Required. Service level of the storage pool |
| `total_iops` | String |  | Optional. Custom Performance Total IOPS of the pool if not provided, it will be calculated based on the total_throughput_mibps |
| `capacity_gib` | String |  | Required. Capacity in GIB of the pool |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use |
| `hot_tier_size_used_gib` | String |  | Output only. Total hot tier data rounded down to the nearest GiB used by the storage pool. |
| `global_access_allowed` | bool |  | Deprecated. Used to allow SO pool to access AD or DNS server from other regions. |
| `kms_config` | String |  | Optional. Specifies the KMS config to be used for volume encryption. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use |
| `available_throughput_mibps` | f64 |  | Output only. Available throughput of the storage pool (in MiB/s). |
| `name` | String |  | Identifier. Name of the storage pool |
| `allow_auto_tiering` | bool |  | Optional. True if the storage pool supports Auto Tiering enabled volumes. Default is false. Auto-tiering can be enabled after storage pool creation but it can't be disabled once enabled. |
| `enable_hot_tier_auto_resize` | bool |  | Optional. Flag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true. The increment will kick in only if the new size after increment is still less than or equal to storage pool size. |
| `cold_tier_size_used_gib` | String |  | Output only. Total cold tier data rounded down to the nearest GiB used by the storage pool. |
| `create_time` | String |  | Output only. Create time of the storage pool |
| `total_throughput_mibps` | String |  | Optional. Custom Performance Total Throughput of the pool (in MiBps) |
| `qos_type` | String |  | Optional. QoS (Quality of Service) Type of the storage pool |
| `state_details` | String |  | Output only. State details of the storage pool |
| `volume_count` | i64 |  | Output only. Volume count of the storage pool |
| `description` | String |  | Optional. Description of the storage pool |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hot_tier_size_gib` | String | Optional. Total hot tier capacity for the Storage Pool. It is applicable only to Flex service level. It should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set. |
| `volume_capacity_gib` | String | Output only. Allocated size of all volumes in GIB in the storage pool |
| `psa_range` | String | Optional. This field is not implemented. The values provided in this field are ignored. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `state` | String | Output only. State of the storage pool |
| `zone` | String | Optional. Specifies the active zone for regional storagePool. |
| `network` | String | Required. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `encryption_type` | String | Output only. Specifies the current pool encryption key source. |
| `replica_zone` | String | Optional. Specifies the replica zone for regional storagePool. |
| `custom_performance_enabled` | bool | Optional. True if using Independent Scaling of capacity and performance (Hyperdisk) By default set to false |
| `active_directory` | String | Optional. Specifies the Active Directory to be used for creating a SMB volume. |
| `ldap_enabled` | bool | Optional. Flag indicating if the pool is NFS LDAP enabled or not. |
| `service_level` | String | Required. Service level of the storage pool |
| `total_iops` | String | Optional. Custom Performance Total IOPS of the pool if not provided, it will be calculated based on the total_throughput_mibps |
| `capacity_gib` | String | Required. Capacity in GIB of the pool |
| `satisfies_pzs` | bool | Output only. Reserved for future use |
| `hot_tier_size_used_gib` | String | Output only. Total hot tier data rounded down to the nearest GiB used by the storage pool. |
| `global_access_allowed` | bool | Deprecated. Used to allow SO pool to access AD or DNS server from other regions. |
| `kms_config` | String | Optional. Specifies the KMS config to be used for volume encryption. |
| `satisfies_pzi` | bool | Output only. Reserved for future use |
| `available_throughput_mibps` | f64 | Output only. Available throughput of the storage pool (in MiB/s). |
| `name` | String | Identifier. Name of the storage pool |
| `allow_auto_tiering` | bool | Optional. True if the storage pool supports Auto Tiering enabled volumes. Default is false. Auto-tiering can be enabled after storage pool creation but it can't be disabled once enabled. |
| `enable_hot_tier_auto_resize` | bool | Optional. Flag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true. The increment will kick in only if the new size after increment is still less than or equal to storage pool size. |
| `cold_tier_size_used_gib` | String | Output only. Total cold tier data rounded down to the nearest GiB used by the storage pool. |
| `create_time` | String | Output only. Create time of the storage pool |
| `total_throughput_mibps` | String | Optional. Custom Performance Total Throughput of the pool (in MiBps) |
| `qos_type` | String | Optional. QoS (Quality of Service) Type of the storage pool |
| `state_details` | String | Output only. State details of the storage pool |
| `volume_count` | i64 | Output only. Volume count of the storage pool |
| `description` | String | Optional. Description of the storage pool |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create storage_pool
storage_pool = provider.netapp_api.Storage_pool {
    parent = "value"  # Required. Value for parent.
}

# Access storage_pool outputs
storage_pool_id = storage_pool.id
storage_pool_hot_tier_size_gib = storage_pool.hot_tier_size_gib
storage_pool_volume_capacity_gib = storage_pool.volume_capacity_gib
storage_pool_psa_range = storage_pool.psa_range
storage_pool_labels = storage_pool.labels
storage_pool_state = storage_pool.state
storage_pool_zone = storage_pool.zone
storage_pool_network = storage_pool.network
storage_pool_encryption_type = storage_pool.encryption_type
storage_pool_replica_zone = storage_pool.replica_zone
storage_pool_custom_performance_enabled = storage_pool.custom_performance_enabled
storage_pool_active_directory = storage_pool.active_directory
storage_pool_ldap_enabled = storage_pool.ldap_enabled
storage_pool_service_level = storage_pool.service_level
storage_pool_total_iops = storage_pool.total_iops
storage_pool_capacity_gib = storage_pool.capacity_gib
storage_pool_satisfies_pzs = storage_pool.satisfies_pzs
storage_pool_hot_tier_size_used_gib = storage_pool.hot_tier_size_used_gib
storage_pool_global_access_allowed = storage_pool.global_access_allowed
storage_pool_kms_config = storage_pool.kms_config
storage_pool_satisfies_pzi = storage_pool.satisfies_pzi
storage_pool_available_throughput_mibps = storage_pool.available_throughput_mibps
storage_pool_name = storage_pool.name
storage_pool_allow_auto_tiering = storage_pool.allow_auto_tiering
storage_pool_enable_hot_tier_auto_resize = storage_pool.enable_hot_tier_auto_resize
storage_pool_cold_tier_size_used_gib = storage_pool.cold_tier_size_used_gib
storage_pool_create_time = storage_pool.create_time
storage_pool_total_throughput_mibps = storage_pool.total_throughput_mibps
storage_pool_qos_type = storage_pool.qos_type
storage_pool_state_details = storage_pool.state_details
storage_pool_volume_count = storage_pool.volume_count
storage_pool_description = storage_pool.description
```

---


### Snapshot

Create a new snapshot for a volume.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `state_details` | String |  | Output only. State details of the storage pool |
| `state` | String |  | Output only. The snapshot state. |
| `create_time` | String |  | Output only. The time when the snapshot was created. |
| `name` | String |  | Identifier. The resource name of the snapshot. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}`. |
| `used_bytes` | f64 |  | Output only. Current storage usage for the snapshot in bytes. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `parent` | String | ✅ | Required. The NetApp volume to create the snapshots of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `state_details` | String | Output only. State details of the storage pool |
| `state` | String | Output only. The snapshot state. |
| `create_time` | String | Output only. The time when the snapshot was created. |
| `name` | String | Identifier. The resource name of the snapshot. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}`. |
| `used_bytes` | f64 | Output only. Current storage usage for the snapshot in bytes. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create snapshot
snapshot = provider.netapp_api.Snapshot {
    parent = "value"  # Required. The NetApp volume to create the snapshots of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}`
}

# Access snapshot outputs
snapshot_id = snapshot.id
snapshot_description = snapshot.description
snapshot_state_details = snapshot.state_details
snapshot_state = snapshot.state
snapshot_create_time = snapshot.create_time
snapshot_name = snapshot.name
snapshot_used_bytes = snapshot.used_bytes
snapshot_labels = snapshot.labels
```

---


### Quota_rule

Creates a new quota rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time of the quota rule |
| `name` | String |  | Identifier. The resource name of the quota rule. Format: `projects/{project_number}/locations/{location_id}/volumes/volumes/{volume_id}/quotaRules/{quota_rule_id}`. |
| `description` | String |  | Optional. Description of the quota rule |
| `disk_limit_mib` | i64 |  | Required. The maximum allowed disk space in MiB. |
| `labels` | HashMap<String, String> |  | Optional. Labels of the quota rule |
| `target` | String |  | Optional. The quota rule applies to the specified user or group, identified by a Unix UID/GID, Windows SID, or null for default. |
| `type` | String |  | Required. The type of quota rule. |
| `state` | String |  | Output only. State of the quota rule |
| `state_details` | String |  | Output only. State details of the quota rule |
| `parent` | String | ✅ | Required. Parent value for CreateQuotaRuleRequest |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time of the quota rule |
| `name` | String | Identifier. The resource name of the quota rule. Format: `projects/{project_number}/locations/{location_id}/volumes/volumes/{volume_id}/quotaRules/{quota_rule_id}`. |
| `description` | String | Optional. Description of the quota rule |
| `disk_limit_mib` | i64 | Required. The maximum allowed disk space in MiB. |
| `labels` | HashMap<String, String> | Optional. Labels of the quota rule |
| `target` | String | Optional. The quota rule applies to the specified user or group, identified by a Unix UID/GID, Windows SID, or null for default. |
| `type` | String | Required. The type of quota rule. |
| `state` | String | Output only. State of the quota rule |
| `state_details` | String | Output only. State details of the quota rule |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create quota_rule
quota_rule = provider.netapp_api.Quota_rule {
    parent = "value"  # Required. Parent value for CreateQuotaRuleRequest
}

# Access quota_rule outputs
quota_rule_id = quota_rule.id
quota_rule_create_time = quota_rule.create_time
quota_rule_name = quota_rule.name
quota_rule_description = quota_rule.description
quota_rule_disk_limit_mib = quota_rule.disk_limit_mib
quota_rule_labels = quota_rule.labels
quota_rule_target = quota_rule.target
quota_rule_type = quota_rule.type
quota_rule_state = quota_rule.state
quota_rule_state_details = quota_rule.state_details
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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple operation resources
operation_0 = provider.netapp_api.Operation {
    name = "value-0"
}
operation_1 = provider.netapp_api.Operation {
    name = "value-1"
}
operation_2 = provider.netapp_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.netapp_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Netapp_api Documentation](https://cloud.google.com/netapp_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
