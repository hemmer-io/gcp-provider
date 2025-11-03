# Spanner_api Service



**Resources**: 14

---

## Overview

The spanner_api service provides access to 14 resource types:

- [Backup_schedule](#backup_schedule) [CRUD]
- [Database_role](#database_role) [CR]
- [Database](#database) [CRUD]
- [Session](#session) [CRD]
- [Instance_partition](#instance_partition) [CRUD]
- [Instance](#instance) [CRUD]
- [Database_operation](#database_operation) [R]
- [Instance_config_operation](#instance_config_operation) [R]
- [Operation](#operation) [CRD]
- [Backup_operation](#backup_operation) [R]
- [Instance_config](#instance_config) [CRUD]
- [Scan](#scan) [R]
- [Instance_partition_operation](#instance_partition_operation) [R]
- [Backup](#backup) [CRUD]

---

## Resources


### Backup_schedule

Creates a new backup schedule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retention_duration` | String |  | Optional. The retention duration of a backup that must be at least 6 hours and at most 366 days. The backup is eligible to be automatically deleted once the retention period has elapsed. |
| `update_time` | String |  | Output only. The timestamp at which the schedule was last updated. If the schedule has never been updated, this field contains the timestamp when the schedule was first created. |
| `full_backup_spec` | String |  | The schedule creates only full backups. |
| `incremental_backup_spec` | String |  | The schedule creates incremental backup chains. |
| `encryption_config` | String |  | Optional. The encryption configuration that is used to encrypt the backup. If this field is not specified, the backup uses the same encryption configuration as the database. |
| `name` | String |  | Identifier. Output only for the CreateBackupSchedule operation. Required for the UpdateBackupSchedule operation. A globally unique identifier for the backup schedule which cannot be changed. Values are of the form `projects//instances//databases//backupSchedules/a-z*[a-z0-9]` The final segment of the name must be between 2 and 60 characters in length. |
| `spec` | String |  | Optional. The schedule specification based on which the backup creations are triggered. |
| `parent` | String | ✅ | Required. The name of the database that this backup schedule applies to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `retention_duration` | String | Optional. The retention duration of a backup that must be at least 6 hours and at most 366 days. The backup is eligible to be automatically deleted once the retention period has elapsed. |
| `update_time` | String | Output only. The timestamp at which the schedule was last updated. If the schedule has never been updated, this field contains the timestamp when the schedule was first created. |
| `full_backup_spec` | String | The schedule creates only full backups. |
| `incremental_backup_spec` | String | The schedule creates incremental backup chains. |
| `encryption_config` | String | Optional. The encryption configuration that is used to encrypt the backup. If this field is not specified, the backup uses the same encryption configuration as the database. |
| `name` | String | Identifier. Output only for the CreateBackupSchedule operation. Required for the UpdateBackupSchedule operation. A globally unique identifier for the backup schedule which cannot be changed. Values are of the form `projects//instances//databases//backupSchedules/a-z*[a-z0-9]` The final segment of the name must be between 2 and 60 characters in length. |
| `spec` | String | Optional. The schedule specification based on which the backup creations are triggered. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_schedule
backup_schedule = provider.spanner_api.Backup_schedule {
    parent = "value"  # Required. The name of the database that this backup schedule applies to.
}

# Access backup_schedule outputs
backup_schedule_id = backup_schedule.id
backup_schedule_retention_duration = backup_schedule.retention_duration
backup_schedule_update_time = backup_schedule.update_time
backup_schedule_full_backup_spec = backup_schedule.full_backup_spec
backup_schedule_incremental_backup_spec = backup_schedule.incremental_backup_spec
backup_schedule_encryption_config = backup_schedule.encryption_config
backup_schedule_name = backup_schedule.name
backup_schedule_spec = backup_schedule.spec
```

---


### Database_role

Returns permissions that the caller has on the specified database or backup resource. Attempting this RPC on a non-existent Cloud Spanner database will result in a NOT_FOUND error if the user has `spanner.databases.list` permission on the containing Cloud Spanner instance. Otherwise returns an empty set of permissions. Calling this method on a backup that does not exist will result in a NOT_FOUND error if the user has `spanner.backups.list` permission on the containing instance. Calling this method on a backup schedule that does not exist will result in a NOT_FOUND error if the user has `spanner.backupSchedules.list` permission on the containing database.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | REQUIRED: The set of permissions to check for 'resource'. Permissions with wildcards (such as '*', 'spanner.*', 'spanner.instances.*') are not allowed. |
| `resource` | String | ✅ | REQUIRED: The Cloud Spanner resource for which permissions are being tested. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for database resources. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `database_roles` | Vec<String> | Database roles that matched the request. |
| `next_page_token` | String | `next_page_token` can be sent in a subsequent ListDatabaseRoles call to fetch more of the matching roles. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database_role
database_role = provider.spanner_api.Database_role {
    resource = "value"  # REQUIRED: The Cloud Spanner resource for which permissions are being tested. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for database resources.
}

# Access database_role outputs
database_role_id = database_role.id
database_role_database_roles = database_role.database_roles
database_role_next_page_token = database_role.next_page_token
```

---


### Database

Creates a new Spanner database and starts to prepare it for serving. The returned long-running operation will have a name of the format `/operations/` and can be used to track preparation of the database. The metadata field type is CreateDatabaseMetadata. The response field type is Database, if successful.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_config` | String |  | Optional. The encryption configuration for the database. If this field is not specified, Cloud Spanner will encrypt/decrypt all data at rest using Google default encryption. |
| `extra_statements` | Vec<String> |  | Optional. A list of DDL statements to run inside the newly created database. Statements can create tables, indexes, etc. These statements execute atomically with the creation of the database: if there is an error in any statement, the database is not created. |
| `create_statement` | String |  | Required. A `CREATE DATABASE` statement, which specifies the ID of the new database. The database ID must conform to the regular expression `a-z*[a-z0-9]` and be between 2 and 30 characters in length. If the database ID is a reserved word or if it contains a hyphen, the database ID must be enclosed in backticks (`` ` ``). |
| `proto_descriptors` | String |  | Optional. Proto descriptors used by `CREATE/ALTER PROTO BUNDLE` statements in 'extra_statements'. Contains a protobuf-serialized [`google.protobuf.FileDescriptorSet`](https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto) descriptor set. To generate it, [install](https://grpc.io/docs/protoc-installation/) and run `protoc` with --include_imports and --descriptor_set_out. For example, to generate for moon/shot/app.proto, run ``` $protoc --proto_path=/app_path --proto_path=/lib_path \ --include_imports \ --descriptor_set_out=descriptors.data \ moon/shot/app.proto ``` For more details, see protobuffer [self description](https://developers.google.com/protocol-buffers/docs/techniques#self-description). |
| `database_dialect` | String |  | Optional. The dialect of the Cloud Spanner Database. |
| `parent` | String | ✅ | Required. The name of the instance that will serve the new database. Values are of the form `projects//instances/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. If exists, the time at which the database creation started. |
| `state` | String | Output only. The current database state. |
| `reconciling` | bool | Output only. If true, the database is being updated. If false, there are no ongoing update operations for the database. |
| `encryption_info` | Vec<String> | Output only. For databases that are using customer managed encryption, this field contains the encryption information for the database, such as all Cloud KMS key versions that are in use. The `encryption_status` field inside of each `EncryptionInfo` is not populated. For databases that are using Google default or other types of encryption, this field is empty. This field is propagated lazily from the backend. There might be a delay from when a key version is being used and when it appears in this field. |
| `restore_info` | String | Output only. Applicable only for restored databases. Contains information about the restore source. |
| `encryption_config` | String | Output only. For databases that are using customer managed encryption, this field contains the encryption configuration for the database. For databases that are using Google default or other types of encryption, this field is empty. |
| `earliest_version_time` | String | Output only. Earliest timestamp at which older versions of the data can be read. This value is continuously updated by Cloud Spanner and becomes stale the moment it is queried. If you are using this value to recover data, make sure to account for the time from the moment when the value is queried to the moment when you initiate the recovery. |
| `enable_drop_protection` | bool | Optional. Whether drop protection is enabled for this database. Defaults to false, if not set. For more details, please see how to [prevent accidental database deletion](https://cloud.google.com/spanner/docs/prevent-database-deletion). |
| `database_dialect` | String | Output only. The dialect of the Cloud Spanner Database. |
| `quorum_info` | String | Output only. Applicable only for databases that use dual-region instance configurations. Contains information about the quorum. |
| `version_retention_period` | String | Output only. The period in which Cloud Spanner retains all versions of data for the database. This is the same as the value of version_retention_period database option set using UpdateDatabaseDdl. Defaults to 1 hour, if not set. |
| `default_leader` | String | Output only. The read-write region which contains the database's leader replicas. This is the same as the value of default_leader database option set using DatabaseAdmin.CreateDatabase or DatabaseAdmin.UpdateDatabaseDdl. If not explicitly set, this is empty. |
| `name` | String | Required. The name of the database. Values are of the form `projects//instances//databases/`, where `` is as specified in the `CREATE DATABASE` statement. This name can be passed to other API methods to identify the database. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.spanner_api.Database {
    parent = "value"  # Required. The name of the instance that will serve the new database. Values are of the form `projects//instances/`.
}

# Access database outputs
database_id = database.id
database_create_time = database.create_time
database_state = database.state
database_reconciling = database.reconciling
database_encryption_info = database.encryption_info
database_restore_info = database.restore_info
database_encryption_config = database.encryption_config
database_earliest_version_time = database.earliest_version_time
database_enable_drop_protection = database.enable_drop_protection
database_database_dialect = database.database_dialect
database_quorum_info = database.quorum_info
database_version_retention_period = database.version_retention_period
database_default_leader = database.default_leader
database_name = database.name
```

---


### Session

Creates a new session. A session can be used to perform transactions that read and/or modify data in a Cloud Spanner database. Sessions are meant to be reused for many consecutive transactions. Sessions can only execute one transaction at a time. To execute multiple concurrent read-write/write-only transactions, create multiple sessions. Note that standalone reads and queries use a transaction internally, and count toward the one transaction limit. Active sessions use additional server resources, so it's a good idea to delete idle and unneeded sessions. Aside from explicit deletes, Cloud Spanner can delete sessions when no operations are sent for more than an hour. If a session is deleted, requests to it return `NOT_FOUND`. Idle sessions can be kept alive by sending a trivial SQL query periodically, for example, `"SELECT 1"`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session` | String |  | Required. The session to create. |
| `database` | String | ✅ | Required. The database in which the new session is created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | Metadata about the result set, such as row type information. |
| `stats` | String | Query plan and execution statistics for the SQL statement that produced this result set. These can be requested by setting ExecuteSqlRequest.query_mode. DML statements always produce stats containing the number of rows modified, unless executed using the ExecuteSqlRequest.QueryMode.PLAN ExecuteSqlRequest.query_mode. Other fields might or might not be populated, based on the ExecuteSqlRequest.query_mode. |
| `rows` | Vec<Vec<String>> | Each element in `rows` is a row whose format is defined by metadata.row_type. The ith element in each row matches the ith field in metadata.row_type. Elements are encoded based on type as described here. |
| `precommit_token` | String | Optional. A precommit token is included if the read-write transaction is on a multiplexed session. Pass the precommit token with the highest sequence number from this transaction attempt to the Commit request for this transaction. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session
session = provider.spanner_api.Session {
    database = "value"  # Required. The database in which the new session is created.
}

# Access session outputs
session_id = session.id
session_metadata = session.metadata
session_stats = session.stats
session_rows = session.rows
session_precommit_token = session.precommit_token
```

---


### Instance_partition

Creates an instance partition and begins preparing it to be used. The returned long-running operation can be used to track the progress of preparing the new instance partition. The instance partition name is assigned by the caller. If the named instance partition already exists, `CreateInstancePartition` returns `ALREADY_EXISTS`. Immediately upon completion of this request: * The instance partition is readable via the API, with all requested attributes but no allocated resources. Its state is `CREATING`. Until completion of the returned operation: * Cancelling the operation renders the instance partition immediately unreadable via the API. * The instance partition can be deleted. * All other attempts to modify the instance partition are rejected. Upon completion of the returned operation: * Billing for all successfully-allocated resources begins (some types may have lower than the requested levels). * Databases can start using this instance partition. * The instance partition's allocated resource levels are readable via the API. * The instance partition's state becomes `READY`. The returned long-running operation will have a name of the format `/operations/` and can be used to track creation of the instance partition. The metadata field type is CreateInstancePartitionMetadata. The response field type is InstancePartition, if successful.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_partition_id` | String |  | Required. The ID of the instance partition to create. Valid identifiers are of the form `a-z*[a-z0-9]` and must be between 2 and 64 characters in length. |
| `instance_partition` | String |  | Required. The instance partition to create. The instance_partition.name may be omitted, but if specified must be `/instancePartitions/`. |
| `parent` | String | ✅ | Required. The name of the instance in which to create the instance partition. Values are of the form `projects//instances/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `referencing_backups` | Vec<String> | Output only. Deprecated: This field is not populated. Output only. The names of the backups that reference this instance partition. Referencing backups should share the parent instance. The existence of any referencing backup prevents the instance partition from being deleted. |
| `referencing_databases` | Vec<String> | Output only. The names of the databases that reference this instance partition. Referencing databases should share the parent instance. The existence of any referencing database prevents the instance partition from being deleted. |
| `display_name` | String | Required. The descriptive name for this instance partition as it appears in UIs. Must be unique per project and between 4 and 30 characters in length. |
| `state` | String | Output only. The current instance partition state. |
| `processing_units` | i64 | The number of processing units allocated to this instance partition. Users can set the `processing_units` field to specify the target number of processing units allocated to the instance partition. This might be zero in API responses for instance partitions that are not yet in the `READY` state. |
| `name` | String | Required. A unique identifier for the instance partition. Values are of the form `projects//instances//instancePartitions/a-z*[a-z0-9]`. The final segment of the name must be between 2 and 64 characters in length. An instance partition's name cannot be changed after the instance partition is created. |
| `create_time` | String | Output only. The time at which the instance partition was created. |
| `update_time` | String | Output only. The time at which the instance partition was most recently updated. |
| `config` | String | Required. The name of the instance partition's configuration. Values are of the form `projects//instanceConfigs/`. See also InstanceConfig and ListInstanceConfigs. |
| `etag` | String | Used for optimistic concurrency control as a way to help prevent simultaneous updates of a instance partition from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform instance partition updates in order to avoid race conditions: An etag is returned in the response which contains instance partitions, and systems are expected to put that etag in the request to update instance partitions to ensure that their change will be applied to the same version of the instance partition. If no etag is provided in the call to update instance partition, then the existing instance partition is overwritten blindly. |
| `node_count` | i64 | The number of nodes allocated to this instance partition. Users can set the `node_count` field to specify the target number of nodes allocated to the instance partition. This may be zero in API responses for instance partitions that are not yet in state `READY`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance_partition
instance_partition = provider.spanner_api.Instance_partition {
    parent = "value"  # Required. The name of the instance in which to create the instance partition. Values are of the form `projects//instances/`.
}

# Access instance_partition outputs
instance_partition_id = instance_partition.id
instance_partition_referencing_backups = instance_partition.referencing_backups
instance_partition_referencing_databases = instance_partition.referencing_databases
instance_partition_display_name = instance_partition.display_name
instance_partition_state = instance_partition.state
instance_partition_processing_units = instance_partition.processing_units
instance_partition_name = instance_partition.name
instance_partition_create_time = instance_partition.create_time
instance_partition_update_time = instance_partition.update_time
instance_partition_config = instance_partition.config
instance_partition_etag = instance_partition.etag
instance_partition_node_count = instance_partition.node_count
```

---


### Instance

Creates an instance and begins preparing it to begin serving. The returned long-running operation can be used to track the progress of preparing the new instance. The instance name is assigned by the caller. If the named instance already exists, `CreateInstance` returns `ALREADY_EXISTS`. Immediately upon completion of this request: * The instance is readable via the API, with all requested attributes but no allocated resources. Its state is `CREATING`. Until completion of the returned operation: * Cancelling the operation renders the instance immediately unreadable via the API. * The instance can be deleted. * All other attempts to modify the instance are rejected. Upon completion of the returned operation: * Billing for all successfully-allocated resources begins (some types may have lower than the requested levels). * Databases can be created in the instance. * The instance's allocated resource levels are readable via the API. * The instance's state becomes `READY`. The returned long-running operation will have a name of the format `/operations/` and can be used to track creation of the instance. The metadata field type is CreateInstanceMetadata. The response field type is Instance, if successful.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance` | String |  | Required. The instance to create. The name may be omitted, but if specified must be `/instances/`. |
| `instance_id` | String |  | Required. The ID of the instance to create. Valid identifiers are of the form `a-z*[a-z0-9]` and must be between 2 and 64 characters in length. |
| `parent` | String | ✅ | Required. The name of the project in which to create the instance. Values are of the form `projects/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The descriptive name for this instance as it appears in UIs. Must be unique per project and between 4 and 30 characters in length. |
| `node_count` | i64 | The number of nodes allocated to this instance. At most, one of either `node_count` or `processing_units` should be present in the message. Users can set the `node_count` field to specify the target number of nodes allocated to the instance. If autoscaling is enabled, `node_count` is treated as an `OUTPUT_ONLY` field and reflects the current number of nodes allocated to the instance. This might be zero in API responses for instances that are not yet in the `READY` state. If the instance has varying node count across replicas (achieved by setting `asymmetric_autoscaling_options` in the autoscaling configuration), the `node_count` set here is the maximum node count across all replicas. For more information, see [Compute capacity, nodes, and processing units](https://cloud.google.com/spanner/docs/compute-capacity). |
| `config` | String | Required. The name of the instance's configuration. Values are of the form `projects//instanceConfigs/`. See also InstanceConfig and ListInstanceConfigs. |
| `free_instance_metadata` | String | Free instance metadata. Only populated for free instances. |
| `labels` | HashMap<String, String> | Cloud Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. Cloud Labels can be used to filter collections of resources. They can be used to control how resource metrics are aggregated. And they can be used as arguments to policy management rules (e.g. route, firewall, load balancing, etc.). * Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `a-z{0,62}`. * Label values must be between 0 and 63 characters long and must conform to the regular expression `[a-z0-9_-]{0,63}`. * No more than 64 labels can be associated with a given resource. See https://goo.gl/xmQnxf for more information on and examples of labels. If you plan to use labels in your own code, please note that additional characters may be allowed in the future. And so you are advised to use an internal label representation, such as JSON, which doesn't rely upon specific characters being disallowed. For example, representing labels as the string: name + "_" + value would prove problematic if we were to allow "_" in a future release. |
| `autoscaling_config` | String | Optional. The autoscaling configuration. Autoscaling is enabled if this field is set. When autoscaling is enabled, node_count and processing_units are treated as OUTPUT_ONLY fields and reflect the current compute capacity allocated to the instance. |
| `endpoint_uris` | Vec<String> | Deprecated. This field is not populated. |
| `processing_units` | i64 | The number of processing units allocated to this instance. At most, one of either `processing_units` or `node_count` should be present in the message. Users can set the `processing_units` field to specify the target number of processing units allocated to the instance. If autoscaling is enabled, `processing_units` is treated as an `OUTPUT_ONLY` field and reflects the current number of processing units allocated to the instance. This might be zero in API responses for instances that are not yet in the `READY` state. If the instance has varying processing units per replica (achieved by setting `asymmetric_autoscaling_options` in the autoscaling configuration), the `processing_units` set here is the maximum processing units across all replicas. For more information, see [Compute capacity, nodes and processing units](https://cloud.google.com/spanner/docs/compute-capacity). |
| `update_time` | String | Output only. The time at which the instance was most recently updated. |
| `name` | String | Required. A unique identifier for the instance, which cannot be changed after the instance is created. Values are of the form `projects//instances/a-z*[a-z0-9]`. The final segment of the name must be between 2 and 64 characters in length. |
| `edition` | String | Optional. The `Edition` of the current instance. |
| `state` | String | Output only. The current instance state. For CreateInstance, the state must be either omitted or set to `CREATING`. For UpdateInstance, the state must be either omitted or set to `READY`. |
| `create_time` | String | Output only. The time at which the instance was created. |
| `replica_compute_capacity` | Vec<String> | Output only. Lists the compute capacity per ReplicaSelection. A replica selection identifies a set of replicas with common properties. Replicas identified by a ReplicaSelection are scaled with the same compute capacity. |
| `default_backup_schedule_type` | String | Optional. Controls the default backup schedule behavior for new databases within the instance. By default, a backup schedule is created automatically when a new database is created in a new instance. Note that the `AUTOMATIC` value isn't permitted for free instances, as backups and backup schedules aren't supported for free instances. In the `GetInstance` or `ListInstances` response, if the value of `default_backup_schedule_type` isn't set, or set to `NONE`, Spanner doesn't create a default backup schedule for new databases in the instance. |
| `instance_type` | String | The `InstanceType` of the current instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.spanner_api.Instance {
    parent = "value"  # Required. The name of the project in which to create the instance. Values are of the form `projects/`.
}

# Access instance outputs
instance_id = instance.id
instance_display_name = instance.display_name
instance_node_count = instance.node_count
instance_config = instance.config
instance_free_instance_metadata = instance.free_instance_metadata
instance_labels = instance.labels
instance_autoscaling_config = instance.autoscaling_config
instance_endpoint_uris = instance.endpoint_uris
instance_processing_units = instance.processing_units
instance_update_time = instance.update_time
instance_name = instance.name
instance_edition = instance.edition
instance_state = instance.state
instance_create_time = instance.create_time
instance_replica_compute_capacity = instance.replica_compute_capacity
instance_default_backup_schedule_type = instance.default_backup_schedule_type
instance_instance_type = instance.instance_type
```

---


### Database_operation

Lists database longrunning-operations. A database operation has a name of the form `projects//instances//databases//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | `next_page_token` can be sent in a subsequent ListDatabaseOperations call to fetch more of the matching metadata. |
| `operations` | Vec<String> | The list of matching database long-running operations. Each operation's name will be prefixed by the database's name. The operation's metadata field type `metadata.type_url` describes the type of the metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access database_operation outputs
database_operation_id = database_operation.id
database_operation_next_page_token = database_operation.next_page_token
database_operation_operations = database_operation.operations
```

---


### Instance_config_operation

Lists the user-managed instance configuration long-running operations in the given project. An instance configuration operation has a name of the form `projects//instanceConfigs//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations. Operations returned are ordered by `operation.metadata.value.start_time` in descending order starting from the most recently started operation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `operations` | Vec<String> | The list of matching instance configuration long-running operations. Each operation's name will be prefixed by the name of the instance configuration. The operation's metadata field type `metadata.type_url` describes the type of the metadata. |
| `next_page_token` | String | `next_page_token` can be sent in a subsequent ListInstanceConfigOperations call to fetch more of the matching metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access instance_config_operation outputs
instance_config_operation_id = instance_config_operation.id
instance_config_operation_operations = instance_config_operation.operations
instance_config_operation_next_page_token = instance_config_operation.next_page_token
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.spanner_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_metadata = operation.metadata
operation_response = operation.response
operation_name = operation.name
operation_done = operation.done
```

---


### Backup_operation

Lists the backup long-running operations in the given instance. A backup operation has a name of the form `projects//instances//backups//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations. Operations returned are ordered by `operation.metadata.value.progress.start_time` in descending order starting from the most recently started operation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | `next_page_token` can be sent in a subsequent ListBackupOperations call to fetch more of the matching metadata. |
| `operations` | Vec<String> | The list of matching backup long-running operations. Each operation's name will be prefixed by the backup's name. The operation's metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that are pending or have completed/failed/canceled within the last 7 days. Operations returned are ordered by `operation.metadata.value.progress.start_time` in descending order starting from the most recently started operation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access backup_operation outputs
backup_operation_id = backup_operation.id
backup_operation_next_page_token = backup_operation.next_page_token
backup_operation_operations = backup_operation.operations
```

---


### Instance_config

Creates an instance configuration and begins preparing it to be used. The returned long-running operation can be used to track the progress of preparing the new instance configuration. The instance configuration name is assigned by the caller. If the named instance configuration already exists, `CreateInstanceConfig` returns `ALREADY_EXISTS`. Immediately after the request returns: * The instance configuration is readable via the API, with all requested attributes. The instance configuration's reconciling field is set to true. Its state is `CREATING`. While the operation is pending: * Cancelling the operation renders the instance configuration immediately unreadable via the API. * Except for deleting the creating resource, all other attempts to modify the instance configuration are rejected. Upon completion of the returned operation: * Instances can be created using the instance configuration. * The instance configuration's reconciling field becomes false. Its state becomes `READY`. The returned long-running operation will have a name of the format `/operations/` and can be used to track creation of the instance configuration. The metadata field type is CreateInstanceConfigMetadata. The response field type is InstanceConfig, if successful. Authorization requires `spanner.instanceConfigs.create` permission on the resource parent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_config_id` | String |  | Required. The ID of the instance configuration to create. Valid identifiers are of the form `custom-[-a-z0-9]*[a-z0-9]` and must be between 2 and 64 characters in length. The `custom-` prefix is required to avoid name conflicts with Google-managed configurations. |
| `validate_only` | bool |  | An option to validate, but not actually execute, a request, and provide the same response. |
| `instance_config` | String |  | Required. The `InstanceConfig` proto of the configuration to create. `instance_config.name` must be `/instanceConfigs/`. `instance_config.base_config` must be a Google-managed configuration name, e.g. /instanceConfigs/us-east1, /instanceConfigs/nam3. |
| `parent` | String | ✅ | Required. The name of the project in which to create the instance configuration. Values are of the form `projects/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `optional_replicas` | Vec<String> | Output only. The available optional replicas to choose from for user-managed configurations. Populated for Google-managed configurations. |
| `base_config` | String | Base configuration name, e.g. projects//instanceConfigs/nam3, based on which this configuration is created. Only set for user-managed configurations. `base_config` must refer to a configuration of type `GOOGLE_MANAGED` in the same project as this configuration. |
| `leader_options` | Vec<String> | Allowed values of the "default_leader" schema option for databases in instances that use this instance configuration. |
| `reconciling` | bool | Output only. If true, the instance configuration is being created or updated. If false, there are no ongoing operations for the instance configuration. |
| `quorum_type` | String | Output only. The `QuorumType` of the instance configuration. |
| `state` | String | Output only. The current instance configuration state. Applicable only for `USER_MANAGED` configurations. |
| `name` | String | A unique identifier for the instance configuration. Values are of the form `projects//instanceConfigs/a-z*`. User instance configuration must start with `custom-`. |
| `replicas` | Vec<String> | The geographic placement of nodes in this instance configuration and their replication properties. To create user-managed configurations, input `replicas` must include all replicas in `replicas` of the `base_config` and include one or more replicas in the `optional_replicas` of the `base_config`. |
| `config_type` | String | Output only. Whether this instance configuration is a Google-managed or user-managed configuration. |
| `labels` | HashMap<String, String> | Cloud Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. Cloud Labels can be used to filter collections of resources. They can be used to control how resource metrics are aggregated. And they can be used as arguments to policy management rules (e.g. route, firewall, load balancing, etc.). * Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `a-z{0,62}`. * Label values must be between 0 and 63 characters long and must conform to the regular expression `[a-z0-9_-]{0,63}`. * No more than 64 labels can be associated with a given resource. See https://goo.gl/xmQnxf for more information on and examples of labels. If you plan to use labels in your own code, please note that additional characters may be allowed in the future. Therefore, you are advised to use an internal label representation, such as JSON, which doesn't rely upon specific characters being disallowed. For example, representing labels as the string: name + "_" + value would prove problematic if we were to allow "_" in a future release. |
| `storage_limit_per_processing_unit` | String | Output only. The storage limit in bytes per processing unit. |
| `display_name` | String | The name of this instance configuration as it appears in UIs. |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a instance configuration from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform instance configuration updates in order to avoid race conditions: An etag is returned in the response which contains instance configurations, and systems are expected to put that etag in the request to update instance configuration to ensure that their change is applied to the same version of the instance configuration. If no etag is provided in the call to update the instance configuration, then the existing instance configuration is overwritten blindly. |
| `free_instance_availability` | String | Output only. Describes whether free instances are available to be created in this instance configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance_config
instance_config = provider.spanner_api.Instance_config {
    parent = "value"  # Required. The name of the project in which to create the instance configuration. Values are of the form `projects/`.
}

# Access instance_config outputs
instance_config_id = instance_config.id
instance_config_optional_replicas = instance_config.optional_replicas
instance_config_base_config = instance_config.base_config
instance_config_leader_options = instance_config.leader_options
instance_config_reconciling = instance_config.reconciling
instance_config_quorum_type = instance_config.quorum_type
instance_config_state = instance_config.state
instance_config_name = instance_config.name
instance_config_replicas = instance_config.replicas
instance_config_config_type = instance_config.config_type
instance_config_labels = instance_config.labels
instance_config_storage_limit_per_processing_unit = instance_config.storage_limit_per_processing_unit
instance_config_display_name = instance_config.display_name
instance_config_etag = instance_config.etag
instance_config_free_instance_availability = instance_config.free_instance_availability
```

---


### Scan

Return available scans given a Database-specific resource name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `scans` | Vec<String> | Available scans based on the list query parameters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access scan outputs
scan_id = scan.id
scan_next_page_token = scan.next_page_token
scan_scans = scan.scans
```

---


### Instance_partition_operation

Lists instance partition long-running operations in the given instance. An instance partition operation has a name of the form `projects//instances//instancePartitions//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations. Operations returned are ordered by `operation.metadata.value.start_time` in descending order starting from the most recently started operation. Authorization requires `spanner.instancePartitionOperations.list` permission on the resource parent.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `operations` | Vec<String> | The list of matching instance partition long-running operations. Each operation's name will be prefixed by the instance partition's name. The operation's metadata field type `metadata.type_url` describes the type of the metadata. |
| `next_page_token` | String | `next_page_token` can be sent in a subsequent ListInstancePartitionOperations call to fetch more of the matching metadata. |
| `unreachable_instance_partitions` | Vec<String> | The list of unreachable instance partitions. It includes the names of instance partitions whose operation metadata could not be retrieved within instance_partition_deadline. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access instance_partition_operation outputs
instance_partition_operation_id = instance_partition_operation.id
instance_partition_operation_operations = instance_partition_operation.operations
instance_partition_operation_next_page_token = instance_partition_operation.next_page_token
instance_partition_operation_unreachable_instance_partitions = instance_partition_operation.unreachable_instance_partitions
```

---


### Backup

Starts creating a new Cloud Spanner Backup. The returned backup long-running operation will have a name of the format `projects//instances//backups//operations/` and can be used to track creation of the backup. The metadata field type is CreateBackupMetadata. The response field type is Backup, if successful. Cancelling the returned operation will stop the creation and delete the backup. There can be only one pending backup creation per database. Backup creation of different databases can run concurrently.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `referencing_backups` | Vec<String> |  | Output only. The names of the destination backups being created by copying this source backup. The backup names are of the form `projects//instances//backups/`. Referencing backups may exist in different instances. The existence of any referencing backup prevents the backup from being deleted. When the copy operation is done (either successfully completed or cancelled or the destination backup is deleted), the reference to the backup is removed. |
| `incremental_backup_chain_id` | String |  | Output only. Populated only for backups in an incremental backup chain. Backups share the same chain id if and only if they belong to the same incremental backup chain. Use this field to determine which backups are part of the same incremental backup chain. The ordering of backups in the chain can be determined by ordering the backup `version_time`. |
| `oldest_version_time` | String |  | Output only. Data deleted at a time older than this is guaranteed not to be retained in order to support this backup. For a backup in an incremental backup chain, this is the version time of the oldest backup that exists or ever existed in the chain. For all other backups, this is the version time of the backup. This field can be used to understand what data is being retained by the backup system. |
| `backup_schedules` | Vec<String> |  | Output only. List of backup schedule URIs that are associated with creating this backup. This is only applicable for scheduled backups, and is empty for on-demand backups. To optimize for storage, whenever possible, multiple schedules are collapsed together to create one backup. In such cases, this field captures the list of all backup schedule URIs that are associated with creating this backup. If collapsing is not done, then this field captures the single backup schedule URI associated with creating this backup. |
| `max_expire_time` | String |  | Output only. The max allowed expiration time of the backup, with microseconds granularity. A backup's expiration time can be configured in multiple APIs: CreateBackup, UpdateBackup, CopyBackup. When updating or copying an existing backup, the expiration time specified must be less than `Backup.max_expire_time`. |
| `size_bytes` | String |  | Output only. Size of the backup in bytes. For a backup in an incremental backup chain, this is the sum of the `exclusive_size_bytes` of itself and all older backups in the chain. |
| `state` | String |  | Output only. The current state of the backup. |
| `expire_time` | String |  | Required for the CreateBackup operation. The expiration time of the backup, with microseconds granularity that must be at least 6 hours and at most 366 days from the time the CreateBackup request is processed. Once the `expire_time` has passed, the backup is eligible to be automatically deleted by Cloud Spanner to free the resources used by the backup. |
| `database_dialect` | String |  | Output only. The database dialect information for the backup. |
| `instance_partitions` | Vec<String> |  | Output only. The instance partition storing the backup. This is the same as the list of the instance partitions that the database recorded at the backup's `version_time`. |
| `encryption_information` | Vec<String> |  | Output only. The encryption information for the backup, whether it is protected by one or more KMS keys. The information includes all Cloud KMS key versions used to encrypt the backup. The `encryption_status` field inside of each `EncryptionInfo` is not populated. At least one of the key versions must be available for the backup to be restored. If a key version is revoked in the middle of a restore, the restore behavior is undefined. |
| `version_time` | String |  | The backup will contain an externally consistent copy of the database at the timestamp specified by `version_time`. If `version_time` is not specified, the system will set `version_time` to the `create_time` of the backup. |
| `exclusive_size_bytes` | String |  | Output only. For a backup in an incremental backup chain, this is the storage space needed to keep the data that has changed since the previous backup. For all other backups, this is always the size of the backup. This value may change if backups on the same chain get deleted or expired. This field can be used to calculate the total storage space used by a set of backups. For example, the total space used by all backups of a database can be computed by summing up this field. |
| `create_time` | String |  | Output only. The time the CreateBackup request is received. If the request does not specify `version_time`, the `version_time` of the backup will be equivalent to the `create_time`. |
| `referencing_databases` | Vec<String> |  | Output only. The names of the restored databases that reference the backup. The database names are of the form `projects//instances//databases/`. Referencing databases may exist in different instances. The existence of any referencing database prevents the backup from being deleted. When a restored database from the backup enters the `READY` state, the reference to the backup is removed. |
| `freeable_size_bytes` | String |  | Output only. The number of bytes that will be freed by deleting this backup. This value will be zero if, for example, this backup is part of an incremental backup chain and younger backups in the chain require that we keep its data. For backups not in an incremental backup chain, this is always the size of the backup. This value may change if backups on the same chain get created, deleted or expired. |
| `name` | String |  | Output only for the CreateBackup operation. Required for the UpdateBackup operation. A globally unique identifier for the backup which cannot be changed. Values are of the form `projects//instances//backups/a-z*[a-z0-9]` The final segment of the name must be between 2 and 60 characters in length. The backup is stored in the location(s) specified in the instance configuration of the instance containing the backup, identified by the prefix of the backup name of the form `projects//instances/`. |
| `encryption_info` | String |  | Output only. The encryption information for the backup. |
| `database` | String |  | Required for the CreateBackup operation. Name of the database from which this backup was created. This needs to be in the same instance as the backup. Values are of the form `projects//instances//databases/`. |
| `parent` | String | ✅ | Required. The name of the instance in which the backup is created. This must be the same instance that contains the database the backup is created from. The backup will be stored in the locations specified in the instance configuration of this instance. Values are of the form `projects//instances/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `referencing_backups` | Vec<String> | Output only. The names of the destination backups being created by copying this source backup. The backup names are of the form `projects//instances//backups/`. Referencing backups may exist in different instances. The existence of any referencing backup prevents the backup from being deleted. When the copy operation is done (either successfully completed or cancelled or the destination backup is deleted), the reference to the backup is removed. |
| `incremental_backup_chain_id` | String | Output only. Populated only for backups in an incremental backup chain. Backups share the same chain id if and only if they belong to the same incremental backup chain. Use this field to determine which backups are part of the same incremental backup chain. The ordering of backups in the chain can be determined by ordering the backup `version_time`. |
| `oldest_version_time` | String | Output only. Data deleted at a time older than this is guaranteed not to be retained in order to support this backup. For a backup in an incremental backup chain, this is the version time of the oldest backup that exists or ever existed in the chain. For all other backups, this is the version time of the backup. This field can be used to understand what data is being retained by the backup system. |
| `backup_schedules` | Vec<String> | Output only. List of backup schedule URIs that are associated with creating this backup. This is only applicable for scheduled backups, and is empty for on-demand backups. To optimize for storage, whenever possible, multiple schedules are collapsed together to create one backup. In such cases, this field captures the list of all backup schedule URIs that are associated with creating this backup. If collapsing is not done, then this field captures the single backup schedule URI associated with creating this backup. |
| `max_expire_time` | String | Output only. The max allowed expiration time of the backup, with microseconds granularity. A backup's expiration time can be configured in multiple APIs: CreateBackup, UpdateBackup, CopyBackup. When updating or copying an existing backup, the expiration time specified must be less than `Backup.max_expire_time`. |
| `size_bytes` | String | Output only. Size of the backup in bytes. For a backup in an incremental backup chain, this is the sum of the `exclusive_size_bytes` of itself and all older backups in the chain. |
| `state` | String | Output only. The current state of the backup. |
| `expire_time` | String | Required for the CreateBackup operation. The expiration time of the backup, with microseconds granularity that must be at least 6 hours and at most 366 days from the time the CreateBackup request is processed. Once the `expire_time` has passed, the backup is eligible to be automatically deleted by Cloud Spanner to free the resources used by the backup. |
| `database_dialect` | String | Output only. The database dialect information for the backup. |
| `instance_partitions` | Vec<String> | Output only. The instance partition storing the backup. This is the same as the list of the instance partitions that the database recorded at the backup's `version_time`. |
| `encryption_information` | Vec<String> | Output only. The encryption information for the backup, whether it is protected by one or more KMS keys. The information includes all Cloud KMS key versions used to encrypt the backup. The `encryption_status` field inside of each `EncryptionInfo` is not populated. At least one of the key versions must be available for the backup to be restored. If a key version is revoked in the middle of a restore, the restore behavior is undefined. |
| `version_time` | String | The backup will contain an externally consistent copy of the database at the timestamp specified by `version_time`. If `version_time` is not specified, the system will set `version_time` to the `create_time` of the backup. |
| `exclusive_size_bytes` | String | Output only. For a backup in an incremental backup chain, this is the storage space needed to keep the data that has changed since the previous backup. For all other backups, this is always the size of the backup. This value may change if backups on the same chain get deleted or expired. This field can be used to calculate the total storage space used by a set of backups. For example, the total space used by all backups of a database can be computed by summing up this field. |
| `create_time` | String | Output only. The time the CreateBackup request is received. If the request does not specify `version_time`, the `version_time` of the backup will be equivalent to the `create_time`. |
| `referencing_databases` | Vec<String> | Output only. The names of the restored databases that reference the backup. The database names are of the form `projects//instances//databases/`. Referencing databases may exist in different instances. The existence of any referencing database prevents the backup from being deleted. When a restored database from the backup enters the `READY` state, the reference to the backup is removed. |
| `freeable_size_bytes` | String | Output only. The number of bytes that will be freed by deleting this backup. This value will be zero if, for example, this backup is part of an incremental backup chain and younger backups in the chain require that we keep its data. For backups not in an incremental backup chain, this is always the size of the backup. This value may change if backups on the same chain get created, deleted or expired. |
| `name` | String | Output only for the CreateBackup operation. Required for the UpdateBackup operation. A globally unique identifier for the backup which cannot be changed. Values are of the form `projects//instances//backups/a-z*[a-z0-9]` The final segment of the name must be between 2 and 60 characters in length. The backup is stored in the location(s) specified in the instance configuration of the instance containing the backup, identified by the prefix of the backup name of the form `projects//instances/`. |
| `encryption_info` | String | Output only. The encryption information for the backup. |
| `database` | String | Required for the CreateBackup operation. Name of the database from which this backup was created. This needs to be in the same instance as the backup. Values are of the form `projects//instances//databases/`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.spanner_api.Backup {
    parent = "value"  # Required. The name of the instance in which the backup is created. This must be the same instance that contains the database the backup is created from. The backup will be stored in the locations specified in the instance configuration of this instance. Values are of the form `projects//instances/`.
}

# Access backup outputs
backup_id = backup.id
backup_referencing_backups = backup.referencing_backups
backup_incremental_backup_chain_id = backup.incremental_backup_chain_id
backup_oldest_version_time = backup.oldest_version_time
backup_backup_schedules = backup.backup_schedules
backup_max_expire_time = backup.max_expire_time
backup_size_bytes = backup.size_bytes
backup_state = backup.state
backup_expire_time = backup.expire_time
backup_database_dialect = backup.database_dialect
backup_instance_partitions = backup.instance_partitions
backup_encryption_information = backup.encryption_information
backup_version_time = backup.version_time
backup_exclusive_size_bytes = backup.exclusive_size_bytes
backup_create_time = backup.create_time
backup_referencing_databases = backup.referencing_databases
backup_freeable_size_bytes = backup.freeable_size_bytes
backup_name = backup.name
backup_encryption_info = backup.encryption_info
backup_database = backup.database
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple backup_schedule resources
backup_schedule_0 = provider.spanner_api.Backup_schedule {
    parent = "value-0"
}
backup_schedule_1 = provider.spanner_api.Backup_schedule {
    parent = "value-1"
}
backup_schedule_2 = provider.spanner_api.Backup_schedule {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    backup_schedule = provider.spanner_api.Backup_schedule {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Spanner_api Documentation](https://cloud.google.com/spanner_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
