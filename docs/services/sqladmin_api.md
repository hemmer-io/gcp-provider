# Sqladmin_api Service



**Resources**: 20

---

## Overview

The sqladmin_api service provides access to 20 resource types:

- [User](#user) [CRUD]
- [Tier](#tier) [R]
- [Database](#database) [CRUD]
- [Backup_run](#backup_run) [CRD]
- [Backup](#backup) [CRUD]
- [Operation](#operation) [CR]
- [Flag](#flag) [R]
- [Instance](#instance) [CRUD]
- [Ssl_cert](#ssl_cert) [CRD]
- [Connect](#connect) [CR]
- [Operation](#operation) [CR]
- [Ssl_cert](#ssl_cert) [CRD]
- [User](#user) [CRUD]
- [Backup_run](#backup_run) [CRD]
- [Instance](#instance) [CRUD]
- [Database](#database) [CRUD]
- [Connect](#connect) [CR]
- [Tier](#tier) [R]
- [Backup](#backup) [CRUD]
- [Flag](#flag) [R]

---

## Resources


### User

Creates a new user in a Cloud SQL instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | The user type. It determines the method to authenticate the user during login. The default is the database's built-in user type. |
| `host` | String |  | Optional. The host from which the user can connect. For `insert` operations, host defaults to an empty string. For `update` operations, host is specified as part of the request URL. The host name cannot be updated after insertion. For a MySQL instance, it's required; for a PostgreSQL or SQL Server instance, it's optional. |
| `instance` | String |  | The name of the Cloud SQL instance. This does not include the project ID. Can be omitted for *update* because it is already specified on the URL. |
| `kind` | String |  | This is always `sql#user`. |
| `iam_status` | String |  | Indicates if a group is active or inactive for IAM database authentication. |
| `etag` | String |  | This field is deprecated and will be removed from a future version of the API. |
| `sqlserver_user_details` | String |  |  |
| `password` | String |  | The password for the user. |
| `project` | String |  | The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable. Can be omitted for *update* because it is already specified on the URL. |
| `iam_email` | String |  | Optional. The full email for an IAM user. For normal database users, this will not be filled. Only applicable to MySQL database users. |
| `name` | String |  | The name of the user in the Cloud SQL instance. Can be omitted for `update` because it is already specified in the URL. |
| `password_policy` | String |  | User level password validation policy. |
| `dual_password_type` | String |  | Dual password status for the user. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |
| `instance` | String | ✅ | Database instance ID. This does not include the project ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | The user type. It determines the method to authenticate the user during login. The default is the database's built-in user type. |
| `host` | String | Optional. The host from which the user can connect. For `insert` operations, host defaults to an empty string. For `update` operations, host is specified as part of the request URL. The host name cannot be updated after insertion. For a MySQL instance, it's required; for a PostgreSQL or SQL Server instance, it's optional. |
| `instance` | String | The name of the Cloud SQL instance. This does not include the project ID. Can be omitted for *update* because it is already specified on the URL. |
| `kind` | String | This is always `sql#user`. |
| `iam_status` | String | Indicates if a group is active or inactive for IAM database authentication. |
| `etag` | String | This field is deprecated and will be removed from a future version of the API. |
| `sqlserver_user_details` | String |  |
| `password` | String | The password for the user. |
| `project` | String | The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable. Can be omitted for *update* because it is already specified on the URL. |
| `iam_email` | String | Optional. The full email for an IAM user. For normal database users, this will not be filled. Only applicable to MySQL database users. |
| `name` | String | The name of the user in the Cloud SQL instance. Can be omitted for `update` because it is already specified in the URL. |
| `password_policy` | String | User level password validation policy. |
| `dual_password_type` | String | Dual password status for the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.sqladmin_api.User {
    project = "value"  # Project ID of the project that contains the instance.
    instance = "value"  # Database instance ID. This does not include the project ID.
}

# Access user outputs
user_id = user.id
user_type = user.type
user_host = user.host
user_instance = user.instance
user_kind = user.kind
user_iam_status = user.iam_status
user_etag = user.etag
user_sqlserver_user_details = user.sqlserver_user_details
user_password = user.password
user_project = user.project
user_iam_email = user.iam_email
user_name = user.name
user_password_policy = user.password_policy
user_dual_password_type = user.dual_password_type
```

---


### Tier

Lists all available machine types (tiers) for Cloud SQL, for example, `db-custom-1-3840`. For related information, see [Pricing](/sql/pricing).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | List of tiers. |
| `kind` | String | This is always `sql#tiersList`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access tier outputs
tier_id = tier.id
tier_items = tier.items
tier_kind = tier.kind
```

---


### Database

Inserts a resource containing information about a database inside a Cloud SQL instance. **Note:** You can't modify the default character set and collation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | This is always `sql#database`. |
| `instance` | String |  | The name of the Cloud SQL instance. This does not include the project ID. |
| `self_link` | String |  | The URI of this resource. |
| `sqlserver_database_details` | String |  |  |
| `charset` | String |  | The Cloud SQL charset value. |
| `project` | String |  | The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable. |
| `collation` | String |  | The Cloud SQL collation value. |
| `etag` | String |  | This field is deprecated and will be removed from a future version of the API. |
| `name` | String |  | The name of the database in the Cloud SQL instance. This does not include the project ID or instance name. |
| `instance` | String | ✅ | Database instance ID. This does not include the project ID. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | This is always `sql#database`. |
| `instance` | String | The name of the Cloud SQL instance. This does not include the project ID. |
| `self_link` | String | The URI of this resource. |
| `sqlserver_database_details` | String |  |
| `charset` | String | The Cloud SQL charset value. |
| `project` | String | The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable. |
| `collation` | String | The Cloud SQL collation value. |
| `etag` | String | This field is deprecated and will be removed from a future version of the API. |
| `name` | String | The name of the database in the Cloud SQL instance. This does not include the project ID or instance name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.sqladmin_api.Database {
    instance = "value"  # Database instance ID. This does not include the project ID.
    project = "value"  # Project ID of the project that contains the instance.
}

# Access database outputs
database_id = database.id
database_kind = database.kind
database_instance = database.instance
database_self_link = database.self_link
database_sqlserver_database_details = database.sqlserver_database_details
database_charset = database.charset
database_project = database.project
database_collation = database.collation
database_etag = database.etag
database_name = database.name
```

---


### Backup_run

Creates a new backup run on demand.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location` | String |  | Location of the backups. |
| `disk_encryption_status` | String |  | Encryption status specific to a backup. |
| `window_start_time` | String |  | The start time of the backup window during which this the backup was attempted in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `id` | String |  | The identifier for this backup run. Unique only for a specific Cloud SQL instance. |
| `description` | String |  | The description of this run, only applicable to on-demand backups. |
| `self_link` | String |  | The URI of this resource. |
| `backup_kind` | String |  | Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT. |
| `instance` | String |  | Name of the database instance. |
| `max_chargeable_bytes` | String |  | Output only. The maximum chargeable bytes for the backup. |
| `start_time` | String |  | The time the backup operation actually started in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `time_zone` | String |  | Backup time zone to prevent restores to an instance with a different time zone. Now relevant only for SQL Server. |
| `status` | String |  | The status of this run. |
| `disk_encryption_configuration` | String |  | Encryption configuration specific to a backup. |
| `end_time` | String |  | The time the backup operation completed in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `error` | String |  | Information about why the backup operation failed. This is only present if the run has the FAILED status. |
| `database_version` | String |  | Output only. The instance database version at the time this backup was made. |
| `type` | String |  | The type of this run; can be either "AUTOMATED" or "ON_DEMAND" or "FINAL". This field defaults to "ON_DEMAND" and is ignored, when specified for insert requests. |
| `kind` | String |  | This is always `sql#backupRun`. |
| `enqueued_time` | String |  | The time the run was enqueued in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `instance` | String | ✅ | Cloud SQL instance ID. This does not include the project ID. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location` | String | Location of the backups. |
| `disk_encryption_status` | String | Encryption status specific to a backup. |
| `window_start_time` | String | The start time of the backup window during which this the backup was attempted in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `id` | String | The identifier for this backup run. Unique only for a specific Cloud SQL instance. |
| `description` | String | The description of this run, only applicable to on-demand backups. |
| `self_link` | String | The URI of this resource. |
| `backup_kind` | String | Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT. |
| `instance` | String | Name of the database instance. |
| `max_chargeable_bytes` | String | Output only. The maximum chargeable bytes for the backup. |
| `start_time` | String | The time the backup operation actually started in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `time_zone` | String | Backup time zone to prevent restores to an instance with a different time zone. Now relevant only for SQL Server. |
| `status` | String | The status of this run. |
| `disk_encryption_configuration` | String | Encryption configuration specific to a backup. |
| `end_time` | String | The time the backup operation completed in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `error` | String | Information about why the backup operation failed. This is only present if the run has the FAILED status. |
| `database_version` | String | Output only. The instance database version at the time this backup was made. |
| `type` | String | The type of this run; can be either "AUTOMATED" or "ON_DEMAND" or "FINAL". This field defaults to "ON_DEMAND" and is ignored, when specified for insert requests. |
| `kind` | String | This is always `sql#backupRun`. |
| `enqueued_time` | String | The time the run was enqueued in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_run
backup_run = provider.sqladmin_api.Backup_run {
    instance = "value"  # Cloud SQL instance ID. This does not include the project ID.
    project = "value"  # Project ID of the project that contains the instance.
}

# Access backup_run outputs
backup_run_id = backup_run.id
backup_run_location = backup_run.location
backup_run_disk_encryption_status = backup_run.disk_encryption_status
backup_run_window_start_time = backup_run.window_start_time
backup_run_id = backup_run.id
backup_run_description = backup_run.description
backup_run_self_link = backup_run.self_link
backup_run_backup_kind = backup_run.backup_kind
backup_run_instance = backup_run.instance
backup_run_max_chargeable_bytes = backup_run.max_chargeable_bytes
backup_run_start_time = backup_run.start_time
backup_run_time_zone = backup_run.time_zone
backup_run_status = backup_run.status
backup_run_disk_encryption_configuration = backup_run.disk_encryption_configuration
backup_run_end_time = backup_run.end_time
backup_run_error = backup_run.error
backup_run_database_version = backup_run.database_version
backup_run_type = backup_run.type
backup_run_kind = backup_run.kind
backup_run_enqueued_time = backup_run.enqueued_time
```

---


### Backup

Creates a backup for a Cloud SQL instance. This API can be used only to create on-demand backups.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `time_zone` | String |  | Output only. This output contains a backup time zone. If a Cloud SQL for SQL Server instance has a different time zone from the backup's time zone, then the restore to the instance doesn't happen. |
| `ttl_days` | String |  | Input only. The time-to-live (TTL) interval for this resource (in days). For example: ttlDays:7, means 7 days from the current time. The expiration time can't exceed 365 days from the time that the backup is created. |
| `backup_interval` | String |  | Output only. This output contains the following values: start_time: All database writes up to this time are available. end_time: Any database writes after this time aren't available. |
| `description` | String |  | The description of this backup. |
| `satisfies_pzs` | bool |  | Output only. This status indicates whether the backup satisfies PZS. The status is reserved for future use. |
| `kms_key` | String |  | Output only. This output contains the encryption configuration for a backup and the resource name of the KMS key for disk encryption. |
| `satisfies_pzi` | bool |  | Output only. This status indicates whether the backup satisfies PZI. The status is reserved for future use. |
| `location` | String |  | The storage location of the backups. The location can be multi-regional. |
| `type` | String |  | Output only. The type of this backup. The type can be "AUTOMATED", "ON_DEMAND", or “FINAL”. |
| `name` | String |  | Output only. The resource name of the backup. Format: projects/{project}/backups/{backup}. |
| `backup_run` | String |  | Output only. The mapping to backup run resource used for IAM validations. |
| `instance_settings` | String |  | Optional. Output only. Instance setting of the source instance that's associated with this backup. |
| `self_link` | String |  | Output only. The URI of this resource. |
| `database_version` | String |  | Output only. The database version of the instance of at the time this backup was made. |
| `expiry_time` | String |  | Backup expiration time. A UTC timestamp of when this resource expired. |
| `backup_kind` | String |  | Output only. Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT. |
| `instance_deletion_time` | String |  | Optional. Output only. Timestamp in UTC of when the instance associated with this backup is deleted. |
| `kms_key_version` | String |  | Output only. This output contains the encryption status for a backup and the version of the KMS key that's used to encrypt the Cloud SQL instance. |
| `state` | String |  | Output only. The state of this backup. |
| `error` | String |  | Output only. Information about why the backup operation fails (for example, when the backup state fails). |
| `instance` | String |  | The name of the database instance. |
| `max_chargeable_bytes` | String |  | Output only. The maximum chargeable bytes for the backup. |
| `kind` | String |  | Output only. This is always `sql#backup`. |
| `parent` | String | ✅ | Required. The parent resource where this backup is created. Format: projects/{project} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backups` | Vec<String> | A list of backups. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, then there aren't subsequent pages. |
| `warnings` | Vec<String> | If a region isn't unavailable or if an unknown error occurs, then a warning message is returned. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.sqladmin_api.Backup {
    parent = "value"  # Required. The parent resource where this backup is created. Format: projects/{project}
}

# Access backup outputs
backup_id = backup.id
backup_backups = backup.backups
backup_next_page_token = backup.next_page_token
backup_warnings = backup.warnings
```

---


### Operation

Cancels an instance operation that has been performed on an instance. Ordinarily, this method name should be `CancelSqlOperation`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project` | String | ✅ | Project ID of the project that contains the instance. |
| `operation` | String | ✅ | Instance operation ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | The status of an operation. |
| `name` | String | An identifier that uniquely identifies the operation. You can use this identifier to retrieve the Operations resource that has information about the operation. |
| `sub_operation_type` | String | Optional. The sub operation based on the operation type. |
| `self_link` | String | The URI of this resource. |
| `import_context` | String | The context for import operation, if applicable. |
| `insert_time` | String | The time this operation was enqueued in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `target_link` | String |  |
| `pre_check_major_version_upgrade_context` | String | The context for pre-check major version upgrade operation, if applicable. This field is only populated when the operation_type is PRE_CHECK_MAJOR_VERSION_UPGRADE. The PreCheckMajorVersionUpgradeContext message itself contains the details for that pre-check, such as the target database version for the upgrade and the results of the check (including any warnings or errors found). |
| `api_warning` | String | An Admin API warning message. |
| `end_time` | String | The time this operation finished in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `operation_type` | String | The type of the operation. Valid values are: * `CREATE` * `DELETE` * `UPDATE` * `RESTART` * `IMPORT` * `EXPORT` * `BACKUP_VOLUME` * `RESTORE_VOLUME` * `CREATE_USER` * `DELETE_USER` * `CREATE_DATABASE` * `DELETE_DATABASE` |
| `target_project` | String | The project ID of the target instance related to this operation. |
| `kind` | String | This is always `sql#operation`. |
| `start_time` | String | The time this operation actually started in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `export_context` | String | The context for export operation, if applicable. |
| `error` | String | If errors occurred during processing of this operation, this field will be populated. |
| `backup_context` | String | The context for backup operation, if applicable. |
| `acquire_ssrs_lease_context` | String | The context for acquire SSRS lease operation, if applicable. |
| `target_id` | String | Name of the resource on which this operation runs. |
| `user` | String | The email address of the user who initiated this operation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.sqladmin_api.Operation {
    project = "value"  # Project ID of the project that contains the instance.
    operation = "value"  # Instance operation ID.
}

# Access operation outputs
operation_id = operation.id
operation_status = operation.status
operation_name = operation.name
operation_sub_operation_type = operation.sub_operation_type
operation_self_link = operation.self_link
operation_import_context = operation.import_context
operation_insert_time = operation.insert_time
operation_target_link = operation.target_link
operation_pre_check_major_version_upgrade_context = operation.pre_check_major_version_upgrade_context
operation_api_warning = operation.api_warning
operation_end_time = operation.end_time
operation_operation_type = operation.operation_type
operation_target_project = operation.target_project
operation_kind = operation.kind
operation_start_time = operation.start_time
operation_export_context = operation.export_context
operation_error = operation.error
operation_backup_context = operation.backup_context
operation_acquire_ssrs_lease_context = operation.acquire_ssrs_lease_context
operation_target_id = operation.target_id
operation_user = operation.user
```

---


### Flag

Lists all available database flags for Cloud SQL instances.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | This is always `sql#flagsList`. |
| `items` | Vec<String> | List of flags. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access flag outputs
flag_id = flag.id
flag_kind = flag.kind
flag_items = flag.items
```

---


### Instance

Creates a new Cloud SQL instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `replica_configuration` | String |  | Configuration specific to failover replicas and read replicas. |
| `connection_name` | String |  | Connection name of the Cloud SQL instance used in connection strings. |
| `current_disk_size` | String |  | The current disk usage of the instance in bytes. This property has been deprecated. Use the "cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud Monitoring API instead. Please see [this announcement](https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ) for details. |
| `include_replicas_for_major_version_upgrade` | bool |  | Input only. Determines whether an in-place major version upgrade of replicas happens when an in-place major version upgrade of a primary instance is initiated. |
| `out_of_disk_report` | String |  | This field represents the report generated by the proactive database wellness job for OutOfDisk issues. * Writers: * the proactive database wellness job for OOD. * Readers: * the proactive database wellness job |
| `satisfies_pzs` | bool |  | This status indicates whether the instance satisfies PZS. The status is reserved for future use. |
| `primary_dns_name` | String |  | Output only. DEPRECATED: please use write_endpoint instead. |
| `etag` | String |  | This field is deprecated and will be removed from a future version of the API. Use the `settings.settingsVersion` field instead. |
| `sql_network_architecture` | String |  | The SQL network architecture for the instance. |
| `on_premises_configuration` | String |  | Configuration specific to on-premises instances. |
| `dns_names` | Vec<String> |  | Output only. The list of DNS names used by this instance. |
| `gemini_config` | String |  | Gemini instance configuration. |
| `maintenance_version` | String |  | The current software version on the instance. |
| `project` | String |  | The project ID of the project containing the Cloud SQL instance. The Google apps domain is prefixed if applicable. |
| `region` | String |  | The geographical region of the Cloud SQL instance. It can be one of the [regions](https://cloud.google.com/sql/docs/mysql/locations#location-r) where Cloud SQL operates: For example, `asia-east1`, `europe-west1`, and `us-central1`. The default value is `us-central1`. |
| `create_time` | String |  | Output only. The time when the instance was created in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `upgradable_database_versions` | Vec<String> |  | Output only. All database versions that are available for upgrade. |
| `backend_type` | String |  | The backend type. `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database server that is not managed by Google. This property is read-only; use the `tier` property in the `settings` object to determine the database type. |
| `secondary_gce_zone` | String |  | The Compute Engine zone that the failover instance is currently serving from for a regional instance. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary/failover zone. |
| `switch_transaction_logs_to_cloud_storage_enabled` | bool |  | Input only. Whether Cloud SQL is enabled to switch storing point-in-time recovery log files from a data disk to Cloud Storage. |
| `ip_addresses` | Vec<String> |  | The assigned IP addresses for the instance. |
| `disk_encryption_status` | String |  | Disk encryption status specific to an instance. |
| `kind` | String |  | This is always `sql#instance`. |
| `satisfies_pzi` | bool |  | Output only. This status indicates whether the instance satisfies PZI. The status is reserved for future use. |
| `scheduled_maintenance` | String |  | The start time of any upcoming scheduled maintenance for this instance. |
| `database_installed_version` | String |  | Output only. Stores the current database version running on the instance including minor version such as `MYSQL_8_0_18`. |
| `database_version` | String |  | The database engine type and version. The `databaseVersion` field cannot be changed after instance creation. |
| `failover_replica` | String |  | The name and status of the failover replica. |
| `nodes` | Vec<String> |  | Output only. Entries containing information about each read pool node of the read pool. |
| `service_account_email_address` | String |  | The service account email address assigned to the instance. \This property is read-only. |
| `state` | String |  | The current serving state of the Cloud SQL instance. |
| `suspension_reason` | Vec<String> |  | If the instance state is SUSPENDED, the reason for the suspension. |
| `psc_service_attachment_link` | String |  | Output only. The link to service attachment of PSC instance. |
| `gce_zone` | String |  | The Compute Engine zone that the instance is currently serving from. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary zone. WARNING: Changing this might restart the instance. |
| `disk_encryption_configuration` | String |  | Disk encryption configuration specific to an instance. |
| `instance_type` | String |  | The instance type. |
| `name` | String |  | Name of the Cloud SQL instance. This does not include the project ID. |
| `root_password` | String |  | Initial root password. Use only on creation. You must set root passwords before you can connect to PostgreSQL instances. |
| `server_ca_cert` | String |  | SSL configuration. |
| `node_count` | i64 |  | The number of read pool nodes in a read pool. |
| `replica_names` | Vec<String> |  | The replicas of the instance. |
| `replication_cluster` | String |  | A primary instance and disaster recovery (DR) replica pair. A DR replica is a cross-region replica that you designate for failover in the event that the primary instance experiences regional failure. Applicable to MySQL and PostgreSQL. |
| `self_link` | String |  | The URI of this resource. |
| `settings` | String |  | The user settings. |
| `write_endpoint` | String |  | Output only. The dns name of the primary instance in a replication group. |
| `dns_name` | String |  | Output only. The dns name of the instance. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys and tag values that are bound to this instance. You must represent each item in the map as: `"" : ""`. For example, a single resource can have the following tags: ``` "123/environment": "production", "123/costCenter": "marketing", ``` For more information on tag creation and management, see https://cloud.google.com/resource-manager/docs/tags/tags-overview. |
| `available_maintenance_versions` | Vec<String> |  | Output only. List all maintenance versions applicable on the instance |
| `ipv6_address` | String |  | The IPv6 address assigned to the instance. (Deprecated) This property was applicable only to First Generation instances. |
| `max_disk_size` | String |  | The maximum disk size of the instance in bytes. |
| `master_instance_name` | String |  | The name of the instance which will act as primary in the replication setup. |
| `project` | String | ✅ | Project ID of the project to which the newly created Cloud SQL instances should belong. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replica_configuration` | String | Configuration specific to failover replicas and read replicas. |
| `connection_name` | String | Connection name of the Cloud SQL instance used in connection strings. |
| `current_disk_size` | String | The current disk usage of the instance in bytes. This property has been deprecated. Use the "cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud Monitoring API instead. Please see [this announcement](https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ) for details. |
| `include_replicas_for_major_version_upgrade` | bool | Input only. Determines whether an in-place major version upgrade of replicas happens when an in-place major version upgrade of a primary instance is initiated. |
| `out_of_disk_report` | String | This field represents the report generated by the proactive database wellness job for OutOfDisk issues. * Writers: * the proactive database wellness job for OOD. * Readers: * the proactive database wellness job |
| `satisfies_pzs` | bool | This status indicates whether the instance satisfies PZS. The status is reserved for future use. |
| `primary_dns_name` | String | Output only. DEPRECATED: please use write_endpoint instead. |
| `etag` | String | This field is deprecated and will be removed from a future version of the API. Use the `settings.settingsVersion` field instead. |
| `sql_network_architecture` | String | The SQL network architecture for the instance. |
| `on_premises_configuration` | String | Configuration specific to on-premises instances. |
| `dns_names` | Vec<String> | Output only. The list of DNS names used by this instance. |
| `gemini_config` | String | Gemini instance configuration. |
| `maintenance_version` | String | The current software version on the instance. |
| `project` | String | The project ID of the project containing the Cloud SQL instance. The Google apps domain is prefixed if applicable. |
| `region` | String | The geographical region of the Cloud SQL instance. It can be one of the [regions](https://cloud.google.com/sql/docs/mysql/locations#location-r) where Cloud SQL operates: For example, `asia-east1`, `europe-west1`, and `us-central1`. The default value is `us-central1`. |
| `create_time` | String | Output only. The time when the instance was created in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `upgradable_database_versions` | Vec<String> | Output only. All database versions that are available for upgrade. |
| `backend_type` | String | The backend type. `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database server that is not managed by Google. This property is read-only; use the `tier` property in the `settings` object to determine the database type. |
| `secondary_gce_zone` | String | The Compute Engine zone that the failover instance is currently serving from for a regional instance. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary/failover zone. |
| `switch_transaction_logs_to_cloud_storage_enabled` | bool | Input only. Whether Cloud SQL is enabled to switch storing point-in-time recovery log files from a data disk to Cloud Storage. |
| `ip_addresses` | Vec<String> | The assigned IP addresses for the instance. |
| `disk_encryption_status` | String | Disk encryption status specific to an instance. |
| `kind` | String | This is always `sql#instance`. |
| `satisfies_pzi` | bool | Output only. This status indicates whether the instance satisfies PZI. The status is reserved for future use. |
| `scheduled_maintenance` | String | The start time of any upcoming scheduled maintenance for this instance. |
| `database_installed_version` | String | Output only. Stores the current database version running on the instance including minor version such as `MYSQL_8_0_18`. |
| `database_version` | String | The database engine type and version. The `databaseVersion` field cannot be changed after instance creation. |
| `failover_replica` | String | The name and status of the failover replica. |
| `nodes` | Vec<String> | Output only. Entries containing information about each read pool node of the read pool. |
| `service_account_email_address` | String | The service account email address assigned to the instance. \This property is read-only. |
| `state` | String | The current serving state of the Cloud SQL instance. |
| `suspension_reason` | Vec<String> | If the instance state is SUSPENDED, the reason for the suspension. |
| `psc_service_attachment_link` | String | Output only. The link to service attachment of PSC instance. |
| `gce_zone` | String | The Compute Engine zone that the instance is currently serving from. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary zone. WARNING: Changing this might restart the instance. |
| `disk_encryption_configuration` | String | Disk encryption configuration specific to an instance. |
| `instance_type` | String | The instance type. |
| `name` | String | Name of the Cloud SQL instance. This does not include the project ID. |
| `root_password` | String | Initial root password. Use only on creation. You must set root passwords before you can connect to PostgreSQL instances. |
| `server_ca_cert` | String | SSL configuration. |
| `node_count` | i64 | The number of read pool nodes in a read pool. |
| `replica_names` | Vec<String> | The replicas of the instance. |
| `replication_cluster` | String | A primary instance and disaster recovery (DR) replica pair. A DR replica is a cross-region replica that you designate for failover in the event that the primary instance experiences regional failure. Applicable to MySQL and PostgreSQL. |
| `self_link` | String | The URI of this resource. |
| `settings` | String | The user settings. |
| `write_endpoint` | String | Output only. The dns name of the primary instance in a replication group. |
| `dns_name` | String | Output only. The dns name of the instance. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys and tag values that are bound to this instance. You must represent each item in the map as: `"" : ""`. For example, a single resource can have the following tags: ``` "123/environment": "production", "123/costCenter": "marketing", ``` For more information on tag creation and management, see https://cloud.google.com/resource-manager/docs/tags/tags-overview. |
| `available_maintenance_versions` | Vec<String> | Output only. List all maintenance versions applicable on the instance |
| `ipv6_address` | String | The IPv6 address assigned to the instance. (Deprecated) This property was applicable only to First Generation instances. |
| `max_disk_size` | String | The maximum disk size of the instance in bytes. |
| `master_instance_name` | String | The name of the instance which will act as primary in the replication setup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.sqladmin_api.Instance {
    project = "value"  # Project ID of the project to which the newly created Cloud SQL instances should belong.
}

# Access instance outputs
instance_id = instance.id
instance_replica_configuration = instance.replica_configuration
instance_connection_name = instance.connection_name
instance_current_disk_size = instance.current_disk_size
instance_include_replicas_for_major_version_upgrade = instance.include_replicas_for_major_version_upgrade
instance_out_of_disk_report = instance.out_of_disk_report
instance_satisfies_pzs = instance.satisfies_pzs
instance_primary_dns_name = instance.primary_dns_name
instance_etag = instance.etag
instance_sql_network_architecture = instance.sql_network_architecture
instance_on_premises_configuration = instance.on_premises_configuration
instance_dns_names = instance.dns_names
instance_gemini_config = instance.gemini_config
instance_maintenance_version = instance.maintenance_version
instance_project = instance.project
instance_region = instance.region
instance_create_time = instance.create_time
instance_upgradable_database_versions = instance.upgradable_database_versions
instance_backend_type = instance.backend_type
instance_secondary_gce_zone = instance.secondary_gce_zone
instance_switch_transaction_logs_to_cloud_storage_enabled = instance.switch_transaction_logs_to_cloud_storage_enabled
instance_ip_addresses = instance.ip_addresses
instance_disk_encryption_status = instance.disk_encryption_status
instance_kind = instance.kind
instance_satisfies_pzi = instance.satisfies_pzi
instance_scheduled_maintenance = instance.scheduled_maintenance
instance_database_installed_version = instance.database_installed_version
instance_database_version = instance.database_version
instance_failover_replica = instance.failover_replica
instance_nodes = instance.nodes
instance_service_account_email_address = instance.service_account_email_address
instance_state = instance.state
instance_suspension_reason = instance.suspension_reason
instance_psc_service_attachment_link = instance.psc_service_attachment_link
instance_gce_zone = instance.gce_zone
instance_disk_encryption_configuration = instance.disk_encryption_configuration
instance_instance_type = instance.instance_type
instance_name = instance.name
instance_root_password = instance.root_password
instance_server_ca_cert = instance.server_ca_cert
instance_node_count = instance.node_count
instance_replica_names = instance.replica_names
instance_replication_cluster = instance.replication_cluster
instance_self_link = instance.self_link
instance_settings = instance.settings
instance_write_endpoint = instance.write_endpoint
instance_dns_name = instance.dns_name
instance_tags = instance.tags
instance_available_maintenance_versions = instance.available_maintenance_versions
instance_ipv6_address = instance.ipv6_address
instance_max_disk_size = instance.max_disk_size
instance_master_instance_name = instance.master_instance_name
```

---


### Ssl_cert

Creates an SSL certificate and returns it along with the private key and server certificate authority. The new certificate will not be usable until the instance is restarted.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `common_name` | String |  | User supplied name. Must be a distinct name from the other certificates for this instance. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |
| `instance` | String | ✅ | Cloud SQL instance ID. This does not include the project ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | The URI of this resource. |
| `kind` | String | This is always `sql#sslCert`. |
| `sha1_fingerprint` | String | Sha1 Fingerprint. |
| `common_name` | String | User supplied name. Constrained to [a-zA-Z.-_ ]+. |
| `expiration_time` | String | The time when the certificate expires in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `cert` | String | PEM representation. |
| `create_time` | String | The time when the certificate was created in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `cert_serial_number` | String | Serial number, as extracted from the certificate. |
| `instance` | String | Name of the database instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ssl_cert
ssl_cert = provider.sqladmin_api.Ssl_cert {
    project = "value"  # Project ID of the project that contains the instance.
    instance = "value"  # Cloud SQL instance ID. This does not include the project ID.
}

# Access ssl_cert outputs
ssl_cert_id = ssl_cert.id
ssl_cert_self_link = ssl_cert.self_link
ssl_cert_kind = ssl_cert.kind
ssl_cert_sha1_fingerprint = ssl_cert.sha1_fingerprint
ssl_cert_common_name = ssl_cert.common_name
ssl_cert_expiration_time = ssl_cert.expiration_time
ssl_cert_cert = ssl_cert.cert
ssl_cert_create_time = ssl_cert.create_time
ssl_cert_cert_serial_number = ssl_cert.cert_serial_number
ssl_cert_instance = ssl_cert.instance
```

---


### Connect

Generates a short-lived X509 certificate containing the provided public key and signed by a private key specific to the target instance. Users may use the certificate to authenticate as themselves when connecting to the database.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `access_token` | String |  | Optional. Access token to include in the signed certificate. |
| `read_time` | String |  | Optional. Optional snapshot read timestamp to trade freshness for performance. |
| `public_key` | String |  | PEM encoded public key to include in the signed certificate. |
| `valid_duration` | String |  | Optional. If set, it will contain the cert valid duration. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |
| `instance` | String | ✅ | Cloud SQL instance ID. This does not include the project ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dns_names` | Vec<String> | Output only. The list of DNS names used by this instance. |
| `kind` | String | This is always `sql#connectSettings`. |
| `database_version` | String | The database engine type and version. The `databaseVersion` field cannot be changed after instance creation. MySQL instances: `MYSQL_8_0`, `MYSQL_5_7` (default), or `MYSQL_5_6`. PostgreSQL instances: `POSTGRES_9_6`, `POSTGRES_10`, `POSTGRES_11` or `POSTGRES_12` (default), `POSTGRES_13`, or `POSTGRES_14`. SQL Server instances: `SQLSERVER_2017_STANDARD` (default), `SQLSERVER_2017_ENTERPRISE`, `SQLSERVER_2017_EXPRESS`, `SQLSERVER_2017_WEB`, `SQLSERVER_2019_STANDARD`, `SQLSERVER_2019_ENTERPRISE`, `SQLSERVER_2019_EXPRESS`, or `SQLSERVER_2019_WEB`. |
| `dns_name` | String | The dns name of the instance. |
| `region` | String | The cloud region for the instance. e.g. `us-central1`, `europe-west1`. The region cannot be changed after instance creation. |
| `psc_enabled` | bool | Whether PSC connectivity is enabled for this instance. |
| `backend_type` | String | `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database server that is not managed by Google. This property is read-only; use the `tier` property in the `settings` object to determine the database type. |
| `node_count` | i64 | The number of read pool nodes in a read pool. |
| `mdx_protocol_support` | Vec<String> | Optional. Output only. mdx_protocol_support controls how the client uses metadata exchange when connecting to the instance. The values in the list representing parts of the MDX protocol that are supported by this instance. When the list is empty, the instance does not support MDX, so the client must not send an MDX request. The default is empty. |
| `ip_addresses` | Vec<String> | The assigned IP addresses for the instance. |
| `custom_subject_alternative_names` | Vec<String> | Custom subject alternative names for the server certificate. |
| `nodes` | Vec<String> | Output only. Entries containing information about each read pool node of the read pool. |
| `server_ca_mode` | String | Specify what type of CA is used for the server certificate. |
| `server_ca_cert` | String | SSL configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connect
connect = provider.sqladmin_api.Connect {
    project = "value"  # Project ID of the project that contains the instance.
    instance = "value"  # Cloud SQL instance ID. This does not include the project ID.
}

# Access connect outputs
connect_id = connect.id
connect_dns_names = connect.dns_names
connect_kind = connect.kind
connect_database_version = connect.database_version
connect_dns_name = connect.dns_name
connect_region = connect.region
connect_psc_enabled = connect.psc_enabled
connect_backend_type = connect.backend_type
connect_node_count = connect.node_count
connect_mdx_protocol_support = connect.mdx_protocol_support
connect_ip_addresses = connect.ip_addresses
connect_custom_subject_alternative_names = connect.custom_subject_alternative_names
connect_nodes = connect.nodes
connect_server_ca_mode = connect.server_ca_mode
connect_server_ca_cert = connect.server_ca_cert
```

---


### Operation

Cancels an instance operation that has been performed on an instance.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `operation` | String | ✅ | Instance operation ID. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pre_check_major_version_upgrade_context` | String | This field is only populated when the operation_type is PRE_CHECK_MAJOR_VERSION_UPGRADE. The PreCheckMajorVersionUpgradeContext message itself contains the details for that pre-check, such as the target database version for the upgrade and the results of the check (including any warnings or errors found). |
| `self_link` | String | The URI of this resource. |
| `user` | String | The email address of the user who initiated this operation. |
| `import_context` | String | The context for import operation, if applicable. |
| `acquire_ssrs_lease_context` | String | The context for acquire SSRS lease operation, if applicable. |
| `kind` | String | This is always `sql#operation`. |
| `name` | String | An identifier that uniquely identifies the operation. You can use this identifier to retrieve the Operations resource that has information about the operation. |
| `sub_operation_type` | String | Optional. The sub operation based on the operation type. |
| `target_project` | String | The project ID of the target instance related to this operation. |
| `export_context` | String | The context for export operation, if applicable. |
| `start_time` | String | The time this operation actually started in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `end_time` | String | The time this operation finished in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `target_link` | String |  |
| `operation_type` | String | The type of the operation. Valid values are: * `CREATE` * `DELETE` * `UPDATE` * `RESTART` * `IMPORT` * `EXPORT` * `BACKUP_VOLUME` * `RESTORE_VOLUME` * `CREATE_USER` * `DELETE_USER` * `CREATE_DATABASE` * `DELETE_DATABASE` |
| `target_id` | String | Name of the resource on which this operation runs. |
| `error` | String | If errors occurred during processing of this operation, this field will be populated. |
| `insert_time` | String | The time this operation was enqueued in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `status` | String | The status of an operation. |
| `api_warning` | String | An Admin API warning message. |
| `backup_context` | String | The context for backup operation, if applicable. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.sqladmin_api.Operation {
    operation = "value"  # Instance operation ID.
    project = "value"  # Project ID of the project that contains the instance.
}

# Access operation outputs
operation_id = operation.id
operation_pre_check_major_version_upgrade_context = operation.pre_check_major_version_upgrade_context
operation_self_link = operation.self_link
operation_user = operation.user
operation_import_context = operation.import_context
operation_acquire_ssrs_lease_context = operation.acquire_ssrs_lease_context
operation_kind = operation.kind
operation_name = operation.name
operation_sub_operation_type = operation.sub_operation_type
operation_target_project = operation.target_project
operation_export_context = operation.export_context
operation_start_time = operation.start_time
operation_end_time = operation.end_time
operation_target_link = operation.target_link
operation_operation_type = operation.operation_type
operation_target_id = operation.target_id
operation_error = operation.error
operation_insert_time = operation.insert_time
operation_status = operation.status
operation_api_warning = operation.api_warning
operation_backup_context = operation.backup_context
```

---


### Ssl_cert

Creates an SSL certificate and returns it along with the private key and server certificate authority. The new certificate will not be usable until the instance is restarted.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `common_name` | String |  | User supplied name. Must be a distinct name from the other certificates for this instance. |
| `instance` | String | ✅ | Cloud SQL instance ID. This does not include the project ID. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cert_serial_number` | String | Serial number, as extracted from the certificate. |
| `expiration_time` | String | The time when the certificate expires in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `create_time` | String | The time when the certificate was created in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z` |
| `self_link` | String | The URI of this resource. |
| `kind` | String | This is always `sql#sslCert`. |
| `instance` | String | Name of the database instance. |
| `common_name` | String | User supplied name. Constrained to [a-zA-Z.-_ ]+. |
| `cert` | String | PEM representation. |
| `sha1_fingerprint` | String | Sha1 Fingerprint. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ssl_cert
ssl_cert = provider.sqladmin_api.Ssl_cert {
    instance = "value"  # Cloud SQL instance ID. This does not include the project ID.
    project = "value"  # Project ID of the project that contains the instance.
}

# Access ssl_cert outputs
ssl_cert_id = ssl_cert.id
ssl_cert_cert_serial_number = ssl_cert.cert_serial_number
ssl_cert_expiration_time = ssl_cert.expiration_time
ssl_cert_create_time = ssl_cert.create_time
ssl_cert_self_link = ssl_cert.self_link
ssl_cert_kind = ssl_cert.kind
ssl_cert_instance = ssl_cert.instance
ssl_cert_common_name = ssl_cert.common_name
ssl_cert_cert = ssl_cert.cert
ssl_cert_sha1_fingerprint = ssl_cert.sha1_fingerprint
```

---


### User

Creates a new user in a Cloud SQL instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the user in the Cloud SQL instance. Can be omitted for `update` because it is already specified in the URL. |
| `iam_status` | String |  | Indicates if a group is active or inactive for IAM database authentication. |
| `iam_email` | String |  | Optional. The full email for an IAM user. For normal database users, this will not be filled. Only applicable to MySQL database users. |
| `password` | String |  | The password for the user. |
| `dual_password_type` | String |  | Dual password status for the user. |
| `password_policy` | String |  | User level password validation policy. |
| `project` | String |  | The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable. Can be omitted for `update` because it is already specified on the URL. |
| `kind` | String |  | This is always `sql#user`. |
| `etag` | String |  | This field is deprecated and will be removed from a future version of the API. |
| `host` | String |  | Optional. The host from which the user can connect. For `insert` operations, host defaults to an empty string. For `update` operations, host is specified as part of the request URL. The host name cannot be updated after insertion. For a MySQL instance, it's required; for a PostgreSQL or SQL Server instance, it's optional. |
| `sqlserver_user_details` | String |  |  |
| `type` | String |  | The user type. It determines the method to authenticate the user during login. The default is the database's built-in user type. |
| `instance` | String |  | The name of the Cloud SQL instance. This does not include the project ID. Can be omitted for `update` because it is already specified on the URL. |
| `instance` | String | ✅ | Database instance ID. This does not include the project ID. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the user in the Cloud SQL instance. Can be omitted for `update` because it is already specified in the URL. |
| `iam_status` | String | Indicates if a group is active or inactive for IAM database authentication. |
| `iam_email` | String | Optional. The full email for an IAM user. For normal database users, this will not be filled. Only applicable to MySQL database users. |
| `password` | String | The password for the user. |
| `dual_password_type` | String | Dual password status for the user. |
| `password_policy` | String | User level password validation policy. |
| `project` | String | The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable. Can be omitted for `update` because it is already specified on the URL. |
| `kind` | String | This is always `sql#user`. |
| `etag` | String | This field is deprecated and will be removed from a future version of the API. |
| `host` | String | Optional. The host from which the user can connect. For `insert` operations, host defaults to an empty string. For `update` operations, host is specified as part of the request URL. The host name cannot be updated after insertion. For a MySQL instance, it's required; for a PostgreSQL or SQL Server instance, it's optional. |
| `sqlserver_user_details` | String |  |
| `type` | String | The user type. It determines the method to authenticate the user during login. The default is the database's built-in user type. |
| `instance` | String | The name of the Cloud SQL instance. This does not include the project ID. Can be omitted for `update` because it is already specified on the URL. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.sqladmin_api.User {
    instance = "value"  # Database instance ID. This does not include the project ID.
    project = "value"  # Project ID of the project that contains the instance.
}

# Access user outputs
user_id = user.id
user_name = user.name
user_iam_status = user.iam_status
user_iam_email = user.iam_email
user_password = user.password
user_dual_password_type = user.dual_password_type
user_password_policy = user.password_policy
user_project = user.project
user_kind = user.kind
user_etag = user.etag
user_host = user.host
user_sqlserver_user_details = user.sqlserver_user_details
user_type = user.type
user_instance = user.instance
```

---


### Backup_run

Creates a new backup run on demand.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_time` | String |  | The time the backup operation actually started in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `location` | String |  | Location of the backups. |
| `end_time` | String |  | The time the backup operation completed in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `self_link` | String |  | The URI of this resource. |
| `type` | String |  | The type of this run; can be either "AUTOMATED" or "ON_DEMAND" or "FINAL". This field defaults to "ON_DEMAND" and is ignored, when specified for insert requests. |
| `backup_kind` | String |  | Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT. |
| `database_version` | String |  | Output only. The instance database version at the time this backup was made. |
| `max_chargeable_bytes` | String |  | Output only. The maximum chargeable bytes for the backup. |
| `disk_encryption_configuration` | String |  | Encryption configuration specific to a backup. |
| `instance` | String |  | Name of the database instance. |
| `window_start_time` | String |  | The start time of the backup window during which this the backup was attempted in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `disk_encryption_status` | String |  | Encryption status specific to a backup. |
| `enqueued_time` | String |  | The time the run was enqueued in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `time_zone` | String |  | Backup time zone to prevent restores to an instance with a different time zone. Now relevant only for SQL Server. |
| `id` | String |  | The identifier for this backup run. Unique only for a specific Cloud SQL instance. |
| `kind` | String |  | This is always `sql#backupRun`. |
| `status` | String |  | The status of this run. |
| `description` | String |  | The description of this run, only applicable to on-demand backups. |
| `error` | String |  | Information about why the backup operation failed. This is only present if the run has the FAILED status. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |
| `instance` | String | ✅ | Cloud SQL instance ID. This does not include the project ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | The time the backup operation actually started in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `location` | String | Location of the backups. |
| `end_time` | String | The time the backup operation completed in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `self_link` | String | The URI of this resource. |
| `type` | String | The type of this run; can be either "AUTOMATED" or "ON_DEMAND" or "FINAL". This field defaults to "ON_DEMAND" and is ignored, when specified for insert requests. |
| `backup_kind` | String | Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT. |
| `database_version` | String | Output only. The instance database version at the time this backup was made. |
| `max_chargeable_bytes` | String | Output only. The maximum chargeable bytes for the backup. |
| `disk_encryption_configuration` | String | Encryption configuration specific to a backup. |
| `instance` | String | Name of the database instance. |
| `window_start_time` | String | The start time of the backup window during which this the backup was attempted in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `disk_encryption_status` | String | Encryption status specific to a backup. |
| `enqueued_time` | String | The time the run was enqueued in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `time_zone` | String | Backup time zone to prevent restores to an instance with a different time zone. Now relevant only for SQL Server. |
| `id` | String | The identifier for this backup run. Unique only for a specific Cloud SQL instance. |
| `kind` | String | This is always `sql#backupRun`. |
| `status` | String | The status of this run. |
| `description` | String | The description of this run, only applicable to on-demand backups. |
| `error` | String | Information about why the backup operation failed. This is only present if the run has the FAILED status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_run
backup_run = provider.sqladmin_api.Backup_run {
    project = "value"  # Project ID of the project that contains the instance.
    instance = "value"  # Cloud SQL instance ID. This does not include the project ID.
}

# Access backup_run outputs
backup_run_id = backup_run.id
backup_run_start_time = backup_run.start_time
backup_run_location = backup_run.location
backup_run_end_time = backup_run.end_time
backup_run_self_link = backup_run.self_link
backup_run_type = backup_run.type
backup_run_backup_kind = backup_run.backup_kind
backup_run_database_version = backup_run.database_version
backup_run_max_chargeable_bytes = backup_run.max_chargeable_bytes
backup_run_disk_encryption_configuration = backup_run.disk_encryption_configuration
backup_run_instance = backup_run.instance
backup_run_window_start_time = backup_run.window_start_time
backup_run_disk_encryption_status = backup_run.disk_encryption_status
backup_run_enqueued_time = backup_run.enqueued_time
backup_run_time_zone = backup_run.time_zone
backup_run_id = backup_run.id
backup_run_kind = backup_run.kind
backup_run_status = backup_run.status
backup_run_description = backup_run.description
backup_run_error = backup_run.error
```

---


### Instance

Creates a new Cloud SQL instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sql_network_architecture` | String |  |  |
| `kind` | String |  | This is always `sql#instance`. |
| `name` | String |  | Name of the Cloud SQL instance. This does not include the project ID. |
| `region` | String |  | The geographical region of the Cloud SQL instance. It can be one of the [regions](https://cloud.google.com/sql/docs/mysql/locations#location-r) where Cloud SQL operates: For example, `asia-east1`, `europe-west1`, and `us-central1`. The default value is `us-central1`. |
| `connection_name` | String |  | Connection name of the Cloud SQL instance used in connection strings. |
| `replica_names` | Vec<String> |  | The replicas of the instance. |
| `server_ca_cert` | String |  | SSL configuration. |
| `self_link` | String |  | The URI of this resource. |
| `write_endpoint` | String |  | Output only. The dns name of the primary instance in a replication group. |
| `disk_encryption_configuration` | String |  | Disk encryption configuration specific to an instance. |
| `ipv6_address` | String |  | The IPv6 address assigned to the instance. (Deprecated) This property was applicable only to First Generation instances. |
| `etag` | String |  | This field is deprecated and will be removed from a future version of the API. Use the `settings.settingsVersion` field instead. |
| `dns_name` | String |  | Output only. The dns name of the instance. |
| `node_count` | i64 |  | The number of read pool nodes in a read pool. |
| `on_premises_configuration` | String |  | Configuration specific to on-premises instances. |
| `satisfies_pzi` | bool |  | Output only. This status indicates whether the instance satisfies PZI. The status is reserved for future use. |
| `settings` | String |  | The user settings. |
| `include_replicas_for_major_version_upgrade` | bool |  | Input only. Determines whether an in-place major version upgrade of replicas happens when an in-place major version upgrade of a primary instance is initiated. |
| `create_time` | String |  | Output only. The time when the instance was created in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `max_disk_size` | String |  | The maximum disk size of the instance in bytes. |
| `failover_replica` | String |  | The name and status of the failover replica. |
| `dns_names` | Vec<String> |  | Output only. The list of DNS names used by this instance. |
| `primary_dns_name` | String |  | Output only. DEPRECATED: please use write_endpoint instead. |
| `maintenance_version` | String |  | The current software version on the instance. |
| `root_password` | String |  | Initial root password. Use only on creation. You must set root passwords before you can connect to PostgreSQL instances. |
| `scheduled_maintenance` | String |  | The start time of any upcoming scheduled maintenance for this instance. |
| `state` | String |  | The current serving state of the Cloud SQL instance. |
| `suspension_reason` | Vec<String> |  | If the instance state is SUSPENDED, the reason for the suspension. |
| `available_maintenance_versions` | Vec<String> |  | Output only. List all maintenance versions applicable on the instance |
| `disk_encryption_status` | String |  | Disk encryption status specific to an instance. |
| `ip_addresses` | Vec<String> |  | The assigned IP addresses for the instance. |
| `replica_configuration` | String |  | Configuration specific to failover replicas and read replicas. |
| `service_account_email_address` | String |  | The service account email address assigned to the instance.\This property is read-only. |
| `current_disk_size` | String |  | The current disk usage of the instance in bytes. This property has been deprecated. Use the "cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud Monitoring API instead. Please see [this announcement](https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ) for details. |
| `instance_type` | String |  | The instance type. |
| `secondary_gce_zone` | String |  | The Compute Engine zone that the failover instance is currently serving from for a regional instance. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary/failover zone. |
| `master_instance_name` | String |  | The name of the instance which will act as primary in the replication setup. |
| `switch_transaction_logs_to_cloud_storage_enabled` | bool |  | Input only. Whether Cloud SQL is enabled to switch storing point-in-time recovery log files from a data disk to Cloud Storage. |
| `out_of_disk_report` | String |  | This field represents the report generated by the proactive database wellness job for OutOfDisk issues. * Writers: * the proactive database wellness job for OOD. * Readers: * the proactive database wellness job |
| `psc_service_attachment_link` | String |  | Output only. The link to service attachment of PSC instance. |
| `satisfies_pzs` | bool |  | This status indicates whether the instance satisfies PZS. The status is reserved for future use. |
| `backend_type` | String |  | The backend type. `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database server that is not managed by Google. This property is read-only; use the `tier` property in the `settings` object to determine the database type. |
| `database_version` | String |  | The database engine type and version. The `databaseVersion` field cannot be changed after instance creation. |
| `gemini_config` | String |  | Gemini instance configuration. |
| `nodes` | Vec<String> |  | Output only. Entries containing information about each read pool node of the read pool. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys and tag values that are bound to this instance. You must represent each item in the map as: `"" : ""`. For example, a single resource can have the following tags: ``` "123/environment": "production", "123/costCenter": "marketing", ``` For more information on tag creation and management, see https://cloud.google.com/resource-manager/docs/tags/tags-overview. |
| `database_installed_version` | String |  | Output only. Stores the current database version running on the instance including minor version such as `MYSQL_8_0_18`. |
| `replication_cluster` | String |  | Optional. A primary instance and disaster recovery (DR) replica pair. A DR replica is a cross-region replica that you designate for failover in the event that the primary instance experiences regional failure. Applicable to MySQL and PostgreSQL. |
| `gce_zone` | String |  | The Compute Engine zone that the instance is currently serving from. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary zone. WARNING: Changing this might restart the instance. |
| `project` | String |  | The project ID of the project containing the Cloud SQL instance. The Google apps domain is prefixed if applicable. |
| `upgradable_database_versions` | Vec<String> |  | Output only. All database versions that are available for upgrade. |
| `project` | String | ✅ | Project ID of the project to which the newly created Cloud SQL instances should belong. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sql_network_architecture` | String |  |
| `kind` | String | This is always `sql#instance`. |
| `name` | String | Name of the Cloud SQL instance. This does not include the project ID. |
| `region` | String | The geographical region of the Cloud SQL instance. It can be one of the [regions](https://cloud.google.com/sql/docs/mysql/locations#location-r) where Cloud SQL operates: For example, `asia-east1`, `europe-west1`, and `us-central1`. The default value is `us-central1`. |
| `connection_name` | String | Connection name of the Cloud SQL instance used in connection strings. |
| `replica_names` | Vec<String> | The replicas of the instance. |
| `server_ca_cert` | String | SSL configuration. |
| `self_link` | String | The URI of this resource. |
| `write_endpoint` | String | Output only. The dns name of the primary instance in a replication group. |
| `disk_encryption_configuration` | String | Disk encryption configuration specific to an instance. |
| `ipv6_address` | String | The IPv6 address assigned to the instance. (Deprecated) This property was applicable only to First Generation instances. |
| `etag` | String | This field is deprecated and will be removed from a future version of the API. Use the `settings.settingsVersion` field instead. |
| `dns_name` | String | Output only. The dns name of the instance. |
| `node_count` | i64 | The number of read pool nodes in a read pool. |
| `on_premises_configuration` | String | Configuration specific to on-premises instances. |
| `satisfies_pzi` | bool | Output only. This status indicates whether the instance satisfies PZI. The status is reserved for future use. |
| `settings` | String | The user settings. |
| `include_replicas_for_major_version_upgrade` | bool | Input only. Determines whether an in-place major version upgrade of replicas happens when an in-place major version upgrade of a primary instance is initiated. |
| `create_time` | String | Output only. The time when the instance was created in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`. |
| `max_disk_size` | String | The maximum disk size of the instance in bytes. |
| `failover_replica` | String | The name and status of the failover replica. |
| `dns_names` | Vec<String> | Output only. The list of DNS names used by this instance. |
| `primary_dns_name` | String | Output only. DEPRECATED: please use write_endpoint instead. |
| `maintenance_version` | String | The current software version on the instance. |
| `root_password` | String | Initial root password. Use only on creation. You must set root passwords before you can connect to PostgreSQL instances. |
| `scheduled_maintenance` | String | The start time of any upcoming scheduled maintenance for this instance. |
| `state` | String | The current serving state of the Cloud SQL instance. |
| `suspension_reason` | Vec<String> | If the instance state is SUSPENDED, the reason for the suspension. |
| `available_maintenance_versions` | Vec<String> | Output only. List all maintenance versions applicable on the instance |
| `disk_encryption_status` | String | Disk encryption status specific to an instance. |
| `ip_addresses` | Vec<String> | The assigned IP addresses for the instance. |
| `replica_configuration` | String | Configuration specific to failover replicas and read replicas. |
| `service_account_email_address` | String | The service account email address assigned to the instance.\This property is read-only. |
| `current_disk_size` | String | The current disk usage of the instance in bytes. This property has been deprecated. Use the "cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud Monitoring API instead. Please see [this announcement](https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ) for details. |
| `instance_type` | String | The instance type. |
| `secondary_gce_zone` | String | The Compute Engine zone that the failover instance is currently serving from for a regional instance. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary/failover zone. |
| `master_instance_name` | String | The name of the instance which will act as primary in the replication setup. |
| `switch_transaction_logs_to_cloud_storage_enabled` | bool | Input only. Whether Cloud SQL is enabled to switch storing point-in-time recovery log files from a data disk to Cloud Storage. |
| `out_of_disk_report` | String | This field represents the report generated by the proactive database wellness job for OutOfDisk issues. * Writers: * the proactive database wellness job for OOD. * Readers: * the proactive database wellness job |
| `psc_service_attachment_link` | String | Output only. The link to service attachment of PSC instance. |
| `satisfies_pzs` | bool | This status indicates whether the instance satisfies PZS. The status is reserved for future use. |
| `backend_type` | String | The backend type. `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database server that is not managed by Google. This property is read-only; use the `tier` property in the `settings` object to determine the database type. |
| `database_version` | String | The database engine type and version. The `databaseVersion` field cannot be changed after instance creation. |
| `gemini_config` | String | Gemini instance configuration. |
| `nodes` | Vec<String> | Output only. Entries containing information about each read pool node of the read pool. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys and tag values that are bound to this instance. You must represent each item in the map as: `"" : ""`. For example, a single resource can have the following tags: ``` "123/environment": "production", "123/costCenter": "marketing", ``` For more information on tag creation and management, see https://cloud.google.com/resource-manager/docs/tags/tags-overview. |
| `database_installed_version` | String | Output only. Stores the current database version running on the instance including minor version such as `MYSQL_8_0_18`. |
| `replication_cluster` | String | Optional. A primary instance and disaster recovery (DR) replica pair. A DR replica is a cross-region replica that you designate for failover in the event that the primary instance experiences regional failure. Applicable to MySQL and PostgreSQL. |
| `gce_zone` | String | The Compute Engine zone that the instance is currently serving from. This value could be different from the zone that was specified when the instance was created if the instance has failed over to its secondary zone. WARNING: Changing this might restart the instance. |
| `project` | String | The project ID of the project containing the Cloud SQL instance. The Google apps domain is prefixed if applicable. |
| `upgradable_database_versions` | Vec<String> | Output only. All database versions that are available for upgrade. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.sqladmin_api.Instance {
    project = "value"  # Project ID of the project to which the newly created Cloud SQL instances should belong.
}

# Access instance outputs
instance_id = instance.id
instance_sql_network_architecture = instance.sql_network_architecture
instance_kind = instance.kind
instance_name = instance.name
instance_region = instance.region
instance_connection_name = instance.connection_name
instance_replica_names = instance.replica_names
instance_server_ca_cert = instance.server_ca_cert
instance_self_link = instance.self_link
instance_write_endpoint = instance.write_endpoint
instance_disk_encryption_configuration = instance.disk_encryption_configuration
instance_ipv6_address = instance.ipv6_address
instance_etag = instance.etag
instance_dns_name = instance.dns_name
instance_node_count = instance.node_count
instance_on_premises_configuration = instance.on_premises_configuration
instance_satisfies_pzi = instance.satisfies_pzi
instance_settings = instance.settings
instance_include_replicas_for_major_version_upgrade = instance.include_replicas_for_major_version_upgrade
instance_create_time = instance.create_time
instance_max_disk_size = instance.max_disk_size
instance_failover_replica = instance.failover_replica
instance_dns_names = instance.dns_names
instance_primary_dns_name = instance.primary_dns_name
instance_maintenance_version = instance.maintenance_version
instance_root_password = instance.root_password
instance_scheduled_maintenance = instance.scheduled_maintenance
instance_state = instance.state
instance_suspension_reason = instance.suspension_reason
instance_available_maintenance_versions = instance.available_maintenance_versions
instance_disk_encryption_status = instance.disk_encryption_status
instance_ip_addresses = instance.ip_addresses
instance_replica_configuration = instance.replica_configuration
instance_service_account_email_address = instance.service_account_email_address
instance_current_disk_size = instance.current_disk_size
instance_instance_type = instance.instance_type
instance_secondary_gce_zone = instance.secondary_gce_zone
instance_master_instance_name = instance.master_instance_name
instance_switch_transaction_logs_to_cloud_storage_enabled = instance.switch_transaction_logs_to_cloud_storage_enabled
instance_out_of_disk_report = instance.out_of_disk_report
instance_psc_service_attachment_link = instance.psc_service_attachment_link
instance_satisfies_pzs = instance.satisfies_pzs
instance_backend_type = instance.backend_type
instance_database_version = instance.database_version
instance_gemini_config = instance.gemini_config
instance_nodes = instance.nodes
instance_tags = instance.tags
instance_database_installed_version = instance.database_installed_version
instance_replication_cluster = instance.replication_cluster
instance_gce_zone = instance.gce_zone
instance_project = instance.project
instance_upgradable_database_versions = instance.upgradable_database_versions
```

---


### Database

Inserts a resource containing information about a database inside a Cloud SQL instance. **Note:** You can't modify the default character set and collation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the database in the Cloud SQL instance. This does not include the project ID or instance name. |
| `kind` | String |  | This is always `sql#database`. |
| `self_link` | String |  | The URI of this resource. |
| `instance` | String |  | The name of the Cloud SQL instance. This does not include the project ID. |
| `project` | String |  | The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable. |
| `charset` | String |  | The Cloud SQL charset value. |
| `etag` | String |  | This field is deprecated and will be removed from a future version of the API. |
| `collation` | String |  | The Cloud SQL collation value. |
| `sqlserver_database_details` | String |  |  |
| `project` | String | ✅ | Project ID of the project that contains the instance. |
| `instance` | String | ✅ | Database instance ID. This does not include the project ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the database in the Cloud SQL instance. This does not include the project ID or instance name. |
| `kind` | String | This is always `sql#database`. |
| `self_link` | String | The URI of this resource. |
| `instance` | String | The name of the Cloud SQL instance. This does not include the project ID. |
| `project` | String | The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable. |
| `charset` | String | The Cloud SQL charset value. |
| `etag` | String | This field is deprecated and will be removed from a future version of the API. |
| `collation` | String | The Cloud SQL collation value. |
| `sqlserver_database_details` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.sqladmin_api.Database {
    project = "value"  # Project ID of the project that contains the instance.
    instance = "value"  # Database instance ID. This does not include the project ID.
}

# Access database outputs
database_id = database.id
database_name = database.name
database_kind = database.kind
database_self_link = database.self_link
database_instance = database.instance
database_project = database.project
database_charset = database.charset
database_etag = database.etag
database_collation = database.collation
database_sqlserver_database_details = database.sqlserver_database_details
```

---


### Connect

Generates a short-lived X509 certificate containing the provided public key and signed by a private key specific to the target instance. Users may use the certificate to authenticate as themselves when connecting to the database.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `public_key` | String |  | PEM encoded public key to include in the signed certificate. |
| `access_token` | String |  | Optional. Access token to include in the signed certificate. |
| `valid_duration` | String |  | Optional. If set, it will contain the cert valid duration. |
| `read_time` | String |  | Optional. Optional snapshot read timestamp to trade freshness for performance. |
| `instance` | String | ✅ | Cloud SQL instance ID. This does not include the project ID. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nodes` | Vec<String> | Output only. Entries containing information about each read pool node of the read pool. |
| `server_ca_mode` | String | Specify what type of CA is used for the server certificate. |
| `backend_type` | String | `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database server that is not managed by Google. This property is read-only; use the `tier` property in the `settings` object to determine the database type. |
| `ip_addresses` | Vec<String> | The assigned IP addresses for the instance. |
| `dns_name` | String | The dns name of the instance. |
| `custom_subject_alternative_names` | Vec<String> | Custom subject alternative names for the server certificate. |
| `mdx_protocol_support` | Vec<String> | Optional. Output only. mdx_protocol_support controls how the client uses metadata exchange when connecting to the instance. The values in the list representing parts of the MDX protocol that are supported by this instance. When the list is empty, the instance does not support MDX, so the client must not send an MDX request. The default is empty. |
| `node_count` | i64 | The number of read pool nodes in a read pool. |
| `dns_names` | Vec<String> | Output only. The list of DNS names used by this instance. |
| `kind` | String | This is always `sql#connectSettings`. |
| `database_version` | String | The database engine type and version. The `databaseVersion` field cannot be changed after instance creation. MySQL instances: `MYSQL_8_0`, `MYSQL_5_7` (default), or `MYSQL_5_6`. PostgreSQL instances: `POSTGRES_9_6`, `POSTGRES_10`, `POSTGRES_11`, `POSTGRES_12` (default), `POSTGRES_13`, or `POSTGRES_14`. SQL Server instances: `SQLSERVER_2017_STANDARD` (default), `SQLSERVER_2017_ENTERPRISE`, `SQLSERVER_2017_EXPRESS`, `SQLSERVER_2017_WEB`, `SQLSERVER_2019_STANDARD`, `SQLSERVER_2019_ENTERPRISE`, `SQLSERVER_2019_EXPRESS`, or `SQLSERVER_2019_WEB`. |
| `psc_enabled` | bool | Whether PSC connectivity is enabled for this instance. |
| `region` | String | The cloud region for the instance. For example, `us-central1`, `europe-west1`. The region cannot be changed after instance creation. |
| `server_ca_cert` | String | SSL configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connect
connect = provider.sqladmin_api.Connect {
    instance = "value"  # Cloud SQL instance ID. This does not include the project ID.
    project = "value"  # Project ID of the project that contains the instance.
}

# Access connect outputs
connect_id = connect.id
connect_nodes = connect.nodes
connect_server_ca_mode = connect.server_ca_mode
connect_backend_type = connect.backend_type
connect_ip_addresses = connect.ip_addresses
connect_dns_name = connect.dns_name
connect_custom_subject_alternative_names = connect.custom_subject_alternative_names
connect_mdx_protocol_support = connect.mdx_protocol_support
connect_node_count = connect.node_count
connect_dns_names = connect.dns_names
connect_kind = connect.kind
connect_database_version = connect.database_version
connect_psc_enabled = connect.psc_enabled
connect_region = connect.region
connect_server_ca_cert = connect.server_ca_cert
```

---


### Tier

Lists all available machine types (tiers) for Cloud SQL, for example, `db-custom-1-3840`. For more information, see https://cloud.google.com/sql/pricing.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | This is always `sql#tiersList`. |
| `items` | Vec<String> | List of tiers. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access tier outputs
tier_id = tier.id
tier_kind = tier.kind
tier_items = tier.items
```

---


### Backup

Creates a backup for a Cloud SQL instance. This API can be used only to create on-demand backups.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Output only. This is always `sql#backup`. |
| `error` | String |  | Output only. Information about why the backup operation fails (for example, when the backup state fails). |
| `description` | String |  | The description of this backup. |
| `backup_kind` | String |  | Output only. Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT. |
| `backup_interval` | String |  | Output only. This output contains the following values: start_time: All database writes up to this time are available. end_time: Any database writes after this time aren't available. |
| `satisfies_pzs` | bool |  | Output only. This status indicates whether the backup satisfies PZS. The status is reserved for future use. |
| `type` | String |  | Output only. The type of this backup. The type can be "AUTOMATED", "ON_DEMAND" or “FINAL”. |
| `self_link` | String |  | Output only. The URI of this resource. |
| `instance_deletion_time` | String |  | Optional. Output only. Timestamp in UTC of when the instance associated with this backup is deleted. |
| `instance` | String |  | The name of the source database instance. |
| `database_version` | String |  | Output only. The database version of the instance of at the time this backup was made. |
| `kms_key_version` | String |  | Output only. This output contains the encryption status for a backup and the version of the KMS key that's used to encrypt the Cloud SQL instance. |
| `location` | String |  | The storage location of the backups. The location can be multi-regional. |
| `max_chargeable_bytes` | String |  | Output only. The maximum chargeable bytes for the backup. |
| `backup_run` | String |  | Output only. The mapping to backup run resource used for IAM validations. |
| `kms_key` | String |  | Output only. This output contains the encryption configuration for a backup and the resource name of the KMS key for disk encryption. |
| `expiry_time` | String |  | Backup expiration time. A UTC timestamp of when this backup expired. |
| `instance_settings` | String |  | Optional. Output only. The instance setting of the source instance that's associated with this backup. |
| `name` | String |  | Output only. The resource name of the backup. Format: projects/{project}/backups/{backup}. |
| `satisfies_pzi` | bool |  | Output only. This status indicates whether the backup satisfies PZI. The status is reserved for future use. |
| `state` | String |  | Output only. The status of this backup. |
| `time_zone` | String |  | Output only. This output contains a backup time zone. If a Cloud SQL for SQL Server instance has a different time zone from the backup's time zone, then the restore to the instance doesn't happen. |
| `ttl_days` | String |  | Input only. The time-to-live (TTL) interval for this resource (in days). For example: ttlDays:7, means 7 days from the current time. The expiration time can't exceed 365 days from the time that the backup is created. |
| `parent` | String | ✅ | Required. The parent resource where this backup is created. Format: projects/{project} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Output only. This is always `sql#backup`. |
| `error` | String | Output only. Information about why the backup operation fails (for example, when the backup state fails). |
| `description` | String | The description of this backup. |
| `backup_kind` | String | Output only. Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT. |
| `backup_interval` | String | Output only. This output contains the following values: start_time: All database writes up to this time are available. end_time: Any database writes after this time aren't available. |
| `satisfies_pzs` | bool | Output only. This status indicates whether the backup satisfies PZS. The status is reserved for future use. |
| `type` | String | Output only. The type of this backup. The type can be "AUTOMATED", "ON_DEMAND" or “FINAL”. |
| `self_link` | String | Output only. The URI of this resource. |
| `instance_deletion_time` | String | Optional. Output only. Timestamp in UTC of when the instance associated with this backup is deleted. |
| `instance` | String | The name of the source database instance. |
| `database_version` | String | Output only. The database version of the instance of at the time this backup was made. |
| `kms_key_version` | String | Output only. This output contains the encryption status for a backup and the version of the KMS key that's used to encrypt the Cloud SQL instance. |
| `location` | String | The storage location of the backups. The location can be multi-regional. |
| `max_chargeable_bytes` | String | Output only. The maximum chargeable bytes for the backup. |
| `backup_run` | String | Output only. The mapping to backup run resource used for IAM validations. |
| `kms_key` | String | Output only. This output contains the encryption configuration for a backup and the resource name of the KMS key for disk encryption. |
| `expiry_time` | String | Backup expiration time. A UTC timestamp of when this backup expired. |
| `instance_settings` | String | Optional. Output only. The instance setting of the source instance that's associated with this backup. |
| `name` | String | Output only. The resource name of the backup. Format: projects/{project}/backups/{backup}. |
| `satisfies_pzi` | bool | Output only. This status indicates whether the backup satisfies PZI. The status is reserved for future use. |
| `state` | String | Output only. The status of this backup. |
| `time_zone` | String | Output only. This output contains a backup time zone. If a Cloud SQL for SQL Server instance has a different time zone from the backup's time zone, then the restore to the instance doesn't happen. |
| `ttl_days` | String | Input only. The time-to-live (TTL) interval for this resource (in days). For example: ttlDays:7, means 7 days from the current time. The expiration time can't exceed 365 days from the time that the backup is created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.sqladmin_api.Backup {
    parent = "value"  # Required. The parent resource where this backup is created. Format: projects/{project}
}

# Access backup outputs
backup_id = backup.id
backup_kind = backup.kind
backup_error = backup.error
backup_description = backup.description
backup_backup_kind = backup.backup_kind
backup_backup_interval = backup.backup_interval
backup_satisfies_pzs = backup.satisfies_pzs
backup_type = backup.type
backup_self_link = backup.self_link
backup_instance_deletion_time = backup.instance_deletion_time
backup_instance = backup.instance
backup_database_version = backup.database_version
backup_kms_key_version = backup.kms_key_version
backup_location = backup.location
backup_max_chargeable_bytes = backup.max_chargeable_bytes
backup_backup_run = backup.backup_run
backup_kms_key = backup.kms_key
backup_expiry_time = backup.expiry_time
backup_instance_settings = backup.instance_settings
backup_name = backup.name
backup_satisfies_pzi = backup.satisfies_pzi
backup_state = backup.state
backup_time_zone = backup.time_zone
backup_ttl_days = backup.ttl_days
```

---


### Flag

Lists all available database flags for Cloud SQL instances.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | List of flags. |
| `kind` | String | This is always `sql#flagsList`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access flag outputs
flag_id = flag.id
flag_items = flag.items
flag_kind = flag.kind
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple user resources
user_0 = provider.sqladmin_api.User {
    project = "value-0"
    instance = "value-0"
}
user_1 = provider.sqladmin_api.User {
    project = "value-1"
    instance = "value-1"
}
user_2 = provider.sqladmin_api.User {
    project = "value-2"
    instance = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    user = provider.sqladmin_api.User {
        project = "production-value"
        instance = "production-value"
    }
```

---

## Related Documentation

- [GCP Sqladmin_api Documentation](https://cloud.google.com/sqladmin_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
