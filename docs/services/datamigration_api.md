# Datamigration_api Service



**Resources**: 12

---

## Overview

The datamigration_api service provides access to 12 resource types:

- [Conversion_workspace](#conversion_workspace) [CRUD]
- [Location](#location) [R]
- [Connection_profile](#connection_profile) [CRUD]
- [Operation](#operation) [CRD]
- [Migration_job](#migration_job) [CRUD]
- [Mapping_rule](#mapping_rule) [CRD]
- [Object](#object) [CR]
- [Private_connection](#private_connection) [CRD]
- [Connection_profile](#connection_profile) [CRUD]
- [Migration_job](#migration_job) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]

---

## Resources


### Conversion_workspace

Creates a new conversion workspace in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `has_uncommitted_changes` | bool |  | Output only. Whether the workspace has uncommitted changes (changes which were made after the workspace was committed). |
| `destination` | String |  | Required. The destination engine details. |
| `latest_commit_time` | String |  | Output only. The timestamp when the workspace was committed. |
| `source_provider` | String |  | Optional. The provider for the source database. |
| `create_time` | String |  | Output only. The timestamp when the workspace resource was created. |
| `update_time` | String |  | Output only. The timestamp when the workspace resource was last updated. |
| `name` | String |  | Full name of the workspace resource, in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}. |
| `destination_provider` | String |  | Optional. The provider for the destination database. |
| `display_name` | String |  | Optional. The display name for the workspace. |
| `latest_commit_id` | String |  | Output only. The latest commit ID. |
| `source` | String |  | Required. The source engine details. |
| `global_settings` | HashMap<String, String> |  | Optional. A generic list of settings for the workspace. The settings are database pair dependant and can indicate default behavior for the mapping rules engine or turn on or off specific features. Such examples can be: convert_foreign_key_to_interleave=true, skip_triggers=false, ignore_non_table_synonyms=true |
| `parent` | String | ✅ | Required. The parent which owns this collection of conversion workspaces. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `has_uncommitted_changes` | bool | Output only. Whether the workspace has uncommitted changes (changes which were made after the workspace was committed). |
| `destination` | String | Required. The destination engine details. |
| `latest_commit_time` | String | Output only. The timestamp when the workspace was committed. |
| `source_provider` | String | Optional. The provider for the source database. |
| `create_time` | String | Output only. The timestamp when the workspace resource was created. |
| `update_time` | String | Output only. The timestamp when the workspace resource was last updated. |
| `name` | String | Full name of the workspace resource, in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}. |
| `destination_provider` | String | Optional. The provider for the destination database. |
| `display_name` | String | Optional. The display name for the workspace. |
| `latest_commit_id` | String | Output only. The latest commit ID. |
| `source` | String | Required. The source engine details. |
| `global_settings` | HashMap<String, String> | Optional. A generic list of settings for the workspace. The settings are database pair dependant and can indicate default behavior for the mapping rules engine or turn on or off specific features. Such examples can be: convert_foreign_key_to_interleave=true, skip_triggers=false, ignore_non_table_synonyms=true |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversion_workspace
conversion_workspace = provider.datamigration_api.Conversion_workspace {
    parent = "value"  # Required. The parent which owns this collection of conversion workspaces.
}

# Access conversion_workspace outputs
conversion_workspace_id = conversion_workspace.id
conversion_workspace_has_uncommitted_changes = conversion_workspace.has_uncommitted_changes
conversion_workspace_destination = conversion_workspace.destination
conversion_workspace_latest_commit_time = conversion_workspace.latest_commit_time
conversion_workspace_source_provider = conversion_workspace.source_provider
conversion_workspace_create_time = conversion_workspace.create_time
conversion_workspace_update_time = conversion_workspace.update_time
conversion_workspace_name = conversion_workspace.name
conversion_workspace_destination_provider = conversion_workspace.destination_provider
conversion_workspace_display_name = conversion_workspace.display_name
conversion_workspace_latest_commit_id = conversion_workspace.latest_commit_id
conversion_workspace_source = conversion_workspace.source
conversion_workspace_global_settings = conversion_workspace.global_settings
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
location_display_name = location.display_name
location_name = location.name
```

---


### Connection_profile

Creates a new connection profile in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The connection profile display name. |
| `labels` | HashMap<String, String> |  | The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `alloydb` | String |  | An AlloyDB cluster connection profile. |
| `cloudsql` | String |  | A CloudSQL database connection profile. |
| `update_time` | String |  | Output only. The timestamp when the resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `role` | String |  | Optional. The connection profile role. |
| `postgresql` | String |  | A PostgreSQL database connection profile. |
| `name` | String |  | The name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `provider` | String |  | The database provider. |
| `state` | String |  | The current connection profile state (e.g. DRAFT, READY, or FAILED). |
| `oracle` | String |  | An Oracle database connection profile. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `mysql` | String |  | A MySQL database connection profile. |
| `error` | String |  | Output only. The error details in case of state FAILED. |
| `sqlserver` | String |  | Connection profile for a SQL Server data source. |
| `parent` | String | ✅ | Required. The parent which owns this collection of connection profiles. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The connection profile display name. |
| `labels` | HashMap<String, String> | The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `alloydb` | String | An AlloyDB cluster connection profile. |
| `cloudsql` | String | A CloudSQL database connection profile. |
| `update_time` | String | Output only. The timestamp when the resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `role` | String | Optional. The connection profile role. |
| `postgresql` | String | A PostgreSQL database connection profile. |
| `name` | String | The name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}. |
| `create_time` | String | Output only. The timestamp when the resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `provider` | String | The database provider. |
| `state` | String | The current connection profile state (e.g. DRAFT, READY, or FAILED). |
| `oracle` | String | An Oracle database connection profile. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `mysql` | String | A MySQL database connection profile. |
| `error` | String | Output only. The error details in case of state FAILED. |
| `sqlserver` | String | Connection profile for a SQL Server data source. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection_profile
connection_profile = provider.datamigration_api.Connection_profile {
    parent = "value"  # Required. The parent which owns this collection of connection profiles.
}

# Access connection_profile outputs
connection_profile_id = connection_profile.id
connection_profile_display_name = connection_profile.display_name
connection_profile_labels = connection_profile.labels
connection_profile_satisfies_pzi = connection_profile.satisfies_pzi
connection_profile_alloydb = connection_profile.alloydb
connection_profile_cloudsql = connection_profile.cloudsql
connection_profile_update_time = connection_profile.update_time
connection_profile_role = connection_profile.role
connection_profile_postgresql = connection_profile.postgresql
connection_profile_name = connection_profile.name
connection_profile_create_time = connection_profile.create_time
connection_profile_provider = connection_profile.provider
connection_profile_state = connection_profile.state
connection_profile_oracle = connection_profile.oracle
connection_profile_satisfies_pzs = connection_profile.satisfies_pzs
connection_profile_mysql = connection_profile.mysql
connection_profile_error = connection_profile.error
connection_profile_sqlserver = connection_profile.sqlserver
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
operation = provider.datamigration_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
```

---


### Migration_job

Creates a new migration job in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Required. The migration job type. |
| `conversion_workspace` | String |  | The conversion workspace used by the migration. |
| `dump_path` | String |  | The path to the dump file in Google Cloud Storage, in the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]). This field and the "dump_flags" field are mutually exclusive. |
| `error` | String |  | Output only. The error details in case of state FAILED. |
| `sqlserver_to_postgres_config` | String |  | Configuration for heterogeneous **SQL Server to Cloud SQL for PostgreSQL** migrations. |
| `phase` | String |  | Output only. The current migration job phase. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `sqlserver_homogeneous_migration_job_config` | String |  | Optional. Configuration for SQL Server homogeneous migration. |
| `create_time` | String |  | Output only. The timestamp when the migration job resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `source` | String |  | Required. The resource name (URI) of the source connection profile. |
| `labels` | HashMap<String, String> |  | The resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`. |
| `name` | String |  | The name (URI) of this migration job resource, in the form of: projects/{project}/locations/{location}/migrationJobs/{migrationJob}. |
| `destination` | String |  | Required. The resource name (URI) of the destination connection profile. |
| `destination_database` | String |  | The database engine type and provider of the destination. |
| `dump_type` | String |  | Optional. The type of the data dump. Supported for MySQL to CloudSQL for MySQL migrations only. |
| `update_time` | String |  | Output only. The timestamp when the migration job resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `duration` | String |  | Output only. The duration of the migration job (in seconds). A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s". |
| `performance_config` | String |  | Optional. Data dump parallelism settings used by the migration. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `source_database` | String |  | The database engine type and provider of the source. |
| `vpc_peering_connectivity` | String |  | The details of the VPC network that the source database is located in. |
| `objects_config` | String |  | Optional. The objects that need to be migrated. |
| `oracle_to_postgres_config` | String |  | Configuration for heterogeneous **Oracle to Cloud SQL for PostgreSQL** and **Oracle to AlloyDB for PostgreSQL** migrations. |
| `display_name` | String |  | The migration job display name. |
| `state` | String |  | The current migration job state. |
| `dump_flags` | String |  | The initial dump flags. This field and the "dump_path" field are mutually exclusive. |
| `cmek_key_name` | String |  | The CMEK (customer-managed encryption key) fully qualified key name used for the migration job. This field supports all migration jobs types except for: * Mysql to Mysql (use the cmek field in the cloudsql connection profile instead). * PostrgeSQL to PostgreSQL (use the cmek field in the cloudsql connection profile instead). * PostgreSQL to AlloyDB (use the kms_key_name field in the alloydb connection profile instead). Each Cloud CMEK key has the following format: projects/[PROJECT]/locations/[REGION]/keyRings/[RING]/cryptoKeys/[KEY_NAME] |
| `end_time` | String |  | Output only. If the migration job is completed, the time when it was completed. |
| `filter` | String |  | This field can be used to select the entities to migrate as part of the migration job. It uses AIP-160 notation to select a subset of the entities configured on the associated conversion-workspace. This field should not be set on migration-jobs that are not associated with a conversion workspace. |
| `reverse_ssh_connectivity` | String |  | The details needed to communicate to the source over Reverse SSH tunnel connectivity. |
| `static_ip_connectivity` | String |  | static ip connectivity data (default, no additional details needed). |
| `parent` | String | ✅ | Required. The parent which owns this collection of migration jobs. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Required. The migration job type. |
| `conversion_workspace` | String | The conversion workspace used by the migration. |
| `dump_path` | String | The path to the dump file in Google Cloud Storage, in the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]). This field and the "dump_flags" field are mutually exclusive. |
| `error` | String | Output only. The error details in case of state FAILED. |
| `sqlserver_to_postgres_config` | String | Configuration for heterogeneous **SQL Server to Cloud SQL for PostgreSQL** migrations. |
| `phase` | String | Output only. The current migration job phase. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `sqlserver_homogeneous_migration_job_config` | String | Optional. Configuration for SQL Server homogeneous migration. |
| `create_time` | String | Output only. The timestamp when the migration job resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `source` | String | Required. The resource name (URI) of the source connection profile. |
| `labels` | HashMap<String, String> | The resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`. |
| `name` | String | The name (URI) of this migration job resource, in the form of: projects/{project}/locations/{location}/migrationJobs/{migrationJob}. |
| `destination` | String | Required. The resource name (URI) of the destination connection profile. |
| `destination_database` | String | The database engine type and provider of the destination. |
| `dump_type` | String | Optional. The type of the data dump. Supported for MySQL to CloudSQL for MySQL migrations only. |
| `update_time` | String | Output only. The timestamp when the migration job resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `duration` | String | Output only. The duration of the migration job (in seconds). A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s". |
| `performance_config` | String | Optional. Data dump parallelism settings used by the migration. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `source_database` | String | The database engine type and provider of the source. |
| `vpc_peering_connectivity` | String | The details of the VPC network that the source database is located in. |
| `objects_config` | String | Optional. The objects that need to be migrated. |
| `oracle_to_postgres_config` | String | Configuration for heterogeneous **Oracle to Cloud SQL for PostgreSQL** and **Oracle to AlloyDB for PostgreSQL** migrations. |
| `display_name` | String | The migration job display name. |
| `state` | String | The current migration job state. |
| `dump_flags` | String | The initial dump flags. This field and the "dump_path" field are mutually exclusive. |
| `cmek_key_name` | String | The CMEK (customer-managed encryption key) fully qualified key name used for the migration job. This field supports all migration jobs types except for: * Mysql to Mysql (use the cmek field in the cloudsql connection profile instead). * PostrgeSQL to PostgreSQL (use the cmek field in the cloudsql connection profile instead). * PostgreSQL to AlloyDB (use the kms_key_name field in the alloydb connection profile instead). Each Cloud CMEK key has the following format: projects/[PROJECT]/locations/[REGION]/keyRings/[RING]/cryptoKeys/[KEY_NAME] |
| `end_time` | String | Output only. If the migration job is completed, the time when it was completed. |
| `filter` | String | This field can be used to select the entities to migrate as part of the migration job. It uses AIP-160 notation to select a subset of the entities configured on the associated conversion-workspace. This field should not be set on migration-jobs that are not associated with a conversion workspace. |
| `reverse_ssh_connectivity` | String | The details needed to communicate to the source over Reverse SSH tunnel connectivity. |
| `static_ip_connectivity` | String | static ip connectivity data (default, no additional details needed). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create migration_job
migration_job = provider.datamigration_api.Migration_job {
    parent = "value"  # Required. The parent which owns this collection of migration jobs.
}

# Access migration_job outputs
migration_job_id = migration_job.id
migration_job_type = migration_job.type
migration_job_conversion_workspace = migration_job.conversion_workspace
migration_job_dump_path = migration_job.dump_path
migration_job_error = migration_job.error
migration_job_sqlserver_to_postgres_config = migration_job.sqlserver_to_postgres_config
migration_job_phase = migration_job.phase
migration_job_satisfies_pzi = migration_job.satisfies_pzi
migration_job_sqlserver_homogeneous_migration_job_config = migration_job.sqlserver_homogeneous_migration_job_config
migration_job_create_time = migration_job.create_time
migration_job_source = migration_job.source
migration_job_labels = migration_job.labels
migration_job_name = migration_job.name
migration_job_destination = migration_job.destination
migration_job_destination_database = migration_job.destination_database
migration_job_dump_type = migration_job.dump_type
migration_job_update_time = migration_job.update_time
migration_job_duration = migration_job.duration
migration_job_performance_config = migration_job.performance_config
migration_job_satisfies_pzs = migration_job.satisfies_pzs
migration_job_source_database = migration_job.source_database
migration_job_vpc_peering_connectivity = migration_job.vpc_peering_connectivity
migration_job_objects_config = migration_job.objects_config
migration_job_oracle_to_postgres_config = migration_job.oracle_to_postgres_config
migration_job_display_name = migration_job.display_name
migration_job_state = migration_job.state
migration_job_dump_flags = migration_job.dump_flags
migration_job_cmek_key_name = migration_job.cmek_key_name
migration_job_end_time = migration_job.end_time
migration_job_filter = migration_job.filter
migration_job_reverse_ssh_connectivity = migration_job.reverse_ssh_connectivity
migration_job_static_ip_connectivity = migration_job.static_ip_connectivity
```

---


### Mapping_rule

Creates a new mapping rule for a given conversion workspace.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | Required. The rule filter |
| `entity_move` | String |  | Optional. Rule to specify how multiple entities should be relocated into a different schema. |
| `filter_table_columns` | String |  | Optional. Rule to specify the list of columns to include or exclude from a table. |
| `set_table_primary_key` | String |  | Optional. Rule to specify the primary key for a table |
| `multi_column_data_type_change` | String |  | Optional. Rule to specify how multiple columns should be converted to a different data type. |
| `single_column_change` | String |  | Optional. Rule to specify how a single column is converted. |
| `revision_id` | String |  | Output only. The revision ID of the mapping rule. A new revision is committed whenever the mapping rule is changed in any way. The format is an 8-character hexadecimal string. |
| `conditional_column_set_value` | String |  | Optional. Rule to specify how the data contained in a column should be transformed (such as trimmed, rounded, etc) provided that the data meets certain criteria. |
| `multi_entity_rename` | String |  | Optional. Rule to specify how multiple entities should be renamed. |
| `rule_order` | String |  | Required. The order in which the rule is applied. Lower order rules are applied before higher value rules so they may end up being overridden. |
| `single_entity_rename` | String |  | Optional. Rule to specify how a single entity should be renamed. |
| `display_name` | String |  | Optional. A human readable name |
| `source_sql_change` | String |  | Optional. Rule to change the sql code for an entity, for example, function, procedure. |
| `state` | String |  | Optional. The mapping rule state |
| `convert_rowid_column` | String |  | Optional. Rule to specify how multiple tables should be converted with an additional rowid column. |
| `rule_scope` | String |  | Required. The rule scope |
| `name` | String |  | Full name of the mapping rule resource, in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{set}/mappingRule/{rule}. |
| `revision_create_time` | String |  | Output only. The timestamp that the revision was created. |
| `single_package_change` | String |  | Optional. Rule to specify how a single package is converted. |
| `parent` | String | ✅ | Required. The parent which owns this collection of mapping rules. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `filter` | String | Required. The rule filter |
| `entity_move` | String | Optional. Rule to specify how multiple entities should be relocated into a different schema. |
| `filter_table_columns` | String | Optional. Rule to specify the list of columns to include or exclude from a table. |
| `set_table_primary_key` | String | Optional. Rule to specify the primary key for a table |
| `multi_column_data_type_change` | String | Optional. Rule to specify how multiple columns should be converted to a different data type. |
| `single_column_change` | String | Optional. Rule to specify how a single column is converted. |
| `revision_id` | String | Output only. The revision ID of the mapping rule. A new revision is committed whenever the mapping rule is changed in any way. The format is an 8-character hexadecimal string. |
| `conditional_column_set_value` | String | Optional. Rule to specify how the data contained in a column should be transformed (such as trimmed, rounded, etc) provided that the data meets certain criteria. |
| `multi_entity_rename` | String | Optional. Rule to specify how multiple entities should be renamed. |
| `rule_order` | String | Required. The order in which the rule is applied. Lower order rules are applied before higher value rules so they may end up being overridden. |
| `single_entity_rename` | String | Optional. Rule to specify how a single entity should be renamed. |
| `display_name` | String | Optional. A human readable name |
| `source_sql_change` | String | Optional. Rule to change the sql code for an entity, for example, function, procedure. |
| `state` | String | Optional. The mapping rule state |
| `convert_rowid_column` | String | Optional. Rule to specify how multiple tables should be converted with an additional rowid column. |
| `rule_scope` | String | Required. The rule scope |
| `name` | String | Full name of the mapping rule resource, in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{set}/mappingRule/{rule}. |
| `revision_create_time` | String | Output only. The timestamp that the revision was created. |
| `single_package_change` | String | Optional. Rule to specify how a single package is converted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mapping_rule
mapping_rule = provider.datamigration_api.Mapping_rule {
    parent = "value"  # Required. The parent which owns this collection of mapping rules.
}

# Access mapping_rule outputs
mapping_rule_id = mapping_rule.id
mapping_rule_filter = mapping_rule.filter
mapping_rule_entity_move = mapping_rule.entity_move
mapping_rule_filter_table_columns = mapping_rule.filter_table_columns
mapping_rule_set_table_primary_key = mapping_rule.set_table_primary_key
mapping_rule_multi_column_data_type_change = mapping_rule.multi_column_data_type_change
mapping_rule_single_column_change = mapping_rule.single_column_change
mapping_rule_revision_id = mapping_rule.revision_id
mapping_rule_conditional_column_set_value = mapping_rule.conditional_column_set_value
mapping_rule_multi_entity_rename = mapping_rule.multi_entity_rename
mapping_rule_rule_order = mapping_rule.rule_order
mapping_rule_single_entity_rename = mapping_rule.single_entity_rename
mapping_rule_display_name = mapping_rule.display_name
mapping_rule_source_sql_change = mapping_rule.source_sql_change
mapping_rule_state = mapping_rule.state
mapping_rule_convert_rowid_column = mapping_rule.convert_rowid_column
mapping_rule_rule_scope = mapping_rule.rule_scope
mapping_rule_name = mapping_rule.name
mapping_rule_revision_create_time = mapping_rule.revision_create_time
mapping_rule_single_package_change = mapping_rule.single_package_change
```

---


### Object

Use this method to look up a migration job object by its source object identifier.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_object_identifier` | String |  | Required. The source object identifier which maps to the migration job object. |
| `parent` | String | ✅ | Required. The parent migration job that owns the collection of objects. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_object` | String | The object identifier in the data source. |
| `heterogeneous_metadata` | String | Output only. Metadata for heterogeneous migration jobs objects. |
| `state` | String | The state of the migration job object. |
| `name` | String | The object's name. |
| `error` | String | Output only. The error details in case of failure. |
| `phase` | String | Output only. The phase of the migration job object. |
| `update_time` | String | Output only. The last update time of the migration job object. |
| `create_time` | String | Output only. The creation time of the migration job object. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create object
object = provider.datamigration_api.Object {
    parent = "value"  # Required. The parent migration job that owns the collection of objects.
}

# Access object outputs
object_id = object.id
object_source_object = object.source_object
object_heterogeneous_metadata = object.heterogeneous_metadata
object_state = object.state
object_name = object.name
object_error = object.error
object_phase = object.phase
object_update_time = object.update_time
object_create_time = object.create_time
```

---


### Private_connection

Creates a new private connection in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The private connection display name. |
| `state` | String |  | Output only. The state of the private connection. |
| `error` | String |  | Output only. The error details in case of state FAILED. |
| `psc_interface_config` | String |  | PSC Interface configuration. |
| `update_time` | String |  | Output only. The last update time of the resource. |
| `create_time` | String |  | Output only. The create time of the resource. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | The resource labels for private connections to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`. |
| `vpc_peering_config` | String |  | VPC peering configuration. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `name` | String |  | The name of the resource. |
| `parent` | String | ✅ | Required. The parent that owns the collection of PrivateConnections. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The private connection display name. |
| `state` | String | Output only. The state of the private connection. |
| `error` | String | Output only. The error details in case of state FAILED. |
| `psc_interface_config` | String | PSC Interface configuration. |
| `update_time` | String | Output only. The last update time of the resource. |
| `create_time` | String | Output only. The create time of the resource. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | The resource labels for private connections to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`. |
| `vpc_peering_config` | String | VPC peering configuration. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `name` | String | The name of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create private_connection
private_connection = provider.datamigration_api.Private_connection {
    parent = "value"  # Required. The parent that owns the collection of PrivateConnections.
}

# Access private_connection outputs
private_connection_id = private_connection.id
private_connection_display_name = private_connection.display_name
private_connection_state = private_connection.state
private_connection_error = private_connection.error
private_connection_psc_interface_config = private_connection.psc_interface_config
private_connection_update_time = private_connection.update_time
private_connection_create_time = private_connection.create_time
private_connection_satisfies_pzs = private_connection.satisfies_pzs
private_connection_labels = private_connection.labels
private_connection_vpc_peering_config = private_connection.vpc_peering_config
private_connection_satisfies_pzi = private_connection.satisfies_pzi
private_connection_name = private_connection.name
```

---


### Connection_profile

Creates a new connection profile in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `error` | String |  | Output only. The error details in case of state FAILED. |
| `name` | String |  | The name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}. |
| `update_time` | String |  | Output only. The timestamp when the resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `labels` | HashMap<String, String> |  | The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`. |
| `display_name` | String |  | The connection profile display name. |
| `mysql` | String |  | A MySQL database connection profile. |
| `state` | String |  | The current connection profile state (e.g. DRAFT, READY, or FAILED). |
| `cloudsql` | String |  | A CloudSQL database connection profile. |
| `provider` | String |  | The database provider. |
| `parent` | String | ✅ | Required. The parent, which owns this collection of connection profiles. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `error` | String | Output only. The error details in case of state FAILED. |
| `name` | String | The name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}. |
| `update_time` | String | Output only. The timestamp when the resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `labels` | HashMap<String, String> | The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`. |
| `display_name` | String | The connection profile display name. |
| `mysql` | String | A MySQL database connection profile. |
| `state` | String | The current connection profile state (e.g. DRAFT, READY, or FAILED). |
| `cloudsql` | String | A CloudSQL database connection profile. |
| `provider` | String | The database provider. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection_profile
connection_profile = provider.datamigration_api.Connection_profile {
    parent = "value"  # Required. The parent, which owns this collection of connection profiles.
}

# Access connection_profile outputs
connection_profile_id = connection_profile.id
connection_profile_create_time = connection_profile.create_time
connection_profile_error = connection_profile.error
connection_profile_name = connection_profile.name
connection_profile_update_time = connection_profile.update_time
connection_profile_labels = connection_profile.labels
connection_profile_display_name = connection_profile.display_name
connection_profile_mysql = connection_profile.mysql
connection_profile_state = connection_profile.state
connection_profile_cloudsql = connection_profile.cloudsql
connection_profile_provider = connection_profile.provider
```

---


### Migration_job

Creates a new migration job in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. If the migration job is completed, the time when it was completed. |
| `dump_path` | String |  | The path to the dump file in Google Cloud Storage, in the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]). |
| `destination_database` | String |  | The database engine type and provider of the destination. |
| `destination` | String |  | Required. The resource name (URI) of the destination connection profile. |
| `source_database` | String |  | The database engine type and provider of the source. |
| `display_name` | String |  | The migration job display name. |
| `phase` | String |  | Output only. The current migration job phase. |
| `reverse_ssh_connectivity` | String |  | The details needed to communicate to the source over Reverse SSH tunnel connectivity. |
| `static_ip_connectivity` | String |  | static ip connectivity data (default, no additional details needed). |
| `update_time` | String |  | Output only. The timestamp when the migration job resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `name` | String |  | The name (URI) of this migration job resource, in the form of: projects/{project}/locations/{location}/migrationJobs/{migrationJob}. |
| `duration` | String |  | Output only. The duration of the migration job (in seconds). A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s". |
| `error` | String |  | Output only. The error details in case of state FAILED. |
| `state` | String |  | The current migration job state. |
| `create_time` | String |  | Output only. The timestamp when the migration job resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `vpc_peering_connectivity` | String |  | The details of the VPC network that the source database is located in. |
| `type` | String |  | Required. The migration job type. |
| `source` | String |  | Required. The resource name (URI) of the source connection profile. |
| `labels` | HashMap<String, String> |  | The resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`. |
| `parent` | String | ✅ | Required. The parent, which owns this collection of migration jobs. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. If the migration job is completed, the time when it was completed. |
| `dump_path` | String | The path to the dump file in Google Cloud Storage, in the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]). |
| `destination_database` | String | The database engine type and provider of the destination. |
| `destination` | String | Required. The resource name (URI) of the destination connection profile. |
| `source_database` | String | The database engine type and provider of the source. |
| `display_name` | String | The migration job display name. |
| `phase` | String | Output only. The current migration job phase. |
| `reverse_ssh_connectivity` | String | The details needed to communicate to the source over Reverse SSH tunnel connectivity. |
| `static_ip_connectivity` | String | static ip connectivity data (default, no additional details needed). |
| `update_time` | String | Output only. The timestamp when the migration job resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `name` | String | The name (URI) of this migration job resource, in the form of: projects/{project}/locations/{location}/migrationJobs/{migrationJob}. |
| `duration` | String | Output only. The duration of the migration job (in seconds). A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s". |
| `error` | String | Output only. The error details in case of state FAILED. |
| `state` | String | The current migration job state. |
| `create_time` | String | Output only. The timestamp when the migration job resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z". |
| `vpc_peering_connectivity` | String | The details of the VPC network that the source database is located in. |
| `type` | String | Required. The migration job type. |
| `source` | String | Required. The resource name (URI) of the source connection profile. |
| `labels` | HashMap<String, String> | The resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create migration_job
migration_job = provider.datamigration_api.Migration_job {
    parent = "value"  # Required. The parent, which owns this collection of migration jobs.
}

# Access migration_job outputs
migration_job_id = migration_job.id
migration_job_end_time = migration_job.end_time
migration_job_dump_path = migration_job.dump_path
migration_job_destination_database = migration_job.destination_database
migration_job_destination = migration_job.destination
migration_job_source_database = migration_job.source_database
migration_job_display_name = migration_job.display_name
migration_job_phase = migration_job.phase
migration_job_reverse_ssh_connectivity = migration_job.reverse_ssh_connectivity
migration_job_static_ip_connectivity = migration_job.static_ip_connectivity
migration_job_update_time = migration_job.update_time
migration_job_name = migration_job.name
migration_job_duration = migration_job.duration
migration_job_error = migration_job.error
migration_job_state = migration_job.state
migration_job_create_time = migration_job.create_time
migration_job_vpc_peering_connectivity = migration_job.vpc_peering_connectivity
migration_job_type = migration_job.type
migration_job_source = migration_job.source
migration_job_labels = migration_job.labels
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_location_id = location.location_id
location_labels = location.labels
location_display_name = location.display_name
location_metadata = location.metadata
location_name = location.name
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.datamigration_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple conversion_workspace resources
conversion_workspace_0 = provider.datamigration_api.Conversion_workspace {
    parent = "value-0"
}
conversion_workspace_1 = provider.datamigration_api.Conversion_workspace {
    parent = "value-1"
}
conversion_workspace_2 = provider.datamigration_api.Conversion_workspace {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    conversion_workspace = provider.datamigration_api.Conversion_workspace {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Datamigration_api Documentation](https://cloud.google.com/datamigration_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
