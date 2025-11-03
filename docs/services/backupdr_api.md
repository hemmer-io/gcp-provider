# Backupdr_api Service



**Resources**: 13

---

## Overview

The backupdr_api service provides access to 13 resource types:

- [Backup_plan](#backup_plan) [CRUD]
- [Backup_plan_association](#backup_plan_association) [CRUD]
- [Operation](#operation) [CRD]
- [Backup_vault](#backup_vault) [CRUD]
- [Trial](#trial) [C]
- [Backup](#backup) [CRUD]
- [Management_server](#management_server) [CRD]
- [Service_config](#service_config) [C]
- [Data_source](#data_source) [CRU]
- [Revision](#revision) [R]
- [Resource_backup_config](#resource_backup_config) [R]
- [Location](#location) [R]
- [Data_source_reference](#data_source_reference) [R]

---

## Resources


### Backup_plan

Create a BackupPlan

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_vault_service_account` | String |  | Output only. The Google Cloud Platform Service Account to be used by the BackupVault for taking backups. Specify the email address of the Backup Vault Service Account. |
| `state` | String |  | Output only. The `State` for the `BackupPlan`. |
| `log_retention_days` | String |  | Optional. Applicable only for CloudSQL resource_type. Configures how long logs will be stored. It is defined in “days”. This value should be greater than or equal to minimum enforced log retention duration of the backup vault. |
| `backup_rules` | Vec<String> |  | Optional. The backup rules for this `BackupPlan`. |
| `backup_vault` | String |  | Required. Resource name of backup vault which will be used as storage location for backups. Format: projects/{project}/locations/{location}/backupVaults/{backupvault} |
| `etag` | String |  | Optional. `etag` is returned from the service in the response. As a user of the service, you may provide an etag value in this field to prevent stale resources. |
| `labels` | HashMap<String, String> |  | Optional. This collection of key/value pairs allows for custom labels to be supplied by the user. Example, {"tag": "Weekly"}. |
| `max_custom_on_demand_retention_days` | i64 |  | Optional. Optional field to configure the maximum number of days for which a backup can be retained. This field is only applicable for on-demand backups taken with custom retention value. |
| `update_time` | String |  | Output only. When the `BackupPlan` was last updated. |
| `name` | String |  | Output only. Identifier. The resource name of the `BackupPlan`. Format: `projects/{project}/locations/{location}/backupPlans/{backup_plan}` |
| `create_time` | String |  | Output only. When the `BackupPlan` was created. |
| `revision_id` | String |  | Output only. The user friendly revision ID of the `BackupPlanRevision`. Example: v0, v1, v2, etc. |
| `revision_name` | String |  | Output only. The resource id of the `BackupPlanRevision`. Format: `projects/{project}/locations/{location}/backupPlans/{backup_plan}/revisions/{revision_id}` |
| `description` | String |  | Optional. The description of the `BackupPlan` resource. The description allows for additional details about `BackupPlan` and its use cases to be provided. An example description is the following: "This is a backup plan that performs a daily backup at 6pm and retains data for 3 months". The description must be at most 2048 characters. |
| `resource_type` | String |  | Required. The resource type to which the `BackupPlan` will be applied. Examples include, "compute.googleapis.com/Instance", "sqladmin.googleapis.com/Instance", "alloydb.googleapis.com/Cluster", "compute.googleapis.com/Disk". |
| `supported_resource_types` | Vec<String> |  | Output only. All resource types to which backupPlan can be applied. |
| `parent` | String | ✅ | Required. The `BackupPlan` project and location in the format `projects/{project}/locations/{location}`. In Cloud BackupDR locations map to GCP regions, for example **us-central1**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_vault_service_account` | String | Output only. The Google Cloud Platform Service Account to be used by the BackupVault for taking backups. Specify the email address of the Backup Vault Service Account. |
| `state` | String | Output only. The `State` for the `BackupPlan`. |
| `log_retention_days` | String | Optional. Applicable only for CloudSQL resource_type. Configures how long logs will be stored. It is defined in “days”. This value should be greater than or equal to minimum enforced log retention duration of the backup vault. |
| `backup_rules` | Vec<String> | Optional. The backup rules for this `BackupPlan`. |
| `backup_vault` | String | Required. Resource name of backup vault which will be used as storage location for backups. Format: projects/{project}/locations/{location}/backupVaults/{backupvault} |
| `etag` | String | Optional. `etag` is returned from the service in the response. As a user of the service, you may provide an etag value in this field to prevent stale resources. |
| `labels` | HashMap<String, String> | Optional. This collection of key/value pairs allows for custom labels to be supplied by the user. Example, {"tag": "Weekly"}. |
| `max_custom_on_demand_retention_days` | i64 | Optional. Optional field to configure the maximum number of days for which a backup can be retained. This field is only applicable for on-demand backups taken with custom retention value. |
| `update_time` | String | Output only. When the `BackupPlan` was last updated. |
| `name` | String | Output only. Identifier. The resource name of the `BackupPlan`. Format: `projects/{project}/locations/{location}/backupPlans/{backup_plan}` |
| `create_time` | String | Output only. When the `BackupPlan` was created. |
| `revision_id` | String | Output only. The user friendly revision ID of the `BackupPlanRevision`. Example: v0, v1, v2, etc. |
| `revision_name` | String | Output only. The resource id of the `BackupPlanRevision`. Format: `projects/{project}/locations/{location}/backupPlans/{backup_plan}/revisions/{revision_id}` |
| `description` | String | Optional. The description of the `BackupPlan` resource. The description allows for additional details about `BackupPlan` and its use cases to be provided. An example description is the following: "This is a backup plan that performs a daily backup at 6pm and retains data for 3 months". The description must be at most 2048 characters. |
| `resource_type` | String | Required. The resource type to which the `BackupPlan` will be applied. Examples include, "compute.googleapis.com/Instance", "sqladmin.googleapis.com/Instance", "alloydb.googleapis.com/Cluster", "compute.googleapis.com/Disk". |
| `supported_resource_types` | Vec<String> | Output only. All resource types to which backupPlan can be applied. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_plan
backup_plan = provider.backupdr_api.Backup_plan {
    parent = "value"  # Required. The `BackupPlan` project and location in the format `projects/{project}/locations/{location}`. In Cloud BackupDR locations map to GCP regions, for example **us-central1**.
}

# Access backup_plan outputs
backup_plan_id = backup_plan.id
backup_plan_backup_vault_service_account = backup_plan.backup_vault_service_account
backup_plan_state = backup_plan.state
backup_plan_log_retention_days = backup_plan.log_retention_days
backup_plan_backup_rules = backup_plan.backup_rules
backup_plan_backup_vault = backup_plan.backup_vault
backup_plan_etag = backup_plan.etag
backup_plan_labels = backup_plan.labels
backup_plan_max_custom_on_demand_retention_days = backup_plan.max_custom_on_demand_retention_days
backup_plan_update_time = backup_plan.update_time
backup_plan_name = backup_plan.name
backup_plan_create_time = backup_plan.create_time
backup_plan_revision_id = backup_plan.revision_id
backup_plan_revision_name = backup_plan.revision_name
backup_plan_description = backup_plan.description
backup_plan_resource_type = backup_plan.resource_type
backup_plan_supported_resource_types = backup_plan.supported_resource_types
```

---


### Backup_plan_association

Create a BackupPlanAssociation

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_plan` | String |  | Required. Resource name of backup plan which needs to be applied on workload. Format: projects/{project}/locations/{location}/backupPlans/{backupPlanId} |
| `backup_plan_revision_id` | String |  | Output only. The user friendly revision ID of the `BackupPlanRevision`. Example: v0, v1, v2, etc. |
| `cloud_sql_instance_backup_plan_association_properties` | String |  | Output only. Cloud SQL instance's backup plan association properties. |
| `create_time` | String |  | Output only. The time when the instance was created. |
| `update_time` | String |  | Output only. The time when the instance was updated. |
| `rules_config_info` | Vec<String> |  | Output only. The config info related to backup rules. |
| `resource_type` | String |  | Required. Immutable. Resource type of workload on which backupplan is applied |
| `backup_plan_revision_name` | String |  | Output only. The resource id of the `BackupPlanRevision`. Format: `projects/{project}/locations/{location}/backupPlans/{backup_plan}/revisions/{revision_id}` |
| `name` | String |  | Output only. Identifier. The resource name of BackupPlanAssociation in below format Format : projects/{project}/locations/{location}/backupPlanAssociations/{backupPlanAssociationId} |
| `data_source` | String |  | Output only. Resource name of data source which will be used as storage location for backups taken. Format : projects/{project}/locations/{location}/backupVaults/{backupvault}/dataSources/{datasource} |
| `resource` | String |  | Required. Immutable. Resource name of workload on which the backup plan is applied. The format can either be the resource name (e.g., "projects/my-project/zones/us-central1-a/instances/my-instance") or the full resource URI (e.g., "https://www.googleapis.com/compute/v1/projects/my-project/zones/us-central1-a/instances/my-instance"). |
| `state` | String |  | Output only. The BackupPlanAssociation resource state. |
| `parent` | String | ✅ | Required. The backup plan association project and location in the format `projects/{project_id}/locations/{location}`. In Cloud BackupDR locations map to GCP regions, for example **us-central1**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_plan` | String | Required. Resource name of backup plan which needs to be applied on workload. Format: projects/{project}/locations/{location}/backupPlans/{backupPlanId} |
| `backup_plan_revision_id` | String | Output only. The user friendly revision ID of the `BackupPlanRevision`. Example: v0, v1, v2, etc. |
| `cloud_sql_instance_backup_plan_association_properties` | String | Output only. Cloud SQL instance's backup plan association properties. |
| `create_time` | String | Output only. The time when the instance was created. |
| `update_time` | String | Output only. The time when the instance was updated. |
| `rules_config_info` | Vec<String> | Output only. The config info related to backup rules. |
| `resource_type` | String | Required. Immutable. Resource type of workload on which backupplan is applied |
| `backup_plan_revision_name` | String | Output only. The resource id of the `BackupPlanRevision`. Format: `projects/{project}/locations/{location}/backupPlans/{backup_plan}/revisions/{revision_id}` |
| `name` | String | Output only. Identifier. The resource name of BackupPlanAssociation in below format Format : projects/{project}/locations/{location}/backupPlanAssociations/{backupPlanAssociationId} |
| `data_source` | String | Output only. Resource name of data source which will be used as storage location for backups taken. Format : projects/{project}/locations/{location}/backupVaults/{backupvault}/dataSources/{datasource} |
| `resource` | String | Required. Immutable. Resource name of workload on which the backup plan is applied. The format can either be the resource name (e.g., "projects/my-project/zones/us-central1-a/instances/my-instance") or the full resource URI (e.g., "https://www.googleapis.com/compute/v1/projects/my-project/zones/us-central1-a/instances/my-instance"). |
| `state` | String | Output only. The BackupPlanAssociation resource state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_plan_association
backup_plan_association = provider.backupdr_api.Backup_plan_association {
    parent = "value"  # Required. The backup plan association project and location in the format `projects/{project_id}/locations/{location}`. In Cloud BackupDR locations map to GCP regions, for example **us-central1**.
}

# Access backup_plan_association outputs
backup_plan_association_id = backup_plan_association.id
backup_plan_association_backup_plan = backup_plan_association.backup_plan
backup_plan_association_backup_plan_revision_id = backup_plan_association.backup_plan_revision_id
backup_plan_association_cloud_sql_instance_backup_plan_association_properties = backup_plan_association.cloud_sql_instance_backup_plan_association_properties
backup_plan_association_create_time = backup_plan_association.create_time
backup_plan_association_update_time = backup_plan_association.update_time
backup_plan_association_rules_config_info = backup_plan_association.rules_config_info
backup_plan_association_resource_type = backup_plan_association.resource_type
backup_plan_association_backup_plan_revision_name = backup_plan_association.backup_plan_revision_name
backup_plan_association_name = backup_plan_association.name
backup_plan_association_data_source = backup_plan_association.data_source
backup_plan_association_resource = backup_plan_association.resource
backup_plan_association_state = backup_plan_association.state
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.backupdr_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
operation_error = operation.error
```

---


### Backup_vault

Creates a new BackupVault in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. No labels currently defined: |
| `name` | String |  | Output only. Identifier. Name of the backup vault to create. It must have the format`"projects/{project}/locations/{location}/backupVaults/{backupvault}"`. `{backupvault}` cannot be changed after creation. It must be between 3-63 characters long and must be unique within the project and location. |
| `deletable` | bool |  | Output only. Set to true when there are no backups nested under this resource. |
| `annotations` | HashMap<String, String> |  | Optional. User annotations. See https://google.aip.dev/128#annotations Stores small amounts of arbitrary data. |
| `service_account` | String |  | Output only. Service account used by the BackupVault Service for this BackupVault. The user should grant this account permissions in their workload project to enable the service to run backups and restores there. |
| `uid` | String |  | Output only. Immutable after resource creation until resource deletion. |
| `backup_minimum_enforced_retention_duration` | String |  | Required. The default and minimum enforced retention for each backup within the backup vault. The enforced retention for each backup can be extended. Note: Longer minimum enforced retention period impacts potential storage costs post introductory trial. We recommend starting with a short duration of 3 days or less. |
| `description` | String |  | Optional. The description of the BackupVault instance (2048 characters or less). |
| `effective_time` | String |  | Optional. Time after which the BackupVault resource is locked. |
| `access_restriction` | String |  | Optional. Note: This field is added for future use case and will not be supported in the current release. Access restriction for the backup vault. Default value is WITHIN_ORGANIZATION if not provided during creation. |
| `state` | String |  | Output only. The BackupVault resource instance state. |
| `update_time` | String |  | Output only. The time when the instance was updated. |
| `create_time` | String |  | Output only. The time when the instance was created. |
| `etag` | String |  | Optional. Server specified ETag for the backup vault resource to prevent simultaneous updates from overwiting each other. |
| `total_stored_bytes` | String |  | Output only. Total size of the storage used by all backup resources. |
| `backup_count` | String |  | Output only. The number of backups in this backup vault. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. No labels currently defined: |
| `name` | String | Output only. Identifier. Name of the backup vault to create. It must have the format`"projects/{project}/locations/{location}/backupVaults/{backupvault}"`. `{backupvault}` cannot be changed after creation. It must be between 3-63 characters long and must be unique within the project and location. |
| `deletable` | bool | Output only. Set to true when there are no backups nested under this resource. |
| `annotations` | HashMap<String, String> | Optional. User annotations. See https://google.aip.dev/128#annotations Stores small amounts of arbitrary data. |
| `service_account` | String | Output only. Service account used by the BackupVault Service for this BackupVault. The user should grant this account permissions in their workload project to enable the service to run backups and restores there. |
| `uid` | String | Output only. Immutable after resource creation until resource deletion. |
| `backup_minimum_enforced_retention_duration` | String | Required. The default and minimum enforced retention for each backup within the backup vault. The enforced retention for each backup can be extended. Note: Longer minimum enforced retention period impacts potential storage costs post introductory trial. We recommend starting with a short duration of 3 days or less. |
| `description` | String | Optional. The description of the BackupVault instance (2048 characters or less). |
| `effective_time` | String | Optional. Time after which the BackupVault resource is locked. |
| `access_restriction` | String | Optional. Note: This field is added for future use case and will not be supported in the current release. Access restriction for the backup vault. Default value is WITHIN_ORGANIZATION if not provided during creation. |
| `state` | String | Output only. The BackupVault resource instance state. |
| `update_time` | String | Output only. The time when the instance was updated. |
| `create_time` | String | Output only. The time when the instance was created. |
| `etag` | String | Optional. Server specified ETag for the backup vault resource to prevent simultaneous updates from overwiting each other. |
| `total_stored_bytes` | String | Output only. Total size of the storage used by all backup resources. |
| `backup_count` | String | Output only. The number of backups in this backup vault. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_vault
backup_vault = provider.backupdr_api.Backup_vault {
    parent = "value"  # Required. Value for parent.
}

# Access backup_vault outputs
backup_vault_id = backup_vault.id
backup_vault_labels = backup_vault.labels
backup_vault_name = backup_vault.name
backup_vault_deletable = backup_vault.deletable
backup_vault_annotations = backup_vault.annotations
backup_vault_service_account = backup_vault.service_account
backup_vault_uid = backup_vault.uid
backup_vault_backup_minimum_enforced_retention_duration = backup_vault.backup_minimum_enforced_retention_duration
backup_vault_description = backup_vault.description
backup_vault_effective_time = backup_vault.effective_time
backup_vault_access_restriction = backup_vault.access_restriction
backup_vault_state = backup_vault.state
backup_vault_update_time = backup_vault.update_time
backup_vault_create_time = backup_vault.create_time
backup_vault_etag = backup_vault.etag
backup_vault_total_stored_bytes = backup_vault.total_stored_bytes
backup_vault_backup_count = backup_vault.backup_count
```

---


### Trial

Subscribes to a trial for a project

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. The project where this trial will be created. Format: projects/{project}/locations/{location} Supported Locations are - us, eu and asia. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trial
trial = provider.backupdr_api.Trial {
    parent = "value"  # Required. The project where this trial will be created. Format: projects/{project}/locations/{location} Supported Locations are - us, eu and asia.
}

```

---


### Backup

Restore from a Backup

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disk_restore_properties` | String |  | Disk properties to be overridden during restore. |
| `disk_target_environment` | String |  | Disk target environment to be used during restore. |
| `compute_instance_restore_properties` | String |  | Compute Engine instance properties to be overridden during restore. |
| `compute_instance_target_environment` | String |  | Compute Engine target environment to be used during restore. |
| `region_disk_target_environment` | String |  | Region disk target environment to be used during restore. |
| `request_id` | String |  | Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes after the first request. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000). |
| `name` | String | ✅ | Required. The resource name of the Backup instance, in the format 'projects/*/locations/*/backupVaults/*/dataSources/*/backups/'. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the instance was created. |
| `resource_size_bytes` | String | Output only. source resource size in bytes at the time of the backup. |
| `backup_appliance_backup_properties` | String | Output only. Backup Appliance specific backup properties. |
| `backup_type` | String | Output only. Type of the backup, unspecified, scheduled or ondemand. |
| `disk_backup_properties` | String | Output only. Disk specific backup properties. |
| `backup_appliance_locks` | Vec<String> | Optional. The list of BackupLocks taken by the accessor Backup Appliance. |
| `cloud_sql_instance_backup_properties` | String | Output only. Cloud SQL specific backup properties. |
| `gcp_backup_plan_info` | String | Output only. Configuration for a Google Cloud resource. |
| `expire_time` | String | Optional. When this backup is automatically expired. |
| `compute_instance_backup_properties` | String | Output only. Compute Engine specific backup properties. |
| `consistency_time` | String | Output only. The point in time when this backup was captured from the source. |
| `gcp_resource` | String | Output only. Unique identifier of the GCP resource that is being backed up. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. No labels currently defined. |
| `etag` | String | Optional. Server specified ETag to prevent updates from overwriting each other. |
| `satisfies_pzs` | bool | Optional. Output only. Reserved for future use. |
| `service_locks` | Vec<String> | Output only. The list of BackupLocks taken by the service to prevent the deletion of the backup. |
| `description` | String | Output only. The description of the Backup instance (2048 characters or less). |
| `update_time` | String | Output only. The time when the instance was updated. |
| `alloy_db_backup_properties` | String | Output only. AlloyDB specific backup properties. |
| `enforced_retention_end_time` | String | Optional. The backup can not be deleted before this time. |
| `name` | String | Output only. Identifier. Name of the backup to create. It must have the format`"projects//locations//backupVaults//dataSources/{datasource}/backups/{backup}"`. `{backup}` cannot be changed after creation. It must be between 3-63 characters long and must be unique within the datasource. |
| `satisfies_pzi` | bool | Optional. Output only. Reserved for future use. |
| `state` | String | Output only. The Backup resource instance state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.backupdr_api.Backup {
    name = "value"  # Required. The resource name of the Backup instance, in the format 'projects/*/locations/*/backupVaults/*/dataSources/*/backups/'.
}

# Access backup outputs
backup_id = backup.id
backup_create_time = backup.create_time
backup_resource_size_bytes = backup.resource_size_bytes
backup_backup_appliance_backup_properties = backup.backup_appliance_backup_properties
backup_backup_type = backup.backup_type
backup_disk_backup_properties = backup.disk_backup_properties
backup_backup_appliance_locks = backup.backup_appliance_locks
backup_cloud_sql_instance_backup_properties = backup.cloud_sql_instance_backup_properties
backup_gcp_backup_plan_info = backup.gcp_backup_plan_info
backup_expire_time = backup.expire_time
backup_compute_instance_backup_properties = backup.compute_instance_backup_properties
backup_consistency_time = backup.consistency_time
backup_gcp_resource = backup.gcp_resource
backup_labels = backup.labels
backup_etag = backup.etag
backup_satisfies_pzs = backup.satisfies_pzs
backup_service_locks = backup.service_locks
backup_description = backup.description
backup_update_time = backup.update_time
backup_alloy_db_backup_properties = backup.alloy_db_backup_properties
backup_enforced_retention_end_time = backup.enforced_retention_end_time
backup_name = backup.name
backup_satisfies_pzi = backup.satisfies_pzi
backup_state = backup.state
```

---


### Management_server

Creates a new ManagementServer in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the instance was created. |
| `etag` | String |  | Optional. Server specified ETag for the ManagementServer resource to prevent simultaneous updates from overwiting each other. |
| `workforce_identity_based_management_uri` | String |  | Output only. The hostnames of the exposed AGM endpoints for both types of user i.e. 1p and 3p, used to connect AGM/RM UI. |
| `workforce_identity_based_oauth2_client_id` | String |  | Output only. The OAuth client IDs for both types of user i.e. 1p and 3p. |
| `ba_proxy_uri` | Vec<String> |  | Output only. The hostname or ip address of the exposed AGM endpoints, used by BAs to connect to BA proxy. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. Labels currently defined: 1. migrate_from_go= If set to true, the MS is created in migration ready mode. |
| `description` | String |  | Optional. The description of the ManagementServer instance (2048 characters or less). |
| `type` | String |  | Optional. The type of the ManagementServer resource. |
| `update_time` | String |  | Output only. The time when the instance was updated. |
| `management_uri` | String |  | Output only. The hostname or ip address of the exposed AGM endpoints, used by clients to connect to AGM/RD graphical user interface and APIs. |
| `name` | String |  | Output only. Identifier. The resource name. |
| `networks` | Vec<String> |  | Optional. VPC networks to which the ManagementServer instance is connected. For this version, only a single network is supported. This field is optional if MS is created without PSA |
| `oauth2_client_id` | String |  | Output only. The OAuth 2.0 client id is required to make API calls to the BackupDR instance API of this ManagementServer. This is the value that should be provided in the 'aud' field of the OIDC ID Token (see openid specification https://openid.net/specs/openid-connect-core-1_0.html#IDToken). |
| `state` | String |  | Output only. The ManagementServer state. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The management server project and location in the format 'projects/{project_id}/locations/{location}'. In Cloud Backup and DR locations map to Google Cloud regions, for example **us-central1**. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the instance was created. |
| `etag` | String | Optional. Server specified ETag for the ManagementServer resource to prevent simultaneous updates from overwiting each other. |
| `workforce_identity_based_management_uri` | String | Output only. The hostnames of the exposed AGM endpoints for both types of user i.e. 1p and 3p, used to connect AGM/RM UI. |
| `workforce_identity_based_oauth2_client_id` | String | Output only. The OAuth client IDs for both types of user i.e. 1p and 3p. |
| `ba_proxy_uri` | Vec<String> | Output only. The hostname or ip address of the exposed AGM endpoints, used by BAs to connect to BA proxy. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. Labels currently defined: 1. migrate_from_go= If set to true, the MS is created in migration ready mode. |
| `description` | String | Optional. The description of the ManagementServer instance (2048 characters or less). |
| `type` | String | Optional. The type of the ManagementServer resource. |
| `update_time` | String | Output only. The time when the instance was updated. |
| `management_uri` | String | Output only. The hostname or ip address of the exposed AGM endpoints, used by clients to connect to AGM/RD graphical user interface and APIs. |
| `name` | String | Output only. Identifier. The resource name. |
| `networks` | Vec<String> | Optional. VPC networks to which the ManagementServer instance is connected. For this version, only a single network is supported. This field is optional if MS is created without PSA |
| `oauth2_client_id` | String | Output only. The OAuth 2.0 client id is required to make API calls to the BackupDR instance API of this ManagementServer. This is the value that should be provided in the 'aud' field of the OIDC ID Token (see openid specification https://openid.net/specs/openid-connect-core-1_0.html#IDToken). |
| `state` | String | Output only. The ManagementServer state. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create management_server
management_server = provider.backupdr_api.Management_server {
    parent = "value"  # Required. The management server project and location in the format 'projects/{project_id}/locations/{location}'. In Cloud Backup and DR locations map to Google Cloud regions, for example **us-central1**.
}

# Access management_server outputs
management_server_id = management_server.id
management_server_create_time = management_server.create_time
management_server_etag = management_server.etag
management_server_workforce_identity_based_management_uri = management_server.workforce_identity_based_management_uri
management_server_workforce_identity_based_oauth2_client_id = management_server.workforce_identity_based_oauth2_client_id
management_server_ba_proxy_uri = management_server.ba_proxy_uri
management_server_labels = management_server.labels
management_server_description = management_server.description
management_server_type = management_server.type
management_server_update_time = management_server.update_time
management_server_management_uri = management_server.management_uri
management_server_name = management_server.name
management_server_networks = management_server.networks
management_server_oauth2_client_id = management_server.oauth2_client_id
management_server_state = management_server.state
management_server_satisfies_pzi = management_server.satisfies_pzi
management_server_satisfies_pzs = management_server.satisfies_pzs
```

---


### Service_config

Initializes the service related config for a project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_id` | String |  | Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and t he request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000). |
| `resource_type` | String |  | Required. The resource type to which the default service config will be applied. Examples include, "compute.googleapis.com/Instance" and "storage.googleapis.com/Bucket". |
| `cloud_sql_instance_initialization_config` | String |  | Optional. The configuration for initializing a Cloud SQL instance. |
| `name` | String | ✅ | Required. The resource name of the serviceConfig used to initialize the service. Format: `projects/{project_id}/locations/{location}/serviceConfig`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_config
service_config = provider.backupdr_api.Service_config {
    name = "value"  # Required. The resource name of the serviceConfig used to initialize the service. Format: `projects/{project_id}/locations/{location}/serviceConfig`.
}

```

---


### Data_source

Internal only. Finalize a backup that was started by a call to InitiateBackup.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consistency_time` | String |  | The point in time when this backup was captured from the source. This will be assigned to the consistency_time field of the newly created Backup. |
| `recovery_range_start_time` | String |  | The earliest timestamp of data available in this Backup. This will set on the newly created Backup. |
| `description` | String |  | This will be assigned to the description field of the newly created Backup. |
| `retention_duration` | String |  | The ExpireTime on the backup will be set to FinalizeTime plus this duration. If the resulting ExpireTime is less than EnforcedRetentionEndTime, then ExpireTime is set to EnforcedRetentionEndTime. |
| `backup_id` | String |  | Required. Resource ID of the Backup resource to be finalized. This must be the same backup_id that was used in the InitiateBackupRequest. |
| `recovery_range_end_time` | String |  | The latest timestamp of data available in this Backup. This will be set on the newly created Backup. |
| `request_id` | String |  | Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes after the first request. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000). |
| `data_source` | String | ✅ | Required. The resource name of the instance, in the format 'projects/*/locations/*/backupVaults/*/dataSources/'. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_source_gcp_resource` | String | The backed up resource is a Google Cloud resource. The word 'DataSource' was included in the names to indicate that this is the representation of the Google Cloud resource used within the DataSource object. |
| `backup_count` | String | Number of backups in the data source. |
| `backup_config_info` | String | Output only. Details of how the resource is configured for backup. |
| `data_source_backup_appliance_application` | String | The backed up resource is a backup appliance application. |
| `state` | String | Output only. The DataSource resource instance state. |
| `total_stored_bytes` | String | The number of bytes (metadata and data) stored in this datasource. |
| `etag` | String | Server specified ETag for the ManagementServer resource to prevent simultaneous updates from overwiting each other. |
| `name` | String | Output only. Identifier. Name of the datasource to create. It must have the format`"projects/{project}/locations/{location}/backupVaults/{backupvault}/dataSources/{datasource}"`. `{datasource}` cannot be changed after creation. It must be between 3-63 characters long and must be unique within the backup vault. |
| `update_time` | String | Output only. The time when the instance was updated. |
| `create_time` | String | Output only. The time when the instance was created. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. No labels currently defined: |
| `config_state` | String | Output only. The backup configuration state. |
| `backup_blocked_by_vault_access_restriction` | bool | Output only. This field is set to true if the backup is blocked by vault access restriction. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_source
data_source = provider.backupdr_api.Data_source {
    data_source = "value"  # Required. The resource name of the instance, in the format 'projects/*/locations/*/backupVaults/*/dataSources/'.
}

# Access data_source outputs
data_source_id = data_source.id
data_source_data_source_gcp_resource = data_source.data_source_gcp_resource
data_source_backup_count = data_source.backup_count
data_source_backup_config_info = data_source.backup_config_info
data_source_data_source_backup_appliance_application = data_source.data_source_backup_appliance_application
data_source_state = data_source.state
data_source_total_stored_bytes = data_source.total_stored_bytes
data_source_etag = data_source.etag
data_source_name = data_source.name
data_source_update_time = data_source.update_time
data_source_create_time = data_source.create_time
data_source_labels = data_source.labels
data_source_config_state = data_source.config_state
data_source_backup_blocked_by_vault_access_restriction = data_source.backup_blocked_by_vault_access_restriction
```

---


### Revision

Gets details of a single BackupPlanRevision.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Identifier. The resource name of the `BackupPlanRevision`. Format: `projects/{project}/locations/{location}/backupPlans/{backup_plan}/revisions/{revision}` |
| `backup_plan_snapshot` | String | The Backup Plan being encompassed by this revision. |
| `revision_id` | String | Output only. The user friendly revision ID of the `BackupPlanRevision`. Example: v0, v1, v2, etc. |
| `create_time` | String | Output only. The timestamp that the revision was created. |
| `state` | String | Output only. Resource State |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access revision outputs
revision_id = revision.id
revision_name = revision.name
revision_backup_plan_snapshot = revision.backup_plan_snapshot
revision_revision_id = revision.revision_id
revision_create_time = revision.create_time
revision_state = revision.state
```

---


### Resource_backup_config

Lists ResourceBackupConfigs.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token identifying a page of results the server should return. |
| `resource_backup_configs` | Vec<String> | The list of ResourceBackupConfigs for the specified scope. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access resource_backup_config outputs
resource_backup_config_id = resource_backup_config.id
resource_backup_config_next_page_token = resource_backup_config.next_page_token
resource_backup_config_resource_backup_configs = resource_backup_config.resource_backup_configs
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_display_name = location.display_name
location_name = location.name
location_metadata = location.metadata
location_labels = location.labels
location_location_id = location.location_id
```

---


### Data_source_reference

Gets details of a single DataSourceReference.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_source_backup_count` | String | Output only. Number of backups in the DataSource. |
| `data_source_backup_config_info` | String | Output only. Information of backup configuration on the DataSource. |
| `create_time` | String | Output only. The time when the DataSourceReference was created. |
| `data_source_backup_config_state` | String | Output only. The backup configuration state of the DataSource. |
| `data_source_gcp_resource_info` | String | Output only. The GCP resource that the DataSource is associated with. |
| `name` | String | Identifier. The resource name of the DataSourceReference. Format: projects/{project}/locations/{location}/dataSourceReferences/{data_source_reference} |
| `total_stored_bytes` | String | Output only. Total size of the storage used by all backup resources for the referenced datasource. |
| `data_source` | String | Output only. The resource name of the DataSource. Format: projects/{project}/locations/{location}/backupVaults/{backupVault}/dataSources/{dataSource} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access data_source_reference outputs
data_source_reference_id = data_source_reference.id
data_source_reference_data_source_backup_count = data_source_reference.data_source_backup_count
data_source_reference_data_source_backup_config_info = data_source_reference.data_source_backup_config_info
data_source_reference_create_time = data_source_reference.create_time
data_source_reference_data_source_backup_config_state = data_source_reference.data_source_backup_config_state
data_source_reference_data_source_gcp_resource_info = data_source_reference.data_source_gcp_resource_info
data_source_reference_name = data_source_reference.name
data_source_reference_total_stored_bytes = data_source_reference.total_stored_bytes
data_source_reference_data_source = data_source_reference.data_source
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple backup_plan resources
backup_plan_0 = provider.backupdr_api.Backup_plan {
    parent = "value-0"
}
backup_plan_1 = provider.backupdr_api.Backup_plan {
    parent = "value-1"
}
backup_plan_2 = provider.backupdr_api.Backup_plan {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    backup_plan = provider.backupdr_api.Backup_plan {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Backupdr_api Documentation](https://cloud.google.com/backupdr_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
