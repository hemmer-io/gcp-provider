# Firestore_api Service



**Resources**: 15

---

## Overview

The firestore_api service provides access to 15 resource types:

- [User_cred](#user_cred) [CRD]
- [Database](#database) [CRUD]
- [Backup_schedule](#backup_schedule) [CRUD]
- [Field](#field) [RU]
- [Location](#location) [R]
- [Backup](#backup) [RD]
- [Operation](#operation) [CRD]
- [Document](#document) [CRUD]
- [Indexe](#indexe) [CRD]
- [Database](#database) [C]
- [Indexe](#indexe) [CRD]
- [Field](#field) [RU]
- [Indexe](#indexe) [CRD]
- [Database](#database) [C]
- [Document](#document) [CRUD]

---

## Resources


### User_cred

Create a user creds.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the UserCreds. Format: `projects/{project}/databases/{database}/userCreds/{user_creds}` |
| `state` | String |  | Output only. Whether the user creds are enabled or disabled. Defaults to ENABLED on creation. |
| `update_time` | String |  | Output only. The time the user creds were last updated. |
| `create_time` | String |  | Output only. The time the user creds were created. |
| `resource_identity` | String |  | Resource Identity descriptor. |
| `secure_password` | String |  | Output only. The plaintext server-generated password for the user creds. Only populated in responses for CreateUserCreds and ResetUserPassword. |
| `parent` | String | ✅ | Required. A parent name of the form `projects/{project_id}/databases/{database_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the UserCreds. Format: `projects/{project}/databases/{database}/userCreds/{user_creds}` |
| `state` | String | Output only. Whether the user creds are enabled or disabled. Defaults to ENABLED on creation. |
| `update_time` | String | Output only. The time the user creds were last updated. |
| `create_time` | String | Output only. The time the user creds were created. |
| `resource_identity` | String | Resource Identity descriptor. |
| `secure_password` | String | Output only. The plaintext server-generated password for the user creds. Only populated in responses for CreateUserCreds and ResetUserPassword. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_cred
user_cred = provider.firestore_api.User_cred {
    parent = "value"  # Required. A parent name of the form `projects/{project_id}/databases/{database_id}`
}

# Access user_cred outputs
user_cred_id = user_cred.id
user_cred_name = user_cred.name
user_cred_state = user_cred.state
user_cred_update_time = user_cred.update_time
user_cred_create_time = user_cred.create_time
user_cred_resource_identity = user_cred.resource_identity
user_cred_secure_password = user_cred.secure_password
```

---


### Database

Create a database.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `point_in_time_recovery_enablement` | String |  | Whether to enable the PITR feature on this database. |
| `firestore_data_access_mode` | String |  | Optional. The Firestore API data access mode to use for this database. If not set on write: - the default value is DATA_ACCESS_MODE_DISABLED for Enterprise Edition. - the default value is DATA_ACCESS_MODE_ENABLED for Standard Edition. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `mongodb_compatible_data_access_mode` | String |  | Optional. The MongoDB compatible API data access mode to use for this database. If not set on write, the default value is DATA_ACCESS_MODE_ENABLED for Enterprise Edition. The value is always DATA_ACCESS_MODE_DISABLED for Standard Edition. |
| `uid` | String |  | Output only. The system-generated UUID4 for this Database. |
| `concurrency_mode` | String |  | The concurrency control mode to use for this database. |
| `earliest_version_time` | String |  | Output only. The earliest timestamp at which older versions of the data can be read from the database. See [version_retention_period] above; this field is populated with `now - version_retention_period`. This value is continuously updated, and becomes stale the moment it is queried. If you are using this value to recover data, make sure to account for the time from the moment when the value is queried to the moment when you initiate the recovery. |
| `location_id` | String |  | The location of the database. Available locations are listed at https://cloud.google.com/firestore/docs/locations. |
| `source_info` | String |  | Output only. Information about the provenance of this database. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `type` | String |  | The type of the database. See https://cloud.google.com/datastore/docs/firestore-or-datastore for information about how to choose. |
| `cmek_config` | String |  | Optional. Presence indicates CMEK is enabled for this database. |
| `version_retention_period` | String |  | Output only. The period during which past versions of data are retained in the database. Any read or query can specify a `read_time` within this window, and will read the state of the database at that time. If the PITR feature is enabled, the retention period is 7 days. Otherwise, the retention period is 1 hour. |
| `create_time` | String |  | Output only. The timestamp at which this database was created. Databases created before 2016 do not populate create_time. |
| `update_time` | String |  | Output only. The timestamp at which this database was most recently updated. Note this only includes updates to the database resource and not data contained by the database. |
| `app_engine_integration_mode` | String |  | The App Engine integration mode to use for this database. |
| `delete_protection_state` | String |  | State of delete protection for the database. |
| `realtime_updates_mode` | String |  | Immutable. The default Realtime Updates mode to use for this database. |
| `delete_time` | String |  | Output only. The timestamp at which this database was deleted. Only set if the database has been deleted. |
| `free_tier` | bool |  | Output only. Background: Free tier is the ability of a Firestore database to use a small amount of resources every day without being charged. Once usage exceeds the free tier limit further usage is charged. Whether this database can make use of the free tier. Only one database per project can be eligible for the free tier. The first (or next) database that is created in a project without a free tier database will be marked as eligible for the free tier. Databases that are created while there is a free tier database will not be eligible for the free tier. |
| `name` | String |  | The resource name of the Database. Format: `projects/{project}/databases/{database}` |
| `key_prefix` | String |  | Output only. The key_prefix for this database. This key_prefix is used, in combination with the project ID ("~") to construct the application ID that is returned from the Cloud Datastore APIs in Google App Engine first generation runtimes. This value may be empty in which case the appid to use for URL-encoded keys is the project_id (eg: foo instead of v~foo). |
| `database_edition` | String |  | Immutable. The edition of the database. |
| `previous_id` | String |  | Output only. The database resource's prior database ID. This field is only populated for deleted databases. |
| `parent` | String | ✅ | Required. A parent name of the form `projects/{project_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `point_in_time_recovery_enablement` | String | Whether to enable the PITR feature on this database. |
| `firestore_data_access_mode` | String | Optional. The Firestore API data access mode to use for this database. If not set on write: - the default value is DATA_ACCESS_MODE_DISABLED for Enterprise Edition. - the default value is DATA_ACCESS_MODE_ENABLED for Standard Edition. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `mongodb_compatible_data_access_mode` | String | Optional. The MongoDB compatible API data access mode to use for this database. If not set on write, the default value is DATA_ACCESS_MODE_ENABLED for Enterprise Edition. The value is always DATA_ACCESS_MODE_DISABLED for Standard Edition. |
| `uid` | String | Output only. The system-generated UUID4 for this Database. |
| `concurrency_mode` | String | The concurrency control mode to use for this database. |
| `earliest_version_time` | String | Output only. The earliest timestamp at which older versions of the data can be read from the database. See [version_retention_period] above; this field is populated with `now - version_retention_period`. This value is continuously updated, and becomes stale the moment it is queried. If you are using this value to recover data, make sure to account for the time from the moment when the value is queried to the moment when you initiate the recovery. |
| `location_id` | String | The location of the database. Available locations are listed at https://cloud.google.com/firestore/docs/locations. |
| `source_info` | String | Output only. Information about the provenance of this database. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `type` | String | The type of the database. See https://cloud.google.com/datastore/docs/firestore-or-datastore for information about how to choose. |
| `cmek_config` | String | Optional. Presence indicates CMEK is enabled for this database. |
| `version_retention_period` | String | Output only. The period during which past versions of data are retained in the database. Any read or query can specify a `read_time` within this window, and will read the state of the database at that time. If the PITR feature is enabled, the retention period is 7 days. Otherwise, the retention period is 1 hour. |
| `create_time` | String | Output only. The timestamp at which this database was created. Databases created before 2016 do not populate create_time. |
| `update_time` | String | Output only. The timestamp at which this database was most recently updated. Note this only includes updates to the database resource and not data contained by the database. |
| `app_engine_integration_mode` | String | The App Engine integration mode to use for this database. |
| `delete_protection_state` | String | State of delete protection for the database. |
| `realtime_updates_mode` | String | Immutable. The default Realtime Updates mode to use for this database. |
| `delete_time` | String | Output only. The timestamp at which this database was deleted. Only set if the database has been deleted. |
| `free_tier` | bool | Output only. Background: Free tier is the ability of a Firestore database to use a small amount of resources every day without being charged. Once usage exceeds the free tier limit further usage is charged. Whether this database can make use of the free tier. Only one database per project can be eligible for the free tier. The first (or next) database that is created in a project without a free tier database will be marked as eligible for the free tier. Databases that are created while there is a free tier database will not be eligible for the free tier. |
| `name` | String | The resource name of the Database. Format: `projects/{project}/databases/{database}` |
| `key_prefix` | String | Output only. The key_prefix for this database. This key_prefix is used, in combination with the project ID ("~") to construct the application ID that is returned from the Cloud Datastore APIs in Google App Engine first generation runtimes. This value may be empty in which case the appid to use for URL-encoded keys is the project_id (eg: foo instead of v~foo). |
| `database_edition` | String | Immutable. The edition of the database. |
| `previous_id` | String | Output only. The database resource's prior database ID. This field is only populated for deleted databases. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.firestore_api.Database {
    parent = "value"  # Required. A parent name of the form `projects/{project_id}`
}

# Access database outputs
database_id = database.id
database_point_in_time_recovery_enablement = database.point_in_time_recovery_enablement
database_firestore_data_access_mode = database.firestore_data_access_mode
database_etag = database.etag
database_mongodb_compatible_data_access_mode = database.mongodb_compatible_data_access_mode
database_uid = database.uid
database_concurrency_mode = database.concurrency_mode
database_earliest_version_time = database.earliest_version_time
database_location_id = database.location_id
database_source_info = database.source_info
database_tags = database.tags
database_type = database.type
database_cmek_config = database.cmek_config
database_version_retention_period = database.version_retention_period
database_create_time = database.create_time
database_update_time = database.update_time
database_app_engine_integration_mode = database.app_engine_integration_mode
database_delete_protection_state = database.delete_protection_state
database_realtime_updates_mode = database.realtime_updates_mode
database_delete_time = database.delete_time
database_free_tier = database.free_tier
database_name = database.name
database_key_prefix = database.key_prefix
database_database_edition = database.database_edition
database_previous_id = database.previous_id
```

---


### Backup_schedule

Creates a backup schedule on a database. At most two backup schedules can be configured on a database, one daily backup schedule and one weekly backup schedule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The unique backup schedule identifier across all locations and databases for the given project. This will be auto-assigned. Format is `projects/{project}/databases/{database}/backupSchedules/{backup_schedule}` |
| `weekly_recurrence` | String |  | For a schedule that runs weekly on a specific day. |
| `create_time` | String |  | Output only. The timestamp at which this backup schedule was created and effective since. No backups will be created for this schedule before this time. |
| `daily_recurrence` | String |  | For a schedule that runs daily. |
| `retention` | String |  | At what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days. The maximum supported retention period is 14 weeks. |
| `update_time` | String |  | Output only. The timestamp at which this backup schedule was most recently updated. When a backup schedule is first created, this is the same as create_time. |
| `parent` | String | ✅ | Required. The parent database. Format `projects/{project}/databases/{database}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The unique backup schedule identifier across all locations and databases for the given project. This will be auto-assigned. Format is `projects/{project}/databases/{database}/backupSchedules/{backup_schedule}` |
| `weekly_recurrence` | String | For a schedule that runs weekly on a specific day. |
| `create_time` | String | Output only. The timestamp at which this backup schedule was created and effective since. No backups will be created for this schedule before this time. |
| `daily_recurrence` | String | For a schedule that runs daily. |
| `retention` | String | At what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days. The maximum supported retention period is 14 weeks. |
| `update_time` | String | Output only. The timestamp at which this backup schedule was most recently updated. When a backup schedule is first created, this is the same as create_time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_schedule
backup_schedule = provider.firestore_api.Backup_schedule {
    parent = "value"  # Required. The parent database. Format `projects/{project}/databases/{database}`
}

# Access backup_schedule outputs
backup_schedule_id = backup_schedule.id
backup_schedule_name = backup_schedule.name
backup_schedule_weekly_recurrence = backup_schedule.weekly_recurrence
backup_schedule_create_time = backup_schedule.create_time
backup_schedule_daily_recurrence = backup_schedule.daily_recurrence
backup_schedule_retention = backup_schedule.retention
backup_schedule_update_time = backup_schedule.update_time
```

---


### Field

Gets the metadata and configuration for a Field.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ttl_config` | String |  | The TTL configuration for this `Field`. Setting or unsetting this will enable or disable the TTL for documents that have this `Field`. |
| `index_config` | String |  | The index configuration for this field. If unset, field indexing will revert to the configuration defined by the `ancestor_field`. To explicitly remove all indexes for this field, specify an index config with an empty list of indexes. |
| `name` | String |  | Required. A field name of the form: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}` A field path can be a simple field name, e.g. `address` or a path to fields within `map_value` , e.g. `address.city`, or a special field path. The only valid special field is `*`, which represents any field. Field paths can be quoted using `` ` `` (backtick). The only character that must be escaped within a quoted field path is the backtick character itself, escaped using a backslash. Special characters in field paths that must be quoted include: `*`, `.`, `` ` `` (backtick), `[`, `]`, as well as any ascii symbolic characters. Examples: `` `address.city` `` represents a field named `address.city`, not the map key `city` in the field `address`. `` `*` `` represents a field named `*`, not any field. A special `Field` contains the default indexing settings for all fields. This field's resource name is: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*` Indexes defined on this `Field` will be applied to all fields which do not have their own `Field` index configuration. |
| `name` | String | ✅ | Required. A field name of the form: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}` A field path can be a simple field name, e.g. `address` or a path to fields within `map_value` , e.g. `address.city`, or a special field path. The only valid special field is `*`, which represents any field. Field paths can be quoted using `` ` `` (backtick). The only character that must be escaped within a quoted field path is the backtick character itself, escaped using a backslash. Special characters in field paths that must be quoted include: `*`, `.`, `` ` `` (backtick), `[`, `]`, as well as any ascii symbolic characters. Examples: `` `address.city` `` represents a field named `address.city`, not the map key `city` in the field `address`. `` `*` `` represents a field named `*`, not any field. A special `Field` contains the default indexing settings for all fields. This field's resource name is: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*` Indexes defined on this `Field` will be applied to all fields which do not have their own `Field` index configuration. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ttl_config` | String | The TTL configuration for this `Field`. Setting or unsetting this will enable or disable the TTL for documents that have this `Field`. |
| `index_config` | String | The index configuration for this field. If unset, field indexing will revert to the configuration defined by the `ancestor_field`. To explicitly remove all indexes for this field, specify an index config with an empty list of indexes. |
| `name` | String | Required. A field name of the form: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}` A field path can be a simple field name, e.g. `address` or a path to fields within `map_value` , e.g. `address.city`, or a special field path. The only valid special field is `*`, which represents any field. Field paths can be quoted using `` ` `` (backtick). The only character that must be escaped within a quoted field path is the backtick character itself, escaped using a backslash. Special characters in field paths that must be quoted include: `*`, `.`, `` ` `` (backtick), `[`, `]`, as well as any ascii symbolic characters. Examples: `` `address.city` `` represents a field named `address.city`, not the map key `city` in the field `address`. `` `*` `` represents a field named `*`, not any field. A special `Field` contains the default indexing settings for all fields. This field's resource name is: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*` Indexes defined on this `Field` will be applied to all fields which do not have their own `Field` index configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access field outputs
field_id = field.id
field_ttl_config = field.ttl_config
field_index_config = field.index_config
field_name = field.name
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


### Backup

Gets information about a backup.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stats` | String | Output only. Statistics about the backup. This data only becomes available after the backup is fully materialized to secondary storage. This field will be empty till then. |
| `expire_time` | String | Output only. The timestamp at which this backup expires. |
| `database_uid` | String | Output only. The system-generated UUID4 for the Firestore database that the backup is from. |
| `name` | String | Output only. The unique resource name of the Backup. Format is `projects/{project}/locations/{location}/backups/{backup}`. |
| `snapshot_time` | String | Output only. The backup contains an externally consistent copy of the database at this time. |
| `database` | String | Output only. Name of the Firestore database that the backup is from. Format is `projects/{project}/databases/{database}`. |
| `state` | String | Output only. The current state of the backup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access backup outputs
backup_id = backup.id
backup_stats = backup.stats
backup_expire_time = backup.expire_time
backup_database_uid = backup.database_uid
backup_name = backup.name
backup_snapshot_time = backup.snapshot_time
backup_database = backup.database
backup_state = backup.state
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
operation = provider.firestore_api.Operation {
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


### Document

Streams batches of document updates and deletes, in order. This method is only available via gRPC or WebChannel (not REST).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stream_token` | String |  | A stream token that was previously sent by the server. The client should set this field to the token from the most recent WriteResponse it has received. This acknowledges that the client has received responses up to this token. After sending this token, earlier tokens may not be used anymore. The server may close the stream if there are too many unacknowledged responses. Leave this field unset when creating a new stream. To resume a stream at a specific point, set this field and the `stream_id` field. Leave this field unset when creating a new stream. |
| `stream_id` | String |  | The ID of the write stream to resume. This may only be set in the first message. When left empty, a new write stream will be created. |
| `writes` | Vec<String> |  | The writes to apply. Always executed atomically and in order. This must be empty on the first request. This may be empty on the last request. This must not be empty on all other requests. |
| `labels` | HashMap<String, String> |  | Labels associated with this write request. |
| `database` | String | ✅ | Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`. This is only required in the first message. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the document, for example `projects/{project_id}/databases/{database_id}/documents/{document_path}`. |
| `fields` | HashMap<String, String> | The document's fields. The map keys represent field names. Field names matching the regular expression `__.*__` are reserved. Reserved field names are forbidden except in certain documented contexts. The field names, represented as UTF-8, must not exceed 1,500 bytes and cannot be empty. Field paths may be used in other contexts to refer to structured fields defined here. For `map_value`, the field path is represented by a dot-delimited (`.`) string of segments. Each segment is either a simple field name (defined below) or a quoted field name. For example, the structured field `"foo" : { map_value: { "x&y" : { string_value: "hello" }}}` would be represented by the field path `` foo.`x&y` ``. A simple field name contains only characters `a` to `z`, `A` to `Z`, `0` to `9`, or `_`, and must not start with `0` to `9`. For example, `foo_bar_17`. A quoted field name starts and ends with `` ` `` and may contain any character. Some characters, including `` ` ``, must be escaped using a `\`. For example, `` `x&y` `` represents `x&y` and `` `bak\`tik` `` represents `` bak`tik ``. |
| `create_time` | String | Output only. The time at which the document was created. This value increases monotonically when a document is deleted then recreated. It can also be compared to values from other documents and the `read_time` of a query. |
| `update_time` | String | Output only. The time at which the document was last changed. This value is initially set to the `create_time` then increases monotonically with each change to the document. It can also be compared to values from other documents and the `read_time` of a query. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.firestore_api.Document {
    database = "value"  # Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`. This is only required in the first message.
}

# Access document outputs
document_id = document.id
document_name = document.name
document_fields = document.fields
document_create_time = document.create_time
document_update_time = document.update_time
```

---


### Indexe

Creates a composite index. This returns a google.longrunning.Operation which may be used to track the status of the creation. The metadata for the operation will be the type IndexOperationMetadata.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_scope` | String |  | The API scope supported by this index. |
| `name` | String |  | Output only. A server defined name for this index. The form of this name for composite indexes will be: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{composite_index_id}` For single field indexes, this field will be empty. |
| `density` | String |  | Immutable. The density configuration of the index. |
| `multikey` | bool |  | Optional. Whether the index is multikey. By default, the index is not multikey. For non-multikey indexes, none of the paths in the index definition reach or traverse an array, except via an explicit array index. For multikey indexes, at most one of the paths in the index definition reach or traverse an array, except via an explicit array index. Violations will result in errors. Note this field only applies to index with MONGODB_COMPATIBLE_API ApiScope. |
| `state` | String |  | Output only. The serving state of the index. |
| `fields` | Vec<String> |  | The fields supported by this index. For composite indexes, this requires a minimum of 2 and a maximum of 100 fields. The last field entry is always for the field path `__name__`. If, on creation, `__name__` was not specified as the last field, it will be added automatically with the same direction as that of the last field defined. If the final field in a composite index is not directional, the `__name__` will be ordered ASCENDING (unless explicitly specified). For single field indexes, this will always be exactly one entry with a field path equal to the field path of the associated field. |
| `shard_count` | i64 |  | Optional. The number of shards for the index. |
| `unique` | bool |  | Optional. Whether it is an unique index. Unique index ensures all values for the indexed field(s) are unique across documents. |
| `query_scope` | String |  | Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the same collection ID. Indexes with a collection group query scope specified allow queries against all collections descended from a specific document, specified at query time, and that have the same collection ID as this index. |
| `parent` | String | ✅ | Required. A parent name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_scope` | String | The API scope supported by this index. |
| `name` | String | Output only. A server defined name for this index. The form of this name for composite indexes will be: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{composite_index_id}` For single field indexes, this field will be empty. |
| `density` | String | Immutable. The density configuration of the index. |
| `multikey` | bool | Optional. Whether the index is multikey. By default, the index is not multikey. For non-multikey indexes, none of the paths in the index definition reach or traverse an array, except via an explicit array index. For multikey indexes, at most one of the paths in the index definition reach or traverse an array, except via an explicit array index. Violations will result in errors. Note this field only applies to index with MONGODB_COMPATIBLE_API ApiScope. |
| `state` | String | Output only. The serving state of the index. |
| `fields` | Vec<String> | The fields supported by this index. For composite indexes, this requires a minimum of 2 and a maximum of 100 fields. The last field entry is always for the field path `__name__`. If, on creation, `__name__` was not specified as the last field, it will be added automatically with the same direction as that of the last field defined. If the final field in a composite index is not directional, the `__name__` will be ordered ASCENDING (unless explicitly specified). For single field indexes, this will always be exactly one entry with a field path equal to the field path of the associated field. |
| `shard_count` | i64 | Optional. The number of shards for the index. |
| `unique` | bool | Optional. Whether it is an unique index. Unique index ensures all values for the indexed field(s) are unique across documents. |
| `query_scope` | String | Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the same collection ID. Indexes with a collection group query scope specified allow queries against all collections descended from a specific document, specified at query time, and that have the same collection ID as this index. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create indexe
indexe = provider.firestore_api.Indexe {
    parent = "value"  # Required. A parent name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}`
}

# Access indexe outputs
indexe_id = indexe.id
indexe_api_scope = indexe.api_scope
indexe_name = indexe.name
indexe_density = indexe.density
indexe_multikey = indexe.multikey
indexe_state = indexe.state
indexe_fields = indexe.fields
indexe_shard_count = indexe.shard_count
indexe_unique = indexe.unique
indexe_query_scope = indexe.query_scope
```

---


### Database

Exports a copy of all or a subset of documents from Google Cloud Firestore to another storage system, such as Google Cloud Storage. Recent updates to documents may not be reflected in the export. The export occurs in the background and its progress can be monitored and managed via the Operation resource that is created. The output of an export may only be used once the associated operation is done. If an export operation is cancelled before completion it may leave partial data behind in Google Cloud Storage.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `collection_ids` | Vec<String> |  | Which collection ids to export. Unspecified means all collections. |
| `output_uri_prefix` | String |  | The output URI. Currently only supports Google Cloud Storage URIs of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the name of the Google Cloud Storage bucket and `NAMESPACE_PATH` is an optional Google Cloud Storage namespace path. When choosing a name, be sure to consider Google Cloud Storage naming guidelines: https://cloud.google.com/storage/docs/naming. If the URI is a bucket (without a namespace path), a prefix will be generated based on the start time. |
| `name` | String | ✅ | Database to export. Should be of the form: `projects/{project_id}/databases/{database_id}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.firestore_api.Database {
    name = "value"  # Database to export. Should be of the form: `projects/{project_id}/databases/{database_id}`.
}

```

---


### Indexe

Creates a composite index. This returns a google.longrunning.Operation which may be used to track the status of the creation. The metadata for the operation will be the type IndexOperationMetadata.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. A server defined name for this index. The form of this name for composite indexes will be: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{composite_index_id}` For single field indexes, this field will be empty. |
| `fields` | Vec<String> |  | The fields supported by this index. For composite indexes, this is always 2 or more fields. The last field entry is always for the field path `__name__`. If, on creation, `__name__` was not specified as the last field, it will be added automatically with the same direction as that of the last field defined. If the final field in a composite index is not directional, the `__name__` will be ordered ASCENDING (unless explicitly specified). For single field indexes, this will always be exactly one entry with a field path equal to the field path of the associated field. |
| `query_scope` | String |  | Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the same collection id. Indexes with a collection group query scope specified allow queries against all collections descended from a specific document, specified at query time, and that have the same collection id as this index. |
| `state` | String |  | Output only. The serving state of the index. |
| `parent` | String | ✅ | A parent name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. A server defined name for this index. The form of this name for composite indexes will be: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{composite_index_id}` For single field indexes, this field will be empty. |
| `fields` | Vec<String> | The fields supported by this index. For composite indexes, this is always 2 or more fields. The last field entry is always for the field path `__name__`. If, on creation, `__name__` was not specified as the last field, it will be added automatically with the same direction as that of the last field defined. If the final field in a composite index is not directional, the `__name__` will be ordered ASCENDING (unless explicitly specified). For single field indexes, this will always be exactly one entry with a field path equal to the field path of the associated field. |
| `query_scope` | String | Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the same collection id. Indexes with a collection group query scope specified allow queries against all collections descended from a specific document, specified at query time, and that have the same collection id as this index. |
| `state` | String | Output only. The serving state of the index. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create indexe
indexe = provider.firestore_api.Indexe {
    parent = "value"  # A parent name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}`
}

# Access indexe outputs
indexe_id = indexe.id
indexe_name = indexe.name
indexe_fields = indexe.fields
indexe_query_scope = indexe.query_scope
indexe_state = indexe.state
```

---


### Field

Gets the metadata and configuration for a Field.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `index_config` | String |  | The index configuration for this field. If unset, field indexing will revert to the configuration defined by the `ancestor_field`. To explicitly remove all indexes for this field, specify an index config with an empty list of indexes. |
| `name` | String |  | A field name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}` A field path may be a simple field name, e.g. `address` or a path to fields within map_value , e.g. `address.city`, or a special field path. The only valid special field is `*`, which represents any field. Field paths may be quoted using ` (backtick). The only character that needs to be escaped within a quoted field path is the backtick character itself, escaped using a backslash. Special characters in field paths that must be quoted include: `*`, `.`, ``` (backtick), `[`, `]`, as well as any ascii symbolic characters. Examples: (Note: Comments here are written in markdown syntax, so there is an additional layer of backticks to represent a code block) `\`address.city\`` represents a field named `address.city`, not the map key `city` in the field `address`. `\`*\`` represents a field named `*`, not any field. A special `Field` contains the default indexing settings for all fields. This field's resource name is: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*` Indexes defined on this `Field` will be applied to all fields which do not have their own `Field` index configuration. |
| `name` | String | ✅ | A field name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}` A field path may be a simple field name, e.g. `address` or a path to fields within map_value , e.g. `address.city`, or a special field path. The only valid special field is `*`, which represents any field. Field paths may be quoted using ` (backtick). The only character that needs to be escaped within a quoted field path is the backtick character itself, escaped using a backslash. Special characters in field paths that must be quoted include: `*`, `.`, ``` (backtick), `[`, `]`, as well as any ascii symbolic characters. Examples: (Note: Comments here are written in markdown syntax, so there is an additional layer of backticks to represent a code block) `\`address.city\`` represents a field named `address.city`, not the map key `city` in the field `address`. `\`*\`` represents a field named `*`, not any field. A special `Field` contains the default indexing settings for all fields. This field's resource name is: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*` Indexes defined on this `Field` will be applied to all fields which do not have their own `Field` index configuration. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `index_config` | String | The index configuration for this field. If unset, field indexing will revert to the configuration defined by the `ancestor_field`. To explicitly remove all indexes for this field, specify an index config with an empty list of indexes. |
| `name` | String | A field name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}` A field path may be a simple field name, e.g. `address` or a path to fields within map_value , e.g. `address.city`, or a special field path. The only valid special field is `*`, which represents any field. Field paths may be quoted using ` (backtick). The only character that needs to be escaped within a quoted field path is the backtick character itself, escaped using a backslash. Special characters in field paths that must be quoted include: `*`, `.`, ``` (backtick), `[`, `]`, as well as any ascii symbolic characters. Examples: (Note: Comments here are written in markdown syntax, so there is an additional layer of backticks to represent a code block) `\`address.city\`` represents a field named `address.city`, not the map key `city` in the field `address`. `\`*\`` represents a field named `*`, not any field. A special `Field` contains the default indexing settings for all fields. This field's resource name is: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*` Indexes defined on this `Field` will be applied to all fields which do not have their own `Field` index configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access field outputs
field_id = field.id
field_index_config = field.index_config
field_name = field.name
```

---


### Indexe

Creates the specified index. A newly created index's initial state is `CREATING`. On completion of the returned google.longrunning.Operation, the state will be `READY`. If the index already exists, the call will return an `ALREADY_EXISTS` status. During creation, the process could result in an error, in which case the index will move to the `ERROR` state. The process can be recovered by fixing the data that caused the error, removing the index with delete, then re-creating the index with create. Indexes with a single field cannot be created.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fields` | Vec<String> |  | The fields to index. |
| `collection_id` | String |  | The collection ID to which this index applies. Required. |
| `name` | String |  | The resource name of the index. Output only. |
| `state` | String |  | The state of the index. Output only. |
| `parent` | String | ✅ | The name of the database this index will apply to. For example: `projects/{project_id}/databases/{database_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fields` | Vec<String> | The fields to index. |
| `collection_id` | String | The collection ID to which this index applies. Required. |
| `name` | String | The resource name of the index. Output only. |
| `state` | String | The state of the index. Output only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create indexe
indexe = provider.firestore_api.Indexe {
    parent = "value"  # The name of the database this index will apply to. For example: `projects/{project_id}/databases/{database_id}`
}

# Access indexe outputs
indexe_id = indexe.id
indexe_fields = indexe.fields
indexe_collection_id = indexe.collection_id
indexe_name = indexe.name
indexe_state = indexe.state
```

---


### Database

Imports documents into Google Cloud Firestore. Existing documents with the same name are overwritten. The import occurs in the background and its progress can be monitored and managed via the Operation resource that is created. If an ImportDocuments operation is cancelled, it is possible that a subset of the data has already been imported to Cloud Firestore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `input_uri_prefix` | String |  | Location of the exported files. This must match the output_uri_prefix of an ExportDocumentsResponse from an export that has completed successfully. See: google.firestore.admin.v1beta1.ExportDocumentsResponse.output_uri_prefix. |
| `collection_ids` | Vec<String> |  | Which collection ids to import. Unspecified means all collections included in the import. |
| `name` | String | ✅ | Database to import into. Should be of the form: `projects/{project_id}/databases/{database_id}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.firestore_api.Database {
    name = "value"  # Database to import into. Should be of the form: `projects/{project_id}/databases/{database_id}`.
}

```

---


### Document

Runs a query.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `structured_query` | String |  | A structured query. |
| `new_transaction` | String |  | Starts a new transaction and reads the documents. Defaults to a read-only transaction. The new transaction ID will be returned as the first response in the stream. |
| `transaction` | String |  | Run the query within an already active transaction. The value here is the opaque transaction ID to execute the query in. |
| `explain_options` | String |  | Optional. Explain options for the query. If set, additional query statistics will be returned. If not, only query results will be returned. |
| `read_time` | String |  | Reads documents as they were at the given time. This must be a microsecond precision timestamp within the past one hour, or if Point-in-Time Recovery is enabled, can additionally be a whole minute timestamp within the past 7 days. |
| `parent` | String | ✅ | Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time at which the document was created. This value increases monotonically when a document is deleted then recreated. It can also be compared to values from other documents and the `read_time` of a query. |
| `fields` | HashMap<String, String> | The document's fields. The map keys represent field names. Field names matching the regular expression `__.*__` are reserved. Reserved field names are forbidden except in certain documented contexts. The field names, represented as UTF-8, must not exceed 1,500 bytes and cannot be empty. Field paths may be used in other contexts to refer to structured fields defined here. For `map_value`, the field path is represented by a dot-delimited (`.`) string of segments. Each segment is either a simple field name (defined below) or a quoted field name. For example, the structured field `"foo" : { map_value: { "x&y" : { string_value: "hello" }}}` would be represented by the field path `` foo.`x&y` ``. A simple field name contains only characters `a` to `z`, `A` to `Z`, `0` to `9`, or `_`, and must not start with `0` to `9`. For example, `foo_bar_17`. A quoted field name starts and ends with `` ` `` and may contain any character. Some characters, including `` ` ``, must be escaped using a `\`. For example, `` `x&y` `` represents `x&y` and `` `bak\`tik` `` represents `` bak`tik ``. |
| `update_time` | String | Output only. The time at which the document was last changed. This value is initially set to the `create_time` then increases monotonically with each change to the document. It can also be compared to values from other documents and the `read_time` of a query. |
| `name` | String | The resource name of the document, for example `projects/{project_id}/databases/{database_id}/documents/{document_path}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.firestore_api.Document {
    parent = "value"  # Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
}

# Access document outputs
document_id = document.id
document_create_time = document.create_time
document_fields = document.fields
document_update_time = document.update_time
document_name = document.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple user_cred resources
user_cred_0 = provider.firestore_api.User_cred {
    parent = "value-0"
}
user_cred_1 = provider.firestore_api.User_cred {
    parent = "value-1"
}
user_cred_2 = provider.firestore_api.User_cred {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    user_cred = provider.firestore_api.User_cred {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Firestore_api Documentation](https://cloud.google.com/firestore_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
