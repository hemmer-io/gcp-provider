# Metastore_api Service



**Resources**: 35

---

## Overview

The metastore_api service provides access to 35 resource types:

- [Migration_execution](#migration_execution) [RD]
- [Location](#location) [R]
- [Metadata_import](#metadata_import) [CRU]
- [Table](#table) [CR]
- [Backup](#backup) [CRD]
- [Database](#database) [CR]
- [Federation](#federation) [CRUD]
- [Operation](#operation) [CRD]
- [Service](#service) [CRUD]
- [Federation](#federation) [CRUD]
- [Location](#location) [R]
- [Database](#database) [CR]
- [Table](#table) [CR]
- [Service](#service) [CRUD]
- [Backup](#backup) [CRD]
- [Metadata_import](#metadata_import) [CRU]
- [Migration_execution](#migration_execution) [RD]
- [Operation](#operation) [CRD]
- [Backup](#backup) [CRD]
- [Migration_execution](#migration_execution) [RD]
- [Service](#service) [CRUD]
- [Service](#service) [CRUD]
- [Backup](#backup) [CRD]
- [Metadata_import](#metadata_import) [CRU]
- [Backup](#backup) [CRD]
- [Federation](#federation) [CRUD]
- [Database](#database) [CR]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Service](#service) [CRUD]
- [Migration_execution](#migration_execution) [RD]
- [Table](#table) [CR]
- [Service](#service) [CRUD]
- [Backup](#backup) [CRD]
- [Migration_execution](#migration_execution) [RD]

---

## Resources


### Migration_execution

Gets details of a single migration execution.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_sql_migration_config` | String | Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore. |
| `create_time` | String | Output only. The time when the migration execution was started. |
| `state` | String | Output only. The current state of the migration execution. |
| `end_time` | String | Output only. The time when the migration execution finished. |
| `phase` | String | Output only. The current phase of the migration execution. |
| `state_message` | String | Output only. Additional information about the current state of the migration execution. |
| `name` | String | Output only. The relative resource name of the migration execution, in the following form: projects/{project_number}/locations/{location_id}/services/{service_id}/migrationExecutions/{migration_execution_id} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access migration_execution outputs
migration_execution_id = migration_execution.id
migration_execution_cloud_sql_migration_config = migration_execution.cloud_sql_migration_config
migration_execution_create_time = migration_execution.create_time
migration_execution_state = migration_execution.state
migration_execution_end_time = migration_execution.end_time
migration_execution_phase = migration_execution.phase
migration_execution_state_message = migration_execution.state_message
migration_execution_name = migration_execution.name
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
| `location_id` | String | The canonical id for this location. For example: "us-east1". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |
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
location_metadata = location.metadata
location_name = location.name
location_labels = location.labels
location_display_name = location.display_name
```

---


### Metadata_import

Creates a new MetadataImport in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. The description of the metadata import. |
| `state` | String |  | Output only. The current state of the metadata import. |
| `create_time` | String |  | Output only. The time when the metadata import was started. |
| `update_time` | String |  | Output only. The time when the metadata import was last updated. |
| `database_dump` | String |  | Immutable. A database dump from a pre-existing metastore's database. |
| `end_time` | String |  | Output only. The time when the metadata import finished. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a metastore import, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. The description of the metadata import. |
| `state` | String | Output only. The current state of the metadata import. |
| `create_time` | String | Output only. The time when the metadata import was started. |
| `update_time` | String | Output only. The time when the metadata import was last updated. |
| `database_dump` | String | Immutable. A database dump from a pre-existing metastore's database. |
| `end_time` | String | Output only. The time when the metadata import finished. |
| `name` | String | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metadata_import
metadata_import = provider.metastore_api.Metadata_import {
    parent = "value"  # Required. The relative resource name of the service in which to create a metastore import, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
}

# Access metadata_import outputs
metadata_import_id = metadata_import.id
metadata_import_description = metadata_import.description
metadata_import_state = metadata_import.state
metadata_import_create_time = metadata_import.create_time
metadata_import_update_time = metadata_import.update_time
metadata_import_database_dump = metadata_import.database_dump
metadata_import_end_time = metadata_import.end_time
metadata_import_name = metadata_import.name
```

---


### Table

Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: "bindings, etag" |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create table
table = provider.metastore_api.Table {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access table outputs
table_id = table.id
table_etag = table.etag
table_version = table.version
table_bindings = table.bindings
table_audit_configs = table.audit_configs
```

---


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `description` | String |  | Optional. The description of the backup. |
| `create_time` | String |  | Output only. The time when the backup was started. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
| `service_revision` | String | Output only. The revision of the service at the time of backup. |
| `state` | String | Output only. The current state of the backup. |
| `end_time` | String | Output only. The time when the backup finished creating. |
| `description` | String | Optional. The description of the backup. |
| `create_time` | String | Output only. The time when the backup was started. |
| `name` | String | Immutable. Identifier. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.metastore_api.Backup {
    parent = "value"  # Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
}

# Access backup outputs
backup_id = backup.id
backup_restoring_services = backup.restoring_services
backup_service_revision = backup.service_revision
backup_state = backup.state
backup_end_time = backup.end_time
backup_description = backup.description
backup_create_time = backup.create_time
backup_name = backup.name
```

---


### Database

Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: "bindings, etag" |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.metastore_api.Database {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access database outputs
database_id = database.id
database_etag = database.etag
database_version = database.version
database_bindings = database.bindings
database_audit_configs = database.audit_configs
```

---


### Federation

Creates a metastore federation in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the metastore federation was created. |
| `state` | String |  | Output only. The current state of the federation. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore federation. |
| `endpoint_uri` | String |  | Output only. The federation endpoint. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `update_time` | String |  | Output only. The time when the metastore federation was last updated. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore federation. |
| `backend_metastores` | HashMap<String, String> |  | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `version` | String |  | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore federation, if available. |
| `name` | String |  | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a federation service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the metastore federation was created. |
| `state` | String | Output only. The current state of the federation. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore federation. |
| `endpoint_uri` | String | Output only. The federation endpoint. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `update_time` | String | Output only. The time when the metastore federation was last updated. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore federation. |
| `backend_metastores` | HashMap<String, String> | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `version` | String | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `state_message` | String | Output only. Additional information about the current state of the metastore federation, if available. |
| `name` | String | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create federation
federation = provider.metastore_api.Federation {
    parent = "value"  # Required. The relative resource name of the location in which to create a federation service, in the following form:projects/{project_number}/locations/{location_id}.
}

# Access federation outputs
federation_id = federation.id
federation_create_time = federation.create_time
federation_state = federation.state
federation_uid = federation.uid
federation_endpoint_uri = federation.endpoint_uri
federation_tags = federation.tags
federation_update_time = federation.update_time
federation_labels = federation.labels
federation_backend_metastores = federation.backend_metastores
federation_version = federation.version
federation_state_message = federation.state_message
federation_name = federation.name
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
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
operation = provider.metastore_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_name = operation.name
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `release_channel` | String |  | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `endpoint_uri` | String |  | Output only. The URI of the endpoint used to access the metastore service. |
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `port` | i64 |  | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `deletion_protection` | bool |  | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `telemetry_config` | String |  | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `artifact_gcs_uri` | String |  | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `network_config` | String |  | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `tier` | String |  | Optional. The tier of the service. |
| `database_type` | String |  | Immutable. The database type that the Metastore service stores its data. |
| `network` | String |  | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `metadata_management_activity` | String |  | Output only. The metadata management activities of the metastore service. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `maintenance_window` | String |  | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `release_channel` | String | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `create_time` | String | Output only. The time when the metastore service was created. |
| `endpoint_uri` | String | Output only. The URI of the endpoint used to access the metastore service. |
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `port` | i64 | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `scheduled_backup` | String | Optional. The configuration of scheduled backup for the metastore service. |
| `deletion_protection` | bool | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `state` | String | Output only. The current state of the metastore service. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `telemetry_config` | String | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `artifact_gcs_uri` | String | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `network_config` | String | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `tier` | String | Optional. The tier of the service. |
| `database_type` | String | Immutable. The database type that the Metastore service stores its data. |
| `network` | String | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `metadata_management_activity` | String | Output only. The metadata management activities of the metastore service. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `name` | String | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `maintenance_window` | String | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.metastore_api.Service {
    parent = "value"  # Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}.
}

# Access service outputs
service_id = service.id
service_encryption_config = service.encryption_config
service_release_channel = service.release_channel
service_create_time = service.create_time
service_endpoint_uri = service.endpoint_uri
service_hive_metastore_config = service.hive_metastore_config
service_port = service.port
service_scheduled_backup = service.scheduled_backup
service_deletion_protection = service.deletion_protection
service_state = service.state
service_tags = service.tags
service_telemetry_config = service.telemetry_config
service_update_time = service.update_time
service_uid = service.uid
service_artifact_gcs_uri = service.artifact_gcs_uri
service_state_message = service.state_message
service_network_config = service.network_config
service_tier = service.tier
service_database_type = service.database_type
service_network = service.network
service_metadata_management_activity = service.metadata_management_activity
service_scaling_config = service.scaling_config
service_name = service.name
service_maintenance_window = service.maintenance_window
service_metadata_integration = service.metadata_integration
service_labels = service.labels
```

---


### Federation

Creates a metastore federation in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `endpoint_uri` | String |  | Output only. The federation endpoint. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `backend_metastores` | HashMap<String, String> |  | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `state` | String |  | Output only. The current state of the federation. |
| `version` | String |  | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `create_time` | String |  | Output only. The time when the metastore federation was created. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore federation. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore federation. |
| `update_time` | String |  | Output only. The time when the metastore federation was last updated. |
| `name` | String |  | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore federation, if available. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a federation service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_uri` | String | Output only. The federation endpoint. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `backend_metastores` | HashMap<String, String> | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `state` | String | Output only. The current state of the federation. |
| `version` | String | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `create_time` | String | Output only. The time when the metastore federation was created. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore federation. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore federation. |
| `update_time` | String | Output only. The time when the metastore federation was last updated. |
| `name` | String | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |
| `state_message` | String | Output only. Additional information about the current state of the metastore federation, if available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create federation
federation = provider.metastore_api.Federation {
    parent = "value"  # Required. The relative resource name of the location in which to create a federation service, in the following form:projects/{project_number}/locations/{location_id}.
}

# Access federation outputs
federation_id = federation.id
federation_endpoint_uri = federation.endpoint_uri
federation_tags = federation.tags
federation_backend_metastores = federation.backend_metastores
federation_state = federation.state
federation_version = federation.version
federation_create_time = federation.create_time
federation_labels = federation.labels
federation_uid = federation.uid
federation_update_time = federation.update_time
federation_name = federation.name
federation_state_message = federation.state_message
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |
| `location_id` | String | The canonical id for this location. For example: "us-east1". |


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
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
```

---


### Database

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the resource. Permissions with wildcards (such as * or storage.*) are not allowed. For more information see IAM Overview (https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.metastore_api.Database {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access database outputs
database_id = database.id
database_etag = database.etag
database_bindings = database.bindings
database_audit_configs = database.audit_configs
database_version = database.version
```

---


### Table

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the resource. Permissions with wildcards (such as * or storage.*) are not allowed. For more information see IAM Overview (https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create table
table = provider.metastore_api.Table {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access table outputs
table_id = table.id
table_etag = table.etag
table_bindings = table.bindings
table_audit_configs = table.audit_configs
table_version = table.version
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `telemetry_config` | String |  | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `artifact_gcs_uri` | String |  | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `maintenance_window` | String |  | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `endpoint_uri` | String |  | Output only. The URI of the endpoint used to access the metastore service. |
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `tier` | String |  | Optional. The tier of the service. |
| `network_config` | String |  | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `port` | i64 |  | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `database_type` | String |  | Immutable. The database type that the Metastore service stores its data. |
| `metadata_management_activity` | String |  | Output only. The metadata management activities of the metastore service. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `deletion_protection` | bool |  | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `release_channel` | String |  | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `multi_region_config` | String |  | Optional. Specifies the multi-region configuration information for the Hive metastore service. |
| `network` | String |  | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `telemetry_config` | String | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `artifact_gcs_uri` | String | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `maintenance_window` | String | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `endpoint_uri` | String | Output only. The URI of the endpoint used to access the metastore service. |
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `tier` | String | Optional. The tier of the service. |
| `network_config` | String | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `port` | i64 | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `database_type` | String | Immutable. The database type that the Metastore service stores its data. |
| `metadata_management_activity` | String | Output only. The metadata management activities of the metastore service. |
| `name` | String | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `deletion_protection` | bool | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `release_channel` | String | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `scheduled_backup` | String | Optional. The configuration of scheduled backup for the metastore service. |
| `state` | String | Output only. The current state of the metastore service. |
| `multi_region_config` | String | Optional. Specifies the multi-region configuration information for the Hive metastore service. |
| `network` | String | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `create_time` | String | Output only. The time when the metastore service was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.metastore_api.Service {
    parent = "value"  # Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}.
}

# Access service outputs
service_id = service.id
service_metadata_integration = service.metadata_integration
service_telemetry_config = service.telemetry_config
service_artifact_gcs_uri = service.artifact_gcs_uri
service_scaling_config = service.scaling_config
service_update_time = service.update_time
service_maintenance_window = service.maintenance_window
service_state_message = service.state_message
service_endpoint_uri = service.endpoint_uri
service_hive_metastore_config = service.hive_metastore_config
service_tier = service.tier
service_network_config = service.network_config
service_uid = service.uid
service_labels = service.labels
service_tags = service.tags
service_port = service.port
service_encryption_config = service.encryption_config
service_database_type = service.database_type
service_metadata_management_activity = service.metadata_management_activity
service_name = service.name
service_deletion_protection = service.deletion_protection
service_release_channel = service.release_channel
service_scheduled_backup = service.scheduled_backup
service_state = service.state
service_multi_region_config = service.multi_region_config
service_network = service.network
service_create_time = service.create_time
```

---


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `description` | String |  | Optional. The description of the backup. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `create_time` | String |  | Output only. The time when the backup was started. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. The time when the backup finished creating. |
| `description` | String | Optional. The description of the backup. |
| `name` | String | Immutable. Identifier. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
| `service_revision` | String | Output only. The revision of the service at the time of backup. |
| `state` | String | Output only. The current state of the backup. |
| `create_time` | String | Output only. The time when the backup was started. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.metastore_api.Backup {
    parent = "value"  # Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
}

# Access backup outputs
backup_id = backup.id
backup_end_time = backup.end_time
backup_description = backup.description
backup_name = backup.name
backup_restoring_services = backup.restoring_services
backup_service_revision = backup.service_revision
backup_state = backup.state
backup_create_time = backup.create_time
```

---


### Metadata_import

Creates a new MetadataImport in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The current state of the metadata import. |
| `create_time` | String |  | Output only. The time when the metadata import was started. |
| `database_dump` | String |  | Immutable. A database dump from a pre-existing metastore's database. |
| `description` | String |  | Optional. The description of the metadata import. |
| `end_time` | String |  | Output only. The time when the metadata import finished. |
| `update_time` | String |  | Output only. The time when the metadata import was last updated. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a metastore import, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current state of the metadata import. |
| `create_time` | String | Output only. The time when the metadata import was started. |
| `database_dump` | String | Immutable. A database dump from a pre-existing metastore's database. |
| `description` | String | Optional. The description of the metadata import. |
| `end_time` | String | Output only. The time when the metadata import finished. |
| `update_time` | String | Output only. The time when the metadata import was last updated. |
| `name` | String | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metadata_import
metadata_import = provider.metastore_api.Metadata_import {
    parent = "value"  # Required. The relative resource name of the service in which to create a metastore import, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
}

# Access metadata_import outputs
metadata_import_id = metadata_import.id
metadata_import_state = metadata_import.state
metadata_import_create_time = metadata_import.create_time
metadata_import_database_dump = metadata_import.database_dump
metadata_import_description = metadata_import.description
metadata_import_end_time = metadata_import.end_time
metadata_import_update_time = metadata_import.update_time
metadata_import_name = metadata_import.name
```

---


### Migration_execution

Gets details of a single migration execution.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The relative resource name of the migration execution, in the following form: projects/{project_number}/locations/{location_id}/services/{service_id}/migrationExecutions/{migration_execution_id} |
| `state_message` | String | Output only. Additional information about the current state of the migration execution. |
| `create_time` | String | Output only. The time when the migration execution was started. |
| `cloud_sql_migration_config` | String | Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore. |
| `phase` | String | Output only. The current phase of the migration execution. |
| `state` | String | Output only. The current state of the migration execution. |
| `end_time` | String | Output only. The time when the migration execution finished. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access migration_execution outputs
migration_execution_id = migration_execution.id
migration_execution_name = migration_execution.name
migration_execution_state_message = migration_execution.state_message
migration_execution_create_time = migration_execution.create_time
migration_execution_cloud_sql_migration_config = migration_execution.cloud_sql_migration_config
migration_execution_phase = migration_execution.phase
migration_execution_state = migration_execution.state
migration_execution_end_time = migration_execution.end_time
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.metastore_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
```

---


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the backup was started. |
| `description` | String |  | The description of the backup. |
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `name` | String |  | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the backup was started. |
| `description` | String | The description of the backup. |
| `end_time` | String | Output only. The time when the backup finished creating. |
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
| `service_revision` | String | Output only. The revision of the service at the time of backup. |
| `state` | String | Output only. The current state of the backup. |
| `name` | String | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.metastore_api.Backup {
    parent = "value"  # Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
}

# Access backup outputs
backup_id = backup.id
backup_create_time = backup.create_time
backup_description = backup.description
backup_end_time = backup.end_time
backup_restoring_services = backup.restoring_services
backup_service_revision = backup.service_revision
backup_state = backup.state
backup_name = backup.name
```

---


### Migration_execution

Gets details of a single migration execution.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_sql_migration_config` | String | Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore. |
| `create_time` | String | Output only. The time when the migration execution was started. |
| `phase` | String | Output only. The current phase of the migration execution. |
| `state` | String | Output only. The current state of the migration execution. |
| `name` | String | Output only. The relative resource name of the migration execution, in the following form: projects/{project_number}/locations/{location_id}/services/{service_id}/migrationExecutions/{migration_execution_id} |
| `state_message` | String | Output only. Additional information about the current state of the migration execution. |
| `end_time` | String | Output only. The time when the migration execution finished. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access migration_execution outputs
migration_execution_id = migration_execution.id
migration_execution_cloud_sql_migration_config = migration_execution.cloud_sql_migration_config
migration_execution_create_time = migration_execution.create_time
migration_execution_phase = migration_execution.phase
migration_execution_state = migration_execution.state
migration_execution_name = migration_execution.name
migration_execution_state_message = migration_execution.state_message
migration_execution_end_time = migration_execution.end_time
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `name` | String |  | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `endpoints` | Vec<String> |  | Output only. The list of endpoints used to access the metastore service. |
| `warehouse_gcs_uri` | String |  | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `state` | String | Output only. The current state of the metastore service. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `name` | String | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `scheduled_backup` | String | Optional. The configuration of scheduled backup for the metastore service. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `create_time` | String | Output only. The time when the metastore service was created. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `endpoints` | Vec<String> | Output only. The list of endpoints used to access the metastore service. |
| `warehouse_gcs_uri` | String | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.metastore_api.Service {
    parent = "value"  # Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}.
}

# Access service outputs
service_id = service.id
service_hive_metastore_config = service.hive_metastore_config
service_state = service.state
service_metadata_integration = service.metadata_integration
service_name = service.name
service_encryption_config = service.encryption_config
service_scheduled_backup = service.scheduled_backup
service_uid = service.uid
service_create_time = service.create_time
service_update_time = service.update_time
service_endpoints = service.endpoints
service_warehouse_gcs_uri = service.warehouse_gcs_uri
service_scaling_config = service.scaling_config
service_state_message = service.state_message
service_labels = service.labels
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `warehouse_gcs_uri` | String |  | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `name` | String |  | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `endpoints` | Vec<String> |  | Output only. The list of endpoints used to access the metastore service. |
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `state` | String | Output only. The current state of the metastore service. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `create_time` | String | Output only. The time when the metastore service was created. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |
| `warehouse_gcs_uri` | String | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `name` | String | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `endpoints` | Vec<String> | Output only. The list of endpoints used to access the metastore service. |
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `scheduled_backup` | String | Optional. The configuration of scheduled backup for the metastore service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.metastore_api.Service {
    parent = "value"  # Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}.
}

# Access service outputs
service_id = service.id
service_state_message = service.state_message
service_state = service.state
service_scaling_config = service.scaling_config
service_create_time = service.create_time
service_uid = service.uid
service_encryption_config = service.encryption_config
service_labels = service.labels
service_warehouse_gcs_uri = service.warehouse_gcs_uri
service_update_time = service.update_time
service_metadata_integration = service.metadata_integration
service_name = service.name
service_endpoints = service.endpoints
service_hive_metastore_config = service.hive_metastore_config
service_scheduled_backup = service.scheduled_backup
```

---


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the backup was started. |
| `description` | String |  | The description of the backup. |
| `name` | String |  | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the backup was started. |
| `description` | String | The description of the backup. |
| `name` | String | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
| `service_revision` | String | Output only. The revision of the service at the time of backup. |
| `state` | String | Output only. The current state of the backup. |
| `end_time` | String | Output only. The time when the backup finished creating. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.metastore_api.Backup {
    parent = "value"  # Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
}

# Access backup outputs
backup_id = backup.id
backup_create_time = backup.create_time
backup_description = backup.description
backup_name = backup.name
backup_restoring_services = backup.restoring_services
backup_service_revision = backup.service_revision
backup_state = backup.state
backup_end_time = backup.end_time
```

---


### Metadata_import

Creates a new MetadataImport in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the metadata import was started. |
| `database_dump` | String |  | Immutable. A database dump from a pre-existing metastore's database. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |
| `description` | String |  | Optional. The description of the metadata import. |
| `end_time` | String |  | Output only. The time when the metadata import finished. |
| `state` | String |  | Output only. The current state of the metadata import. |
| `update_time` | String |  | Output only. The time when the metadata import was last updated. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a metastore import, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the metadata import was started. |
| `database_dump` | String | Immutable. A database dump from a pre-existing metastore's database. |
| `name` | String | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |
| `description` | String | Optional. The description of the metadata import. |
| `end_time` | String | Output only. The time when the metadata import finished. |
| `state` | String | Output only. The current state of the metadata import. |
| `update_time` | String | Output only. The time when the metadata import was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metadata_import
metadata_import = provider.metastore_api.Metadata_import {
    parent = "value"  # Required. The relative resource name of the service in which to create a metastore import, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
}

# Access metadata_import outputs
metadata_import_id = metadata_import.id
metadata_import_create_time = metadata_import.create_time
metadata_import_database_dump = metadata_import.database_dump
metadata_import_name = metadata_import.name
metadata_import_description = metadata_import.description
metadata_import_end_time = metadata_import.end_time
metadata_import_state = metadata_import.state
metadata_import_update_time = metadata_import.update_time
```

---


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `state` | String |  | Output only. The current state of the backup. |
| `create_time` | String |  | Output only. The time when the backup was started. |
| `description` | String |  | Optional. The description of the backup. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. The time when the backup finished creating. |
| `state` | String | Output only. The current state of the backup. |
| `create_time` | String | Output only. The time when the backup was started. |
| `description` | String | Optional. The description of the backup. |
| `name` | String | Immutable. Identifier. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
| `service_revision` | String | Output only. The revision of the service at the time of backup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.metastore_api.Backup {
    parent = "value"  # Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
}

# Access backup outputs
backup_id = backup.id
backup_end_time = backup.end_time
backup_state = backup.state
backup_create_time = backup.create_time
backup_description = backup.description
backup_name = backup.name
backup_restoring_services = backup.restoring_services
backup_service_revision = backup.service_revision
```

---


### Federation

Creates a metastore federation in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore federation. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `name` | String |  | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore federation. |
| `update_time` | String |  | Output only. The time when the metastore federation was last updated. |
| `version` | String |  | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `create_time` | String |  | Output only. The time when the metastore federation was created. |
| `endpoint_uri` | String |  | Output only. The federation endpoint. |
| `backend_metastores` | HashMap<String, String> |  | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore federation, if available. |
| `state` | String |  | Output only. The current state of the federation. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a federation service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | User-defined labels for the metastore federation. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `name` | String | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore federation. |
| `update_time` | String | Output only. The time when the metastore federation was last updated. |
| `version` | String | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `create_time` | String | Output only. The time when the metastore federation was created. |
| `endpoint_uri` | String | Output only. The federation endpoint. |
| `backend_metastores` | HashMap<String, String> | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `state_message` | String | Output only. Additional information about the current state of the metastore federation, if available. |
| `state` | String | Output only. The current state of the federation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create federation
federation = provider.metastore_api.Federation {
    parent = "value"  # Required. The relative resource name of the location in which to create a federation service, in the following form:projects/{project_number}/locations/{location_id}.
}

# Access federation outputs
federation_id = federation.id
federation_labels = federation.labels
federation_tags = federation.tags
federation_name = federation.name
federation_uid = federation.uid
federation_update_time = federation.update_time
federation_version = federation.version
federation_create_time = federation.create_time
federation_endpoint_uri = federation.endpoint_uri
federation_backend_metastores = federation.backend_metastores
federation_state_message = federation.state_message
federation_state = federation.state
```

---


### Database

Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: "bindings, etag" |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.metastore_api.Database {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access database outputs
database_id = database.id
database_audit_configs = database.audit_configs
database_version = database.version
database_etag = database.etag
database_bindings = database.bindings
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
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
operation = provider.metastore_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_metadata = operation.metadata
operation_done = operation.done
operation_response = operation.response
operation_error = operation.error
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: "us-east1". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |


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
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
location_name = location.name
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `multi_region_config` | String |  | Optional. Specifies the multi-region configuration information for the Hive metastore service. |
| `endpoint_uri` | String |  | Output only. The URI of the endpoint used to access the metastore service. |
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `metadata_management_activity` | String |  | Output only. The metadata management activities of the metastore service. |
| `release_channel` | String |  | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `network` | String |  | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `telemetry_config` | String |  | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `network_config` | String |  | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `tier` | String |  | Optional. The tier of the service. |
| `database_type` | String |  | Immutable. The database type that the Metastore service stores its data. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `deletion_protection` | bool |  | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `artifact_gcs_uri` | String |  | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `maintenance_window` | String |  | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `port` | i64 |  | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `multi_region_config` | String | Optional. Specifies the multi-region configuration information for the Hive metastore service. |
| `endpoint_uri` | String | Output only. The URI of the endpoint used to access the metastore service. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `metadata_management_activity` | String | Output only. The metadata management activities of the metastore service. |
| `release_channel` | String | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `create_time` | String | Output only. The time when the metastore service was created. |
| `network` | String | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `scheduled_backup` | String | Optional. The configuration of scheduled backup for the metastore service. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `telemetry_config` | String | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `state` | String | Output only. The current state of the metastore service. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `network_config` | String | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `tier` | String | Optional. The tier of the service. |
| `database_type` | String | Immutable. The database type that the Metastore service stores its data. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |
| `deletion_protection` | bool | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `artifact_gcs_uri` | String | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `maintenance_window` | String | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `name` | String | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `port` | i64 | Optional. The TCP port at which the metastore service is reached. Default: 9083. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.metastore_api.Service {
    parent = "value"  # Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}.
}

# Access service outputs
service_id = service.id
service_hive_metastore_config = service.hive_metastore_config
service_encryption_config = service.encryption_config
service_multi_region_config = service.multi_region_config
service_endpoint_uri = service.endpoint_uri
service_metadata_integration = service.metadata_integration
service_metadata_management_activity = service.metadata_management_activity
service_release_channel = service.release_channel
service_create_time = service.create_time
service_network = service.network
service_scheduled_backup = service.scheduled_backup
service_scaling_config = service.scaling_config
service_telemetry_config = service.telemetry_config
service_state = service.state
service_state_message = service.state_message
service_network_config = service.network_config
service_tier = service.tier
service_database_type = service.database_type
service_uid = service.uid
service_update_time = service.update_time
service_labels = service.labels
service_deletion_protection = service.deletion_protection
service_artifact_gcs_uri = service.artifact_gcs_uri
service_maintenance_window = service.maintenance_window
service_tags = service.tags
service_name = service.name
service_port = service.port
```

---


### Migration_execution

Gets details of a single migration execution.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. The time when the migration execution finished. |
| `phase` | String | Output only. The current phase of the migration execution. |
| `create_time` | String | Output only. The time when the migration execution was started. |
| `name` | String | Output only. The relative resource name of the migration execution, in the following form: projects/{project_number}/locations/{location_id}/services/{service_id}/migrationExecutions/{migration_execution_id} |
| `state_message` | String | Output only. Additional information about the current state of the migration execution. |
| `state` | String | Output only. The current state of the migration execution. |
| `cloud_sql_migration_config` | String | Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access migration_execution outputs
migration_execution_id = migration_execution.id
migration_execution_end_time = migration_execution.end_time
migration_execution_phase = migration_execution.phase
migration_execution_create_time = migration_execution.create_time
migration_execution_name = migration_execution.name
migration_execution_state_message = migration_execution.state_message
migration_execution_state = migration_execution.state
migration_execution_cloud_sql_migration_config = migration_execution.cloud_sql_migration_config
```

---


### Table

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the resource. Permissions with wildcards (such as * or storage.*) are not allowed. For more information see IAM Overview (https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create table
table = provider.metastore_api.Table {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access table outputs
table_id = table.id
table_audit_configs = table.audit_configs
table_version = table.version
table_etag = table.etag
table_bindings = table.bindings
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `endpoints` | Vec<String> |  | Output only. The list of endpoints used to access the metastore service. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `warehouse_gcs_uri` | String |  | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `name` | String |  | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `endpoints` | Vec<String> | Output only. The list of endpoints used to access the metastore service. |
| `scheduled_backup` | String | Optional. The configuration of scheduled backup for the metastore service. |
| `create_time` | String | Output only. The time when the metastore service was created. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `state` | String | Output only. The current state of the metastore service. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `warehouse_gcs_uri` | String | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `name` | String | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.metastore_api.Service {
    parent = "value"  # Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}.
}

# Access service outputs
service_id = service.id
service_encryption_config = service.encryption_config
service_endpoints = service.endpoints
service_scheduled_backup = service.scheduled_backup
service_create_time = service.create_time
service_scaling_config = service.scaling_config
service_state = service.state
service_state_message = service.state_message
service_uid = service.uid
service_hive_metastore_config = service.hive_metastore_config
service_metadata_integration = service.metadata_integration
service_warehouse_gcs_uri = service.warehouse_gcs_uri
service_name = service.name
service_update_time = service.update_time
service_labels = service.labels
```

---


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `description` | String |  | The description of the backup. |
| `name` | String |  | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `create_time` | String |  | Output only. The time when the backup was started. |
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_revision` | String | Output only. The revision of the service at the time of backup. |
| `state` | String | Output only. The current state of the backup. |
| `end_time` | String | Output only. The time when the backup finished creating. |
| `description` | String | The description of the backup. |
| `name` | String | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `create_time` | String | Output only. The time when the backup was started. |
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.metastore_api.Backup {
    parent = "value"  # Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}.
}

# Access backup outputs
backup_id = backup.id
backup_service_revision = backup.service_revision
backup_state = backup.state
backup_end_time = backup.end_time
backup_description = backup.description
backup_name = backup.name
backup_create_time = backup.create_time
backup_restoring_services = backup.restoring_services
```

---


### Migration_execution

Gets details of a single migration execution.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. The time when the migration execution finished. |
| `name` | String | Output only. The relative resource name of the migration execution, in the following form: projects/{project_number}/locations/{location_id}/services/{service_id}/migrationExecutions/{migration_execution_id} |
| `state` | String | Output only. The current state of the migration execution. |
| `state_message` | String | Output only. Additional information about the current state of the migration execution. |
| `create_time` | String | Output only. The time when the migration execution was started. |
| `phase` | String | Output only. The current phase of the migration execution. |
| `cloud_sql_migration_config` | String | Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access migration_execution outputs
migration_execution_id = migration_execution.id
migration_execution_end_time = migration_execution.end_time
migration_execution_name = migration_execution.name
migration_execution_state = migration_execution.state
migration_execution_state_message = migration_execution.state_message
migration_execution_create_time = migration_execution.create_time
migration_execution_phase = migration_execution.phase
migration_execution_cloud_sql_migration_config = migration_execution.cloud_sql_migration_config
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple migration_execution resources
migration_execution_0 = provider.metastore_api.Migration_execution {
}
migration_execution_1 = provider.metastore_api.Migration_execution {
}
migration_execution_2 = provider.metastore_api.Migration_execution {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    migration_execution = provider.metastore_api.Migration_execution {
    }
```

---

## Related Documentation

- [GCP Metastore_api Documentation](https://cloud.google.com/metastore_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
