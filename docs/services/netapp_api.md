# Netapp_api Service



**Resources**: 24

---

## Overview

The netapp_api service provides access to 24 resource types:

- [Kms_config](#kms_config) [CRUD]
- [Backup_vault](#backup_vault) [CRUD]
- [Snapshot](#snapshot) [CRUD]
- [Storage_pool](#storage_pool) [CRUD]
- [Backup_policie](#backup_policie) [CRUD]
- [Quota_rule](#quota_rule) [CRUD]
- [Active_directorie](#active_directorie) [CRUD]
- [Backup](#backup) [CRUD]
- [Location](#location) [R]
- [Volume](#volume) [CRUD]
- [Operation](#operation) [CRD]
- [Replication](#replication) [CRUD]
- [Operation](#operation) [CRD]
- [Quota_rule](#quota_rule) [CRUD]
- [Backup_policie](#backup_policie) [CRUD]
- [Location](#location) [R]
- [Replication](#replication) [CRUD]
- [Volume](#volume) [CRUD]
- [Kms_config](#kms_config) [CRUD]
- [Active_directorie](#active_directorie) [CRUD]
- [Backup_vault](#backup_vault) [CRUD]
- [Storage_pool](#storage_pool) [CRUD]
- [Backup](#backup) [CRUD]
- [Snapshot](#snapshot) [CRUD]

---

## Resources


### Kms_config

Creates a new KMS config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time of the KmsConfig. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `service_account` | String |  | Output only. The Service account which will have access to the customer provided encryption key. |
| `crypto_key_name` | String |  | Required. Customer managed crypto key resource full name. Format: projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}. |
| `description` | String |  | Description of the KmsConfig. |
| `state_details` | String |  | Output only. State details of the KmsConfig. |
| `instructions` | String |  | Output only. Instructions to provide the access to the customer provided encryption key. |
| `state` | String |  | Output only. State of the KmsConfig. |
| `name` | String |  | Identifier. Name of the KmsConfig. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time of the KmsConfig. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `service_account` | String | Output only. The Service account which will have access to the customer provided encryption key. |
| `crypto_key_name` | String | Required. Customer managed crypto key resource full name. Format: projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}. |
| `description` | String | Description of the KmsConfig. |
| `state_details` | String | Output only. State details of the KmsConfig. |
| `instructions` | String | Output only. Instructions to provide the access to the customer provided encryption key. |
| `state` | String | Output only. State of the KmsConfig. |
| `name` | String | Identifier. Name of the KmsConfig. |


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
kms_config_create_time = kms_config.create_time
kms_config_labels = kms_config.labels
kms_config_service_account = kms_config.service_account
kms_config_crypto_key_name = kms_config.crypto_key_name
kms_config_description = kms_config.description
kms_config_state_details = kms_config.state_details
kms_config_instructions = kms_config.instructions
kms_config_state = kms_config.state
kms_config_name = kms_config.name
```

---


### Backup_vault

Creates new backup vault

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time of the backup vault. |
| `backup_vault_type` | String |  | Optional. Type of backup vault to be created. Default is IN_REGION. |
| `description` | String |  | Description of the backup vault. |
| `backup_retention_policy` | String |  | Optional. Backup retention policy defining the retenton of backups. |
| `name` | String |  | Identifier. The resource name of the backup vault. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`. |
| `source_backup_vault` | String |  | Output only. Name of the Backup vault created in source region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `state` | String |  | Output only. The backup vault state. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `source_region` | String |  | Output only. Region in which the backup vault is created. Format: `projects/{project_id}/locations/{location}` |
| `destination_backup_vault` | String |  | Output only. Name of the Backup vault created in backup region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `backup_region` | String |  | Optional. Region where the backups are stored. Format: `projects/{project_id}/locations/{location}` |
| `parent` | String | ✅ | Required. The location to create the backup vaults, in the format `projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time of the backup vault. |
| `backup_vault_type` | String | Optional. Type of backup vault to be created. Default is IN_REGION. |
| `description` | String | Description of the backup vault. |
| `backup_retention_policy` | String | Optional. Backup retention policy defining the retenton of backups. |
| `name` | String | Identifier. The resource name of the backup vault. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`. |
| `source_backup_vault` | String | Output only. Name of the Backup vault created in source region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `state` | String | Output only. The backup vault state. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `source_region` | String | Output only. Region in which the backup vault is created. Format: `projects/{project_id}/locations/{location}` |
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
backup_vault_create_time = backup_vault.create_time
backup_vault_backup_vault_type = backup_vault.backup_vault_type
backup_vault_description = backup_vault.description
backup_vault_backup_retention_policy = backup_vault.backup_retention_policy
backup_vault_name = backup_vault.name
backup_vault_source_backup_vault = backup_vault.source_backup_vault
backup_vault_state = backup_vault.state
backup_vault_labels = backup_vault.labels
backup_vault_source_region = backup_vault.source_region
backup_vault_destination_backup_vault = backup_vault.destination_backup_vault
backup_vault_backup_region = backup_vault.backup_region
```

---


### Snapshot

Create a new snapshot for a volume.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the snapshot was created. |
| `state` | String |  | Output only. The snapshot state. |
| `name` | String |  | Identifier. The resource name of the snapshot. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}`. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `state_details` | String |  | Output only. State details of the storage pool |
| `used_bytes` | f64 |  | Output only. Current storage usage for the snapshot in bytes. |
| `description` | String |  | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `parent` | String | ✅ | Required. The NetApp volume to create the snapshots of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the snapshot was created. |
| `state` | String | Output only. The snapshot state. |
| `name` | String | Identifier. The resource name of the snapshot. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}`. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `state_details` | String | Output only. State details of the storage pool |
| `used_bytes` | f64 | Output only. Current storage usage for the snapshot in bytes. |
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
snapshot = provider.netapp_api.Snapshot {
    parent = "value"  # Required. The NetApp volume to create the snapshots of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}`
}

# Access snapshot outputs
snapshot_id = snapshot.id
snapshot_create_time = snapshot.create_time
snapshot_state = snapshot.state
snapshot_name = snapshot.name
snapshot_labels = snapshot.labels
snapshot_state_details = snapshot.state_details
snapshot_used_bytes = snapshot.used_bytes
snapshot_description = snapshot.description
```

---


### Storage_pool

Creates a new storage pool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_type` | String |  | Output only. Specifies the current pool encryption key source. |
| `name` | String |  | Identifier. Name of the storage pool |
| `capacity_gib` | String |  | Required. Capacity in GIB of the pool |
| `total_iops` | String |  | Optional. Custom Performance Total IOPS of the pool if not provided, it will be calculated based on the total_throughput_mibps |
| `description` | String |  | Optional. Description of the storage pool |
| `service_level` | String |  | Required. Service level of the storage pool |
| `hot_tier_size_used_gib` | String |  | Output only. Total hot tier data rounded down to the nearest GiB used by the storage pool. |
| `state_details` | String |  | Output only. State details of the storage pool |
| `volume_capacity_gib` | String |  | Output only. Allocated size of all volumes in GIB in the storage pool |
| `available_throughput_mibps` | f64 |  | Output only. Available throughput of the storage pool (in MiB/s). |
| `allow_auto_tiering` | bool |  | Optional. True if the storage pool supports Auto Tiering enabled volumes. Default is false. Auto-tiering can be enabled after storage pool creation but it can't be disabled once enabled. |
| `cold_tier_size_used_gib` | String |  | Output only. Total cold tier data rounded down to the nearest GiB used by the storage pool. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `network` | String |  | Required. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `active_directory` | String |  | Optional. Specifies the Active Directory to be used for creating a SMB volume. |
| `kms_config` | String |  | Optional. Specifies the KMS config to be used for volume encryption. |
| `total_throughput_mibps` | String |  | Optional. Custom Performance Total Throughput of the pool (in MiBps) |
| `create_time` | String |  | Output only. Create time of the storage pool |
| `zone` | String |  | Optional. Specifies the active zone for regional storagePool. |
| `ldap_enabled` | bool |  | Optional. Flag indicating if the pool is NFS LDAP enabled or not. |
| `qos_type` | String |  | Optional. QoS (Quality of Service) Type of the storage pool |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use |
| `hot_tier_size_gib` | String |  | Optional. Total hot tier capacity for the Storage Pool. It is applicable only to Flex service level. It should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set. |
| `state` | String |  | Output only. State of the storage pool |
| `replica_zone` | String |  | Optional. Specifies the replica zone for regional storagePool. |
| `global_access_allowed` | bool |  | Deprecated. Used to allow SO pool to access AD or DNS server from other regions. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use |
| `volume_count` | i64 |  | Output only. Volume count of the storage pool |
| `custom_performance_enabled` | bool |  | Optional. True if using Independent Scaling of capacity and performance (Hyperdisk) By default set to false |
| `psa_range` | String |  | Optional. This field is not implemented. The values provided in this field are ignored. |
| `enable_hot_tier_auto_resize` | bool |  | Optional. Flag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true. The increment will kick in only if the new size after increment is still less than or equal to storage pool size. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_type` | String | Output only. Specifies the current pool encryption key source. |
| `name` | String | Identifier. Name of the storage pool |
| `capacity_gib` | String | Required. Capacity in GIB of the pool |
| `total_iops` | String | Optional. Custom Performance Total IOPS of the pool if not provided, it will be calculated based on the total_throughput_mibps |
| `description` | String | Optional. Description of the storage pool |
| `service_level` | String | Required. Service level of the storage pool |
| `hot_tier_size_used_gib` | String | Output only. Total hot tier data rounded down to the nearest GiB used by the storage pool. |
| `state_details` | String | Output only. State details of the storage pool |
| `volume_capacity_gib` | String | Output only. Allocated size of all volumes in GIB in the storage pool |
| `available_throughput_mibps` | f64 | Output only. Available throughput of the storage pool (in MiB/s). |
| `allow_auto_tiering` | bool | Optional. True if the storage pool supports Auto Tiering enabled volumes. Default is false. Auto-tiering can be enabled after storage pool creation but it can't be disabled once enabled. |
| `cold_tier_size_used_gib` | String | Output only. Total cold tier data rounded down to the nearest GiB used by the storage pool. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `network` | String | Required. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `active_directory` | String | Optional. Specifies the Active Directory to be used for creating a SMB volume. |
| `kms_config` | String | Optional. Specifies the KMS config to be used for volume encryption. |
| `total_throughput_mibps` | String | Optional. Custom Performance Total Throughput of the pool (in MiBps) |
| `create_time` | String | Output only. Create time of the storage pool |
| `zone` | String | Optional. Specifies the active zone for regional storagePool. |
| `ldap_enabled` | bool | Optional. Flag indicating if the pool is NFS LDAP enabled or not. |
| `qos_type` | String | Optional. QoS (Quality of Service) Type of the storage pool |
| `satisfies_pzs` | bool | Output only. Reserved for future use |
| `hot_tier_size_gib` | String | Optional. Total hot tier capacity for the Storage Pool. It is applicable only to Flex service level. It should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set. |
| `state` | String | Output only. State of the storage pool |
| `replica_zone` | String | Optional. Specifies the replica zone for regional storagePool. |
| `global_access_allowed` | bool | Deprecated. Used to allow SO pool to access AD or DNS server from other regions. |
| `satisfies_pzi` | bool | Output only. Reserved for future use |
| `volume_count` | i64 | Output only. Volume count of the storage pool |
| `custom_performance_enabled` | bool | Optional. True if using Independent Scaling of capacity and performance (Hyperdisk) By default set to false |
| `psa_range` | String | Optional. This field is not implemented. The values provided in this field are ignored. |
| `enable_hot_tier_auto_resize` | bool | Optional. Flag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true. The increment will kick in only if the new size after increment is still less than or equal to storage pool size. |


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
storage_pool_encryption_type = storage_pool.encryption_type
storage_pool_name = storage_pool.name
storage_pool_capacity_gib = storage_pool.capacity_gib
storage_pool_total_iops = storage_pool.total_iops
storage_pool_description = storage_pool.description
storage_pool_service_level = storage_pool.service_level
storage_pool_hot_tier_size_used_gib = storage_pool.hot_tier_size_used_gib
storage_pool_state_details = storage_pool.state_details
storage_pool_volume_capacity_gib = storage_pool.volume_capacity_gib
storage_pool_available_throughput_mibps = storage_pool.available_throughput_mibps
storage_pool_allow_auto_tiering = storage_pool.allow_auto_tiering
storage_pool_cold_tier_size_used_gib = storage_pool.cold_tier_size_used_gib
storage_pool_labels = storage_pool.labels
storage_pool_network = storage_pool.network
storage_pool_active_directory = storage_pool.active_directory
storage_pool_kms_config = storage_pool.kms_config
storage_pool_total_throughput_mibps = storage_pool.total_throughput_mibps
storage_pool_create_time = storage_pool.create_time
storage_pool_zone = storage_pool.zone
storage_pool_ldap_enabled = storage_pool.ldap_enabled
storage_pool_qos_type = storage_pool.qos_type
storage_pool_satisfies_pzs = storage_pool.satisfies_pzs
storage_pool_hot_tier_size_gib = storage_pool.hot_tier_size_gib
storage_pool_state = storage_pool.state
storage_pool_replica_zone = storage_pool.replica_zone
storage_pool_global_access_allowed = storage_pool.global_access_allowed
storage_pool_satisfies_pzi = storage_pool.satisfies_pzi
storage_pool_volume_count = storage_pool.volume_count
storage_pool_custom_performance_enabled = storage_pool.custom_performance_enabled
storage_pool_psa_range = storage_pool.psa_range
storage_pool_enable_hot_tier_auto_resize = storage_pool.enable_hot_tier_auto_resize
```

---


### Backup_policie

Creates new backup policy

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the backup policy was created. |
| `enabled` | bool |  | If enabled, make backups automatically according to the schedules. This will be applied to all volumes that have this policy attached and enforced on volume level. If not specified, default is true. |
| `monthly_backup_limit` | i64 |  | Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `daily_backup_limit` | i64 |  | Number of daily backups to keep. Note that the minimum daily backup limit is 2. |
| `assigned_volume_count` | i64 |  | Output only. The total number of volumes assigned by this backup policy. |
| `description` | String |  | Description of the backup policy. |
| `weekly_backup_limit` | i64 |  | Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `name` | String |  | Identifier. The resource name of the backup policy. Format: `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `state` | String |  | Output only. The backup policy state. |
| `parent` | String | ✅ | Required. The location to create the backup policies of, in the format `projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the backup policy was created. |
| `enabled` | bool | If enabled, make backups automatically according to the schedules. This will be applied to all volumes that have this policy attached and enforced on volume level. If not specified, default is true. |
| `monthly_backup_limit` | i64 | Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `daily_backup_limit` | i64 | Number of daily backups to keep. Note that the minimum daily backup limit is 2. |
| `assigned_volume_count` | i64 | Output only. The total number of volumes assigned by this backup policy. |
| `description` | String | Description of the backup policy. |
| `weekly_backup_limit` | i64 | Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `name` | String | Identifier. The resource name of the backup policy. Format: `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
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
backup_policie_create_time = backup_policie.create_time
backup_policie_enabled = backup_policie.enabled
backup_policie_monthly_backup_limit = backup_policie.monthly_backup_limit
backup_policie_daily_backup_limit = backup_policie.daily_backup_limit
backup_policie_assigned_volume_count = backup_policie.assigned_volume_count
backup_policie_description = backup_policie.description
backup_policie_weekly_backup_limit = backup_policie.weekly_backup_limit
backup_policie_name = backup_policie.name
backup_policie_labels = backup_policie.labels
backup_policie_state = backup_policie.state
```

---


### Quota_rule

Creates a new quota rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Required. The type of quota rule. |
| `labels` | HashMap<String, String> |  | Optional. Labels of the quota rule |
| `state` | String |  | Output only. State of the quota rule |
| `disk_limit_mib` | i64 |  | Required. The maximum allowed disk space in MiB. |
| `name` | String |  | Identifier. The resource name of the quota rule. Format: `projects/{project_number}/locations/{location_id}/volumes/volumes/{volume_id}/quotaRules/{quota_rule_id}`. |
| `description` | String |  | Optional. Description of the quota rule |
| `target` | String |  | Optional. The quota rule applies to the specified user or group, identified by a Unix UID/GID, Windows SID, or null for default. |
| `create_time` | String |  | Output only. Create time of the quota rule |
| `state_details` | String |  | Output only. State details of the quota rule |
| `parent` | String | ✅ | Required. Parent value for CreateQuotaRuleRequest |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Required. The type of quota rule. |
| `labels` | HashMap<String, String> | Optional. Labels of the quota rule |
| `state` | String | Output only. State of the quota rule |
| `disk_limit_mib` | i64 | Required. The maximum allowed disk space in MiB. |
| `name` | String | Identifier. The resource name of the quota rule. Format: `projects/{project_number}/locations/{location_id}/volumes/volumes/{volume_id}/quotaRules/{quota_rule_id}`. |
| `description` | String | Optional. Description of the quota rule |
| `target` | String | Optional. The quota rule applies to the specified user or group, identified by a Unix UID/GID, Windows SID, or null for default. |
| `create_time` | String | Output only. Create time of the quota rule |
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
quota_rule_type = quota_rule.type
quota_rule_labels = quota_rule.labels
quota_rule_state = quota_rule.state
quota_rule_disk_limit_mib = quota_rule.disk_limit_mib
quota_rule_name = quota_rule.name
quota_rule_description = quota_rule.description
quota_rule_target = quota_rule.target
quota_rule_create_time = quota_rule.create_time
quota_rule_state_details = quota_rule.state_details
```

---


### Active_directorie

CreateActiveDirectory Creates the active directory specified in the request.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_operators` | Vec<String> |  | Optional. Users to be added to the Built-in Backup Operator active directory group. |
| `labels` | HashMap<String, String> |  | Labels for the active directory. |
| `domain` | String |  | Required. Name of the Active Directory domain |
| `nfs_users_with_ldap` | bool |  | If enabled, will allow access to local users and LDAP users. If access is needed for only LDAP users, it has to be disabled. |
| `description` | String |  | Description of the active directory. |
| `kdc_ip` | String |  | KDC server IP address for the active directory machine. |
| `kdc_hostname` | String |  | Name of the active directory machine. This optional parameter is used only while creating kerberos volume |
| `organizational_unit` | String |  | The Organizational Unit (OU) within the Windows Active Directory the user belongs to. |
| `username` | String |  | Required. Username of the Active Directory domain administrator. |
| `ldap_signing` | bool |  | Specifies whether or not the LDAP traffic needs to be signed. |
| `administrators` | Vec<String> |  | Optional. Users to be added to the Built-in Admininstrators group. |
| `aes_encryption` | bool |  | If enabled, AES encryption will be enabled for SMB communication. |
| `name` | String |  | Identifier. The resource name of the active directory. Format: `projects/{project_number}/locations/{location_id}/activeDirectories/{active_directory_id}`. |
| `state` | String |  | Output only. The state of the AD. |
| `encrypt_dc_connections` | bool |  | If enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted. |
| `state_details` | String |  | Output only. The state details of the Active Directory. |
| `create_time` | String |  | Output only. Create time of the active directory. |
| `dns` | String |  | Required. Comma separated list of DNS server IP addresses for the Active Directory domain. |
| `net_bios_prefix` | String |  | Required. NetBIOSPrefix is used as a prefix for SMB server name. |
| `security_operators` | Vec<String> |  | Optional. Domain users to be given the SeSecurityPrivilege. |
| `password` | String |  | Required. Password of the Active Directory domain administrator. |
| `site` | String |  | The Active Directory site the service will limit Domain Controller discovery too. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_operators` | Vec<String> | Optional. Users to be added to the Built-in Backup Operator active directory group. |
| `labels` | HashMap<String, String> | Labels for the active directory. |
| `domain` | String | Required. Name of the Active Directory domain |
| `nfs_users_with_ldap` | bool | If enabled, will allow access to local users and LDAP users. If access is needed for only LDAP users, it has to be disabled. |
| `description` | String | Description of the active directory. |
| `kdc_ip` | String | KDC server IP address for the active directory machine. |
| `kdc_hostname` | String | Name of the active directory machine. This optional parameter is used only while creating kerberos volume |
| `organizational_unit` | String | The Organizational Unit (OU) within the Windows Active Directory the user belongs to. |
| `username` | String | Required. Username of the Active Directory domain administrator. |
| `ldap_signing` | bool | Specifies whether or not the LDAP traffic needs to be signed. |
| `administrators` | Vec<String> | Optional. Users to be added to the Built-in Admininstrators group. |
| `aes_encryption` | bool | If enabled, AES encryption will be enabled for SMB communication. |
| `name` | String | Identifier. The resource name of the active directory. Format: `projects/{project_number}/locations/{location_id}/activeDirectories/{active_directory_id}`. |
| `state` | String | Output only. The state of the AD. |
| `encrypt_dc_connections` | bool | If enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted. |
| `state_details` | String | Output only. The state details of the Active Directory. |
| `create_time` | String | Output only. Create time of the active directory. |
| `dns` | String | Required. Comma separated list of DNS server IP addresses for the Active Directory domain. |
| `net_bios_prefix` | String | Required. NetBIOSPrefix is used as a prefix for SMB server name. |
| `security_operators` | Vec<String> | Optional. Domain users to be given the SeSecurityPrivilege. |
| `password` | String | Required. Password of the Active Directory domain administrator. |
| `site` | String | The Active Directory site the service will limit Domain Controller discovery too. |


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
active_directorie_backup_operators = active_directorie.backup_operators
active_directorie_labels = active_directorie.labels
active_directorie_domain = active_directorie.domain
active_directorie_nfs_users_with_ldap = active_directorie.nfs_users_with_ldap
active_directorie_description = active_directorie.description
active_directorie_kdc_ip = active_directorie.kdc_ip
active_directorie_kdc_hostname = active_directorie.kdc_hostname
active_directorie_organizational_unit = active_directorie.organizational_unit
active_directorie_username = active_directorie.username
active_directorie_ldap_signing = active_directorie.ldap_signing
active_directorie_administrators = active_directorie.administrators
active_directorie_aes_encryption = active_directorie.aes_encryption
active_directorie_name = active_directorie.name
active_directorie_state = active_directorie.state
active_directorie_encrypt_dc_connections = active_directorie.encrypt_dc_connections
active_directorie_state_details = active_directorie.state_details
active_directorie_create_time = active_directorie.create_time
active_directorie_dns = active_directorie.dns
active_directorie_net_bios_prefix = active_directorie.net_bios_prefix
active_directorie_security_operators = active_directorie.security_operators
active_directorie_password = active_directorie.password
active_directorie_site = active_directorie.site
```

---


### Backup

Creates a backup from the volume specified in the request The backup can be created from the given snapshot if specified in the request. If no snapshot specified, there'll be a new snapshot taken to initiate the backup creation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `volume_usage_bytes` | String |  | Output only. Size of the file system when the backup was created. When creating a new volume from the backup, the volume capacity will have to be at least as big. |
| `name` | String |  | Identifier. The resource name of the backup. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`. |
| `source_volume` | String |  | Volume full name of this backup belongs to. Format: `projects/{projects_id}/locations/{location}/volumes/{volume_id}` |
| `state` | String |  | Output only. The backup state. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use |
| `description` | String |  | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `chain_storage_bytes` | String |  | Output only. Total size of all backups in a chain in bytes = baseline backup size + sum(incremental backup size) |
| `source_snapshot` | String |  | If specified, backup will be created from the given snapshot. If not specified, there will be a new snapshot taken to initiate the backup creation. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}` |
| `volume_region` | String |  | Output only. Region of the volume from which the backup was created. Format: `projects/{project_id}/locations/{location}` |
| `create_time` | String |  | Output only. The time when the backup was created. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `backup_region` | String |  | Output only. Region in which backup is stored. Format: `projects/{project_id}/locations/{location}` |
| `backup_type` | String |  | Output only. Type of backup, manually created or created by a backup policy. |
| `enforced_retention_end_time` | String |  | Output only. The time until which the backup is not deletable. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use |
| `parent` | String | ✅ | Required. The NetApp backupVault to create the backups of, in the format `projects/*/locations/*/backupVaults/{backup_vault_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `volume_usage_bytes` | String | Output only. Size of the file system when the backup was created. When creating a new volume from the backup, the volume capacity will have to be at least as big. |
| `name` | String | Identifier. The resource name of the backup. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`. |
| `source_volume` | String | Volume full name of this backup belongs to. Format: `projects/{projects_id}/locations/{location}/volumes/{volume_id}` |
| `state` | String | Output only. The backup state. |
| `satisfies_pzi` | bool | Output only. Reserved for future use |
| `description` | String | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `chain_storage_bytes` | String | Output only. Total size of all backups in a chain in bytes = baseline backup size + sum(incremental backup size) |
| `source_snapshot` | String | If specified, backup will be created from the given snapshot. If not specified, there will be a new snapshot taken to initiate the backup creation. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}` |
| `volume_region` | String | Output only. Region of the volume from which the backup was created. Format: `projects/{project_id}/locations/{location}` |
| `create_time` | String | Output only. The time when the backup was created. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `backup_region` | String | Output only. Region in which backup is stored. Format: `projects/{project_id}/locations/{location}` |
| `backup_type` | String | Output only. Type of backup, manually created or created by a backup policy. |
| `enforced_retention_end_time` | String | Output only. The time until which the backup is not deletable. |
| `satisfies_pzs` | bool | Output only. Reserved for future use |


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
backup_volume_usage_bytes = backup.volume_usage_bytes
backup_name = backup.name
backup_source_volume = backup.source_volume
backup_state = backup.state
backup_satisfies_pzi = backup.satisfies_pzi
backup_description = backup.description
backup_chain_storage_bytes = backup.chain_storage_bytes
backup_source_snapshot = backup.source_snapshot
backup_volume_region = backup.volume_region
backup_create_time = backup.create_time
backup_labels = backup.labels
backup_backup_region = backup.backup_region
backup_backup_type = backup.backup_type
backup_enforced_retention_end_time = backup.enforced_retention_end_time
backup_satisfies_pzs = backup.satisfies_pzs
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_location_id = location.location_id
location_metadata = location.metadata
location_labels = location.labels
location_name = location.name
```

---


### Volume

Creates a new Volume in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `used_gib` | String |  | Output only. Used capacity in GIB of the volume. This is computed periodically and it does not represent the realtime usage. |
| `throughput_mibps` | f64 |  | Optional. Throughput of the volume (in MiB/s) |
| `kerberos_enabled` | bool |  | Optional. Flag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p). |
| `protocols` | Vec<String> |  | Required. Protocols required for the volume |
| `psa_range` | String |  | Output only. This field is not implemented. The values provided in this field are ignored. |
| `restricted_actions` | Vec<String> |  | Optional. List of actions that are restricted on this volume. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `state` | String |  | Output only. State of the volume |
| `unix_permissions` | String |  | Optional. Default unix style permission (e.g. 777) the mount point will be created with. Applicable for NFS protocol types only. |
| `state_details` | String |  | Output only. State details of the volume |
| `large_capacity` | bool |  | Optional. Flag indicating if the volume will be a large capacity volume or a regular volume. |
| `cold_tier_size_gib` | String |  | Output only. Size of the volume cold tier data rounded down to the nearest GiB. |
| `smb_settings` | Vec<String> |  | Optional. SMB share settings for the volume. |
| `service_level` | String |  | Output only. Service level of the volume |
| `description` | String |  | Optional. Description of the volume |
| `mount_options` | Vec<String> |  | Output only. Mount options of this volume |
| `snapshot_policy` | String |  | Optional. SnapshotPolicy for a volume. |
| `encryption_type` | String |  | Output only. Specified the current volume encryption key source. |
| `active_directory` | String |  | Output only. Specifies the ActiveDirectory name of a SMB volume. |
| `kms_config` | String |  | Output only. Specifies the KMS config to be used for volume encryption. |
| `backup_config` | String |  | BackupConfig of the volume. |
| `cache_parameters` | String |  | Optional. Cache parameters for the volume. |
| `ldap_enabled` | bool |  | Output only. Flag indicating if the volume is NFS LDAP enabled or not. |
| `replica_zone` | String |  | Output only. Specifies the replica zone for regional volume. |
| `zone` | String |  | Output only. Specifies the active zone for regional volume. |
| `share_name` | String |  | Required. Share name of the volume |
| `security_style` | String |  | Optional. Security Style of the Volume |
| `snap_reserve` | f64 |  | Optional. Snap_reserve specifies percentage of volume storage reserved for snapshot storage. Default is 0 percent. |
| `storage_pool` | String |  | Required. StoragePool name of the volume |
| `network` | String |  | Output only. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `tiering_policy` | String |  | Tiering policy for the volume. |
| `hot_tier_size_used_gib` | String |  | Output only. Total hot tier data rounded down to the nearest GiB used by the Volume. This field is only used for flex Service Level |
| `capacity_gib` | String |  | Required. Capacity in GIB of the volume |
| `create_time` | String |  | Output only. Create time of the volume |
| `export_policy` | String |  | Optional. Export policy of the volume |
| `multiple_endpoints` | bool |  | Optional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints. Only the volume with large_capacity will be allowed to have multiple endpoints. |
| `name` | String |  | Identifier. Name of the volume |
| `restore_parameters` | String |  | Optional. Specifies the source of the volume to be created from. |
| `hybrid_replication_parameters` | String |  | Optional. The Hybrid Replication parameters for the volume. |
| `has_replication` | bool |  | Output only. Indicates whether the volume is part of a replication relationship. |
| `snapshot_directory` | bool |  | Optional. Snapshot_directory if enabled (true) the volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `used_gib` | String | Output only. Used capacity in GIB of the volume. This is computed periodically and it does not represent the realtime usage. |
| `throughput_mibps` | f64 | Optional. Throughput of the volume (in MiB/s) |
| `kerberos_enabled` | bool | Optional. Flag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p). |
| `protocols` | Vec<String> | Required. Protocols required for the volume |
| `psa_range` | String | Output only. This field is not implemented. The values provided in this field are ignored. |
| `restricted_actions` | Vec<String> | Optional. List of actions that are restricted on this volume. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `state` | String | Output only. State of the volume |
| `unix_permissions` | String | Optional. Default unix style permission (e.g. 777) the mount point will be created with. Applicable for NFS protocol types only. |
| `state_details` | String | Output only. State details of the volume |
| `large_capacity` | bool | Optional. Flag indicating if the volume will be a large capacity volume or a regular volume. |
| `cold_tier_size_gib` | String | Output only. Size of the volume cold tier data rounded down to the nearest GiB. |
| `smb_settings` | Vec<String> | Optional. SMB share settings for the volume. |
| `service_level` | String | Output only. Service level of the volume |
| `description` | String | Optional. Description of the volume |
| `mount_options` | Vec<String> | Output only. Mount options of this volume |
| `snapshot_policy` | String | Optional. SnapshotPolicy for a volume. |
| `encryption_type` | String | Output only. Specified the current volume encryption key source. |
| `active_directory` | String | Output only. Specifies the ActiveDirectory name of a SMB volume. |
| `kms_config` | String | Output only. Specifies the KMS config to be used for volume encryption. |
| `backup_config` | String | BackupConfig of the volume. |
| `cache_parameters` | String | Optional. Cache parameters for the volume. |
| `ldap_enabled` | bool | Output only. Flag indicating if the volume is NFS LDAP enabled or not. |
| `replica_zone` | String | Output only. Specifies the replica zone for regional volume. |
| `zone` | String | Output only. Specifies the active zone for regional volume. |
| `share_name` | String | Required. Share name of the volume |
| `security_style` | String | Optional. Security Style of the Volume |
| `snap_reserve` | f64 | Optional. Snap_reserve specifies percentage of volume storage reserved for snapshot storage. Default is 0 percent. |
| `storage_pool` | String | Required. StoragePool name of the volume |
| `network` | String | Output only. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `tiering_policy` | String | Tiering policy for the volume. |
| `hot_tier_size_used_gib` | String | Output only. Total hot tier data rounded down to the nearest GiB used by the Volume. This field is only used for flex Service Level |
| `capacity_gib` | String | Required. Capacity in GIB of the volume |
| `create_time` | String | Output only. Create time of the volume |
| `export_policy` | String | Optional. Export policy of the volume |
| `multiple_endpoints` | bool | Optional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints. Only the volume with large_capacity will be allowed to have multiple endpoints. |
| `name` | String | Identifier. Name of the volume |
| `restore_parameters` | String | Optional. Specifies the source of the volume to be created from. |
| `hybrid_replication_parameters` | String | Optional. The Hybrid Replication parameters for the volume. |
| `has_replication` | bool | Output only. Indicates whether the volume is part of a replication relationship. |
| `snapshot_directory` | bool | Optional. Snapshot_directory if enabled (true) the volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. |


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
volume_used_gib = volume.used_gib
volume_throughput_mibps = volume.throughput_mibps
volume_kerberos_enabled = volume.kerberos_enabled
volume_protocols = volume.protocols
volume_psa_range = volume.psa_range
volume_restricted_actions = volume.restricted_actions
volume_labels = volume.labels
volume_state = volume.state
volume_unix_permissions = volume.unix_permissions
volume_state_details = volume.state_details
volume_large_capacity = volume.large_capacity
volume_cold_tier_size_gib = volume.cold_tier_size_gib
volume_smb_settings = volume.smb_settings
volume_service_level = volume.service_level
volume_description = volume.description
volume_mount_options = volume.mount_options
volume_snapshot_policy = volume.snapshot_policy
volume_encryption_type = volume.encryption_type
volume_active_directory = volume.active_directory
volume_kms_config = volume.kms_config
volume_backup_config = volume.backup_config
volume_cache_parameters = volume.cache_parameters
volume_ldap_enabled = volume.ldap_enabled
volume_replica_zone = volume.replica_zone
volume_zone = volume.zone
volume_share_name = volume.share_name
volume_security_style = volume.security_style
volume_snap_reserve = volume.snap_reserve
volume_storage_pool = volume.storage_pool
volume_network = volume.network
volume_tiering_policy = volume.tiering_policy
volume_hot_tier_size_used_gib = volume.hot_tier_size_used_gib
volume_capacity_gib = volume.capacity_gib
volume_create_time = volume.create_time
volume_export_policy = volume.export_policy
volume_multiple_endpoints = volume.multiple_endpoints
volume_name = volume.name
volume_restore_parameters = volume.restore_parameters
volume_hybrid_replication_parameters = volume.hybrid_replication_parameters
volume_has_replication = volume.has_replication
volume_snapshot_directory = volume.snapshot_directory
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.netapp_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_done = operation.done
operation_response = operation.response
```

---


### Replication

Create a new replication for a volume.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transfer_stats` | String |  | Output only. Replication transfer statistics. |
| `destination_volume_parameters` | String |  | Required. Input only. Destination volume parameters |
| `state` | String |  | Output only. State of the replication. |
| `create_time` | String |  | Output only. Replication create time. |
| `destination_volume` | String |  | Output only. Full name of destination volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `name` | String |  | Identifier. The resource name of the Replication. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}`. |
| `description` | String |  | A description about this replication relationship. |
| `replication_schedule` | String |  | Required. Indicates the schedule for replication. |
| `hybrid_peering_details` | String |  | Output only. Hybrid peering details. |
| `hybrid_replication_user_commands` | String |  | Output only. Copy pastable snapmirror commands to be executed on onprem cluster by the customer. |
| `state_details` | String |  | Output only. State details of the replication. |
| `source_volume` | String |  | Output only. Full name of source volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `role` | String |  | Output only. Indicates whether this points to source or destination. |
| `mirror_state` | String |  | Output only. Indicates the state of mirroring. |
| `healthy` | bool |  | Output only. Condition of the relationship. Can be one of the following: - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer. - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer. |
| `cluster_location` | String |  | Optional. Location of the user cluster. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `hybrid_replication_type` | String |  | Output only. Type of the hybrid replication. |
| `parent` | String | ✅ | Required. The NetApp volume to create the replications of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transfer_stats` | String | Output only. Replication transfer statistics. |
| `destination_volume_parameters` | String | Required. Input only. Destination volume parameters |
| `state` | String | Output only. State of the replication. |
| `create_time` | String | Output only. Replication create time. |
| `destination_volume` | String | Output only. Full name of destination volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `name` | String | Identifier. The resource name of the Replication. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}`. |
| `description` | String | A description about this replication relationship. |
| `replication_schedule` | String | Required. Indicates the schedule for replication. |
| `hybrid_peering_details` | String | Output only. Hybrid peering details. |
| `hybrid_replication_user_commands` | String | Output only. Copy pastable snapmirror commands to be executed on onprem cluster by the customer. |
| `state_details` | String | Output only. State details of the replication. |
| `source_volume` | String | Output only. Full name of source volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `role` | String | Output only. Indicates whether this points to source or destination. |
| `mirror_state` | String | Output only. Indicates the state of mirroring. |
| `healthy` | bool | Output only. Condition of the relationship. Can be one of the following: - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer. - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer. |
| `cluster_location` | String | Optional. Location of the user cluster. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
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
replication_transfer_stats = replication.transfer_stats
replication_destination_volume_parameters = replication.destination_volume_parameters
replication_state = replication.state
replication_create_time = replication.create_time
replication_destination_volume = replication.destination_volume
replication_name = replication.name
replication_description = replication.description
replication_replication_schedule = replication.replication_schedule
replication_hybrid_peering_details = replication.hybrid_peering_details
replication_hybrid_replication_user_commands = replication.hybrid_replication_user_commands
replication_state_details = replication.state_details
replication_source_volume = replication.source_volume
replication_role = replication.role
replication_mirror_state = replication.mirror_state
replication_healthy = replication.healthy
replication_cluster_location = replication.cluster_location
replication_labels = replication.labels
replication_hybrid_replication_type = replication.hybrid_replication_type
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.netapp_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
```

---


### Quota_rule

Creates a new quota rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disk_limit_mib` | i64 |  | Required. The maximum allowed disk space in MiB. |
| `state` | String |  | Output only. State of the quota rule |
| `target` | String |  | Optional. The quota rule applies to the specified user or group, identified by a Unix UID/GID, Windows SID, or null for default. |
| `create_time` | String |  | Output only. Create time of the quota rule |
| `state_details` | String |  | Output only. State details of the quota rule |
| `description` | String |  | Optional. Description of the quota rule |
| `name` | String |  | Identifier. The resource name of the quota rule. Format: `projects/{project_number}/locations/{location_id}/volumes/volumes/{volume_id}/quotaRules/{quota_rule_id}`. |
| `type` | String |  | Required. The type of quota rule. |
| `labels` | HashMap<String, String> |  | Optional. Labels of the quota rule |
| `parent` | String | ✅ | Required. Parent value for CreateQuotaRuleRequest |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disk_limit_mib` | i64 | Required. The maximum allowed disk space in MiB. |
| `state` | String | Output only. State of the quota rule |
| `target` | String | Optional. The quota rule applies to the specified user or group, identified by a Unix UID/GID, Windows SID, or null for default. |
| `create_time` | String | Output only. Create time of the quota rule |
| `state_details` | String | Output only. State details of the quota rule |
| `description` | String | Optional. Description of the quota rule |
| `name` | String | Identifier. The resource name of the quota rule. Format: `projects/{project_number}/locations/{location_id}/volumes/volumes/{volume_id}/quotaRules/{quota_rule_id}`. |
| `type` | String | Required. The type of quota rule. |
| `labels` | HashMap<String, String> | Optional. Labels of the quota rule |


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
quota_rule_disk_limit_mib = quota_rule.disk_limit_mib
quota_rule_state = quota_rule.state
quota_rule_target = quota_rule.target
quota_rule_create_time = quota_rule.create_time
quota_rule_state_details = quota_rule.state_details
quota_rule_description = quota_rule.description
quota_rule_name = quota_rule.name
quota_rule_type = quota_rule.type
quota_rule_labels = quota_rule.labels
```

---


### Backup_policie

Creates new backup policy

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Description of the backup policy. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `daily_backup_limit` | i64 |  | Number of daily backups to keep. Note that the minimum daily backup limit is 2. |
| `name` | String |  | Identifier. The resource name of the backup policy. Format: `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`. |
| `create_time` | String |  | Output only. The time when the backup policy was created. |
| `weekly_backup_limit` | i64 |  | Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `assigned_volume_count` | i64 |  | Output only. The total number of volumes assigned by this backup policy. |
| `state` | String |  | Output only. The backup policy state. |
| `enabled` | bool |  | If enabled, make backups automatically according to the schedules. This will be applied to all volumes that have this policy attached and enforced on volume level. If not specified, default is true. |
| `monthly_backup_limit` | i64 |  | Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `parent` | String | ✅ | Required. The location to create the backup policies of, in the format `projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Description of the backup policy. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `daily_backup_limit` | i64 | Number of daily backups to keep. Note that the minimum daily backup limit is 2. |
| `name` | String | Identifier. The resource name of the backup policy. Format: `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`. |
| `create_time` | String | Output only. The time when the backup policy was created. |
| `weekly_backup_limit` | i64 | Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1. |
| `assigned_volume_count` | i64 | Output only. The total number of volumes assigned by this backup policy. |
| `state` | String | Output only. The backup policy state. |
| `enabled` | bool | If enabled, make backups automatically according to the schedules. This will be applied to all volumes that have this policy attached and enforced on volume level. If not specified, default is true. |
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
backup_policie_description = backup_policie.description
backup_policie_labels = backup_policie.labels
backup_policie_daily_backup_limit = backup_policie.daily_backup_limit
backup_policie_name = backup_policie.name
backup_policie_create_time = backup_policie.create_time
backup_policie_weekly_backup_limit = backup_policie.weekly_backup_limit
backup_policie_assigned_volume_count = backup_policie.assigned_volume_count
backup_policie_state = backup_policie.state
backup_policie_enabled = backup_policie.enabled
backup_policie_monthly_backup_limit = backup_policie.monthly_backup_limit
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |


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
location_labels = location.labels
location_location_id = location.location_id
```

---


### Replication

Create a new replication for a volume.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `description` | String |  | A description about this replication relationship. |
| `create_time` | String |  | Output only. Replication create time. |
| `source_volume` | String |  | Output only. Full name of source volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `state_details` | String |  | Output only. State details of the replication. |
| `transfer_stats` | String |  | Output only. Replication transfer statistics. |
| `healthy` | bool |  | Output only. Condition of the relationship. Can be one of the following: - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer. - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer. |
| `hybrid_replication_type` | String |  | Output only. Type of the hybrid replication. |
| `destination_volume` | String |  | Output only. Full name of destination volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `hybrid_peering_details` | String |  | Output only. Hybrid peering details. |
| `replication_schedule` | String |  | Required. Indicates the schedule for replication. |
| `cluster_location` | String |  | Optional. Location of the user cluster. |
| `hybrid_replication_user_commands` | String |  | Output only. Copy pastable snapmirror commands to be executed on onprem cluster by the customer. |
| `destination_volume_parameters` | String |  | Required. Input only. Destination volume parameters |
| `mirror_state` | String |  | Output only. Indicates the state of mirroring. |
| `role` | String |  | Output only. Indicates whether this points to source or destination. |
| `name` | String |  | Identifier. The resource name of the Replication. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}`. |
| `state` | String |  | Output only. State of the replication. |
| `parent` | String | ✅ | Required. The NetApp volume to create the replications of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `description` | String | A description about this replication relationship. |
| `create_time` | String | Output only. Replication create time. |
| `source_volume` | String | Output only. Full name of source volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `state_details` | String | Output only. State details of the replication. |
| `transfer_stats` | String | Output only. Replication transfer statistics. |
| `healthy` | bool | Output only. Condition of the relationship. Can be one of the following: - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer. - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer. |
| `hybrid_replication_type` | String | Output only. Type of the hybrid replication. |
| `destination_volume` | String | Output only. Full name of destination volume resource. Example : "projects/{project}/locations/{location}/volumes/{volume_id}" |
| `hybrid_peering_details` | String | Output only. Hybrid peering details. |
| `replication_schedule` | String | Required. Indicates the schedule for replication. |
| `cluster_location` | String | Optional. Location of the user cluster. |
| `hybrid_replication_user_commands` | String | Output only. Copy pastable snapmirror commands to be executed on onprem cluster by the customer. |
| `destination_volume_parameters` | String | Required. Input only. Destination volume parameters |
| `mirror_state` | String | Output only. Indicates the state of mirroring. |
| `role` | String | Output only. Indicates whether this points to source or destination. |
| `name` | String | Identifier. The resource name of the Replication. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}`. |
| `state` | String | Output only. State of the replication. |


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
replication_labels = replication.labels
replication_description = replication.description
replication_create_time = replication.create_time
replication_source_volume = replication.source_volume
replication_state_details = replication.state_details
replication_transfer_stats = replication.transfer_stats
replication_healthy = replication.healthy
replication_hybrid_replication_type = replication.hybrid_replication_type
replication_destination_volume = replication.destination_volume
replication_hybrid_peering_details = replication.hybrid_peering_details
replication_replication_schedule = replication.replication_schedule
replication_cluster_location = replication.cluster_location
replication_hybrid_replication_user_commands = replication.hybrid_replication_user_commands
replication_destination_volume_parameters = replication.destination_volume_parameters
replication_mirror_state = replication.mirror_state
replication_role = replication.role
replication_name = replication.name
replication_state = replication.state
```

---


### Volume

Creates a new Volume in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_type` | String |  | Output only. Specified the current volume encryption key source. |
| `protocols` | Vec<String> |  | Required. Protocols required for the volume |
| `hybrid_replication_parameters` | String |  | Optional. The Hybrid Replication parameters for the volume. |
| `psa_range` | String |  | Output only. This field is not implemented. The values provided in this field are ignored. |
| `capacity_gib` | String |  | Required. Capacity in GIB of the volume |
| `has_replication` | bool |  | Output only. Indicates whether the volume is part of a replication relationship. |
| `active_directory` | String |  | Output only. Specifies the ActiveDirectory name of a SMB volume. |
| `kms_config` | String |  | Output only. Specifies the KMS config to be used for volume encryption. |
| `create_time` | String |  | Output only. Create time of the volume |
| `hot_tier_size_used_gib` | String |  | Output only. Total hot tier data rounded down to the nearest GiB used by the Volume. This field is only used for flex Service Level |
| `large_capacity` | bool |  | Optional. Flag indicating if the volume will be a large capacity volume or a regular volume. |
| `cold_tier_size_gib` | String |  | Output only. Size of the volume cold tier data rounded down to the nearest GiB. |
| `name` | String |  | Identifier. Name of the volume |
| `network` | String |  | Output only. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `multiple_endpoints` | bool |  | Optional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints. Only the volume with large_capacity will be allowed to have multiple endpoints. |
| `state` | String |  | Output only. State of the volume |
| `service_level` | String |  | Output only. Service level of the volume |
| `storage_pool` | String |  | Required. StoragePool name of the volume |
| `unix_permissions` | String |  | Optional. Default unix style permission (e.g. 777) the mount point will be created with. Applicable for NFS protocol types only. |
| `state_details` | String |  | Output only. State details of the volume |
| `cache_parameters` | String |  | Optional. Cache parameters for the volume. |
| `tiering_policy` | String |  | Tiering policy for the volume. |
| `export_policy` | String |  | Optional. Export policy of the volume |
| `share_name` | String |  | Required. Share name of the volume |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `backup_config` | String |  | BackupConfig of the volume. |
| `security_style` | String |  | Optional. Security Style of the Volume |
| `restore_parameters` | String |  | Optional. Specifies the source of the volume to be created from. |
| `smb_settings` | Vec<String> |  | Optional. SMB share settings for the volume. |
| `throughput_mibps` | f64 |  | Optional. Throughput of the volume (in MiB/s) |
| `snapshot_policy` | String |  | Optional. SnapshotPolicy for a volume. |
| `kerberos_enabled` | bool |  | Optional. Flag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p). |
| `ldap_enabled` | bool |  | Output only. Flag indicating if the volume is NFS LDAP enabled or not. |
| `replica_zone` | String |  | Output only. Specifies the replica zone for regional volume. |
| `mount_options` | Vec<String> |  | Output only. Mount options of this volume |
| `snap_reserve` | f64 |  | Optional. Snap_reserve specifies percentage of volume storage reserved for snapshot storage. Default is 0 percent. |
| `restricted_actions` | Vec<String> |  | Optional. List of actions that are restricted on this volume. |
| `snapshot_directory` | bool |  | Optional. Snapshot_directory if enabled (true) the volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. |
| `description` | String |  | Optional. Description of the volume |
| `zone` | String |  | Output only. Specifies the active zone for regional volume. |
| `used_gib` | String |  | Output only. Used capacity in GIB of the volume. This is computed periodically and it does not represent the realtime usage. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_type` | String | Output only. Specified the current volume encryption key source. |
| `protocols` | Vec<String> | Required. Protocols required for the volume |
| `hybrid_replication_parameters` | String | Optional. The Hybrid Replication parameters for the volume. |
| `psa_range` | String | Output only. This field is not implemented. The values provided in this field are ignored. |
| `capacity_gib` | String | Required. Capacity in GIB of the volume |
| `has_replication` | bool | Output only. Indicates whether the volume is part of a replication relationship. |
| `active_directory` | String | Output only. Specifies the ActiveDirectory name of a SMB volume. |
| `kms_config` | String | Output only. Specifies the KMS config to be used for volume encryption. |
| `create_time` | String | Output only. Create time of the volume |
| `hot_tier_size_used_gib` | String | Output only. Total hot tier data rounded down to the nearest GiB used by the Volume. This field is only used for flex Service Level |
| `large_capacity` | bool | Optional. Flag indicating if the volume will be a large capacity volume or a regular volume. |
| `cold_tier_size_gib` | String | Output only. Size of the volume cold tier data rounded down to the nearest GiB. |
| `name` | String | Identifier. Name of the volume |
| `network` | String | Output only. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `multiple_endpoints` | bool | Optional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints. Only the volume with large_capacity will be allowed to have multiple endpoints. |
| `state` | String | Output only. State of the volume |
| `service_level` | String | Output only. Service level of the volume |
| `storage_pool` | String | Required. StoragePool name of the volume |
| `unix_permissions` | String | Optional. Default unix style permission (e.g. 777) the mount point will be created with. Applicable for NFS protocol types only. |
| `state_details` | String | Output only. State details of the volume |
| `cache_parameters` | String | Optional. Cache parameters for the volume. |
| `tiering_policy` | String | Tiering policy for the volume. |
| `export_policy` | String | Optional. Export policy of the volume |
| `share_name` | String | Required. Share name of the volume |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `backup_config` | String | BackupConfig of the volume. |
| `security_style` | String | Optional. Security Style of the Volume |
| `restore_parameters` | String | Optional. Specifies the source of the volume to be created from. |
| `smb_settings` | Vec<String> | Optional. SMB share settings for the volume. |
| `throughput_mibps` | f64 | Optional. Throughput of the volume (in MiB/s) |
| `snapshot_policy` | String | Optional. SnapshotPolicy for a volume. |
| `kerberos_enabled` | bool | Optional. Flag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p). |
| `ldap_enabled` | bool | Output only. Flag indicating if the volume is NFS LDAP enabled or not. |
| `replica_zone` | String | Output only. Specifies the replica zone for regional volume. |
| `mount_options` | Vec<String> | Output only. Mount options of this volume |
| `snap_reserve` | f64 | Optional. Snap_reserve specifies percentage of volume storage reserved for snapshot storage. Default is 0 percent. |
| `restricted_actions` | Vec<String> | Optional. List of actions that are restricted on this volume. |
| `snapshot_directory` | bool | Optional. Snapshot_directory if enabled (true) the volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. |
| `description` | String | Optional. Description of the volume |
| `zone` | String | Output only. Specifies the active zone for regional volume. |
| `used_gib` | String | Output only. Used capacity in GIB of the volume. This is computed periodically and it does not represent the realtime usage. |


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
volume_encryption_type = volume.encryption_type
volume_protocols = volume.protocols
volume_hybrid_replication_parameters = volume.hybrid_replication_parameters
volume_psa_range = volume.psa_range
volume_capacity_gib = volume.capacity_gib
volume_has_replication = volume.has_replication
volume_active_directory = volume.active_directory
volume_kms_config = volume.kms_config
volume_create_time = volume.create_time
volume_hot_tier_size_used_gib = volume.hot_tier_size_used_gib
volume_large_capacity = volume.large_capacity
volume_cold_tier_size_gib = volume.cold_tier_size_gib
volume_name = volume.name
volume_network = volume.network
volume_multiple_endpoints = volume.multiple_endpoints
volume_state = volume.state
volume_service_level = volume.service_level
volume_storage_pool = volume.storage_pool
volume_unix_permissions = volume.unix_permissions
volume_state_details = volume.state_details
volume_cache_parameters = volume.cache_parameters
volume_tiering_policy = volume.tiering_policy
volume_export_policy = volume.export_policy
volume_share_name = volume.share_name
volume_labels = volume.labels
volume_backup_config = volume.backup_config
volume_security_style = volume.security_style
volume_restore_parameters = volume.restore_parameters
volume_smb_settings = volume.smb_settings
volume_throughput_mibps = volume.throughput_mibps
volume_snapshot_policy = volume.snapshot_policy
volume_kerberos_enabled = volume.kerberos_enabled
volume_ldap_enabled = volume.ldap_enabled
volume_replica_zone = volume.replica_zone
volume_mount_options = volume.mount_options
volume_snap_reserve = volume.snap_reserve
volume_restricted_actions = volume.restricted_actions
volume_snapshot_directory = volume.snapshot_directory
volume_description = volume.description
volume_zone = volume.zone
volume_used_gib = volume.used_gib
```

---


### Kms_config

Creates a new KMS config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Name of the KmsConfig. |
| `create_time` | String |  | Output only. Create time of the KmsConfig. |
| `description` | String |  | Description of the KmsConfig. |
| `instructions` | String |  | Output only. Instructions to provide the access to the customer provided encryption key. |
| `service_account` | String |  | Output only. The Service account which will have access to the customer provided encryption key. |
| `state_details` | String |  | Output only. State details of the KmsConfig. |
| `state` | String |  | Output only. State of the KmsConfig. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `crypto_key_name` | String |  | Required. Customer managed crypto key resource full name. Format: projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Name of the KmsConfig. |
| `create_time` | String | Output only. Create time of the KmsConfig. |
| `description` | String | Description of the KmsConfig. |
| `instructions` | String | Output only. Instructions to provide the access to the customer provided encryption key. |
| `service_account` | String | Output only. The Service account which will have access to the customer provided encryption key. |
| `state_details` | String | Output only. State details of the KmsConfig. |
| `state` | String | Output only. State of the KmsConfig. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `crypto_key_name` | String | Required. Customer managed crypto key resource full name. Format: projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}. |


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
kms_config_name = kms_config.name
kms_config_create_time = kms_config.create_time
kms_config_description = kms_config.description
kms_config_instructions = kms_config.instructions
kms_config_service_account = kms_config.service_account
kms_config_state_details = kms_config.state_details
kms_config_state = kms_config.state
kms_config_labels = kms_config.labels
kms_config_crypto_key_name = kms_config.crypto_key_name
```

---


### Active_directorie

CreateActiveDirectory Creates the active directory specified in the request.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kdc_ip` | String |  | KDC server IP address for the active directory machine. |
| `backup_operators` | Vec<String> |  | Optional. Users to be added to the Built-in Backup Operator active directory group. |
| `net_bios_prefix` | String |  | Required. NetBIOSPrefix is used as a prefix for SMB server name. |
| `security_operators` | Vec<String> |  | Optional. Domain users to be given the SeSecurityPrivilege. |
| `kdc_hostname` | String |  | Name of the active directory machine. This optional parameter is used only while creating kerberos volume |
| `organizational_unit` | String |  | The Organizational Unit (OU) within the Windows Active Directory the user belongs to. |
| `administrators` | Vec<String> |  | Optional. Users to be added to the Built-in Admininstrators group. |
| `description` | String |  | Description of the active directory. |
| `password` | String |  | Required. Password of the Active Directory domain administrator. |
| `aes_encryption` | bool |  | If enabled, AES encryption will be enabled for SMB communication. |
| `encrypt_dc_connections` | bool |  | If enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted. |
| `state_details` | String |  | Output only. The state details of the Active Directory. |
| `username` | String |  | Required. Username of the Active Directory domain administrator. |
| `domain` | String |  | Required. Name of the Active Directory domain |
| `labels` | HashMap<String, String> |  | Labels for the active directory. |
| `site` | String |  | The Active Directory site the service will limit Domain Controller discovery too. |
| `nfs_users_with_ldap` | bool |  | If enabled, will allow access to local users and LDAP users. If access is needed for only LDAP users, it has to be disabled. |
| `name` | String |  | Identifier. The resource name of the active directory. Format: `projects/{project_number}/locations/{location_id}/activeDirectories/{active_directory_id}`. |
| `ldap_signing` | bool |  | Specifies whether or not the LDAP traffic needs to be signed. |
| `create_time` | String |  | Output only. Create time of the active directory. |
| `dns` | String |  | Required. Comma separated list of DNS server IP addresses for the Active Directory domain. |
| `state` | String |  | Output only. The state of the AD. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kdc_ip` | String | KDC server IP address for the active directory machine. |
| `backup_operators` | Vec<String> | Optional. Users to be added to the Built-in Backup Operator active directory group. |
| `net_bios_prefix` | String | Required. NetBIOSPrefix is used as a prefix for SMB server name. |
| `security_operators` | Vec<String> | Optional. Domain users to be given the SeSecurityPrivilege. |
| `kdc_hostname` | String | Name of the active directory machine. This optional parameter is used only while creating kerberos volume |
| `organizational_unit` | String | The Organizational Unit (OU) within the Windows Active Directory the user belongs to. |
| `administrators` | Vec<String> | Optional. Users to be added to the Built-in Admininstrators group. |
| `description` | String | Description of the active directory. |
| `password` | String | Required. Password of the Active Directory domain administrator. |
| `aes_encryption` | bool | If enabled, AES encryption will be enabled for SMB communication. |
| `encrypt_dc_connections` | bool | If enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted. |
| `state_details` | String | Output only. The state details of the Active Directory. |
| `username` | String | Required. Username of the Active Directory domain administrator. |
| `domain` | String | Required. Name of the Active Directory domain |
| `labels` | HashMap<String, String> | Labels for the active directory. |
| `site` | String | The Active Directory site the service will limit Domain Controller discovery too. |
| `nfs_users_with_ldap` | bool | If enabled, will allow access to local users and LDAP users. If access is needed for only LDAP users, it has to be disabled. |
| `name` | String | Identifier. The resource name of the active directory. Format: `projects/{project_number}/locations/{location_id}/activeDirectories/{active_directory_id}`. |
| `ldap_signing` | bool | Specifies whether or not the LDAP traffic needs to be signed. |
| `create_time` | String | Output only. Create time of the active directory. |
| `dns` | String | Required. Comma separated list of DNS server IP addresses for the Active Directory domain. |
| `state` | String | Output only. The state of the AD. |


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
active_directorie_kdc_ip = active_directorie.kdc_ip
active_directorie_backup_operators = active_directorie.backup_operators
active_directorie_net_bios_prefix = active_directorie.net_bios_prefix
active_directorie_security_operators = active_directorie.security_operators
active_directorie_kdc_hostname = active_directorie.kdc_hostname
active_directorie_organizational_unit = active_directorie.organizational_unit
active_directorie_administrators = active_directorie.administrators
active_directorie_description = active_directorie.description
active_directorie_password = active_directorie.password
active_directorie_aes_encryption = active_directorie.aes_encryption
active_directorie_encrypt_dc_connections = active_directorie.encrypt_dc_connections
active_directorie_state_details = active_directorie.state_details
active_directorie_username = active_directorie.username
active_directorie_domain = active_directorie.domain
active_directorie_labels = active_directorie.labels
active_directorie_site = active_directorie.site
active_directorie_nfs_users_with_ldap = active_directorie.nfs_users_with_ldap
active_directorie_name = active_directorie.name
active_directorie_ldap_signing = active_directorie.ldap_signing
active_directorie_create_time = active_directorie.create_time
active_directorie_dns = active_directorie.dns
active_directorie_state = active_directorie.state
```

---


### Backup_vault

Creates new backup vault

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_region` | String |  | Optional. Region where the backups are stored. Format: `projects/{project_id}/locations/{location}` |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `name` | String |  | Identifier. The resource name of the backup vault. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`. |
| `destination_backup_vault` | String |  | Output only. Name of the Backup vault created in backup region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `source_backup_vault` | String |  | Output only. Name of the Backup vault created in source region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `create_time` | String |  | Output only. Create time of the backup vault. |
| `backup_vault_type` | String |  | Optional. Type of backup vault to be created. Default is IN_REGION. |
| `backup_retention_policy` | String |  | Optional. Backup retention policy defining the retenton of backups. |
| `source_region` | String |  | Output only. Region in which the backup vault is created. Format: `projects/{project_id}/locations/{location}` |
| `state` | String |  | Output only. The backup vault state. |
| `description` | String |  | Description of the backup vault. |
| `parent` | String | ✅ | Required. The location to create the backup vaults, in the format `projects/{project_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_region` | String | Optional. Region where the backups are stored. Format: `projects/{project_id}/locations/{location}` |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `name` | String | Identifier. The resource name of the backup vault. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`. |
| `destination_backup_vault` | String | Output only. Name of the Backup vault created in backup region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `source_backup_vault` | String | Output only. Name of the Backup vault created in source region. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}` |
| `create_time` | String | Output only. Create time of the backup vault. |
| `backup_vault_type` | String | Optional. Type of backup vault to be created. Default is IN_REGION. |
| `backup_retention_policy` | String | Optional. Backup retention policy defining the retenton of backups. |
| `source_region` | String | Output only. Region in which the backup vault is created. Format: `projects/{project_id}/locations/{location}` |
| `state` | String | Output only. The backup vault state. |
| `description` | String | Description of the backup vault. |


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
backup_vault_backup_region = backup_vault.backup_region
backup_vault_labels = backup_vault.labels
backup_vault_name = backup_vault.name
backup_vault_destination_backup_vault = backup_vault.destination_backup_vault
backup_vault_source_backup_vault = backup_vault.source_backup_vault
backup_vault_create_time = backup_vault.create_time
backup_vault_backup_vault_type = backup_vault.backup_vault_type
backup_vault_backup_retention_policy = backup_vault.backup_retention_policy
backup_vault_source_region = backup_vault.source_region
backup_vault_state = backup_vault.state
backup_vault_description = backup_vault.description
```

---


### Storage_pool

Creates a new storage pool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time of the storage pool |
| `encryption_type` | String |  | Output only. Specifies the current pool encryption key source. |
| `hot_tier_size_gib` | String |  | Optional. Total hot tier capacity for the Storage Pool. It is applicable only to Flex service level. It should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set. |
| `total_throughput_mibps` | String |  | Optional. Custom Performance Total Throughput of the pool (in MiBps) |
| `state` | String |  | Output only. State of the storage pool |
| `state_details` | String |  | Output only. State details of the storage pool |
| `available_throughput_mibps` | f64 |  | Output only. Available throughput of the storage pool (in MiB/s). |
| `description` | String |  | Optional. Description of the storage pool |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use |
| `service_level` | String |  | Required. Service level of the storage pool |
| `custom_performance_enabled` | bool |  | Optional. True if using Independent Scaling of capacity and performance (Hyperdisk) By default set to false |
| `total_iops` | String |  | Optional. Custom Performance Total IOPS of the pool if not provided, it will be calculated based on the total_throughput_mibps |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `ldap_enabled` | bool |  | Optional. Flag indicating if the pool is NFS LDAP enabled or not. |
| `capacity_gib` | String |  | Required. Capacity in GIB of the pool |
| `active_directory` | String |  | Optional. Specifies the Active Directory to be used for creating a SMB volume. |
| `enable_hot_tier_auto_resize` | bool |  | Optional. Flag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true. The increment will kick in only if the new size after increment is still less than or equal to storage pool size. |
| `volume_count` | i64 |  | Output only. Volume count of the storage pool |
| `allow_auto_tiering` | bool |  | Optional. True if the storage pool supports Auto Tiering enabled volumes. Default is false. Auto-tiering can be enabled after storage pool creation but it can't be disabled once enabled. |
| `cold_tier_size_used_gib` | String |  | Output only. Total cold tier data rounded down to the nearest GiB used by the storage pool. |
| `volume_capacity_gib` | String |  | Output only. Allocated size of all volumes in GIB in the storage pool |
| `replica_zone` | String |  | Optional. Specifies the replica zone for regional storagePool. |
| `global_access_allowed` | bool |  | Deprecated. Used to allow SO pool to access AD or DNS server from other regions. |
| `kms_config` | String |  | Optional. Specifies the KMS config to be used for volume encryption. |
| `hot_tier_size_used_gib` | String |  | Output only. Total hot tier data rounded down to the nearest GiB used by the storage pool. |
| `psa_range` | String |  | Optional. This field is not implemented. The values provided in this field are ignored. |
| `zone` | String |  | Optional. Specifies the active zone for regional storagePool. |
| `name` | String |  | Identifier. Name of the storage pool |
| `qos_type` | String |  | Optional. QoS (Quality of Service) Type of the storage pool |
| `network` | String |  | Required. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time of the storage pool |
| `encryption_type` | String | Output only. Specifies the current pool encryption key source. |
| `hot_tier_size_gib` | String | Optional. Total hot tier capacity for the Storage Pool. It is applicable only to Flex service level. It should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set. |
| `total_throughput_mibps` | String | Optional. Custom Performance Total Throughput of the pool (in MiBps) |
| `state` | String | Output only. State of the storage pool |
| `state_details` | String | Output only. State details of the storage pool |
| `available_throughput_mibps` | f64 | Output only. Available throughput of the storage pool (in MiB/s). |
| `description` | String | Optional. Description of the storage pool |
| `satisfies_pzs` | bool | Output only. Reserved for future use |
| `service_level` | String | Required. Service level of the storage pool |
| `custom_performance_enabled` | bool | Optional. True if using Independent Scaling of capacity and performance (Hyperdisk) By default set to false |
| `total_iops` | String | Optional. Custom Performance Total IOPS of the pool if not provided, it will be calculated based on the total_throughput_mibps |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `ldap_enabled` | bool | Optional. Flag indicating if the pool is NFS LDAP enabled or not. |
| `capacity_gib` | String | Required. Capacity in GIB of the pool |
| `active_directory` | String | Optional. Specifies the Active Directory to be used for creating a SMB volume. |
| `enable_hot_tier_auto_resize` | bool | Optional. Flag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true. The increment will kick in only if the new size after increment is still less than or equal to storage pool size. |
| `volume_count` | i64 | Output only. Volume count of the storage pool |
| `allow_auto_tiering` | bool | Optional. True if the storage pool supports Auto Tiering enabled volumes. Default is false. Auto-tiering can be enabled after storage pool creation but it can't be disabled once enabled. |
| `cold_tier_size_used_gib` | String | Output only. Total cold tier data rounded down to the nearest GiB used by the storage pool. |
| `volume_capacity_gib` | String | Output only. Allocated size of all volumes in GIB in the storage pool |
| `replica_zone` | String | Optional. Specifies the replica zone for regional storagePool. |
| `global_access_allowed` | bool | Deprecated. Used to allow SO pool to access AD or DNS server from other regions. |
| `kms_config` | String | Optional. Specifies the KMS config to be used for volume encryption. |
| `hot_tier_size_used_gib` | String | Output only. Total hot tier data rounded down to the nearest GiB used by the storage pool. |
| `psa_range` | String | Optional. This field is not implemented. The values provided in this field are ignored. |
| `zone` | String | Optional. Specifies the active zone for regional storagePool. |
| `name` | String | Identifier. Name of the storage pool |
| `qos_type` | String | Optional. QoS (Quality of Service) Type of the storage pool |
| `network` | String | Required. VPC Network name. Format: projects/{project}/global/networks/{network} |
| `satisfies_pzi` | bool | Output only. Reserved for future use |


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
storage_pool_create_time = storage_pool.create_time
storage_pool_encryption_type = storage_pool.encryption_type
storage_pool_hot_tier_size_gib = storage_pool.hot_tier_size_gib
storage_pool_total_throughput_mibps = storage_pool.total_throughput_mibps
storage_pool_state = storage_pool.state
storage_pool_state_details = storage_pool.state_details
storage_pool_available_throughput_mibps = storage_pool.available_throughput_mibps
storage_pool_description = storage_pool.description
storage_pool_satisfies_pzs = storage_pool.satisfies_pzs
storage_pool_service_level = storage_pool.service_level
storage_pool_custom_performance_enabled = storage_pool.custom_performance_enabled
storage_pool_total_iops = storage_pool.total_iops
storage_pool_labels = storage_pool.labels
storage_pool_ldap_enabled = storage_pool.ldap_enabled
storage_pool_capacity_gib = storage_pool.capacity_gib
storage_pool_active_directory = storage_pool.active_directory
storage_pool_enable_hot_tier_auto_resize = storage_pool.enable_hot_tier_auto_resize
storage_pool_volume_count = storage_pool.volume_count
storage_pool_allow_auto_tiering = storage_pool.allow_auto_tiering
storage_pool_cold_tier_size_used_gib = storage_pool.cold_tier_size_used_gib
storage_pool_volume_capacity_gib = storage_pool.volume_capacity_gib
storage_pool_replica_zone = storage_pool.replica_zone
storage_pool_global_access_allowed = storage_pool.global_access_allowed
storage_pool_kms_config = storage_pool.kms_config
storage_pool_hot_tier_size_used_gib = storage_pool.hot_tier_size_used_gib
storage_pool_psa_range = storage_pool.psa_range
storage_pool_zone = storage_pool.zone
storage_pool_name = storage_pool.name
storage_pool_qos_type = storage_pool.qos_type
storage_pool_network = storage_pool.network
storage_pool_satisfies_pzi = storage_pool.satisfies_pzi
```

---


### Backup

Creates a backup from the volume specified in the request The backup can be created from the given snapshot if specified in the request. If no snapshot specified, there'll be a new snapshot taken to initiate the backup creation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `enforced_retention_end_time` | String |  | Output only. The time until which the backup is not deletable. |
| `backup_type` | String |  | Output only. Type of backup, manually created or created by a backup policy. |
| `name` | String |  | Identifier. The resource name of the backup. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`. |
| `backup_region` | String |  | Output only. Region in which backup is stored. Format: `projects/{project_id}/locations/{location}` |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use |
| `volume_usage_bytes` | String |  | Output only. Size of the file system when the backup was created. When creating a new volume from the backup, the volume capacity will have to be at least as big. |
| `chain_storage_bytes` | String |  | Output only. Total size of all backups in a chain in bytes = baseline backup size + sum(incremental backup size) |
| `source_volume` | String |  | Volume full name of this backup belongs to. Format: `projects/{projects_id}/locations/{location}/volumes/{volume_id}` |
| `state` | String |  | Output only. The backup state. |
| `volume_region` | String |  | Output only. Region of the volume from which the backup was created. Format: `projects/{project_id}/locations/{location}` |
| `source_snapshot` | String |  | If specified, backup will be created from the given snapshot. If not specified, there will be a new snapshot taken to initiate the backup creation. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}` |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use |
| `create_time` | String |  | Output only. The time when the backup was created. |
| `parent` | String | ✅ | Required. The NetApp backupVault to create the backups of, in the format `projects/*/locations/*/backupVaults/{backup_vault_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `enforced_retention_end_time` | String | Output only. The time until which the backup is not deletable. |
| `backup_type` | String | Output only. Type of backup, manually created or created by a backup policy. |
| `name` | String | Identifier. The resource name of the backup. Format: `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`. |
| `backup_region` | String | Output only. Region in which backup is stored. Format: `projects/{project_id}/locations/{location}` |
| `satisfies_pzi` | bool | Output only. Reserved for future use |
| `volume_usage_bytes` | String | Output only. Size of the file system when the backup was created. When creating a new volume from the backup, the volume capacity will have to be at least as big. |
| `chain_storage_bytes` | String | Output only. Total size of all backups in a chain in bytes = baseline backup size + sum(incremental backup size) |
| `source_volume` | String | Volume full name of this backup belongs to. Format: `projects/{projects_id}/locations/{location}/volumes/{volume_id}` |
| `state` | String | Output only. The backup state. |
| `volume_region` | String | Output only. Region of the volume from which the backup was created. Format: `projects/{project_id}/locations/{location}` |
| `source_snapshot` | String | If specified, backup will be created from the given snapshot. If not specified, there will be a new snapshot taken to initiate the backup creation. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}` |
| `satisfies_pzs` | bool | Output only. Reserved for future use |
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
backup_description = backup.description
backup_labels = backup.labels
backup_enforced_retention_end_time = backup.enforced_retention_end_time
backup_backup_type = backup.backup_type
backup_name = backup.name
backup_backup_region = backup.backup_region
backup_satisfies_pzi = backup.satisfies_pzi
backup_volume_usage_bytes = backup.volume_usage_bytes
backup_chain_storage_bytes = backup.chain_storage_bytes
backup_source_volume = backup.source_volume
backup_state = backup.state
backup_volume_region = backup.volume_region
backup_source_snapshot = backup.source_snapshot
backup_satisfies_pzs = backup.satisfies_pzs
backup_create_time = backup.create_time
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
| `labels` | HashMap<String, String> |  | Resource labels to represent user provided metadata. |
| `description` | String |  | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `state` | String |  | Output only. The snapshot state. |
| `create_time` | String |  | Output only. The time when the snapshot was created. |
| `state_details` | String |  | Output only. State details of the storage pool |
| `parent` | String | ✅ | Required. The NetApp volume to create the snapshots of, in the format `projects/{project_id}/locations/{location}/volumes/{volume_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `used_bytes` | f64 | Output only. Current storage usage for the snapshot in bytes. |
| `name` | String | Identifier. The resource name of the snapshot. Format: `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}`. |
| `labels` | HashMap<String, String> | Resource labels to represent user provided metadata. |
| `description` | String | A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected. |
| `state` | String | Output only. The snapshot state. |
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
snapshot_labels = snapshot.labels
snapshot_description = snapshot.description
snapshot_state = snapshot.state
snapshot_create_time = snapshot.create_time
snapshot_state_details = snapshot.state_details
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple kms_config resources
kms_config_0 = provider.netapp_api.Kms_config {
    parent = "value-0"
}
kms_config_1 = provider.netapp_api.Kms_config {
    parent = "value-1"
}
kms_config_2 = provider.netapp_api.Kms_config {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    kms_config = provider.netapp_api.Kms_config {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Netapp_api Documentation](https://cloud.google.com/netapp_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
