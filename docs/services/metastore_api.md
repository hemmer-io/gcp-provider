# Metastore_api Service



**Resources**: 35

---

## Overview

The metastore_api service provides access to 35 resource types:

- [Backup](#backup) [CRD]
- [Service](#service) [CRUD]
- [Location](#location) [R]
- [Metadata_import](#metadata_import) [CRU]
- [Database](#database) [CR]
- [Operation](#operation) [CRD]
- [Federation](#federation) [CRUD]
- [Migration_execution](#migration_execution) [RD]
- [Table](#table) [CR]
- [Migration_execution](#migration_execution) [RD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Service](#service) [CRUD]
- [Backup](#backup) [CRD]
- [Federation](#federation) [CRUD]
- [Table](#table) [CR]
- [Metadata_import](#metadata_import) [CRU]
- [Database](#database) [CR]
- [Service](#service) [CRUD]
- [Migration_execution](#migration_execution) [RD]
- [Backup](#backup) [CRD]
- [Service](#service) [CRUD]
- [Backup](#backup) [CRD]
- [Service](#service) [CRUD]
- [Database](#database) [CR]
- [Location](#location) [R]
- [Backup](#backup) [CRD]
- [Table](#table) [CR]
- [Federation](#federation) [CRUD]
- [Migration_execution](#migration_execution) [RD]
- [Metadata_import](#metadata_import) [CRU]
- [Operation](#operation) [CRD]
- [Migration_execution](#migration_execution) [RD]
- [Backup](#backup) [CRD]
- [Service](#service) [CRUD]

---

## Resources


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `description` | String |  | Optional. The description of the backup. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `create_time` | String |  | Output only. The time when the backup was started. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
| `state` | String | Output only. The current state of the backup. |
| `end_time` | String | Output only. The time when the backup finished creating. |
| `description` | String | Optional. The description of the backup. |
| `name` | String | Immutable. Identifier. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `create_time` | String | Output only. The time when the backup was started. |
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
backup_restoring_services = backup.restoring_services
backup_state = backup.state
backup_end_time = backup.end_time
backup_description = backup.description
backup_name = backup.name
backup_create_time = backup.create_time
backup_service_revision = backup.service_revision
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `tier` | String |  | Optional. The tier of the service. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `database_type` | String |  | Immutable. The database type that the Metastore service stores its data. |
| `metadata_management_activity` | String |  | Output only. The metadata management activities of the metastore service. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `telemetry_config` | String |  | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `network` | String |  | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `artifact_gcs_uri` | String |  | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `release_channel` | String |  | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `deletion_protection` | bool |  | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `port` | i64 |  | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `maintenance_window` | String |  | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `network_config` | String |  | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `endpoint_uri` | String |  | Output only. The URI of the endpoint used to access the metastore service. |
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the metastore service was created. |
| `tier` | String | Optional. The tier of the service. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `database_type` | String | Immutable. The database type that the Metastore service stores its data. |
| `metadata_management_activity` | String | Output only. The metadata management activities of the metastore service. |
| `state` | String | Output only. The current state of the metastore service. |
| `telemetry_config` | String | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `scheduled_backup` | String | Optional. The configuration of scheduled backup for the metastore service. |
| `network` | String | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `artifact_gcs_uri` | String | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `release_channel` | String | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `deletion_protection` | bool | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `port` | i64 | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `maintenance_window` | String | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `network_config` | String | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `endpoint_uri` | String | Output only. The URI of the endpoint used to access the metastore service. |
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |
| `name` | String | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |


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
service_create_time = service.create_time
service_tier = service.tier
service_uid = service.uid
service_database_type = service.database_type
service_metadata_management_activity = service.metadata_management_activity
service_state = service.state
service_telemetry_config = service.telemetry_config
service_scheduled_backup = service.scheduled_backup
service_network = service.network
service_scaling_config = service.scaling_config
service_update_time = service.update_time
service_artifact_gcs_uri = service.artifact_gcs_uri
service_release_channel = service.release_channel
service_deletion_protection = service.deletion_protection
service_port = service.port
service_metadata_integration = service.metadata_integration
service_maintenance_window = service.maintenance_window
service_tags = service.tags
service_hive_metastore_config = service.hive_metastore_config
service_network_config = service.network_config
service_endpoint_uri = service.endpoint_uri
service_encryption_config = service.encryption_config
service_state_message = service.state_message
service_labels = service.labels
service_name = service.name
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |
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


### Metadata_import

Creates a new MetadataImport in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |
| `state` | String |  | Output only. The current state of the metadata import. |
| `update_time` | String |  | Output only. The time when the metadata import was last updated. |
| `create_time` | String |  | Output only. The time when the metadata import was started. |
| `description` | String |  | Optional. The description of the metadata import. |
| `end_time` | String |  | Output only. The time when the metadata import finished. |
| `database_dump` | String |  | Immutable. A database dump from a pre-existing metastore's database. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a metastore import, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |
| `state` | String | Output only. The current state of the metadata import. |
| `update_time` | String | Output only. The time when the metadata import was last updated. |
| `create_time` | String | Output only. The time when the metadata import was started. |
| `description` | String | Optional. The description of the metadata import. |
| `end_time` | String | Output only. The time when the metadata import finished. |
| `database_dump` | String | Immutable. A database dump from a pre-existing metastore's database. |


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
metadata_import_name = metadata_import.name
metadata_import_state = metadata_import.state
metadata_import_update_time = metadata_import.update_time
metadata_import_create_time = metadata_import.create_time
metadata_import_description = metadata_import.description
metadata_import_end_time = metadata_import.end_time
metadata_import_database_dump = metadata_import.database_dump
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
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |


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
database_bindings = database.bindings
database_version = database.version
database_audit_configs = database.audit_configs
database_etag = database.etag
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
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
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
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
operation_error = operation.error
operation_name = operation.name
```

---


### Federation

Creates a metastore federation in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_message` | String |  | Output only. Additional information about the current state of the metastore federation, if available. |
| `create_time` | String |  | Output only. The time when the metastore federation was created. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore federation. |
| `state` | String |  | Output only. The current state of the federation. |
| `backend_metastores` | HashMap<String, String> |  | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore federation. |
| `update_time` | String |  | Output only. The time when the metastore federation was last updated. |
| `version` | String |  | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `name` | String |  | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |
| `endpoint_uri` | String |  | Output only. The federation endpoint. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a federation service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state_message` | String | Output only. Additional information about the current state of the metastore federation, if available. |
| `create_time` | String | Output only. The time when the metastore federation was created. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore federation. |
| `state` | String | Output only. The current state of the federation. |
| `backend_metastores` | HashMap<String, String> | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore federation. |
| `update_time` | String | Output only. The time when the metastore federation was last updated. |
| `version` | String | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `name` | String | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |
| `endpoint_uri` | String | Output only. The federation endpoint. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |


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
federation_state_message = federation.state_message
federation_create_time = federation.create_time
federation_labels = federation.labels
federation_state = federation.state
federation_backend_metastores = federation.backend_metastores
federation_uid = federation.uid
federation_update_time = federation.update_time
federation_version = federation.version
federation_name = federation.name
federation_endpoint_uri = federation.endpoint_uri
federation_tags = federation.tags
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
| `state_message` | String | Output only. Additional information about the current state of the migration execution. |
| `phase` | String | Output only. The current phase of the migration execution. |
| `cloud_sql_migration_config` | String | Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore. |
| `create_time` | String | Output only. The time when the migration execution was started. |
| `name` | String | Output only. The relative resource name of the migration execution, in the following form: projects/{project_number}/locations/{location_id}/services/{service_id}/migrationExecutions/{migration_execution_id} |
| `state` | String | Output only. The current state of the migration execution. |


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
migration_execution_state_message = migration_execution.state_message
migration_execution_phase = migration_execution.phase
migration_execution_cloud_sql_migration_config = migration_execution.cloud_sql_migration_config
migration_execution_create_time = migration_execution.create_time
migration_execution_name = migration_execution.name
migration_execution_state = migration_execution.state
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
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |


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
table_bindings = table.bindings
table_version = table.version
table_audit_configs = table.audit_configs
table_etag = table.etag
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
| `state` | String | Output only. The current state of the migration execution. |
| `state_message` | String | Output only. Additional information about the current state of the migration execution. |
| `cloud_sql_migration_config` | String | Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore. |
| `create_time` | String | Output only. The time when the migration execution was started. |
| `name` | String | Output only. The relative resource name of the migration execution, in the following form: projects/{project_number}/locations/{location_id}/services/{service_id}/migrationExecutions/{migration_execution_id} |
| `phase` | String | Output only. The current phase of the migration execution. |


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
migration_execution_state = migration_execution.state
migration_execution_state_message = migration_execution.state_message
migration_execution_cloud_sql_migration_config = migration_execution.cloud_sql_migration_config
migration_execution_create_time = migration_execution.create_time
migration_execution_name = migration_execution.name
migration_execution_phase = migration_execution.phase
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
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |


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
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_error = operation.error
operation_response = operation.response
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_labels = location.labels
location_metadata = location.metadata
location_display_name = location.display_name
location_name = location.name
location_location_id = location.location_id
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `database_type` | String |  | Immutable. The database type that the Metastore service stores its data. |
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `port` | i64 |  | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `network` | String |  | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `artifact_gcs_uri` | String |  | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `maintenance_window` | String |  | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `endpoint_uri` | String |  | Output only. The URI of the endpoint used to access the metastore service. |
| `multi_region_config` | String |  | Optional. Specifies the multi-region configuration information for the Hive metastore service. |
| `release_channel` | String |  | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `network_config` | String |  | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `telemetry_config` | String |  | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `tier` | String |  | Optional. The tier of the service. |
| `deletion_protection` | bool |  | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `metadata_management_activity` | String |  | Output only. The metadata management activities of the metastore service. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `create_time` | String | Output only. The time when the metastore service was created. |
| `database_type` | String | Immutable. The database type that the Metastore service stores its data. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `port` | i64 | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `network` | String | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `artifact_gcs_uri` | String | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `state` | String | Output only. The current state of the metastore service. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `maintenance_window` | String | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `endpoint_uri` | String | Output only. The URI of the endpoint used to access the metastore service. |
| `multi_region_config` | String | Optional. Specifies the multi-region configuration information for the Hive metastore service. |
| `release_channel` | String | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `network_config` | String | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `scheduled_backup` | String | Optional. The configuration of scheduled backup for the metastore service. |
| `telemetry_config` | String | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `tier` | String | Optional. The tier of the service. |
| `deletion_protection` | bool | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `name` | String | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `metadata_management_activity` | String | Output only. The metadata management activities of the metastore service. |


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
service_uid = service.uid
service_create_time = service.create_time
service_database_type = service.database_type
service_metadata_integration = service.metadata_integration
service_port = service.port
service_hive_metastore_config = service.hive_metastore_config
service_tags = service.tags
service_update_time = service.update_time
service_network = service.network
service_artifact_gcs_uri = service.artifact_gcs_uri
service_labels = service.labels
service_encryption_config = service.encryption_config
service_scaling_config = service.scaling_config
service_state = service.state
service_state_message = service.state_message
service_maintenance_window = service.maintenance_window
service_endpoint_uri = service.endpoint_uri
service_multi_region_config = service.multi_region_config
service_release_channel = service.release_channel
service_network_config = service.network_config
service_scheduled_backup = service.scheduled_backup
service_telemetry_config = service.telemetry_config
service_tier = service.tier
service_deletion_protection = service.deletion_protection
service_name = service.name
service_metadata_management_activity = service.metadata_management_activity
```

---


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the backup was started. |
| `description` | String |  | Optional. The description of the backup. |
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the backup was started. |
| `description` | String | Optional. The description of the backup. |
| `end_time` | String | Output only. The time when the backup finished creating. |
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
| `service_revision` | String | Output only. The revision of the service at the time of backup. |
| `state` | String | Output only. The current state of the backup. |
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
backup_create_time = backup.create_time
backup_description = backup.description
backup_end_time = backup.end_time
backup_restoring_services = backup.restoring_services
backup_service_revision = backup.service_revision
backup_state = backup.state
backup_name = backup.name
```

---


### Federation

Creates a metastore federation in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore federation. |
| `create_time` | String |  | Output only. The time when the metastore federation was created. |
| `version` | String |  | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `endpoint_uri` | String |  | Output only. The federation endpoint. |
| `update_time` | String |  | Output only. The time when the metastore federation was last updated. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore federation. |
| `state` | String |  | Output only. The current state of the federation. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore federation, if available. |
| `backend_metastores` | HashMap<String, String> |  | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `name` | String |  | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a federation service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. The globally unique resource identifier of the metastore federation. |
| `create_time` | String | Output only. The time when the metastore federation was created. |
| `version` | String | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `endpoint_uri` | String | Output only. The federation endpoint. |
| `update_time` | String | Output only. The time when the metastore federation was last updated. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore federation. |
| `state` | String | Output only. The current state of the federation. |
| `state_message` | String | Output only. Additional information about the current state of the metastore federation, if available. |
| `backend_metastores` | HashMap<String, String> | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
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
federation_uid = federation.uid
federation_create_time = federation.create_time
federation_version = federation.version
federation_endpoint_uri = federation.endpoint_uri
federation_update_time = federation.update_time
federation_labels = federation.labels
federation_state = federation.state
federation_state_message = federation.state_message
federation_backend_metastores = federation.backend_metastores
federation_tags = federation.tags
federation_name = federation.name
```

---


### Table

Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: "bindings, etag" |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
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

# Create table
table = provider.metastore_api.Table {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access table outputs
table_id = table.id
table_audit_configs = table.audit_configs
table_version = table.version
table_etag = table.etag
table_bindings = table.bindings
```

---


### Metadata_import

Creates a new MetadataImport in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the metadata import was started. |
| `end_time` | String |  | Output only. The time when the metadata import finished. |
| `database_dump` | String |  | Immutable. A database dump from a pre-existing metastore's database. |
| `state` | String |  | Output only. The current state of the metadata import. |
| `update_time` | String |  | Output only. The time when the metadata import was last updated. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |
| `description` | String |  | Optional. The description of the metadata import. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a metastore import, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the metadata import was started. |
| `end_time` | String | Output only. The time when the metadata import finished. |
| `database_dump` | String | Immutable. A database dump from a pre-existing metastore's database. |
| `state` | String | Output only. The current state of the metadata import. |
| `update_time` | String | Output only. The time when the metadata import was last updated. |
| `name` | String | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |
| `description` | String | Optional. The description of the metadata import. |


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
metadata_import_end_time = metadata_import.end_time
metadata_import_database_dump = metadata_import.database_dump
metadata_import_state = metadata_import.state
metadata_import_update_time = metadata_import.update_time
metadata_import_name = metadata_import.name
metadata_import_description = metadata_import.description
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
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access database outputs
database_id = database.id
database_audit_configs = database.audit_configs
database_version = database.version
database_etag = database.etag
database_bindings = database.bindings
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
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `name` | String |  | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `warehouse_gcs_uri` | String |  | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `endpoints` | Vec<String> | Output only. The list of endpoints used to access the metastore service. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `state` | String | Output only. The current state of the metastore service. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `name` | String | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `create_time` | String | Output only. The time when the metastore service was created. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `warehouse_gcs_uri` | String | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
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
service_encryption_config = service.encryption_config
service_endpoints = service.endpoints
service_metadata_integration = service.metadata_integration
service_state = service.state
service_uid = service.uid
service_hive_metastore_config = service.hive_metastore_config
service_name = service.name
service_labels = service.labels
service_scaling_config = service.scaling_config
service_create_time = service.create_time
service_state_message = service.state_message
service_update_time = service.update_time
service_warehouse_gcs_uri = service.warehouse_gcs_uri
service_scheduled_backup = service.scheduled_backup
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
| `cloud_sql_migration_config` | String | Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore. |
| `state` | String | Output only. The current state of the migration execution. |
| `create_time` | String | Output only. The time when the migration execution was started. |
| `state_message` | String | Output only. Additional information about the current state of the migration execution. |
| `end_time` | String | Output only. The time when the migration execution finished. |
| `phase` | String | Output only. The current phase of the migration execution. |


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
migration_execution_cloud_sql_migration_config = migration_execution.cloud_sql_migration_config
migration_execution_state = migration_execution.state
migration_execution_create_time = migration_execution.create_time
migration_execution_state_message = migration_execution.state_message
migration_execution_end_time = migration_execution.end_time
migration_execution_phase = migration_execution.phase
```

---


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description of the backup. |
| `name` | String |  | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `create_time` | String |  | Output only. The time when the backup was started. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The description of the backup. |
| `name` | String | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
| `end_time` | String | Output only. The time when the backup finished creating. |
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
backup_description = backup.description
backup_name = backup.name
backup_restoring_services = backup.restoring_services
backup_end_time = backup.end_time
backup_service_revision = backup.service_revision
backup_state = backup.state
backup_create_time = backup.create_time
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The current state of the metastore service. |
| `name` | String |  | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `endpoints` | Vec<String> |  | Output only. The list of endpoints used to access the metastore service. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `warehouse_gcs_uri` | String |  | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current state of the metastore service. |
| `name` | String | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `endpoints` | Vec<String> | Output only. The list of endpoints used to access the metastore service. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `create_time` | String | Output only. The time when the metastore service was created. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `warehouse_gcs_uri` | String | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
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
service_state = service.state
service_name = service.name
service_scaling_config = service.scaling_config
service_metadata_integration = service.metadata_integration
service_endpoints = service.endpoints
service_labels = service.labels
service_update_time = service.update_time
service_uid = service.uid
service_create_time = service.create_time
service_state_message = service.state_message
service_warehouse_gcs_uri = service.warehouse_gcs_uri
service_encryption_config = service.encryption_config
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
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `description` | String |  | The description of the backup. |
| `create_time` | String |  | Output only. The time when the backup was started. |
| `name` | String |  | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `state` | String |  | Output only. The current state of the backup. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
| `service_revision` | String | Output only. The revision of the service at the time of backup. |
| `end_time` | String | Output only. The time when the backup finished creating. |
| `description` | String | The description of the backup. |
| `create_time` | String | Output only. The time when the backup was started. |
| `name` | String | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `state` | String | Output only. The current state of the backup. |


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
backup_end_time = backup.end_time
backup_description = backup.description
backup_create_time = backup.create_time
backup_name = backup.name
backup_state = backup.state
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `endpoint_uri` | String |  | Output only. The URI of the endpoint used to access the metastore service. |
| `metadata_management_activity` | String |  | Output only. The metadata management activities of the metastore service. |
| `multi_region_config` | String |  | Optional. Specifies the multi-region configuration information for the Hive metastore service. |
| `artifact_gcs_uri` | String |  | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `network_config` | String |  | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `tier` | String |  | Optional. The tier of the service. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `database_type` | String |  | Immutable. The database type that the Metastore service stores its data. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `network` | String |  | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `deletion_protection` | bool |  | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `telemetry_config` | String |  | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `port` | i64 |  | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `release_channel` | String |  | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `maintenance_window` | String |  | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_uri` | String | Output only. The URI of the endpoint used to access the metastore service. |
| `metadata_management_activity` | String | Output only. The metadata management activities of the metastore service. |
| `multi_region_config` | String | Optional. Specifies the multi-region configuration information for the Hive metastore service. |
| `artifact_gcs_uri` | String | Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored. |
| `network_config` | String | Optional. The configuration specifying the network settings for the Dataproc Metastore service. |
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `scheduled_backup` | String | Optional. The configuration of scheduled backup for the metastore service. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |
| `tier` | String | Optional. The tier of the service. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `database_type` | String | Immutable. The database type that the Metastore service stores its data. |
| `state` | String | Output only. The current state of the metastore service. |
| `network` | String | Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}. |
| `deletion_protection` | bool | Optional. Indicates if the dataproc metastore should be protected against accidental deletions. |
| `name` | String | Immutable. Identifier. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `telemetry_config` | String | Optional. The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `create_time` | String | Output only. The time when the metastore service was created. |
| `port` | i64 | Optional. The TCP port at which the metastore service is reached. Default: 9083. |
| `release_channel` | String | Immutable. The release channel of the service. If unspecified, defaults to STABLE. |
| `maintenance_window` | String | Optional. The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type. |


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
service_endpoint_uri = service.endpoint_uri
service_metadata_management_activity = service.metadata_management_activity
service_multi_region_config = service.multi_region_config
service_artifact_gcs_uri = service.artifact_gcs_uri
service_network_config = service.network_config
service_hive_metastore_config = service.hive_metastore_config
service_scaling_config = service.scaling_config
service_scheduled_backup = service.scheduled_backup
service_state_message = service.state_message
service_tags = service.tags
service_labels = service.labels
service_tier = service.tier
service_uid = service.uid
service_encryption_config = service.encryption_config
service_database_type = service.database_type
service_state = service.state
service_network = service.network
service_deletion_protection = service.deletion_protection
service_name = service.name
service_telemetry_config = service.telemetry_config
service_update_time = service.update_time
service_metadata_integration = service.metadata_integration
service_create_time = service.create_time
service_port = service.port
service_release_channel = service.release_channel
service_maintenance_window = service.maintenance_window
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
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |
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
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access database outputs
database_id = database.id
database_bindings = database.bindings
database_version = database.version
database_etag = database.etag
database_audit_configs = database.audit_configs
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |


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
location_display_name = location.display_name
location_labels = location.labels
```

---


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `description` | String |  | Optional. The description of the backup. |
| `state` | String |  | Output only. The current state of the backup. |
| `create_time` | String |  | Output only. The time when the backup was started. |
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. The time when the backup finished creating. |
| `service_revision` | String | Output only. The revision of the service at the time of backup. |
| `description` | String | Optional. The description of the backup. |
| `state` | String | Output only. The current state of the backup. |
| `create_time` | String | Output only. The time when the backup was started. |
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
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
backup_end_time = backup.end_time
backup_service_revision = backup.service_revision
backup_description = backup.description
backup_state = backup.state
backup_create_time = backup.create_time
backup_restoring_services = backup.restoring_services
backup_name = backup.name
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
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |
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
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access table outputs
table_id = table.id
table_bindings = table.bindings
table_version = table.version
table_etag = table.etag
table_audit_configs = table.audit_configs
```

---


### Federation

Creates a metastore federation in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the metastore federation was created. |
| `endpoint_uri` | String |  | Output only. The federation endpoint. |
| `backend_metastores` | HashMap<String, String> |  | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `update_time` | String |  | Output only. The time when the metastore federation was last updated. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore federation, if available. |
| `version` | String |  | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore federation. |
| `state` | String |  | Output only. The current state of the federation. |
| `name` | String |  | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore federation. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a federation service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the metastore federation was created. |
| `endpoint_uri` | String | Output only. The federation endpoint. |
| `backend_metastores` | HashMap<String, String> | A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `update_time` | String | Output only. The time when the metastore federation was last updated. |
| `state_message` | String | Output only. Additional information about the current state of the metastore federation, if available. |
| `version` | String | Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore federation. |
| `state` | String | Output only. The current state of the federation. |
| `name` | String | Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore federation. |


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
federation_endpoint_uri = federation.endpoint_uri
federation_backend_metastores = federation.backend_metastores
federation_tags = federation.tags
federation_update_time = federation.update_time
federation_state_message = federation.state_message
federation_version = federation.version
federation_labels = federation.labels
federation_state = federation.state
federation_name = federation.name
federation_uid = federation.uid
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
| `state` | String | Output only. The current state of the migration execution. |
| `create_time` | String | Output only. The time when the migration execution was started. |
| `state_message` | String | Output only. Additional information about the current state of the migration execution. |
| `cloud_sql_migration_config` | String | Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore. |
| `phase` | String | Output only. The current phase of the migration execution. |
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
migration_execution_state = migration_execution.state
migration_execution_create_time = migration_execution.create_time
migration_execution_state_message = migration_execution.state_message
migration_execution_cloud_sql_migration_config = migration_execution.cloud_sql_migration_config
migration_execution_phase = migration_execution.phase
migration_execution_end_time = migration_execution.end_time
```

---


### Metadata_import

Creates a new MetadataImport in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. The description of the metadata import. |
| `end_time` | String |  | Output only. The time when the metadata import finished. |
| `database_dump` | String |  | Immutable. A database dump from a pre-existing metastore's database. |
| `name` | String |  | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |
| `state` | String |  | Output only. The current state of the metadata import. |
| `update_time` | String |  | Output only. The time when the metadata import was last updated. |
| `create_time` | String |  | Output only. The time when the metadata import was started. |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a metastore import, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. The description of the metadata import. |
| `end_time` | String | Output only. The time when the metadata import finished. |
| `database_dump` | String | Immutable. A database dump from a pre-existing metastore's database. |
| `name` | String | Immutable. Identifier. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}. |
| `state` | String | Output only. The current state of the metadata import. |
| `update_time` | String | Output only. The time when the metadata import was last updated. |
| `create_time` | String | Output only. The time when the metadata import was started. |


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
metadata_import_end_time = metadata_import.end_time
metadata_import_database_dump = metadata_import.database_dump
metadata_import_name = metadata_import.name
metadata_import_state = metadata_import.state
metadata_import_update_time = metadata_import.update_time
metadata_import_create_time = metadata_import.create_time
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |
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
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_metadata = operation.metadata
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
| `create_time` | String | Output only. The time when the migration execution was started. |
| `end_time` | String | Output only. The time when the migration execution finished. |
| `state_message` | String | Output only. Additional information about the current state of the migration execution. |
| `cloud_sql_migration_config` | String | Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore. |
| `phase` | String | Output only. The current phase of the migration execution. |
| `name` | String | Output only. The relative resource name of the migration execution, in the following form: projects/{project_number}/locations/{location_id}/services/{service_id}/migrationExecutions/{migration_execution_id} |
| `state` | String | Output only. The current state of the migration execution. |


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
migration_execution_create_time = migration_execution.create_time
migration_execution_end_time = migration_execution.end_time
migration_execution_state_message = migration_execution.state_message
migration_execution_cloud_sql_migration_config = migration_execution.cloud_sql_migration_config
migration_execution_phase = migration_execution.phase
migration_execution_name = migration_execution.name
migration_execution_state = migration_execution.state
```

---


### Backup

Creates a new backup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the backup was started. |
| `state` | String |  | Output only. The current state of the backup. |
| `end_time` | String |  | Output only. The time when the backup finished creating. |
| `restoring_services` | Vec<String> |  | Output only. Services that are restoring from the backup. |
| `service_revision` | String |  | Output only. The revision of the service at the time of backup. |
| `description` | String |  | The description of the backup. |
| `name` | String |  | Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id} |
| `parent` | String | ✅ | Required. The relative resource name of the service in which to create a backup of the following form:projects/{project_number}/locations/{location_id}/services/{service_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the backup was started. |
| `state` | String | Output only. The current state of the backup. |
| `end_time` | String | Output only. The time when the backup finished creating. |
| `restoring_services` | Vec<String> | Output only. Services that are restoring from the backup. |
| `service_revision` | String | Output only. The revision of the service at the time of backup. |
| `description` | String | The description of the backup. |
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
backup_state = backup.state
backup_end_time = backup.end_time
backup_restoring_services = backup.restoring_services
backup_service_revision = backup.service_revision
backup_description = backup.description
backup_name = backup.name
```

---


### Service

Creates a metastore service in a project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the metastore service was created. |
| `state_message` | String |  | Output only. Additional information about the current state of the metastore service, if available. |
| `hive_metastore_config` | String |  | Configuration information specific to running Hive metastore software as the metastore service. |
| `uid` | String |  | Output only. The globally unique resource identifier of the metastore service. |
| `update_time` | String |  | Output only. The time when the metastore service was last updated. |
| `endpoints` | Vec<String> |  | Output only. The list of endpoints used to access the metastore service. |
| `scaling_config` | String |  | Optional. Scaling configuration of the metastore service. |
| `warehouse_gcs_uri` | String |  | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `labels` | HashMap<String, String> |  | User-defined labels for the metastore service. |
| `metadata_integration` | String |  | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `name` | String |  | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `state` | String |  | Output only. The current state of the metastore service. |
| `encryption_config` | String |  | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
| `scheduled_backup` | String |  | Optional. The configuration of scheduled backup for the metastore service. |
| `parent` | String | ✅ | Required. The relative resource name of the location in which to create a metastore service, in the following form:projects/{project_number}/locations/{location_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the metastore service was created. |
| `state_message` | String | Output only. Additional information about the current state of the metastore service, if available. |
| `hive_metastore_config` | String | Configuration information specific to running Hive metastore software as the metastore service. |
| `uid` | String | Output only. The globally unique resource identifier of the metastore service. |
| `update_time` | String | Output only. The time when the metastore service was last updated. |
| `endpoints` | Vec<String> | Output only. The list of endpoints used to access the metastore service. |
| `scaling_config` | String | Optional. Scaling configuration of the metastore service. |
| `warehouse_gcs_uri` | String | Required. A Cloud Storage URI (starting with gs://) that specifies the default warehouse directory of the Hive Metastore. |
| `labels` | HashMap<String, String> | User-defined labels for the metastore service. |
| `metadata_integration` | String | Optional. The setting that defines how metastore metadata should be integrated with external services and systems. |
| `name` | String | Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}. |
| `state` | String | Output only. The current state of the metastore service. |
| `encryption_config` | String | Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated. |
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
service_create_time = service.create_time
service_state_message = service.state_message
service_hive_metastore_config = service.hive_metastore_config
service_uid = service.uid
service_update_time = service.update_time
service_endpoints = service.endpoints
service_scaling_config = service.scaling_config
service_warehouse_gcs_uri = service.warehouse_gcs_uri
service_labels = service.labels
service_metadata_integration = service.metadata_integration
service_name = service.name
service_state = service.state
service_encryption_config = service.encryption_config
service_scheduled_backup = service.scheduled_backup
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple backup resources
backup_0 = provider.metastore_api.Backup {
    parent = "value-0"
}
backup_1 = provider.metastore_api.Backup {
    parent = "value-1"
}
backup_2 = provider.metastore_api.Backup {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    backup = provider.metastore_api.Backup {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Metastore_api Documentation](https://cloud.google.com/metastore_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
