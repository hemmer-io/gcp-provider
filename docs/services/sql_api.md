# Sql_api Service



**Resources**: 8

---

## Overview

The sql_api service provides access to 8 resource types:

- [Ssl_cert](#ssl_cert) [CRD]
- [User](#user) [CRUD]
- [Database](#database) [CRUD]
- [Instance](#instance) [CRUD]
- [Tier](#tier) [R]
- [Flag](#flag) [R]
- [Operation](#operation) [R]
- [Backup_run](#backup_run) [CRD]

---

## Resources


### Ssl_cert

Creates an SSL certificate and returns it along with the private key and
server certificate authority.  The new certificate will not be usable until
the instance is restarted.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `common_name` | String |  | User supplied name.  Must be a distinct name from the other certificates
for this instance. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |
| `instance` | String | ✅ | Cloud SQL instance ID. This does not include the project ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cert` | String | PEM representation. |
| `cert_serial_number` | String | Serial number, as extracted from the certificate. |
| `sha1_fingerprint` | String | Sha1 Fingerprint. |
| `kind` | String | This is always <code>sql#sslCert</code>. |
| `create_time` | String | The time when the certificate was created in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code> |
| `common_name` | String | User supplied name.  Constrained to [a-zA-Z.-_ ]+. |
| `instance` | String | Name of the database instance. |
| `self_link` | String | The URI of this resource. |
| `expiration_time` | String | The time when the certificate expires in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code>. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ssl_cert
ssl_cert = provider.sql_api.Ssl_cert {
    project = "value"  # Project ID of the project that contains the instance.
    instance = "value"  # Cloud SQL instance ID. This does not include the project ID.
}

# Access ssl_cert outputs
ssl_cert_id = ssl_cert.id
ssl_cert_cert = ssl_cert.cert
ssl_cert_cert_serial_number = ssl_cert.cert_serial_number
ssl_cert_sha1_fingerprint = ssl_cert.sha1_fingerprint
ssl_cert_kind = ssl_cert.kind
ssl_cert_create_time = ssl_cert.create_time
ssl_cert_common_name = ssl_cert.common_name
ssl_cert_instance = ssl_cert.instance
ssl_cert_self_link = ssl_cert.self_link
ssl_cert_expiration_time = ssl_cert.expiration_time
```

---


### User

Creates a new user in a Cloud SQL instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project` | String |  | The project ID of the project containing the Cloud SQL database. The Google
apps domain is prefixed if applicable. Can be omitted for
<code>update</code> since it is already specified on the URL. |
| `name` | String |  | The name of the user in the Cloud SQL instance. Can be omitted for
<code>update</code> since it is already specified in the URL. |
| `sqlserver_user_details` | String |  |  |
| `etag` | String |  | This field is deprecated and will be removed from a future version of the
API. |
| `host` | String |  | The host name from which the user can connect. For <code>insert</code>
operations, host defaults to an empty string. For <code>update</code>
operations, host is specified as part of the request URL. The host name
cannot be updated after insertion. |
| `password` | String |  | The password for the user. |
| `instance` | String |  | The name of the Cloud SQL instance. This does not include the project ID.
Can be omitted for <code>update</code> since it is already specified on the
URL. |
| `kind` | String |  | This is always <code>sql#user</code>. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |
| `instance` | String | ✅ | Database instance ID. This does not include the project ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | This is always <code>sql#usersList</code>. |
| `next_page_token` | String | An identifier that uniquely identifies the operation. You can use this
identifier to retrieve the Operations resource that has information about
the operation. |
| `items` | Vec<String> | List of user resources in the instance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.sql_api.User {
    project = "value"  # Project ID of the project that contains the instance.
    instance = "value"  # Database instance ID. This does not include the project ID.
}

# Access user outputs
user_id = user.id
user_kind = user.kind
user_next_page_token = user.next_page_token
user_items = user.items
```

---


### Database

Inserts a resource containing information about a database inside a Cloud
SQL instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `charset` | String |  | The MySQL charset value. |
| `etag` | String |  | This field is deprecated and will be removed from a future version of the
API. |
| `kind` | String |  | This is always <code>sql#database</code>. |
| `name` | String |  | The name of the database in the Cloud SQL instance. This does not include
the project ID or instance name. |
| `sqlserver_database_details` | String |  |  |
| `instance` | String |  | The name of the Cloud SQL instance. This does not include the project ID. |
| `collation` | String |  | The MySQL collation value. |
| `self_link` | String |  | The URI of this resource. |
| `project` | String |  | The project ID of the project containing the Cloud SQL database. The Google
apps domain is prefixed if applicable. |
| `instance` | String | ✅ | Database instance ID. This does not include the project ID. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `charset` | String | The MySQL charset value. |
| `etag` | String | This field is deprecated and will be removed from a future version of the
API. |
| `kind` | String | This is always <code>sql#database</code>. |
| `name` | String | The name of the database in the Cloud SQL instance. This does not include
the project ID or instance name. |
| `sqlserver_database_details` | String |  |
| `instance` | String | The name of the Cloud SQL instance. This does not include the project ID. |
| `collation` | String | The MySQL collation value. |
| `self_link` | String | The URI of this resource. |
| `project` | String | The project ID of the project containing the Cloud SQL database. The Google
apps domain is prefixed if applicable. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.sql_api.Database {
    instance = "value"  # Database instance ID. This does not include the project ID.
    project = "value"  # Project ID of the project that contains the instance.
}

# Access database outputs
database_id = database.id
database_charset = database.charset
database_etag = database.etag
database_kind = database.kind
database_name = database.name
database_sqlserver_database_details = database.sqlserver_database_details
database_instance = database.instance
database_collation = database.collation
database_self_link = database.self_link
database_project = database.project
```

---


### Instance

Creates a new Cloud SQL instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `root_password` | String |  | Initial root password. Use only on creation. |
| `current_disk_size` | String |  | The current disk usage of the instance in bytes. This property has been
deprecated. Users should use the
"cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud
Monitoring API instead. Please see <a
href="https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ">this
announcement</a> for details. |
| `ip_addresses` | Vec<String> |  | The assigned IP addresses for the instance. |
| `region` | String |  | The geographical region. Can be <code>us-central</code>
(<code>FIRST_GEN</code> instances only), <code>us-central1</code>
(<code>SECOND_GEN</code> instances only), <code>asia-east1</code> or
<code>europe-west1</code>. Defaults to <code>us-central</code> or
<code>us-central1</code> depending on the instance type (First Generation
or Second Generation). The region can not be changed after instance
creation. |
| `state` | String |  | The current serving state of the Cloud SQL instance. This can be one of the
following. <br><code>RUNNABLE</code>: The instance is running, or is ready
to run when accessed. <br><code>SUSPENDED</code>: The instance is not
available, for example due to problems with billing.
<br><code>PENDING_CREATE</code>: The instance is being created.
<br><code>MAINTENANCE</code>: The instance is down for maintenance.
<br><code>FAILED</code>: The instance creation failed.
<br><code>UNKNOWN_STATE</code>: The state of the instance is unknown. |
| `disk_encryption_configuration` | String |  | Disk encryption configuration specific to an instance.
Applies only to Second Generation instances. |
| `connection_name` | String |  | Connection name of the Cloud SQL instance used in connection strings. |
| `master_instance_name` | String |  | The name of the instance which will act as master in the replication setup. |
| `max_disk_size` | String |  | The maximum disk size of the instance in bytes. |
| `failover_replica` | String |  | The name and status of the failover replica. This property is applicable
only to Second Generation instances. |
| `kind` | String |  | This is always <code>sql#instance</code>. |
| `replica_names` | Vec<String> |  | The replicas of the instance. |
| `disk_encryption_status` | String |  | Disk encryption status specific to an instance.
Applies only to Second Generation instances. |
| `instance_type` | String |  | The instance type. This can be one of the following.
<br><code>CLOUD_SQL_INSTANCE</code>: A Cloud SQL instance that is not
replicating from a master. <br><code>ON_PREMISES_INSTANCE</code>: An
instance running on the
customer's premises. <br><code>READ_REPLICA_INSTANCE</code>: A Cloud SQL
instance configured as a read-replica. |
| `scheduled_maintenance` | String |  | The start time of any upcoming scheduled maintenance for this instance. |
| `suspension_reason` | Vec<String> |  | If the instance state is SUSPENDED, the reason for the suspension. |
| `project` | String |  | The project ID of the project containing the Cloud SQL instance. The Google
apps domain is prefixed if applicable. |
| `backend_type` | String |  | <code>FIRST_GEN</code>: First Generation instance. MySQL only. <br
/><code>SECOND_GEN</code>: Second Generation instance or PostgreSQL
instance. <br /><code>EXTERNAL</code>: A database server that is not
managed by Google. <br>This property is read-only; use the
<code>tier</code> property in the <code>settings</code> object to determine
the database type and Second or First Generation. |
| `gce_zone` | String |  | The Compute Engine zone that the instance is currently serving from. This
value could be different from the zone that was specified when the instance
was created if the instance has failed over to its secondary zone. |
| `ipv6_address` | String |  | The IPv6 address assigned to the instance. This property is applicable only
to First Generation instances. |
| `replica_configuration` | String |  | Configuration specific to failover replicas and read replicas. |
| `etag` | String |  | This field is deprecated and will be removed from a future version of the
API. Use the <code>settings.settingsVersion</code> field instead. |
| `service_account_email_address` | String |  | The service account email address assigned to the instance. This property
is applicable only to Second Generation instances. |
| `name` | String |  | Name of the Cloud SQL instance. This does not include the project ID. |
| `settings` | String |  | The user settings. |
| `server_ca_cert` | String |  | SSL configuration. |
| `self_link` | String |  | The URI of this resource. |
| `on_premises_configuration` | String |  | Configuration specific to on-premises instances. |
| `database_version` | String |  | The database engine type and version. The <code>databaseVersion</code>
field can not be changed after instance creation.  MySQL Second Generation
instances: <code>MYSQL_5_7</code> (default) or <code>MYSQL_5_6</code>.
PostgreSQL instances: <code>POSTGRES_9_6</code> (default) or
<code>POSTGRES_11 Beta</code> MySQL First Generation
instances: <code>MYSQL_5_6</code> (default) or <code>MYSQL_5_5</code> |
| `project` | String | ✅ | Project ID of the project to which the newly created Cloud SQL instances
should belong. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `root_password` | String | Initial root password. Use only on creation. |
| `current_disk_size` | String | The current disk usage of the instance in bytes. This property has been
deprecated. Users should use the
"cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud
Monitoring API instead. Please see <a
href="https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ">this
announcement</a> for details. |
| `ip_addresses` | Vec<String> | The assigned IP addresses for the instance. |
| `region` | String | The geographical region. Can be <code>us-central</code>
(<code>FIRST_GEN</code> instances only), <code>us-central1</code>
(<code>SECOND_GEN</code> instances only), <code>asia-east1</code> or
<code>europe-west1</code>. Defaults to <code>us-central</code> or
<code>us-central1</code> depending on the instance type (First Generation
or Second Generation). The region can not be changed after instance
creation. |
| `state` | String | The current serving state of the Cloud SQL instance. This can be one of the
following. <br><code>RUNNABLE</code>: The instance is running, or is ready
to run when accessed. <br><code>SUSPENDED</code>: The instance is not
available, for example due to problems with billing.
<br><code>PENDING_CREATE</code>: The instance is being created.
<br><code>MAINTENANCE</code>: The instance is down for maintenance.
<br><code>FAILED</code>: The instance creation failed.
<br><code>UNKNOWN_STATE</code>: The state of the instance is unknown. |
| `disk_encryption_configuration` | String | Disk encryption configuration specific to an instance.
Applies only to Second Generation instances. |
| `connection_name` | String | Connection name of the Cloud SQL instance used in connection strings. |
| `master_instance_name` | String | The name of the instance which will act as master in the replication setup. |
| `max_disk_size` | String | The maximum disk size of the instance in bytes. |
| `failover_replica` | String | The name and status of the failover replica. This property is applicable
only to Second Generation instances. |
| `kind` | String | This is always <code>sql#instance</code>. |
| `replica_names` | Vec<String> | The replicas of the instance. |
| `disk_encryption_status` | String | Disk encryption status specific to an instance.
Applies only to Second Generation instances. |
| `instance_type` | String | The instance type. This can be one of the following.
<br><code>CLOUD_SQL_INSTANCE</code>: A Cloud SQL instance that is not
replicating from a master. <br><code>ON_PREMISES_INSTANCE</code>: An
instance running on the
customer's premises. <br><code>READ_REPLICA_INSTANCE</code>: A Cloud SQL
instance configured as a read-replica. |
| `scheduled_maintenance` | String | The start time of any upcoming scheduled maintenance for this instance. |
| `suspension_reason` | Vec<String> | If the instance state is SUSPENDED, the reason for the suspension. |
| `project` | String | The project ID of the project containing the Cloud SQL instance. The Google
apps domain is prefixed if applicable. |
| `backend_type` | String | <code>FIRST_GEN</code>: First Generation instance. MySQL only. <br
/><code>SECOND_GEN</code>: Second Generation instance or PostgreSQL
instance. <br /><code>EXTERNAL</code>: A database server that is not
managed by Google. <br>This property is read-only; use the
<code>tier</code> property in the <code>settings</code> object to determine
the database type and Second or First Generation. |
| `gce_zone` | String | The Compute Engine zone that the instance is currently serving from. This
value could be different from the zone that was specified when the instance
was created if the instance has failed over to its secondary zone. |
| `ipv6_address` | String | The IPv6 address assigned to the instance. This property is applicable only
to First Generation instances. |
| `replica_configuration` | String | Configuration specific to failover replicas and read replicas. |
| `etag` | String | This field is deprecated and will be removed from a future version of the
API. Use the <code>settings.settingsVersion</code> field instead. |
| `service_account_email_address` | String | The service account email address assigned to the instance. This property
is applicable only to Second Generation instances. |
| `name` | String | Name of the Cloud SQL instance. This does not include the project ID. |
| `settings` | String | The user settings. |
| `server_ca_cert` | String | SSL configuration. |
| `self_link` | String | The URI of this resource. |
| `on_premises_configuration` | String | Configuration specific to on-premises instances. |
| `database_version` | String | The database engine type and version. The <code>databaseVersion</code>
field can not be changed after instance creation.  MySQL Second Generation
instances: <code>MYSQL_5_7</code> (default) or <code>MYSQL_5_6</code>.
PostgreSQL instances: <code>POSTGRES_9_6</code> (default) or
<code>POSTGRES_11 Beta</code> MySQL First Generation
instances: <code>MYSQL_5_6</code> (default) or <code>MYSQL_5_5</code> |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.sql_api.Instance {
    project = "value"  # Project ID of the project to which the newly created Cloud SQL instances
should belong.
}

# Access instance outputs
instance_id = instance.id
instance_root_password = instance.root_password
instance_current_disk_size = instance.current_disk_size
instance_ip_addresses = instance.ip_addresses
instance_region = instance.region
instance_state = instance.state
instance_disk_encryption_configuration = instance.disk_encryption_configuration
instance_connection_name = instance.connection_name
instance_master_instance_name = instance.master_instance_name
instance_max_disk_size = instance.max_disk_size
instance_failover_replica = instance.failover_replica
instance_kind = instance.kind
instance_replica_names = instance.replica_names
instance_disk_encryption_status = instance.disk_encryption_status
instance_instance_type = instance.instance_type
instance_scheduled_maintenance = instance.scheduled_maintenance
instance_suspension_reason = instance.suspension_reason
instance_project = instance.project
instance_backend_type = instance.backend_type
instance_gce_zone = instance.gce_zone
instance_ipv6_address = instance.ipv6_address
instance_replica_configuration = instance.replica_configuration
instance_etag = instance.etag
instance_service_account_email_address = instance.service_account_email_address
instance_name = instance.name
instance_settings = instance.settings
instance_server_ca_cert = instance.server_ca_cert
instance_self_link = instance.self_link
instance_on_premises_configuration = instance.on_premises_configuration
instance_database_version = instance.database_version
```

---


### Tier

Lists all available machine types (tiers) for Cloud SQL, for example,
db-n1-standard-1. For related information, see <a
href="/sql/pricing">Pricing</a>.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | This is always <code>sql#tiersList</code>. |
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


### Flag

List all available database flags for Cloud SQL instances.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | This is always <code>sql#flagsList</code>. |
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


### Operation

Retrieves an instance operation that has been performed on an instance.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | This is always <code>sql#operation</code>. |
| `self_link` | String | The URI of this resource. |
| `target_id` | String | Name of the database instance related to this operation. |
| `target_project` | String | The project ID of the target instance related to this operation. |
| `user` | String | The email address of the user who initiated this operation. |
| `name` | String | An identifier that uniquely identifies the operation. You can use this
identifier to retrieve the Operations resource that has information about
the operation. |
| `operation_type` | String | The type of the operation. Valid values are <code>CREATE</code>,
<code>DELETE</code>, <code>UPDATE</code>, <code>RESTART</code>,
<code>IMPORT</code>, <code>EXPORT</code>, <code>BACKUP_VOLUME</code>,
<code>RESTORE_VOLUME</code>, <code>CREATE_USER</code>,
<code>DELETE_USER</code>, <code>CREATE_DATABASE</code>,
<code>DELETE_DATABASE</code> . |
| `error` | String | If errors occurred during processing of this operation, this field will be
populated. |
| `status` | String | The status of an operation. Valid values are <code>PENDING</code>,
<code>RUNNING</code>, <code>DONE</code>,
<code>SQL_OPERATION_STATUS_UNSPECIFIED</code>. |
| `end_time` | String | The time this operation finished in UTC timezone in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code>. |
| `target_link` | String |  |
| `insert_time` | String | The time this operation was enqueued in UTC timezone in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code>. |
| `export_context` | String | The context for export operation, if applicable. |
| `start_time` | String | The time this operation actually started in UTC timezone in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code>. |
| `import_context` | String | The context for import operation, if applicable. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_kind = operation.kind
operation_self_link = operation.self_link
operation_target_id = operation.target_id
operation_target_project = operation.target_project
operation_user = operation.user
operation_name = operation.name
operation_operation_type = operation.operation_type
operation_error = operation.error
operation_status = operation.status
operation_end_time = operation.end_time
operation_target_link = operation.target_link
operation_insert_time = operation.insert_time
operation_export_context = operation.export_context
operation_start_time = operation.start_time
operation_import_context = operation.import_context
```

---


### Backup_run

Creates a new backup run on demand. This method is applicable only to
Second Generation instances.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | The type of this run; can be either "AUTOMATED" or "ON_DEMAND". |
| `end_time` | String |  | The time the backup operation completed in UTC timezone in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code>. |
| `window_start_time` | String |  | The start time of the backup window during which this the backup was
attempted in <a href="https://tools.ietf.org/html/rfc3339">RFC 3339</a>
format, for example <code>2012-11-15T16:19:00.094Z</code>. |
| `id` | String |  | The identifier for this backup run. Unique only for a specific Cloud SQL
instance. |
| `description` | String |  | The description of this run, only applicable to on-demand backups. |
| `instance` | String |  | Name of the database instance. |
| `enqueued_time` | String |  | The time the run was enqueued in UTC timezone in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code>. |
| `location` | String |  | Location of the backups. |
| `error` | String |  | Information about why the backup operation failed. This is only present if
the run has the FAILED status. |
| `kind` | String |  | This is always <code>sql#backupRun</code>. |
| `disk_encryption_configuration` | String |  | Encryption configuration specific to a backup.
Applies only to Second Generation instances. |
| `disk_encryption_status` | String |  | Encryption status specific to a backup.
Applies only to Second Generation instances. |
| `start_time` | String |  | The time the backup operation actually started in UTC timezone in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code>. |
| `self_link` | String |  | The URI of this resource. |
| `status` | String |  | The status of this run. |
| `instance` | String | ✅ | Cloud SQL instance ID. This does not include the project ID. |
| `project` | String | ✅ | Project ID of the project that contains the instance. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | The type of this run; can be either "AUTOMATED" or "ON_DEMAND". |
| `end_time` | String | The time the backup operation completed in UTC timezone in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code>. |
| `window_start_time` | String | The start time of the backup window during which this the backup was
attempted in <a href="https://tools.ietf.org/html/rfc3339">RFC 3339</a>
format, for example <code>2012-11-15T16:19:00.094Z</code>. |
| `id` | String | The identifier for this backup run. Unique only for a specific Cloud SQL
instance. |
| `description` | String | The description of this run, only applicable to on-demand backups. |
| `instance` | String | Name of the database instance. |
| `enqueued_time` | String | The time the run was enqueued in UTC timezone in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code>. |
| `location` | String | Location of the backups. |
| `error` | String | Information about why the backup operation failed. This is only present if
the run has the FAILED status. |
| `kind` | String | This is always <code>sql#backupRun</code>. |
| `disk_encryption_configuration` | String | Encryption configuration specific to a backup.
Applies only to Second Generation instances. |
| `disk_encryption_status` | String | Encryption status specific to a backup.
Applies only to Second Generation instances. |
| `start_time` | String | The time the backup operation actually started in UTC timezone in <a
href="https://tools.ietf.org/html/rfc3339">RFC 3339</a> format, for example
<code>2012-11-15T16:19:00.094Z</code>. |
| `self_link` | String | The URI of this resource. |
| `status` | String | The status of this run. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_run
backup_run = provider.sql_api.Backup_run {
    instance = "value"  # Cloud SQL instance ID. This does not include the project ID.
    project = "value"  # Project ID of the project that contains the instance.
}

# Access backup_run outputs
backup_run_id = backup_run.id
backup_run_type = backup_run.type
backup_run_end_time = backup_run.end_time
backup_run_window_start_time = backup_run.window_start_time
backup_run_id = backup_run.id
backup_run_description = backup_run.description
backup_run_instance = backup_run.instance
backup_run_enqueued_time = backup_run.enqueued_time
backup_run_location = backup_run.location
backup_run_error = backup_run.error
backup_run_kind = backup_run.kind
backup_run_disk_encryption_configuration = backup_run.disk_encryption_configuration
backup_run_disk_encryption_status = backup_run.disk_encryption_status
backup_run_start_time = backup_run.start_time
backup_run_self_link = backup_run.self_link
backup_run_status = backup_run.status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple ssl_cert resources
ssl_cert_0 = provider.sql_api.Ssl_cert {
    project = "value-0"
    instance = "value-0"
}
ssl_cert_1 = provider.sql_api.Ssl_cert {
    project = "value-1"
    instance = "value-1"
}
ssl_cert_2 = provider.sql_api.Ssl_cert {
    project = "value-2"
    instance = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    ssl_cert = provider.sql_api.Ssl_cert {
        project = "production-value"
        instance = "production-value"
    }
```

---

## Related Documentation

- [GCP Sql_api Documentation](https://cloud.google.com/sql_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
