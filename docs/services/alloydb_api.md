# Alloydb_api Service



**Resources**: 21

---

## Overview

The alloydb_api service provides access to 21 resource types:

- [Location](#location) [R]
- [Cluster](#cluster) [CRUD]
- [Backup](#backup) [CRUD]
- [Operation](#operation) [CRD]
- [User](#user) [CRUD]
- [Instance](#instance) [CRUD]
- [Supported_database_flag](#supported_database_flag) [R]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [Backup](#backup) [CRUD]
- [Cluster](#cluster) [CRUD]
- [Supported_database_flag](#supported_database_flag) [R]
- [Instance](#instance) [CRUD]
- [User](#user) [CRUD]
- [Instance](#instance) [CRUD]
- [Backup](#backup) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [User](#user) [CRUD]
- [Supported_database_flag](#supported_database_flag) [R]
- [Cluster](#cluster) [CRUD]

---

## Resources


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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
location_labels = location.labels
```

---


### Cluster

Creates a new Cluster in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `continuous_backup_info` | String |  | Output only. Continuous backup properties for this cluster. |
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `dataplex_config` | String |  | Optional. Configuration for Dataplex integration. |
| `maintenance_schedule` | String |  | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `migration_source` | String |  | Output only. Cluster created via DMS migration. |
| `encryption_info` | String |  | Output only. The encryption information for the cluster. |
| `backupdr_info` | String |  | Output only. Output only information about BackupDR protection for this cluster. |
| `ssl_config` | String |  | SSL configuration for this AlloyDB cluster. |
| `automated_backup_policy` | String |  | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `initial_user` | String |  | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `psc_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `trial_metadata` | String |  | Output only. Metadata for free trial clusters |
| `primary_config` | String |  | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `create_time` | String |  | Output only. Create time stamp |
| `network` | String |  | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `display_name` | String |  | User-settable and human-readable display name for the Cluster. |
| `cluster_type` | String |  | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `backupdr_backup_source` | String |  | Output only. Cluster created from a BackupDR backup. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `backup_source` | String |  | Output only. Cluster created from backup. |
| `state` | String |  | Output only. The current serving state of the cluster. |
| `cloudsql_backup_run_source` | String |  | Output only. Cluster created from CloudSQL snapshot. |
| `continuous_backup_config` | String |  | Optional. Continuous backup configuration for this cluster. |
| `name` | String |  | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `delete_time` | String |  | Output only. Delete time stamp |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `database_version` | String |  | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `secondary_config` | String |  | Cross Region replication config specific to SECONDARY cluster. |
| `maintenance_update_policy` | String |  | Optional. The maintenance update policy determines when to allow or deny updates. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `subscription_type` | String |  | Optional. Subscription type of the cluster. |
| `network_config` | String |  |  |
| `update_time` | String |  | Output only. Update time stamp |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `parent` | String | ✅ | Required. The location of the new cluster. For the required format, see the comment on the Cluster.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `continuous_backup_info` | String | Output only. Continuous backup properties for this cluster. |
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `dataplex_config` | String | Optional. Configuration for Dataplex integration. |
| `maintenance_schedule` | String | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `migration_source` | String | Output only. Cluster created via DMS migration. |
| `encryption_info` | String | Output only. The encryption information for the cluster. |
| `backupdr_info` | String | Output only. Output only information about BackupDR protection for this cluster. |
| `ssl_config` | String | SSL configuration for this AlloyDB cluster. |
| `automated_backup_policy` | String | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `initial_user` | String | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `psc_config` | String | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `trial_metadata` | String | Output only. Metadata for free trial clusters |
| `primary_config` | String | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `create_time` | String | Output only. Create time stamp |
| `network` | String | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `display_name` | String | User-settable and human-readable display name for the Cluster. |
| `cluster_type` | String | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `backupdr_backup_source` | String | Output only. Cluster created from a BackupDR backup. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `backup_source` | String | Output only. Cluster created from backup. |
| `state` | String | Output only. The current serving state of the cluster. |
| `cloudsql_backup_run_source` | String | Output only. Cluster created from CloudSQL snapshot. |
| `continuous_backup_config` | String | Optional. Continuous backup configuration for this cluster. |
| `name` | String | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `delete_time` | String | Output only. Delete time stamp |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `database_version` | String | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `secondary_config` | String | Cross Region replication config specific to SECONDARY cluster. |
| `maintenance_update_policy` | String | Optional. The maintenance update policy determines when to allow or deny updates. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `subscription_type` | String | Optional. Subscription type of the cluster. |
| `network_config` | String |  |
| `update_time` | String | Output only. Update time stamp |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.alloydb_api.Cluster {
    parent = "value"  # Required. The location of the new cluster. For the required format, see the comment on the Cluster.name field.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_continuous_backup_info = cluster.continuous_backup_info
cluster_encryption_config = cluster.encryption_config
cluster_dataplex_config = cluster.dataplex_config
cluster_maintenance_schedule = cluster.maintenance_schedule
cluster_migration_source = cluster.migration_source
cluster_encryption_info = cluster.encryption_info
cluster_backupdr_info = cluster.backupdr_info
cluster_ssl_config = cluster.ssl_config
cluster_automated_backup_policy = cluster.automated_backup_policy
cluster_initial_user = cluster.initial_user
cluster_psc_config = cluster.psc_config
cluster_trial_metadata = cluster.trial_metadata
cluster_primary_config = cluster.primary_config
cluster_create_time = cluster.create_time
cluster_network = cluster.network
cluster_display_name = cluster.display_name
cluster_cluster_type = cluster.cluster_type
cluster_etag = cluster.etag
cluster_backupdr_backup_source = cluster.backupdr_backup_source
cluster_labels = cluster.labels
cluster_uid = cluster.uid
cluster_backup_source = cluster.backup_source
cluster_state = cluster.state
cluster_cloudsql_backup_run_source = cluster.cloudsql_backup_run_source
cluster_continuous_backup_config = cluster.continuous_backup_config
cluster_name = cluster.name
cluster_delete_time = cluster.delete_time
cluster_tags = cluster.tags
cluster_database_version = cluster.database_version
cluster_secondary_config = cluster.secondary_config
cluster_maintenance_update_policy = cluster.maintenance_update_policy
cluster_reconciling = cluster.reconciling
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_subscription_type = cluster.subscription_type
cluster_network_config = cluster.network_config
cluster_update_time = cluster.update_time
cluster_annotations = cluster.annotations
```

---


### Backup

Creates a new Backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_completion_time` | String |  | Output only. Timestamp when the resource finished being created. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `create_time` | String |  | Output only. Create time stamp |
| `database_version` | String |  | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `display_name` | String |  | User-settable and human-readable display name for the Backup. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `cluster_uid` | String |  | Output only. The system-generated UID of the cluster which was used to create this resource. |
| `size_bytes` | String |  | Output only. The size of the backup in bytes. |
| `type` | String |  | The backup type, which suggests the trigger for the backup. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `description` | String |  | User-provided description of the backup. |
| `name` | String |  | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `encryption_info` | String |  | Output only. The encryption information for the backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `cluster_name` | String |  | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `expiry_quantity` | String |  | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `expiry_time` | String |  | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `update_time` | String |  | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_completion_time` | String | Output only. Timestamp when the resource finished being created. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `create_time` | String | Output only. Create time stamp |
| `database_version` | String | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `display_name` | String | User-settable and human-readable display name for the Backup. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `cluster_uid` | String | Output only. The system-generated UID of the cluster which was used to create this resource. |
| `size_bytes` | String | Output only. The size of the backup in bytes. |
| `type` | String | The backup type, which suggests the trigger for the backup. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `description` | String | User-provided description of the backup. |
| `name` | String | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `encryption_info` | String | Output only. The encryption information for the backup. |
| `state` | String | Output only. The current state of the backup. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `cluster_name` | String | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `expiry_quantity` | String | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `expiry_time` | String | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `update_time` | String | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `delete_time` | String | Output only. Delete time stamp |
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.alloydb_api.Backup {
    parent = "value"  # Required. Value for parent.
}

# Access backup outputs
backup_id = backup.id
backup_create_completion_time = backup.create_completion_time
backup_annotations = backup.annotations
backup_create_time = backup.create_time
backup_database_version = backup.database_version
backup_display_name = backup.display_name
backup_reconciling = backup.reconciling
backup_satisfies_pzs = backup.satisfies_pzs
backup_cluster_uid = backup.cluster_uid
backup_size_bytes = backup.size_bytes
backup_type = backup.type
backup_labels = backup.labels
backup_description = backup.description
backup_name = backup.name
backup_encryption_info = backup.encryption_info
backup_state = backup.state
backup_uid = backup.uid
backup_cluster_name = backup.cluster_name
backup_expiry_quantity = backup.expiry_quantity
backup_etag = backup.etag
backup_expiry_time = backup.expiry_time
backup_tags = backup.tags
backup_update_time = backup.update_time
backup_delete_time = backup.delete_time
backup_encryption_config = backup.encryption_config
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.alloydb_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_name = operation.name
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
```

---


### User

Creates a new User in a given project, location, and cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |
| `database_roles` | Vec<String> |  | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `password` | String |  | Input only. Password for the user. |
| `user_type` | String |  | Optional. Type of this user. |
| `keep_extra_roles` | bool |  | Input only. If the user already exists and it has additional roles, keep them granted. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |
| `database_roles` | Vec<String> | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `password` | String | Input only. Password for the user. |
| `user_type` | String | Optional. Type of this user. |
| `keep_extra_roles` | bool | Input only. If the user already exists and it has additional roles, keep them granted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.alloydb_api.User {
    parent = "value"  # Required. Value for parent.
}

# Access user outputs
user_id = user.id
user_name = user.name
user_database_roles = user.database_roles
user_password = user.password
user_user_type = user.user_type
user_keep_extra_roles = user.keep_extra_roles
```

---


### Instance

Creates a new Instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `writable_node` | String |  | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `gce_zone` | String |  | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `name` | String |  | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `create_time` | String |  | Output only. Create time stamp |
| `read_pool_config` | String |  | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `update_time` | String |  | Output only. Update time stamp |
| `observability_config` | String |  | Configuration for observability. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `public_ip_address` | String |  | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `psc_instance_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `machine_config` | String |  | Configurations for the machines that host the underlying database engine. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `outbound_public_ip_addresses` | Vec<String> |  | Output only. All outbound public IP addresses configured for the instance. |
| `database_flags` | HashMap<String, String> |  | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `instance_type` | String |  | Required. The type of the instance. Specified at creation time. |
| `query_insights_config` | String |  | Configuration for query insights. |
| `client_connection_config` | String |  | Optional. Client connection specific configurations |
| `availability_type` | String |  | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `activation_policy` | String |  | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `connection_pool_config` | String |  | Optional. The configuration for Managed Connection Pool (MCP). |
| `state` | String |  | Output only. The current serving state of the instance. |
| `network_config` | String |  | Optional. Instance-level network configuration. |
| `display_name` | String |  | User-settable and human-readable display name for the Instance. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `nodes` | Vec<String> |  | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `ip_address` | String |  | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `parent` | String | ✅ | Required. The name of the parent resource. For the required format, see the comment on the Instance.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `writable_node` | String | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `gce_zone` | String | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `name` | String | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `create_time` | String | Output only. Create time stamp |
| `read_pool_config` | String | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `update_time` | String | Output only. Update time stamp |
| `observability_config` | String | Configuration for observability. |
| `delete_time` | String | Output only. Delete time stamp |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `public_ip_address` | String | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `psc_instance_config` | String | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `machine_config` | String | Configurations for the machines that host the underlying database engine. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `outbound_public_ip_addresses` | Vec<String> | Output only. All outbound public IP addresses configured for the instance. |
| `database_flags` | HashMap<String, String> | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `instance_type` | String | Required. The type of the instance. Specified at creation time. |
| `query_insights_config` | String | Configuration for query insights. |
| `client_connection_config` | String | Optional. Client connection specific configurations |
| `availability_type` | String | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `activation_policy` | String | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `connection_pool_config` | String | Optional. The configuration for Managed Connection Pool (MCP). |
| `state` | String | Output only. The current serving state of the instance. |
| `network_config` | String | Optional. Instance-level network configuration. |
| `display_name` | String | User-settable and human-readable display name for the Instance. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `nodes` | Vec<String> | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `ip_address` | String | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.alloydb_api.Instance {
    parent = "value"  # Required. The name of the parent resource. For the required format, see the comment on the Instance.name field.
}

# Access instance outputs
instance_id = instance.id
instance_writable_node = instance.writable_node
instance_gce_zone = instance.gce_zone
instance_name = instance.name
instance_create_time = instance.create_time
instance_read_pool_config = instance.read_pool_config
instance_labels = instance.labels
instance_update_time = instance.update_time
instance_observability_config = instance.observability_config
instance_delete_time = instance.delete_time
instance_etag = instance.etag
instance_public_ip_address = instance.public_ip_address
instance_psc_instance_config = instance.psc_instance_config
instance_machine_config = instance.machine_config
instance_reconciling = instance.reconciling
instance_outbound_public_ip_addresses = instance.outbound_public_ip_addresses
instance_database_flags = instance.database_flags
instance_instance_type = instance.instance_type
instance_query_insights_config = instance.query_insights_config
instance_client_connection_config = instance.client_connection_config
instance_availability_type = instance.availability_type
instance_uid = instance.uid
instance_satisfies_pzs = instance.satisfies_pzs
instance_activation_policy = instance.activation_policy
instance_connection_pool_config = instance.connection_pool_config
instance_state = instance.state
instance_network_config = instance.network_config
instance_display_name = instance.display_name
instance_annotations = instance.annotations
instance_nodes = instance.nodes
instance_ip_address = instance.ip_address
```

---


### Supported_database_flag

Lists SupportedDatabaseFlags for a given project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token identifying a page of results the server should return. |
| `supported_database_flags` | Vec<String> | The list of SupportedDatabaseFlags. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access supported_database_flag outputs
supported_database_flag_id = supported_database_flag.id
supported_database_flag_next_page_token = supported_database_flag.next_page_token
supported_database_flag_supported_database_flags = supported_database_flag.supported_database_flags
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
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.alloydb_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Backup

Creates a new Backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `size_bytes` | String |  | Output only. The size of the backup in bytes. |
| `expiry_quantity` | String |  | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `description` | String |  | User-provided description of the backup. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `cluster_name` | String |  | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `create_time` | String |  | Output only. Create time stamp |
| `display_name` | String |  | User-settable and human-readable display name for the Backup. |
| `encryption_info` | String |  | Output only. The encryption information for the backup. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `cluster_uid` | String |  | Output only. The system-generated UID of the cluster which was used to create this resource. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `name` | String |  | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `create_completion_time` | String |  | Output only. Timestamp when the resource finished being created. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `type` | String |  | The backup type, which suggests the trigger for the backup. |
| `update_time` | String |  | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `state` | String |  | Output only. The current state of the backup. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `database_version` | String |  | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `expiry_time` | String |  | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `size_bytes` | String | Output only. The size of the backup in bytes. |
| `expiry_quantity` | String | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `description` | String | User-provided description of the backup. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `cluster_name` | String | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `create_time` | String | Output only. Create time stamp |
| `display_name` | String | User-settable and human-readable display name for the Backup. |
| `encryption_info` | String | Output only. The encryption information for the backup. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `cluster_uid` | String | Output only. The system-generated UID of the cluster which was used to create this resource. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `name` | String | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `create_completion_time` | String | Output only. Timestamp when the resource finished being created. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `type` | String | The backup type, which suggests the trigger for the backup. |
| `update_time` | String | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `state` | String | Output only. The current state of the backup. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `database_version` | String | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `expiry_time` | String | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `delete_time` | String | Output only. Delete time stamp |
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.alloydb_api.Backup {
    parent = "value"  # Required. Value for parent.
}

# Access backup outputs
backup_id = backup.id
backup_size_bytes = backup.size_bytes
backup_expiry_quantity = backup.expiry_quantity
backup_description = backup.description
backup_labels = backup.labels
backup_satisfies_pzi = backup.satisfies_pzi
backup_satisfies_pzs = backup.satisfies_pzs
backup_cluster_name = backup.cluster_name
backup_create_time = backup.create_time
backup_display_name = backup.display_name
backup_encryption_info = backup.encryption_info
backup_annotations = backup.annotations
backup_cluster_uid = backup.cluster_uid
backup_etag = backup.etag
backup_name = backup.name
backup_create_completion_time = backup.create_completion_time
backup_reconciling = backup.reconciling
backup_uid = backup.uid
backup_type = backup.type
backup_update_time = backup.update_time
backup_state = backup.state
backup_tags = backup.tags
backup_database_version = backup.database_version
backup_expiry_time = backup.expiry_time
backup_delete_time = backup.delete_time
backup_encryption_config = backup.encryption_config
```

---


### Cluster

Creates a new Cluster in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `maintenance_version_selection_policy` | String |  | Input only. Policy to use to automatically select the maintenance version to which to update the cluster's instances. |
| `subscription_type` | String |  | Optional. Subscription type of the cluster. |
| `backupdr_backup_source` | String |  | Output only. Cluster created from a BackupDR backup. |
| `primary_config` | String |  | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `continuous_backup_info` | String |  | Output only. Continuous backup properties for this cluster. |
| `encryption_info` | String |  | Output only. The encryption information for the cluster. |
| `cloudsql_backup_run_source` | String |  | Output only. Cluster created from CloudSQL snapshot. |
| `dataplex_config` | String |  | Optional. Configuration for Dataplex integration. |
| `create_time` | String |  | Output only. Create time stamp |
| `database_version` | String |  | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `name` | String |  | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `network` | String |  | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `initial_user` | String |  | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `state` | String |  | Output only. The current serving state of the cluster. |
| `display_name` | String |  | User-settable and human-readable display name for the Cluster. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `backup_source` | String |  | Output only. Cluster created from backup. |
| `update_time` | String |  | Output only. Update time stamp |
| `migration_source` | String |  | Output only. Cluster created via DMS migration. |
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `gemini_config` | String |  | Optional. Deprecated and unused. This field will be removed in the near future. |
| `continuous_backup_config` | String |  | Optional. Continuous backup configuration for this cluster. |
| `psc_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `trial_metadata` | String |  | Output only. Metadata for free trial clusters |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `maintenance_schedule` | String |  | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `cluster_type` | String |  | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `maintenance_update_policy` | String |  | Optional. The maintenance update policy determines when to allow or deny updates. |
| `secondary_config` | String |  | Cross Region replication config specific to SECONDARY cluster. |
| `network_config` | String |  |  |
| `ssl_config` | String |  | SSL configuration for this AlloyDB cluster. |
| `backupdr_info` | String |  | Output only. Output only information about BackupDR protection for this cluster. |
| `service_account_email` | String |  | Output only. AlloyDB per-cluster service account. This service account is created per-cluster per-project, and is different from the per-project service account. The per-cluster service account naming format is subject to change. |
| `automated_backup_policy` | String |  | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `parent` | String | ✅ | Required. The location of the new cluster. For the required format, see the comment on the Cluster.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `maintenance_version_selection_policy` | String | Input only. Policy to use to automatically select the maintenance version to which to update the cluster's instances. |
| `subscription_type` | String | Optional. Subscription type of the cluster. |
| `backupdr_backup_source` | String | Output only. Cluster created from a BackupDR backup. |
| `primary_config` | String | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `continuous_backup_info` | String | Output only. Continuous backup properties for this cluster. |
| `encryption_info` | String | Output only. The encryption information for the cluster. |
| `cloudsql_backup_run_source` | String | Output only. Cluster created from CloudSQL snapshot. |
| `dataplex_config` | String | Optional. Configuration for Dataplex integration. |
| `create_time` | String | Output only. Create time stamp |
| `database_version` | String | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `delete_time` | String | Output only. Delete time stamp |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `name` | String | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `network` | String | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `initial_user` | String | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `state` | String | Output only. The current serving state of the cluster. |
| `display_name` | String | User-settable and human-readable display name for the Cluster. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `backup_source` | String | Output only. Cluster created from backup. |
| `update_time` | String | Output only. Update time stamp |
| `migration_source` | String | Output only. Cluster created via DMS migration. |
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `gemini_config` | String | Optional. Deprecated and unused. This field will be removed in the near future. |
| `continuous_backup_config` | String | Optional. Continuous backup configuration for this cluster. |
| `psc_config` | String | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `trial_metadata` | String | Output only. Metadata for free trial clusters |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `maintenance_schedule` | String | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `cluster_type` | String | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `maintenance_update_policy` | String | Optional. The maintenance update policy determines when to allow or deny updates. |
| `secondary_config` | String | Cross Region replication config specific to SECONDARY cluster. |
| `network_config` | String |  |
| `ssl_config` | String | SSL configuration for this AlloyDB cluster. |
| `backupdr_info` | String | Output only. Output only information about BackupDR protection for this cluster. |
| `service_account_email` | String | Output only. AlloyDB per-cluster service account. This service account is created per-cluster per-project, and is different from the per-project service account. The per-cluster service account naming format is subject to change. |
| `automated_backup_policy` | String | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.alloydb_api.Cluster {
    parent = "value"  # Required. The location of the new cluster. For the required format, see the comment on the Cluster.name field.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_maintenance_version_selection_policy = cluster.maintenance_version_selection_policy
cluster_subscription_type = cluster.subscription_type
cluster_backupdr_backup_source = cluster.backupdr_backup_source
cluster_primary_config = cluster.primary_config
cluster_annotations = cluster.annotations
cluster_continuous_backup_info = cluster.continuous_backup_info
cluster_encryption_info = cluster.encryption_info
cluster_cloudsql_backup_run_source = cluster.cloudsql_backup_run_source
cluster_dataplex_config = cluster.dataplex_config
cluster_create_time = cluster.create_time
cluster_database_version = cluster.database_version
cluster_uid = cluster.uid
cluster_delete_time = cluster.delete_time
cluster_reconciling = cluster.reconciling
cluster_name = cluster.name
cluster_network = cluster.network
cluster_initial_user = cluster.initial_user
cluster_state = cluster.state
cluster_display_name = cluster.display_name
cluster_satisfies_pzi = cluster.satisfies_pzi
cluster_labels = cluster.labels
cluster_backup_source = cluster.backup_source
cluster_update_time = cluster.update_time
cluster_migration_source = cluster.migration_source
cluster_encryption_config = cluster.encryption_config
cluster_gemini_config = cluster.gemini_config
cluster_continuous_backup_config = cluster.continuous_backup_config
cluster_psc_config = cluster.psc_config
cluster_tags = cluster.tags
cluster_trial_metadata = cluster.trial_metadata
cluster_etag = cluster.etag
cluster_maintenance_schedule = cluster.maintenance_schedule
cluster_cluster_type = cluster.cluster_type
cluster_maintenance_update_policy = cluster.maintenance_update_policy
cluster_secondary_config = cluster.secondary_config
cluster_network_config = cluster.network_config
cluster_ssl_config = cluster.ssl_config
cluster_backupdr_info = cluster.backupdr_info
cluster_service_account_email = cluster.service_account_email
cluster_automated_backup_policy = cluster.automated_backup_policy
```

---


### Supported_database_flag

Lists SupportedDatabaseFlags for a given project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `supported_database_flags` | Vec<String> | The list of SupportedDatabaseFlags. |
| `next_page_token` | String | A token identifying a page of results the server should return. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access supported_database_flag outputs
supported_database_flag_id = supported_database_flag.id
supported_database_flag_supported_database_flags = supported_database_flag.supported_database_flags
supported_database_flag_next_page_token = supported_database_flag.next_page_token
```

---


### Instance

Creates a new Instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `network_config` | String |  | Optional. Instance-level network configuration. |
| `client_connection_config` | String |  | Optional. Client connection specific configurations |
| `gca_config` | String |  | Output only. Configuration parameters related to Gemini Cloud Assist. |
| `observability_config` | String |  | Configuration for observability. |
| `ip_address` | String |  | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `outbound_public_ip_addresses` | Vec<String> |  | Output only. All outbound public IP addresses configured for the instance. |
| `database_flags` | HashMap<String, String> |  | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `read_pool_config` | String |  | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `name` | String |  | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `state` | String |  | Output only. The current serving state of the instance. |
| `psc_instance_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `public_ip_address` | String |  | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `display_name` | String |  | User-settable and human-readable display name for the Instance. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `writable_node` | String |  | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `update_policy` | String |  | Update policy that will be applied during instance update. This field is not persisted when you update the instance. To use a non-default update policy, you must specify explicitly specify the value in each update request. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `create_time` | String |  | Output only. Create time stamp |
| `machine_config` | String |  | Configurations for the machines that host the underlying database engine. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `update_time` | String |  | Output only. Update time stamp |
| `instance_type` | String |  | Required. The type of the instance. Specified at creation time. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `activation_policy` | String |  | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `gce_zone` | String |  | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `nodes` | Vec<String> |  | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `availability_type` | String |  | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `query_insights_config` | String |  | Configuration for query insights. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `gemini_config` | String |  | Optional. Deprecated and unused. This field will be removed in the near future. |
| `connection_pool_config` | String |  | Optional. The configuration for Managed Connection Pool (MCP). |
| `parent` | String | ✅ | Required. The name of the parent resource. For the required format, see the comment on the Instance.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `network_config` | String | Optional. Instance-level network configuration. |
| `client_connection_config` | String | Optional. Client connection specific configurations |
| `gca_config` | String | Output only. Configuration parameters related to Gemini Cloud Assist. |
| `observability_config` | String | Configuration for observability. |
| `ip_address` | String | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `outbound_public_ip_addresses` | Vec<String> | Output only. All outbound public IP addresses configured for the instance. |
| `database_flags` | HashMap<String, String> | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `read_pool_config` | String | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `name` | String | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `state` | String | Output only. The current serving state of the instance. |
| `psc_instance_config` | String | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `public_ip_address` | String | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `display_name` | String | User-settable and human-readable display name for the Instance. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `writable_node` | String | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `update_policy` | String | Update policy that will be applied during instance update. This field is not persisted when you update the instance. To use a non-default update policy, you must specify explicitly specify the value in each update request. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `create_time` | String | Output only. Create time stamp |
| `machine_config` | String | Configurations for the machines that host the underlying database engine. |
| `delete_time` | String | Output only. Delete time stamp |
| `update_time` | String | Output only. Update time stamp |
| `instance_type` | String | Required. The type of the instance. Specified at creation time. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `activation_policy` | String | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `gce_zone` | String | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `nodes` | Vec<String> | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `availability_type` | String | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `query_insights_config` | String | Configuration for query insights. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `gemini_config` | String | Optional. Deprecated and unused. This field will be removed in the near future. |
| `connection_pool_config` | String | Optional. The configuration for Managed Connection Pool (MCP). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.alloydb_api.Instance {
    parent = "value"  # Required. The name of the parent resource. For the required format, see the comment on the Instance.name field.
}

# Access instance outputs
instance_id = instance.id
instance_reconciling = instance.reconciling
instance_network_config = instance.network_config
instance_client_connection_config = instance.client_connection_config
instance_gca_config = instance.gca_config
instance_observability_config = instance.observability_config
instance_ip_address = instance.ip_address
instance_outbound_public_ip_addresses = instance.outbound_public_ip_addresses
instance_database_flags = instance.database_flags
instance_read_pool_config = instance.read_pool_config
instance_name = instance.name
instance_state = instance.state
instance_psc_instance_config = instance.psc_instance_config
instance_public_ip_address = instance.public_ip_address
instance_display_name = instance.display_name
instance_satisfies_pzs = instance.satisfies_pzs
instance_writable_node = instance.writable_node
instance_uid = instance.uid
instance_update_policy = instance.update_policy
instance_annotations = instance.annotations
instance_create_time = instance.create_time
instance_machine_config = instance.machine_config
instance_delete_time = instance.delete_time
instance_update_time = instance.update_time
instance_instance_type = instance.instance_type
instance_etag = instance.etag
instance_activation_policy = instance.activation_policy
instance_gce_zone = instance.gce_zone
instance_nodes = instance.nodes
instance_availability_type = instance.availability_type
instance_labels = instance.labels
instance_query_insights_config = instance.query_insights_config
instance_satisfies_pzi = instance.satisfies_pzi
instance_gemini_config = instance.gemini_config
instance_connection_pool_config = instance.connection_pool_config
```

---


### User

Creates a new User in a given project, location, and cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `password` | String |  | Input only. Password for the user. |
| `name` | String |  | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |
| `keep_extra_roles` | bool |  | Input only. If the user already exists and it has additional roles, keep them granted. |
| `database_roles` | Vec<String> |  | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `user_type` | String |  | Optional. Type of this user. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `password` | String | Input only. Password for the user. |
| `name` | String | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |
| `keep_extra_roles` | bool | Input only. If the user already exists and it has additional roles, keep them granted. |
| `database_roles` | Vec<String> | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `user_type` | String | Optional. Type of this user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.alloydb_api.User {
    parent = "value"  # Required. Value for parent.
}

# Access user outputs
user_id = user.id
user_password = user.password
user_name = user.name
user_keep_extra_roles = user.keep_extra_roles
user_database_roles = user.database_roles
user_user_type = user.user_type
```

---


### Instance

Creates a new Instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connection_pool_config` | String |  | Optional. The configuration for Managed Connection Pool (MCP). |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `machine_config` | String |  | Configurations for the machines that host the underlying database engine. |
| `gemini_config` | String |  | Optional. Deprecated and unused. This field will be removed in the near future. |
| `name` | String |  | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `delete_time` | String |  | Output only. Delete time stamp |
| `nodes` | Vec<String> |  | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `state` | String |  | Output only. The current serving state of the instance. |
| `writable_node` | String |  | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `client_connection_config` | String |  | Optional. Client connection specific configurations |
| `observability_config` | String |  | Configuration for observability. |
| `read_pool_config` | String |  | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `update_time` | String |  | Output only. Update time stamp |
| `instance_type` | String |  | Required. The type of the instance. Specified at creation time. |
| `psc_instance_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `gce_zone` | String |  | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `availability_type` | String |  | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `query_insights_config` | String |  | Configuration for query insights. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `ip_address` | String |  | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `update_policy` | String |  | Update policy that will be applied during instance update. This field is not persisted when you update the instance. To use a non-default update policy, you must specify explicitly specify the value in each update request. |
| `create_time` | String |  | Output only. Create time stamp |
| `network_config` | String |  | Optional. Instance-level network configuration. |
| `outbound_public_ip_addresses` | Vec<String> |  | Output only. All outbound public IP addresses configured for the instance. |
| `database_flags` | HashMap<String, String> |  | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `display_name` | String |  | User-settable and human-readable display name for the Instance. |
| `gca_config` | String |  | Output only. Configuration parameters related to Gemini Cloud Assist. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `public_ip_address` | String |  | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `activation_policy` | String |  | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `parent` | String | ✅ | Required. The name of the parent resource. For the required format, see the comment on the Instance.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connection_pool_config` | String | Optional. The configuration for Managed Connection Pool (MCP). |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `machine_config` | String | Configurations for the machines that host the underlying database engine. |
| `gemini_config` | String | Optional. Deprecated and unused. This field will be removed in the near future. |
| `name` | String | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `delete_time` | String | Output only. Delete time stamp |
| `nodes` | Vec<String> | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `state` | String | Output only. The current serving state of the instance. |
| `writable_node` | String | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `client_connection_config` | String | Optional. Client connection specific configurations |
| `observability_config` | String | Configuration for observability. |
| `read_pool_config` | String | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `update_time` | String | Output only. Update time stamp |
| `instance_type` | String | Required. The type of the instance. Specified at creation time. |
| `psc_instance_config` | String | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `gce_zone` | String | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `availability_type` | String | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `query_insights_config` | String | Configuration for query insights. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `ip_address` | String | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `update_policy` | String | Update policy that will be applied during instance update. This field is not persisted when you update the instance. To use a non-default update policy, you must specify explicitly specify the value in each update request. |
| `create_time` | String | Output only. Create time stamp |
| `network_config` | String | Optional. Instance-level network configuration. |
| `outbound_public_ip_addresses` | Vec<String> | Output only. All outbound public IP addresses configured for the instance. |
| `database_flags` | HashMap<String, String> | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `display_name` | String | User-settable and human-readable display name for the Instance. |
| `gca_config` | String | Output only. Configuration parameters related to Gemini Cloud Assist. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `public_ip_address` | String | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `activation_policy` | String | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `labels` | HashMap<String, String> | Labels as key value pairs |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.alloydb_api.Instance {
    parent = "value"  # Required. The name of the parent resource. For the required format, see the comment on the Instance.name field.
}

# Access instance outputs
instance_id = instance.id
instance_connection_pool_config = instance.connection_pool_config
instance_uid = instance.uid
instance_machine_config = instance.machine_config
instance_gemini_config = instance.gemini_config
instance_name = instance.name
instance_delete_time = instance.delete_time
instance_nodes = instance.nodes
instance_state = instance.state
instance_writable_node = instance.writable_node
instance_client_connection_config = instance.client_connection_config
instance_observability_config = instance.observability_config
instance_read_pool_config = instance.read_pool_config
instance_reconciling = instance.reconciling
instance_update_time = instance.update_time
instance_instance_type = instance.instance_type
instance_psc_instance_config = instance.psc_instance_config
instance_etag = instance.etag
instance_gce_zone = instance.gce_zone
instance_availability_type = instance.availability_type
instance_query_insights_config = instance.query_insights_config
instance_annotations = instance.annotations
instance_ip_address = instance.ip_address
instance_update_policy = instance.update_policy
instance_create_time = instance.create_time
instance_network_config = instance.network_config
instance_outbound_public_ip_addresses = instance.outbound_public_ip_addresses
instance_database_flags = instance.database_flags
instance_display_name = instance.display_name
instance_gca_config = instance.gca_config
instance_satisfies_pzs = instance.satisfies_pzs
instance_public_ip_address = instance.public_ip_address
instance_activation_policy = instance.activation_policy
instance_labels = instance.labels
```

---


### Backup

Creates a new Backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `name` | String |  | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `display_name` | String |  | User-settable and human-readable display name for the Backup. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `size_bytes` | String |  | Output only. The size of the backup in bytes. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `encryption_info` | String |  | Output only. The encryption information for the backup. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `type` | String |  | The backup type, which suggests the trigger for the backup. |
| `update_time` | String |  | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `expiry_quantity` | String |  | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `database_version` | String |  | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `cluster_name` | String |  | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `state` | String |  | Output only. The current state of the backup. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `expiry_time` | String |  | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `create_completion_time` | String |  | Output only. Timestamp when the resource finished being created. |
| `create_time` | String |  | Output only. Create time stamp |
| `description` | String |  | User-provided description of the backup. |
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `cluster_uid` | String |  | Output only. The system-generated UID of the cluster which was used to create this resource. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `name` | String | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `display_name` | String | User-settable and human-readable display name for the Backup. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `size_bytes` | String | Output only. The size of the backup in bytes. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `encryption_info` | String | Output only. The encryption information for the backup. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `type` | String | The backup type, which suggests the trigger for the backup. |
| `update_time` | String | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `expiry_quantity` | String | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `database_version` | String | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `cluster_name` | String | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `state` | String | Output only. The current state of the backup. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `delete_time` | String | Output only. Delete time stamp |
| `expiry_time` | String | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `create_completion_time` | String | Output only. Timestamp when the resource finished being created. |
| `create_time` | String | Output only. Create time stamp |
| `description` | String | User-provided description of the backup. |
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `cluster_uid` | String | Output only. The system-generated UID of the cluster which was used to create this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.alloydb_api.Backup {
    parent = "value"  # Required. Value for parent.
}

# Access backup outputs
backup_id = backup.id
backup_labels = backup.labels
backup_name = backup.name
backup_display_name = backup.display_name
backup_satisfies_pzs = backup.satisfies_pzs
backup_size_bytes = backup.size_bytes
backup_reconciling = backup.reconciling
backup_encryption_info = backup.encryption_info
backup_annotations = backup.annotations
backup_type = backup.type
backup_update_time = backup.update_time
backup_expiry_quantity = backup.expiry_quantity
backup_database_version = backup.database_version
backup_cluster_name = backup.cluster_name
backup_state = backup.state
backup_uid = backup.uid
backup_delete_time = backup.delete_time
backup_expiry_time = backup.expiry_time
backup_create_completion_time = backup.create_completion_time
backup_create_time = backup.create_time
backup_description = backup.description
backup_encryption_config = backup.encryption_config
backup_etag = backup.etag
backup_tags = backup.tags
backup_cluster_uid = backup.cluster_uid
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.alloydb_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
```

---


### User

Creates a new User in a given project, location, and cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `keep_extra_roles` | bool |  | Input only. If the user already exists and it has additional roles, keep them granted. |
| `database_roles` | Vec<String> |  | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `user_type` | String |  | Optional. Type of this user. |
| `password` | String |  | Input only. Password for the user. |
| `name` | String |  | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `keep_extra_roles` | bool | Input only. If the user already exists and it has additional roles, keep them granted. |
| `database_roles` | Vec<String> | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `user_type` | String | Optional. Type of this user. |
| `password` | String | Input only. Password for the user. |
| `name` | String | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.alloydb_api.User {
    parent = "value"  # Required. Value for parent.
}

# Access user outputs
user_id = user.id
user_keep_extra_roles = user.keep_extra_roles
user_database_roles = user.database_roles
user_user_type = user.user_type
user_password = user.password
user_name = user.name
```

---


### Supported_database_flag

Lists SupportedDatabaseFlags for a given project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `supported_database_flags` | Vec<String> | The list of SupportedDatabaseFlags. |
| `next_page_token` | String | A token identifying a page of results the server should return. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access supported_database_flag outputs
supported_database_flag_id = supported_database_flag.id
supported_database_flag_supported_database_flags = supported_database_flag.supported_database_flags
supported_database_flag_next_page_token = supported_database_flag.next_page_token
```

---


### Cluster

Creates a new Cluster in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backupdr_backup_source` | String |  | Output only. Cluster created from a BackupDR backup. |
| `network` | String |  | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `name` | String |  | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `cloudsql_backup_run_source` | String |  | Output only. Cluster created from CloudSQL snapshot. |
| `create_time` | String |  | Output only. Create time stamp |
| `display_name` | String |  | User-settable and human-readable display name for the Cluster. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `automated_backup_policy` | String |  | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `migration_source` | String |  | Output only. Cluster created via DMS migration. |
| `psc_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `service_account_email` | String |  | Output only. AlloyDB per-cluster service account. This service account is created per-cluster per-project, and is different from the per-project service account. The per-cluster service account naming format is subject to change. |
| `trial_metadata` | String |  | Output only. Metadata for free trial clusters |
| `primary_config` | String |  | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `initial_user` | String |  | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `maintenance_update_policy` | String |  | Optional. The maintenance update policy determines when to allow or deny updates. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `continuous_backup_config` | String |  | Optional. Continuous backup configuration for this cluster. |
| `backupdr_info` | String |  | Output only. Output only information about BackupDR protection for this cluster. |
| `backup_source` | String |  | Output only. Cluster created from backup. |
| `maintenance_version_selection_policy` | String |  | Input only. Policy to use to automatically select the maintenance version to which to update the cluster's instances. |
| `cluster_type` | String |  | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `ssl_config` | String |  | SSL configuration for this AlloyDB cluster. |
| `state` | String |  | Output only. The current serving state of the cluster. |
| `continuous_backup_info` | String |  | Output only. Continuous backup properties for this cluster. |
| `subscription_type` | String |  | Optional. Subscription type of the cluster. |
| `update_time` | String |  | Output only. Update time stamp |
| `database_version` | String |  | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `maintenance_schedule` | String |  | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `gemini_config` | String |  | Optional. Deprecated and unused. This field will be removed in the near future. |
| `encryption_info` | String |  | Output only. The encryption information for the cluster. |
| `dataplex_config` | String |  | Optional. Configuration for Dataplex integration. |
| `network_config` | String |  |  |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `secondary_config` | String |  | Cross Region replication config specific to SECONDARY cluster. |
| `parent` | String | ✅ | Required. The location of the new cluster. For the required format, see the comment on the Cluster.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backupdr_backup_source` | String | Output only. Cluster created from a BackupDR backup. |
| `network` | String | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `name` | String | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `cloudsql_backup_run_source` | String | Output only. Cluster created from CloudSQL snapshot. |
| `create_time` | String | Output only. Create time stamp |
| `display_name` | String | User-settable and human-readable display name for the Cluster. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `automated_backup_policy` | String | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `migration_source` | String | Output only. Cluster created via DMS migration. |
| `psc_config` | String | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `service_account_email` | String | Output only. AlloyDB per-cluster service account. This service account is created per-cluster per-project, and is different from the per-project service account. The per-cluster service account naming format is subject to change. |
| `trial_metadata` | String | Output only. Metadata for free trial clusters |
| `primary_config` | String | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `initial_user` | String | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `maintenance_update_policy` | String | Optional. The maintenance update policy determines when to allow or deny updates. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `continuous_backup_config` | String | Optional. Continuous backup configuration for this cluster. |
| `backupdr_info` | String | Output only. Output only information about BackupDR protection for this cluster. |
| `backup_source` | String | Output only. Cluster created from backup. |
| `maintenance_version_selection_policy` | String | Input only. Policy to use to automatically select the maintenance version to which to update the cluster's instances. |
| `cluster_type` | String | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `ssl_config` | String | SSL configuration for this AlloyDB cluster. |
| `state` | String | Output only. The current serving state of the cluster. |
| `continuous_backup_info` | String | Output only. Continuous backup properties for this cluster. |
| `subscription_type` | String | Optional. Subscription type of the cluster. |
| `update_time` | String | Output only. Update time stamp |
| `database_version` | String | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `delete_time` | String | Output only. Delete time stamp |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `maintenance_schedule` | String | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `gemini_config` | String | Optional. Deprecated and unused. This field will be removed in the near future. |
| `encryption_info` | String | Output only. The encryption information for the cluster. |
| `dataplex_config` | String | Optional. Configuration for Dataplex integration. |
| `network_config` | String |  |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `secondary_config` | String | Cross Region replication config specific to SECONDARY cluster. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.alloydb_api.Cluster {
    parent = "value"  # Required. The location of the new cluster. For the required format, see the comment on the Cluster.name field.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_backupdr_backup_source = cluster.backupdr_backup_source
cluster_network = cluster.network
cluster_encryption_config = cluster.encryption_config
cluster_tags = cluster.tags
cluster_name = cluster.name
cluster_cloudsql_backup_run_source = cluster.cloudsql_backup_run_source
cluster_create_time = cluster.create_time
cluster_display_name = cluster.display_name
cluster_etag = cluster.etag
cluster_automated_backup_policy = cluster.automated_backup_policy
cluster_migration_source = cluster.migration_source
cluster_psc_config = cluster.psc_config
cluster_service_account_email = cluster.service_account_email
cluster_trial_metadata = cluster.trial_metadata
cluster_primary_config = cluster.primary_config
cluster_initial_user = cluster.initial_user
cluster_maintenance_update_policy = cluster.maintenance_update_policy
cluster_labels = cluster.labels
cluster_uid = cluster.uid
cluster_continuous_backup_config = cluster.continuous_backup_config
cluster_backupdr_info = cluster.backupdr_info
cluster_backup_source = cluster.backup_source
cluster_maintenance_version_selection_policy = cluster.maintenance_version_selection_policy
cluster_cluster_type = cluster.cluster_type
cluster_ssl_config = cluster.ssl_config
cluster_state = cluster.state
cluster_continuous_backup_info = cluster.continuous_backup_info
cluster_subscription_type = cluster.subscription_type
cluster_update_time = cluster.update_time
cluster_database_version = cluster.database_version
cluster_delete_time = cluster.delete_time
cluster_annotations = cluster.annotations
cluster_maintenance_schedule = cluster.maintenance_schedule
cluster_gemini_config = cluster.gemini_config
cluster_encryption_info = cluster.encryption_info
cluster_dataplex_config = cluster.dataplex_config
cluster_network_config = cluster.network_config
cluster_reconciling = cluster.reconciling
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_secondary_config = cluster.secondary_config
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple location resources
location_0 = provider.alloydb_api.Location {
}
location_1 = provider.alloydb_api.Location {
}
location_2 = provider.alloydb_api.Location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.alloydb_api.Location {
    }
```

---

## Related Documentation

- [GCP Alloydb_api Documentation](https://cloud.google.com/alloydb_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
