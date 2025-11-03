# Alloydb_api Service



**Resources**: 21

---

## Overview

The alloydb_api service provides access to 21 resource types:

- [Cluster](#cluster) [CRUD]
- [Operation](#operation) [CRD]
- [Supported_database_flag](#supported_database_flag) [R]
- [Location](#location) [R]
- [Backup](#backup) [CRUD]
- [User](#user) [CRUD]
- [Instance](#instance) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Cluster](#cluster) [CRUD]
- [Backup](#backup) [CRUD]
- [Instance](#instance) [CRUD]
- [Supported_database_flag](#supported_database_flag) [R]
- [User](#user) [CRUD]
- [Location](#location) [R]
- [Cluster](#cluster) [CRUD]
- [Instance](#instance) [CRUD]
- [User](#user) [CRUD]
- [Supported_database_flag](#supported_database_flag) [R]
- [Backup](#backup) [CRUD]
- [Operation](#operation) [CRD]

---

## Resources


### Cluster

Creates a new Cluster in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `encryption_info` | String |  | Output only. The encryption information for the cluster. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `primary_config` | String |  | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `psc_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `continuous_backup_config` | String |  | Optional. Continuous backup configuration for this cluster. |
| `dataplex_config` | String |  | Optional. Configuration for Dataplex integration. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `delete_time` | String |  | Output only. Delete time stamp |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `continuous_backup_info` | String |  | Output only. Continuous backup properties for this cluster. |
| `network` | String |  | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `network_config` | String |  |  |
| `trial_metadata` | String |  | Output only. Metadata for free trial clusters |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `update_time` | String |  | Output only. Update time stamp |
| `maintenance_schedule` | String |  | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `state` | String |  | Output only. The current serving state of the cluster. |
| `backupdr_info` | String |  | Output only. Output only information about BackupDR protection for this cluster. |
| `subscription_type` | String |  | Optional. Subscription type of the cluster. |
| `secondary_config` | String |  | Cross Region replication config specific to SECONDARY cluster. |
| `automated_backup_policy` | String |  | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `database_version` | String |  | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `cluster_type` | String |  | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `maintenance_update_policy` | String |  | Optional. The maintenance update policy determines when to allow or deny updates. |
| `name` | String |  | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `display_name` | String |  | User-settable and human-readable display name for the Cluster. |
| `backupdr_backup_source` | String |  | Output only. Cluster created from a BackupDR backup. |
| `initial_user` | String |  | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `create_time` | String |  | Output only. Create time stamp |
| `migration_source` | String |  | Output only. Cluster created via DMS migration. |
| `ssl_config` | String |  | SSL configuration for this AlloyDB cluster. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `backup_source` | String |  | Output only. Cluster created from backup. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `cloudsql_backup_run_source` | String |  | Output only. Cluster created from CloudSQL snapshot. |
| `parent` | String | ✅ | Required. The location of the new cluster. For the required format, see the comment on the Cluster.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `encryption_info` | String | Output only. The encryption information for the cluster. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `primary_config` | String | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `psc_config` | String | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `continuous_backup_config` | String | Optional. Continuous backup configuration for this cluster. |
| `dataplex_config` | String | Optional. Configuration for Dataplex integration. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `delete_time` | String | Output only. Delete time stamp |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `continuous_backup_info` | String | Output only. Continuous backup properties for this cluster. |
| `network` | String | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `network_config` | String |  |
| `trial_metadata` | String | Output only. Metadata for free trial clusters |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `update_time` | String | Output only. Update time stamp |
| `maintenance_schedule` | String | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `state` | String | Output only. The current serving state of the cluster. |
| `backupdr_info` | String | Output only. Output only information about BackupDR protection for this cluster. |
| `subscription_type` | String | Optional. Subscription type of the cluster. |
| `secondary_config` | String | Cross Region replication config specific to SECONDARY cluster. |
| `automated_backup_policy` | String | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `database_version` | String | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `cluster_type` | String | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `maintenance_update_policy` | String | Optional. The maintenance update policy determines when to allow or deny updates. |
| `name` | String | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `display_name` | String | User-settable and human-readable display name for the Cluster. |
| `backupdr_backup_source` | String | Output only. Cluster created from a BackupDR backup. |
| `initial_user` | String | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `create_time` | String | Output only. Create time stamp |
| `migration_source` | String | Output only. Cluster created via DMS migration. |
| `ssl_config` | String | SSL configuration for this AlloyDB cluster. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `backup_source` | String | Output only. Cluster created from backup. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `cloudsql_backup_run_source` | String | Output only. Cluster created from CloudSQL snapshot. |


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
cluster_encryption_config = cluster.encryption_config
cluster_encryption_info = cluster.encryption_info
cluster_etag = cluster.etag
cluster_primary_config = cluster.primary_config
cluster_psc_config = cluster.psc_config
cluster_continuous_backup_config = cluster.continuous_backup_config
cluster_dataplex_config = cluster.dataplex_config
cluster_labels = cluster.labels
cluster_delete_time = cluster.delete_time
cluster_annotations = cluster.annotations
cluster_continuous_backup_info = cluster.continuous_backup_info
cluster_network = cluster.network
cluster_network_config = cluster.network_config
cluster_trial_metadata = cluster.trial_metadata
cluster_uid = cluster.uid
cluster_update_time = cluster.update_time
cluster_maintenance_schedule = cluster.maintenance_schedule
cluster_state = cluster.state
cluster_backupdr_info = cluster.backupdr_info
cluster_subscription_type = cluster.subscription_type
cluster_secondary_config = cluster.secondary_config
cluster_automated_backup_policy = cluster.automated_backup_policy
cluster_database_version = cluster.database_version
cluster_cluster_type = cluster.cluster_type
cluster_maintenance_update_policy = cluster.maintenance_update_policy
cluster_name = cluster.name
cluster_reconciling = cluster.reconciling
cluster_display_name = cluster.display_name
cluster_backupdr_backup_source = cluster.backupdr_backup_source
cluster_initial_user = cluster.initial_user
cluster_create_time = cluster.create_time
cluster_migration_source = cluster.migration_source
cluster_ssl_config = cluster.ssl_config
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_backup_source = cluster.backup_source
cluster_tags = cluster.tags
cluster_cloudsql_backup_run_source = cluster.cloudsql_backup_run_source
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.alloydb_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_done = operation.done
operation_response = operation.response
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_name = location.name
location_labels = location.labels
location_display_name = location.display_name
location_metadata = location.metadata
```

---


### Backup

Creates a new Backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `type` | String |  | The backup type, which suggests the trigger for the backup. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `expiry_quantity` | String |  | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `update_time` | String |  | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `description` | String |  | User-provided description of the backup. |
| `cluster_uid` | String |  | Output only. The system-generated UID of the cluster which was used to create this resource. |
| `display_name` | String |  | User-settable and human-readable display name for the Backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `database_version` | String |  | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `create_time` | String |  | Output only. Create time stamp |
| `create_completion_time` | String |  | Output only. Timestamp when the resource finished being created. |
| `encryption_info` | String |  | Output only. The encryption information for the backup. |
| `name` | String |  | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `size_bytes` | String |  | Output only. The size of the backup in bytes. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `expiry_time` | String |  | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `cluster_name` | String |  | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `type` | String | The backup type, which suggests the trigger for the backup. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `delete_time` | String | Output only. Delete time stamp |
| `expiry_quantity` | String | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `update_time` | String | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `description` | String | User-provided description of the backup. |
| `cluster_uid` | String | Output only. The system-generated UID of the cluster which was used to create this resource. |
| `display_name` | String | User-settable and human-readable display name for the Backup. |
| `state` | String | Output only. The current state of the backup. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `database_version` | String | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `create_time` | String | Output only. Create time stamp |
| `create_completion_time` | String | Output only. Timestamp when the resource finished being created. |
| `encryption_info` | String | Output only. The encryption information for the backup. |
| `name` | String | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `size_bytes` | String | Output only. The size of the backup in bytes. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `expiry_time` | String | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `cluster_name` | String | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
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
backup_labels = backup.labels
backup_annotations = backup.annotations
backup_type = backup.type
backup_satisfies_pzs = backup.satisfies_pzs
backup_delete_time = backup.delete_time
backup_expiry_quantity = backup.expiry_quantity
backup_update_time = backup.update_time
backup_description = backup.description
backup_cluster_uid = backup.cluster_uid
backup_display_name = backup.display_name
backup_state = backup.state
backup_etag = backup.etag
backup_reconciling = backup.reconciling
backup_database_version = backup.database_version
backup_create_time = backup.create_time
backup_create_completion_time = backup.create_completion_time
backup_encryption_info = backup.encryption_info
backup_name = backup.name
backup_tags = backup.tags
backup_size_bytes = backup.size_bytes
backup_uid = backup.uid
backup_expiry_time = backup.expiry_time
backup_cluster_name = backup.cluster_name
backup_encryption_config = backup.encryption_config
```

---


### User

Creates a new User in a given project, location, and cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `password` | String |  | Input only. Password for the user. |
| `database_roles` | Vec<String> |  | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `keep_extra_roles` | bool |  | Input only. If the user already exists and it has additional roles, keep them granted. |
| `user_type` | String |  | Optional. Type of this user. |
| `name` | String |  | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `password` | String | Input only. Password for the user. |
| `database_roles` | Vec<String> | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `keep_extra_roles` | bool | Input only. If the user already exists and it has additional roles, keep them granted. |
| `user_type` | String | Optional. Type of this user. |
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
user_password = user.password
user_database_roles = user.database_roles
user_keep_extra_roles = user.keep_extra_roles
user_user_type = user.user_type
user_name = user.name
```

---


### Instance

Creates a new Instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | User-settable and human-readable display name for the Instance. |
| `gce_zone` | String |  | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `ip_address` | String |  | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `activation_policy` | String |  | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `psc_instance_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `writable_node` | String |  | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `name` | String |  | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `state` | String |  | Output only. The current serving state of the instance. |
| `observability_config` | String |  | Configuration for observability. |
| `database_flags` | HashMap<String, String> |  | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `nodes` | Vec<String> |  | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `network_config` | String |  | Optional. Instance-level network configuration. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `connection_pool_config` | String |  | Optional. The configuration for Managed Connection Pool (MCP). |
| `machine_config` | String |  | Configurations for the machines that host the underlying database engine. |
| `public_ip_address` | String |  | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `read_pool_config` | String |  | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `outbound_public_ip_addresses` | Vec<String> |  | Output only. All outbound public IP addresses configured for the instance. |
| `query_insights_config` | String |  | Configuration for query insights. |
| `update_time` | String |  | Output only. Update time stamp |
| `instance_type` | String |  | Required. The type of the instance. Specified at creation time. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `availability_type` | String |  | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `create_time` | String |  | Output only. Create time stamp |
| `client_connection_config` | String |  | Optional. Client connection specific configurations |
| `parent` | String | ✅ | Required. The name of the parent resource. For the required format, see the comment on the Instance.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | User-settable and human-readable display name for the Instance. |
| `gce_zone` | String | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `delete_time` | String | Output only. Delete time stamp |
| `ip_address` | String | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `activation_policy` | String | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `psc_instance_config` | String | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `writable_node` | String | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `name` | String | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `state` | String | Output only. The current serving state of the instance. |
| `observability_config` | String | Configuration for observability. |
| `database_flags` | HashMap<String, String> | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `nodes` | Vec<String> | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `network_config` | String | Optional. Instance-level network configuration. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `connection_pool_config` | String | Optional. The configuration for Managed Connection Pool (MCP). |
| `machine_config` | String | Configurations for the machines that host the underlying database engine. |
| `public_ip_address` | String | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `read_pool_config` | String | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `outbound_public_ip_addresses` | Vec<String> | Output only. All outbound public IP addresses configured for the instance. |
| `query_insights_config` | String | Configuration for query insights. |
| `update_time` | String | Output only. Update time stamp |
| `instance_type` | String | Required. The type of the instance. Specified at creation time. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `availability_type` | String | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `create_time` | String | Output only. Create time stamp |
| `client_connection_config` | String | Optional. Client connection specific configurations |


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
instance_display_name = instance.display_name
instance_gce_zone = instance.gce_zone
instance_uid = instance.uid
instance_delete_time = instance.delete_time
instance_ip_address = instance.ip_address
instance_activation_policy = instance.activation_policy
instance_psc_instance_config = instance.psc_instance_config
instance_writable_node = instance.writable_node
instance_reconciling = instance.reconciling
instance_name = instance.name
instance_state = instance.state
instance_observability_config = instance.observability_config
instance_database_flags = instance.database_flags
instance_etag = instance.etag
instance_nodes = instance.nodes
instance_labels = instance.labels
instance_network_config = instance.network_config
instance_annotations = instance.annotations
instance_connection_pool_config = instance.connection_pool_config
instance_machine_config = instance.machine_config
instance_public_ip_address = instance.public_ip_address
instance_read_pool_config = instance.read_pool_config
instance_outbound_public_ip_addresses = instance.outbound_public_ip_addresses
instance_query_insights_config = instance.query_insights_config
instance_update_time = instance.update_time
instance_instance_type = instance.instance_type
instance_satisfies_pzs = instance.satisfies_pzs
instance_availability_type = instance.availability_type
instance_create_time = instance.create_time
instance_client_connection_config = instance.client_connection_config
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
operation = provider.alloydb_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_metadata = location.metadata
location_display_name = location.display_name
location_location_id = location.location_id
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
| `delete_time` | String |  | Output only. Delete time stamp |
| `update_time` | String |  | Output only. Update time stamp |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `network_config` | String |  |  |
| `create_time` | String |  | Output only. Create time stamp |
| `state` | String |  | Output only. The current serving state of the cluster. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `psc_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `encryption_info` | String |  | Output only. The encryption information for the cluster. |
| `maintenance_update_policy` | String |  | Optional. The maintenance update policy determines when to allow or deny updates. |
| `subscription_type` | String |  | Optional. Subscription type of the cluster. |
| `service_account_email` | String |  | Output only. AlloyDB per-cluster service account. This service account is created per-cluster per-project, and is different from the per-project service account. The per-cluster service account naming format is subject to change. |
| `cloudsql_backup_run_source` | String |  | Output only. Cluster created from CloudSQL snapshot. |
| `backup_source` | String |  | Output only. Cluster created from backup. |
| `trial_metadata` | String |  | Output only. Metadata for free trial clusters |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `gemini_config` | String |  | Optional. Deprecated and unused. This field will be removed in the near future. |
| `maintenance_version_selection_policy` | String |  | Input only. Policy to use to automatically select the maintenance version to which to update the cluster's instances. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `migration_source` | String |  | Output only. Cluster created via DMS migration. |
| `primary_config` | String |  | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `continuous_backup_info` | String |  | Output only. Continuous backup properties for this cluster. |
| `continuous_backup_config` | String |  | Optional. Continuous backup configuration for this cluster. |
| `display_name` | String |  | User-settable and human-readable display name for the Cluster. |
| `name` | String |  | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `database_version` | String |  | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `secondary_config` | String |  | Cross Region replication config specific to SECONDARY cluster. |
| `initial_user` | String |  | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `dataplex_config` | String |  | Optional. Configuration for Dataplex integration. |
| `automated_backup_policy` | String |  | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `network` | String |  | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `cluster_type` | String |  | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `backupdr_info` | String |  | Output only. Output only information about BackupDR protection for this cluster. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `backupdr_backup_source` | String |  | Output only. Cluster created from a BackupDR backup. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `ssl_config` | String |  | SSL configuration for this AlloyDB cluster. |
| `maintenance_schedule` | String |  | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `parent` | String | ✅ | Required. The location of the new cluster. For the required format, see the comment on the Cluster.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. Delete time stamp |
| `update_time` | String | Output only. Update time stamp |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `network_config` | String |  |
| `create_time` | String | Output only. Create time stamp |
| `state` | String | Output only. The current serving state of the cluster. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `psc_config` | String | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `encryption_info` | String | Output only. The encryption information for the cluster. |
| `maintenance_update_policy` | String | Optional. The maintenance update policy determines when to allow or deny updates. |
| `subscription_type` | String | Optional. Subscription type of the cluster. |
| `service_account_email` | String | Output only. AlloyDB per-cluster service account. This service account is created per-cluster per-project, and is different from the per-project service account. The per-cluster service account naming format is subject to change. |
| `cloudsql_backup_run_source` | String | Output only. Cluster created from CloudSQL snapshot. |
| `backup_source` | String | Output only. Cluster created from backup. |
| `trial_metadata` | String | Output only. Metadata for free trial clusters |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `gemini_config` | String | Optional. Deprecated and unused. This field will be removed in the near future. |
| `maintenance_version_selection_policy` | String | Input only. Policy to use to automatically select the maintenance version to which to update the cluster's instances. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `migration_source` | String | Output only. Cluster created via DMS migration. |
| `primary_config` | String | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `continuous_backup_info` | String | Output only. Continuous backup properties for this cluster. |
| `continuous_backup_config` | String | Optional. Continuous backup configuration for this cluster. |
| `display_name` | String | User-settable and human-readable display name for the Cluster. |
| `name` | String | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `database_version` | String | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `secondary_config` | String | Cross Region replication config specific to SECONDARY cluster. |
| `initial_user` | String | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `dataplex_config` | String | Optional. Configuration for Dataplex integration. |
| `automated_backup_policy` | String | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `network` | String | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `cluster_type` | String | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `backupdr_info` | String | Output only. Output only information about BackupDR protection for this cluster. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `backupdr_backup_source` | String | Output only. Cluster created from a BackupDR backup. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `ssl_config` | String | SSL configuration for this AlloyDB cluster. |
| `maintenance_schedule` | String | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |


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
cluster_delete_time = cluster.delete_time
cluster_update_time = cluster.update_time
cluster_annotations = cluster.annotations
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_network_config = cluster.network_config
cluster_create_time = cluster.create_time
cluster_state = cluster.state
cluster_tags = cluster.tags
cluster_psc_config = cluster.psc_config
cluster_encryption_config = cluster.encryption_config
cluster_encryption_info = cluster.encryption_info
cluster_maintenance_update_policy = cluster.maintenance_update_policy
cluster_subscription_type = cluster.subscription_type
cluster_service_account_email = cluster.service_account_email
cluster_cloudsql_backup_run_source = cluster.cloudsql_backup_run_source
cluster_backup_source = cluster.backup_source
cluster_trial_metadata = cluster.trial_metadata
cluster_uid = cluster.uid
cluster_gemini_config = cluster.gemini_config
cluster_maintenance_version_selection_policy = cluster.maintenance_version_selection_policy
cluster_etag = cluster.etag
cluster_migration_source = cluster.migration_source
cluster_primary_config = cluster.primary_config
cluster_continuous_backup_info = cluster.continuous_backup_info
cluster_continuous_backup_config = cluster.continuous_backup_config
cluster_display_name = cluster.display_name
cluster_name = cluster.name
cluster_reconciling = cluster.reconciling
cluster_database_version = cluster.database_version
cluster_secondary_config = cluster.secondary_config
cluster_initial_user = cluster.initial_user
cluster_dataplex_config = cluster.dataplex_config
cluster_automated_backup_policy = cluster.automated_backup_policy
cluster_network = cluster.network
cluster_cluster_type = cluster.cluster_type
cluster_backupdr_info = cluster.backupdr_info
cluster_labels = cluster.labels
cluster_backupdr_backup_source = cluster.backupdr_backup_source
cluster_satisfies_pzi = cluster.satisfies_pzi
cluster_ssl_config = cluster.ssl_config
cluster_maintenance_schedule = cluster.maintenance_schedule
```

---


### Backup

Creates a new Backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `create_time` | String |  | Output only. Create time stamp |
| `delete_time` | String |  | Output only. Delete time stamp |
| `name` | String |  | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `display_name` | String |  | User-settable and human-readable display name for the Backup. |
| `type` | String |  | The backup type, which suggests the trigger for the backup. |
| `update_time` | String |  | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `encryption_info` | String |  | Output only. The encryption information for the backup. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `create_completion_time` | String |  | Output only. Timestamp when the resource finished being created. |
| `state` | String |  | Output only. The current state of the backup. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `database_version` | String |  | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |
| `description` | String |  | User-provided description of the backup. |
| `cluster_name` | String |  | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `expiry_time` | String |  | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `size_bytes` | String |  | Output only. The size of the backup in bytes. |
| `expiry_quantity` | String |  | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `cluster_uid` | String |  | Output only. The system-generated UID of the cluster which was used to create this resource. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `create_time` | String | Output only. Create time stamp |
| `delete_time` | String | Output only. Delete time stamp |
| `name` | String | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `display_name` | String | User-settable and human-readable display name for the Backup. |
| `type` | String | The backup type, which suggests the trigger for the backup. |
| `update_time` | String | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `encryption_info` | String | Output only. The encryption information for the backup. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `create_completion_time` | String | Output only. Timestamp when the resource finished being created. |
| `state` | String | Output only. The current state of the backup. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `database_version` | String | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |
| `description` | String | User-provided description of the backup. |
| `cluster_name` | String | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `expiry_time` | String | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `size_bytes` | String | Output only. The size of the backup in bytes. |
| `expiry_quantity` | String | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
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
backup_etag = backup.etag
backup_uid = backup.uid
backup_create_time = backup.create_time
backup_delete_time = backup.delete_time
backup_name = backup.name
backup_display_name = backup.display_name
backup_type = backup.type
backup_update_time = backup.update_time
backup_tags = backup.tags
backup_encryption_info = backup.encryption_info
backup_annotations = backup.annotations
backup_create_completion_time = backup.create_completion_time
backup_state = backup.state
backup_satisfies_pzi = backup.satisfies_pzi
backup_database_version = backup.database_version
backup_encryption_config = backup.encryption_config
backup_description = backup.description
backup_cluster_name = backup.cluster_name
backup_expiry_time = backup.expiry_time
backup_labels = backup.labels
backup_reconciling = backup.reconciling
backup_satisfies_pzs = backup.satisfies_pzs
backup_size_bytes = backup.size_bytes
backup_expiry_quantity = backup.expiry_quantity
backup_cluster_uid = backup.cluster_uid
```

---


### Instance

Creates a new Instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. Delete time stamp |
| `network_config` | String |  | Optional. Instance-level network configuration. |
| `state` | String |  | Output only. The current serving state of the instance. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `read_pool_config` | String |  | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `gemini_config` | String |  | Optional. Deprecated and unused. This field will be removed in the near future. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `writable_node` | String |  | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `public_ip_address` | String |  | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `observability_config` | String |  | Configuration for observability. |
| `update_time` | String |  | Output only. Update time stamp |
| `query_insights_config` | String |  | Configuration for query insights. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `gca_config` | String |  | Output only. Configuration parameters related to Gemini Cloud Assist. |
| `name` | String |  | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `outbound_public_ip_addresses` | Vec<String> |  | Output only. All outbound public IP addresses configured for the instance. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_policy` | String |  | Update policy that will be applied during instance update. This field is not persisted when you update the instance. To use a non-default update policy, you must specify explicitly specify the value in each update request. |
| `nodes` | Vec<String> |  | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `database_flags` | HashMap<String, String> |  | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `connection_pool_config` | String |  | Optional. The configuration for Managed Connection Pool (MCP). |
| `availability_type` | String |  | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `activation_policy` | String |  | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `create_time` | String |  | Output only. Create time stamp |
| `instance_type` | String |  | Required. The type of the instance. Specified at creation time. |
| `ip_address` | String |  | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `machine_config` | String |  | Configurations for the machines that host the underlying database engine. |
| `gce_zone` | String |  | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `client_connection_config` | String |  | Optional. Client connection specific configurations |
| `display_name` | String |  | User-settable and human-readable display name for the Instance. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `psc_instance_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `parent` | String | ✅ | Required. The name of the parent resource. For the required format, see the comment on the Instance.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. Delete time stamp |
| `network_config` | String | Optional. Instance-level network configuration. |
| `state` | String | Output only. The current serving state of the instance. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `read_pool_config` | String | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `gemini_config` | String | Optional. Deprecated and unused. This field will be removed in the near future. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `writable_node` | String | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `public_ip_address` | String | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `observability_config` | String | Configuration for observability. |
| `update_time` | String | Output only. Update time stamp |
| `query_insights_config` | String | Configuration for query insights. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `gca_config` | String | Output only. Configuration parameters related to Gemini Cloud Assist. |
| `name` | String | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `outbound_public_ip_addresses` | Vec<String> | Output only. All outbound public IP addresses configured for the instance. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_policy` | String | Update policy that will be applied during instance update. This field is not persisted when you update the instance. To use a non-default update policy, you must specify explicitly specify the value in each update request. |
| `nodes` | Vec<String> | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `database_flags` | HashMap<String, String> | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `connection_pool_config` | String | Optional. The configuration for Managed Connection Pool (MCP). |
| `availability_type` | String | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `activation_policy` | String | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `create_time` | String | Output only. Create time stamp |
| `instance_type` | String | Required. The type of the instance. Specified at creation time. |
| `ip_address` | String | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `machine_config` | String | Configurations for the machines that host the underlying database engine. |
| `gce_zone` | String | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `client_connection_config` | String | Optional. Client connection specific configurations |
| `display_name` | String | User-settable and human-readable display name for the Instance. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `psc_instance_config` | String | Optional. The configuration for Private Service Connect (PSC) for the instance. |


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
instance_delete_time = instance.delete_time
instance_network_config = instance.network_config
instance_state = instance.state
instance_uid = instance.uid
instance_read_pool_config = instance.read_pool_config
instance_gemini_config = instance.gemini_config
instance_annotations = instance.annotations
instance_writable_node = instance.writable_node
instance_public_ip_address = instance.public_ip_address
instance_labels = instance.labels
instance_observability_config = instance.observability_config
instance_update_time = instance.update_time
instance_query_insights_config = instance.query_insights_config
instance_reconciling = instance.reconciling
instance_satisfies_pzi = instance.satisfies_pzi
instance_gca_config = instance.gca_config
instance_name = instance.name
instance_outbound_public_ip_addresses = instance.outbound_public_ip_addresses
instance_satisfies_pzs = instance.satisfies_pzs
instance_update_policy = instance.update_policy
instance_nodes = instance.nodes
instance_database_flags = instance.database_flags
instance_connection_pool_config = instance.connection_pool_config
instance_availability_type = instance.availability_type
instance_activation_policy = instance.activation_policy
instance_create_time = instance.create_time
instance_instance_type = instance.instance_type
instance_ip_address = instance.ip_address
instance_machine_config = instance.machine_config
instance_gce_zone = instance.gce_zone
instance_client_connection_config = instance.client_connection_config
instance_display_name = instance.display_name
instance_etag = instance.etag
instance_psc_instance_config = instance.psc_instance_config
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


### User

Creates a new User in a given project, location, and cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |
| `password` | String |  | Input only. Password for the user. |
| `database_roles` | Vec<String> |  | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `keep_extra_roles` | bool |  | Input only. If the user already exists and it has additional roles, keep them granted. |
| `user_type` | String |  | Optional. Type of this user. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |
| `password` | String | Input only. Password for the user. |
| `database_roles` | Vec<String> | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `keep_extra_roles` | bool | Input only. If the user already exists and it has additional roles, keep them granted. |
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
user_name = user.name
user_password = user.password
user_database_roles = user.database_roles
user_keep_extra_roles = user.keep_extra_roles
user_user_type = user.user_type
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_location_id = location.location_id
location_name = location.name
location_labels = location.labels
location_metadata = location.metadata
```

---


### Cluster

Creates a new Cluster in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_source` | String |  | Output only. Cluster created from backup. |
| `secondary_config` | String |  | Cross Region replication config specific to SECONDARY cluster. |
| `display_name` | String |  | User-settable and human-readable display name for the Cluster. |
| `ssl_config` | String |  | SSL configuration for this AlloyDB cluster. |
| `backupdr_info` | String |  | Output only. Output only information about BackupDR protection for this cluster. |
| `dataplex_config` | String |  | Optional. Configuration for Dataplex integration. |
| `service_account_email` | String |  | Output only. AlloyDB per-cluster service account. This service account is created per-cluster per-project, and is different from the per-project service account. The per-cluster service account naming format is subject to change. |
| `initial_user` | String |  | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `maintenance_version_selection_policy` | String |  | Input only. Policy to use to automatically select the maintenance version to which to update the cluster's instances. |
| `cloudsql_backup_run_source` | String |  | Output only. Cluster created from CloudSQL snapshot. |
| `cluster_type` | String |  | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `maintenance_update_policy` | String |  | Optional. The maintenance update policy determines when to allow or deny updates. |
| `database_version` | String |  | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `backupdr_backup_source` | String |  | Output only. Cluster created from a BackupDR backup. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `state` | String |  | Output only. The current serving state of the cluster. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `network` | String |  | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `trial_metadata` | String |  | Output only. Metadata for free trial clusters |
| `continuous_backup_info` | String |  | Output only. Continuous backup properties for this cluster. |
| `network_config` | String |  |  |
| `subscription_type` | String |  | Optional. Subscription type of the cluster. |
| `encryption_info` | String |  | Output only. The encryption information for the cluster. |
| `maintenance_schedule` | String |  | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `continuous_backup_config` | String |  | Optional. Continuous backup configuration for this cluster. |
| `create_time` | String |  | Output only. Create time stamp |
| `primary_config` | String |  | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `automated_backup_policy` | String |  | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `migration_source` | String |  | Output only. Cluster created via DMS migration. |
| `gemini_config` | String |  | Optional. Deprecated and unused. This field will be removed in the near future. |
| `name` | String |  | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `update_time` | String |  | Output only. Update time stamp |
| `psc_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the cluster. |
| `parent` | String | ✅ | Required. The location of the new cluster. For the required format, see the comment on the Cluster.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_source` | String | Output only. Cluster created from backup. |
| `secondary_config` | String | Cross Region replication config specific to SECONDARY cluster. |
| `display_name` | String | User-settable and human-readable display name for the Cluster. |
| `ssl_config` | String | SSL configuration for this AlloyDB cluster. |
| `backupdr_info` | String | Output only. Output only information about BackupDR protection for this cluster. |
| `dataplex_config` | String | Optional. Configuration for Dataplex integration. |
| `service_account_email` | String | Output only. AlloyDB per-cluster service account. This service account is created per-cluster per-project, and is different from the per-project service account. The per-cluster service account naming format is subject to change. |
| `initial_user` | String | Input only. Initial user to setup during cluster creation. Required. If used in `RestoreCluster` this is ignored. |
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data. |
| `maintenance_version_selection_policy` | String | Input only. Policy to use to automatically select the maintenance version to which to update the cluster's instances. |
| `cloudsql_backup_run_source` | String | Output only. Cluster created from CloudSQL snapshot. |
| `cluster_type` | String | Output only. The type of the cluster. This is an output-only field and it's populated at the Cluster creation time or the Cluster promotion time. The cluster type is determined by which RPC was used to create the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster` |
| `maintenance_update_policy` | String | Optional. The maintenance update policy determines when to allow or deny updates. |
| `database_version` | String | Optional. The database engine major version. This is an optional field and it is populated at the Cluster creation time. If a database version is not supplied at cluster creation time, then a default database version will be used. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `backupdr_backup_source` | String | Output only. Cluster created from a BackupDR backup. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `state` | String | Output only. The current serving state of the cluster. |
| `delete_time` | String | Output only. Delete time stamp |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `network` | String | Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: `projects/{project}/global/networks/{network_id}`. This is required to create a cluster. Deprecated, use network_config.network instead. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `trial_metadata` | String | Output only. Metadata for free trial clusters |
| `continuous_backup_info` | String | Output only. Continuous backup properties for this cluster. |
| `network_config` | String |  |
| `subscription_type` | String | Optional. Subscription type of the cluster. |
| `encryption_info` | String | Output only. The encryption information for the cluster. |
| `maintenance_schedule` | String | Output only. The maintenance schedule for the cluster, generated for a specific rollout if a maintenance window is set. |
| `continuous_backup_config` | String | Optional. Continuous backup configuration for this cluster. |
| `create_time` | String | Output only. Create time stamp |
| `primary_config` | String | Output only. Cross Region replication config specific to PRIMARY cluster. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `automated_backup_policy` | String | The automated backup policy for this cluster. If no policy is provided then the default policy will be used. If backups are supported for the cluster, the default policy takes one backup a day, has a backup window of 1 hour, and retains backups for 14 days. For more information on the defaults, consult the documentation for the message type. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `migration_source` | String | Output only. Cluster created via DMS migration. |
| `gemini_config` | String | Optional. Deprecated and unused. This field will be removed in the near future. |
| `name` | String | Output only. The name of the cluster resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id} where the cluster ID segment should satisfy the regex expression `[a-z0-9-]+`. For more details see https://google.aip.dev/122. The prefix of the cluster resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `update_time` | String | Output only. Update time stamp |
| `psc_config` | String | Optional. The configuration for Private Service Connect (PSC) for the cluster. |


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
cluster_backup_source = cluster.backup_source
cluster_secondary_config = cluster.secondary_config
cluster_display_name = cluster.display_name
cluster_ssl_config = cluster.ssl_config
cluster_backupdr_info = cluster.backupdr_info
cluster_dataplex_config = cluster.dataplex_config
cluster_service_account_email = cluster.service_account_email
cluster_initial_user = cluster.initial_user
cluster_encryption_config = cluster.encryption_config
cluster_maintenance_version_selection_policy = cluster.maintenance_version_selection_policy
cluster_cloudsql_backup_run_source = cluster.cloudsql_backup_run_source
cluster_cluster_type = cluster.cluster_type
cluster_maintenance_update_policy = cluster.maintenance_update_policy
cluster_database_version = cluster.database_version
cluster_reconciling = cluster.reconciling
cluster_backupdr_backup_source = cluster.backupdr_backup_source
cluster_tags = cluster.tags
cluster_state = cluster.state
cluster_delete_time = cluster.delete_time
cluster_etag = cluster.etag
cluster_network = cluster.network
cluster_satisfies_pzs = cluster.satisfies_pzs
cluster_trial_metadata = cluster.trial_metadata
cluster_continuous_backup_info = cluster.continuous_backup_info
cluster_network_config = cluster.network_config
cluster_subscription_type = cluster.subscription_type
cluster_encryption_info = cluster.encryption_info
cluster_maintenance_schedule = cluster.maintenance_schedule
cluster_continuous_backup_config = cluster.continuous_backup_config
cluster_create_time = cluster.create_time
cluster_primary_config = cluster.primary_config
cluster_uid = cluster.uid
cluster_annotations = cluster.annotations
cluster_automated_backup_policy = cluster.automated_backup_policy
cluster_labels = cluster.labels
cluster_migration_source = cluster.migration_source
cluster_gemini_config = cluster.gemini_config
cluster_name = cluster.name
cluster_update_time = cluster.update_time
cluster_psc_config = cluster.psc_config
```

---


### Instance

Creates a new Instance in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `update_time` | String |  | Output only. Update time stamp |
| `network_config` | String |  | Optional. Instance-level network configuration. |
| `availability_type` | String |  | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `create_time` | String |  | Output only. Create time stamp |
| `observability_config` | String |  | Configuration for observability. |
| `gca_config` | String |  | Output only. Configuration parameters related to Gemini Cloud Assist. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `read_pool_config` | String |  | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `client_connection_config` | String |  | Optional. Client connection specific configurations |
| `state` | String |  | Output only. The current serving state of the instance. |
| `ip_address` | String |  | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `psc_instance_config` | String |  | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `query_insights_config` | String |  | Configuration for query insights. |
| `display_name` | String |  | User-settable and human-readable display name for the Instance. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `gemini_config` | String |  | Optional. Deprecated and unused. This field will be removed in the near future. |
| `public_ip_address` | String |  | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `machine_config` | String |  | Configurations for the machines that host the underlying database engine. |
| `nodes` | Vec<String> |  | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `database_flags` | HashMap<String, String> |  | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `connection_pool_config` | String |  | Optional. The configuration for Managed Connection Pool (MCP). |
| `instance_type` | String |  | Required. The type of the instance. Specified at creation time. |
| `outbound_public_ip_addresses` | Vec<String> |  | Output only. All outbound public IP addresses configured for the instance. |
| `gce_zone` | String |  | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `activation_policy` | String |  | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `update_policy` | String |  | Update policy that will be applied during instance update. This field is not persisted when you update the instance. To use a non-default update policy, you must specify explicitly specify the value in each update request. |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `writable_node` | String |  | Output only. This is set for the read-write VM of the PRIMARY instance only. |
| `parent` | String | ✅ | Required. The name of the parent resource. For the required format, see the comment on the Instance.name field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The name of the instance resource with the format: * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id} where the cluster and instance ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the instance resource name is the name of the parent resource: * projects/{project}/locations/{region}/clusters/{cluster_id} |
| `update_time` | String | Output only. Update time stamp |
| `network_config` | String | Optional. Instance-level network configuration. |
| `availability_type` | String | Availability type of an Instance. If empty, defaults to REGIONAL for primary instances. For read pools, availability_type is always UNSPECIFIED. Instances in the read pools are evenly distributed across available zones within the region (i.e. read pools with more than one node will have a node in at least two zones). |
| `create_time` | String | Output only. Create time stamp |
| `observability_config` | String | Configuration for observability. |
| `gca_config` | String | Output only. Configuration parameters related to Gemini Cloud Assist. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `read_pool_config` | String | Read pool instance configuration. This is required if the value of instanceType is READ_POOL. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `client_connection_config` | String | Optional. Client connection specific configurations |
| `state` | String | Output only. The current serving state of the instance. |
| `ip_address` | String | Output only. The IP address for the Instance. This is the connection endpoint for an end-user application. |
| `psc_instance_config` | String | Optional. The configuration for Private Service Connect (PSC) for the instance. |
| `delete_time` | String | Output only. Delete time stamp |
| `query_insights_config` | String | Configuration for query insights. |
| `display_name` | String | User-settable and human-readable display name for the Instance. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `gemini_config` | String | Optional. Deprecated and unused. This field will be removed in the near future. |
| `public_ip_address` | String | Output only. The public IP addresses for the Instance. This is available ONLY when enable_public_ip is set. This is the connection endpoint for an end-user application. |
| `machine_config` | String | Configurations for the machines that host the underlying database engine. |
| `nodes` | Vec<String> | Output only. List of available read-only VMs in this instance, including the standby for a PRIMARY instance. |
| `database_flags` | HashMap<String, String> | Database flags. Set at the instance level. They are copied from the primary instance on secondary instance creation. Flags that have restrictions default to the value at primary instance on read instances during creation. Read instances can set new flags or override existing flags that are relevant for reads, for example, for enabling columnar cache on a read instance. Flags set on read instance might or might not be present on the primary instance. This is a list of "key": "value" pairs. "key": The name of the flag. These flags are passed at instance setup time, so include both server options and system variables for Postgres. Flags are specified with underscores, not hyphens. "value": The value of the flag. Booleans are set to **on** for true and **off** for false. This field must be omitted if the flag doesn't take a value. |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation). Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `connection_pool_config` | String | Optional. The configuration for Managed Connection Pool (MCP). |
| `instance_type` | String | Required. The type of the instance. Specified at creation time. |
| `outbound_public_ip_addresses` | Vec<String> | Output only. All outbound public IP addresses configured for the instance. |
| `gce_zone` | String | The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity. |
| `activation_policy` | String | Optional. Specifies whether an instance needs to spin up. Once the instance is active, the activation policy can be updated to the `NEVER` to stop the instance. Likewise, the activation policy can be updated to `ALWAYS` to start the instance. There are restrictions around when an instance can/cannot be activated (for example, a read pool instance should be stopped before stopping primary etc.). Please refer to the API documentation for more details. |
| `update_policy` | String | Update policy that will be applied during instance update. This field is not persisted when you update the instance. To use a non-default update policy, you must specify explicitly specify the value in each update request. |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `writable_node` | String | Output only. This is set for the read-write VM of the PRIMARY instance only. |


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
instance_name = instance.name
instance_update_time = instance.update_time
instance_network_config = instance.network_config
instance_availability_type = instance.availability_type
instance_create_time = instance.create_time
instance_observability_config = instance.observability_config
instance_gca_config = instance.gca_config
instance_uid = instance.uid
instance_read_pool_config = instance.read_pool_config
instance_satisfies_pzs = instance.satisfies_pzs
instance_client_connection_config = instance.client_connection_config
instance_state = instance.state
instance_ip_address = instance.ip_address
instance_psc_instance_config = instance.psc_instance_config
instance_delete_time = instance.delete_time
instance_query_insights_config = instance.query_insights_config
instance_display_name = instance.display_name
instance_etag = instance.etag
instance_gemini_config = instance.gemini_config
instance_public_ip_address = instance.public_ip_address
instance_machine_config = instance.machine_config
instance_nodes = instance.nodes
instance_database_flags = instance.database_flags
instance_reconciling = instance.reconciling
instance_labels = instance.labels
instance_connection_pool_config = instance.connection_pool_config
instance_instance_type = instance.instance_type
instance_outbound_public_ip_addresses = instance.outbound_public_ip_addresses
instance_gce_zone = instance.gce_zone
instance_activation_policy = instance.activation_policy
instance_update_policy = instance.update_policy
instance_annotations = instance.annotations
instance_writable_node = instance.writable_node
```

---


### User

Creates a new User in a given project, location, and cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_type` | String |  | Optional. Type of this user. |
| `database_roles` | Vec<String> |  | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `name` | String |  | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |
| `keep_extra_roles` | bool |  | Input only. If the user already exists and it has additional roles, keep them granted. |
| `password` | String |  | Input only. Password for the user. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_type` | String | Optional. Type of this user. |
| `database_roles` | Vec<String> | Optional. List of database roles this user has. The database role strings are subject to the PostgreSQL naming conventions. |
| `name` | String | Output only. Name of the resource in the form of projects/{project}/locations/{location}/cluster/{cluster}/users/{user}. |
| `keep_extra_roles` | bool | Input only. If the user already exists and it has additional roles, keep them granted. |
| `password` | String | Input only. Password for the user. |


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
user_user_type = user.user_type
user_database_roles = user.database_roles
user_name = user.name
user_keep_extra_roles = user.keep_extra_roles
user_password = user.password
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


### Backup

Creates a new Backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_config` | String |  | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |
| `etag` | String |  | For Resource freshness validation (https://google.aip.dev/154) |
| `expiry_quantity` | String |  | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `create_completion_time` | String |  | Output only. Timestamp when the resource finished being created. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `name` | String |  | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `reconciling` | bool |  | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `create_time` | String |  | Output only. Create time stamp |
| `annotations` | HashMap<String, String> |  | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `cluster_uid` | String |  | Output only. The system-generated UID of the cluster which was used to create this resource. |
| `expiry_time` | String |  | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `database_version` | String |  | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `delete_time` | String |  | Output only. Delete time stamp |
| `state` | String |  | Output only. The current state of the backup. |
| `display_name` | String |  | User-settable and human-readable display name for the Backup. |
| `description` | String |  | User-provided description of the backup. |
| `uid` | String |  | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `size_bytes` | String |  | Output only. The size of the backup in bytes. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `type` | String |  | The backup type, which suggests the trigger for the backup. |
| `update_time` | String |  | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `cluster_name` | String |  | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `encryption_info` | String |  | Output only. The encryption information for the backup. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_config` | String | Optional. The encryption config can be specified to encrypt the backup with a customer-managed encryption key (CMEK). When this field is not specified, the backup will then use default encryption scheme to protect the user data. |
| `etag` | String | For Resource freshness validation (https://google.aip.dev/154) |
| `expiry_quantity` | String | Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy. Once the expiry quantity is over retention, the backup is eligible to be garbage collected. |
| `create_completion_time` | String | Output only. Timestamp when the resource finished being created. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `name` | String | Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backup_id} where the cluster and backup ID segments should satisfy the regex expression `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`, e.g. 1-63 characters of lowercase letters, numbers, and dashes, starting with a letter, and ending with a letter or number. For more details see https://google.aip.dev/122. The prefix of the backup resource name is the name of the parent resource: * projects/{project}/locations/{region} |
| `reconciling` | bool | Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource. This can happen due to user-triggered updates or system actions like failover or maintenance. |
| `create_time` | String | Output only. Create time stamp |
| `annotations` | HashMap<String, String> | Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128 |
| `cluster_uid` | String | Output only. The system-generated UID of the cluster which was used to create this resource. |
| `expiry_time` | String | Output only. The time at which after the backup is eligible to be garbage collected. It is the duration specified by the backup's retention policy, added to the backup's create_time. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `database_version` | String | Output only. The database engine major version of the cluster this backup was created from. Any restored cluster created from this backup will have the same database version. |
| `delete_time` | String | Output only. Delete time stamp |
| `state` | String | Output only. The current state of the backup. |
| `display_name` | String | User-settable and human-readable display name for the Backup. |
| `description` | String | User-provided description of the backup. |
| `uid` | String | Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted. |
| `size_bytes` | String | Output only. The size of the backup in bytes. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: ``` "123/environment": "production", "123/costCenter": "marketing" ``` |
| `type` | String | The backup type, which suggests the trigger for the backup. |
| `update_time` | String | Output only. Update time stamp Users should not infer any meaning from this field. Its value is generally unrelated to the timing of the backup creation operation. |
| `cluster_name` | String | Required. The full resource name of the backup source cluster (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}). |
| `encryption_info` | String | Output only. The encryption information for the backup. |


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
backup_encryption_config = backup.encryption_config
backup_etag = backup.etag
backup_expiry_quantity = backup.expiry_quantity
backup_create_completion_time = backup.create_completion_time
backup_labels = backup.labels
backup_name = backup.name
backup_reconciling = backup.reconciling
backup_create_time = backup.create_time
backup_annotations = backup.annotations
backup_cluster_uid = backup.cluster_uid
backup_expiry_time = backup.expiry_time
backup_satisfies_pzs = backup.satisfies_pzs
backup_database_version = backup.database_version
backup_delete_time = backup.delete_time
backup_state = backup.state
backup_display_name = backup.display_name
backup_description = backup.description
backup_uid = backup.uid
backup_size_bytes = backup.size_bytes
backup_tags = backup.tags
backup_type = backup.type
backup_update_time = backup.update_time
backup_cluster_name = backup.cluster_name
backup_encryption_info = backup.encryption_info
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation_metadata = operation.metadata
operation_done = operation.done
operation_response = operation.response
operation_name = operation.name
operation_error = operation.error
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple cluster resources
cluster_0 = provider.alloydb_api.Cluster {
    parent = "value-0"
}
cluster_1 = provider.alloydb_api.Cluster {
    parent = "value-1"
}
cluster_2 = provider.alloydb_api.Cluster {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    cluster = provider.alloydb_api.Cluster {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Alloydb_api Documentation](https://cloud.google.com/alloydb_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
