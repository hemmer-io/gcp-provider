# Gkebackup_api Service



**Resources**: 12

---

## Overview

The gkebackup_api service provides access to 12 resource types:

- [Operation](#operation) [CRD]
- [Backup_plan_binding](#backup_plan_binding) [R]
- [Backup_plan](#backup_plan) [CRUD]
- [Restore_channel](#restore_channel) [CRUD]
- [Restore_plan_binding](#restore_plan_binding) [R]
- [Backup](#backup) [CRUD]
- [Volume_backup](#volume_backup) [CR]
- [Restore_plan](#restore_plan) [CRUD]
- [Restore](#restore) [CRUD]
- [Location](#location) [R]
- [Volume_restore](#volume_restore) [CR]
- [Backup_channel](#backup_channel) [CRUD]

---

## Resources


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
operation = provider.gkebackup_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
```

---


### Backup_plan_binding

Retrieve the details of a single BackupPlanBinding.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster` | String | Output only. Immutable. The fully qualified name of the cluster that is being backed up Valid formats: - `projects/*/locations/*/clusters/*` - `projects/*/zones/*/clusters/*` |
| `update_time` | String | Output only. The timestamp when this binding was created. |
| `create_time` | String | Output only. The timestamp when this binding was created. |
| `backup_plan` | String | Output only. Immutable. The fully qualified name of the BackupPlan bound with the parent BackupChannel. `projects/*/locations/*/backupPlans/{backup_plan}` |
| `name` | String | Identifier. The fully qualified name of the BackupPlanBinding. `projects/*/locations/*/backupChannels/*/backupPlanBindings/*` |
| `backup_plan_details` | String | Output only. Contains details about the backup plan/backup. |
| `uid` | String | Output only. Server generated global unique identifier of [UUID4](https://en.wikipedia.org/wiki/Universally_unique_identifier) |
| `etag` | String | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a BackupPlanBinding from overwriting each other. It is strongly suggested that systems make use of the 'etag' in the read-modify-write cycle to perform BackupPlanBinding updates in order to avoid race conditions: An `etag` is returned in the response to `GetBackupPlanBinding`, and systems are expected to put that etag in the request to `UpdateBackupPlanBinding` or `DeleteBackupPlanBinding` to ensure that their change will be applied to the same version of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access backup_plan_binding outputs
backup_plan_binding_id = backup_plan_binding.id
backup_plan_binding_cluster = backup_plan_binding.cluster
backup_plan_binding_update_time = backup_plan_binding.update_time
backup_plan_binding_create_time = backup_plan_binding.create_time
backup_plan_binding_backup_plan = backup_plan_binding.backup_plan
backup_plan_binding_name = backup_plan_binding.name
backup_plan_binding_backup_plan_details = backup_plan_binding.backup_plan_details
backup_plan_binding_uid = backup_plan_binding.uid
backup_plan_binding_etag = backup_plan_binding.etag
```

---


### Backup_plan

Creates a new BackupPlan in a given location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Identifier. The full name of the BackupPlan resource. Format: `projects/*/locations/*/backupPlans/*` |
| `protected_pod_count` | i64 |  | Output only. The number of Kubernetes Pods backed up in the last successful Backup created via this BackupPlan. |
| `description` | String |  | Optional. User specified descriptive string for this BackupPlan. |
| `rpo_risk_level` | i64 |  | Output only. A number that represents the current risk level of this BackupPlan from RPO perspective with 1 being no risk and 5 being highest risk. |
| `cluster` | String |  | Required. Immutable. The source cluster from which Backups will be created via this BackupPlan. Valid formats: - `projects/*/locations/*/clusters/*` - `projects/*/zones/*/clusters/*` |
| `state` | String |  | Output only. State of the BackupPlan. This State field reflects the various stages a BackupPlan can be in during the Create operation. It will be set to "DEACTIVATED" if the BackupPlan is deactivated on an Update |
| `deactivated` | bool |  | Optional. This flag indicates whether this BackupPlan has been deactivated. Setting this field to True locks the BackupPlan such that no further updates will be allowed (except deletes), including the deactivated field itself. It also prevents any new Backups from being created via this BackupPlan (including scheduled Backups). Default: False |
| `backup_schedule` | String |  | Optional. Defines a schedule for automatic Backup creation via this BackupPlan. |
| `last_successful_backup_time` | String |  | Output only. Completion time of the last successful Backup. This is sourced from a successful Backup's complete_time field. This field is added to maintain consistency with BackupPlanBinding to display last successful backup time. |
| `retention_policy` | String |  | Optional. RetentionPolicy governs lifecycle of Backups created under this plan. |
| `backup_config` | String |  | Optional. Defines the configuration of Backups created via this BackupPlan. |
| `create_time` | String |  | Output only. The timestamp when this BackupPlan resource was created. |
| `uid` | String |  | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `labels` | HashMap<String, String> |  | Optional. A set of custom labels supplied by user. |
| `state_reason` | String |  | Output only. Human-readable description of why BackupPlan is in the current `state`. This field is only meant for human readability and should not be used programmatically as this field is not guaranteed to be consistent. |
| `backup_channel` | String |  | Output only. The fully qualified name of the BackupChannel to be used to create a backup. This field is set only if the cluster being backed up is in a different project. `projects/*/locations/*/backupChannels/*` |
| `rpo_risk_reason` | String |  | Output only. Human-readable description of why the BackupPlan is in the current rpo_risk_level and action items if any. |
| `update_time` | String |  | Output only. The timestamp when this BackupPlan resource was last updated. |
| `etag` | String |  | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a backup plan from overwriting each other. It is strongly suggested that systems make use of the 'etag' in the read-modify-write cycle to perform BackupPlan updates in order to avoid race conditions: An `etag` is returned in the response to `GetBackupPlan`, and systems are expected to put that etag in the request to `UpdateBackupPlan` or `DeleteBackupPlan` to ensure that their change will be applied to the same version of the resource. |
| `parent` | String | ✅ | Required. The location within which to create the BackupPlan. Format: `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Identifier. The full name of the BackupPlan resource. Format: `projects/*/locations/*/backupPlans/*` |
| `protected_pod_count` | i64 | Output only. The number of Kubernetes Pods backed up in the last successful Backup created via this BackupPlan. |
| `description` | String | Optional. User specified descriptive string for this BackupPlan. |
| `rpo_risk_level` | i64 | Output only. A number that represents the current risk level of this BackupPlan from RPO perspective with 1 being no risk and 5 being highest risk. |
| `cluster` | String | Required. Immutable. The source cluster from which Backups will be created via this BackupPlan. Valid formats: - `projects/*/locations/*/clusters/*` - `projects/*/zones/*/clusters/*` |
| `state` | String | Output only. State of the BackupPlan. This State field reflects the various stages a BackupPlan can be in during the Create operation. It will be set to "DEACTIVATED" if the BackupPlan is deactivated on an Update |
| `deactivated` | bool | Optional. This flag indicates whether this BackupPlan has been deactivated. Setting this field to True locks the BackupPlan such that no further updates will be allowed (except deletes), including the deactivated field itself. It also prevents any new Backups from being created via this BackupPlan (including scheduled Backups). Default: False |
| `backup_schedule` | String | Optional. Defines a schedule for automatic Backup creation via this BackupPlan. |
| `last_successful_backup_time` | String | Output only. Completion time of the last successful Backup. This is sourced from a successful Backup's complete_time field. This field is added to maintain consistency with BackupPlanBinding to display last successful backup time. |
| `retention_policy` | String | Optional. RetentionPolicy governs lifecycle of Backups created under this plan. |
| `backup_config` | String | Optional. Defines the configuration of Backups created via this BackupPlan. |
| `create_time` | String | Output only. The timestamp when this BackupPlan resource was created. |
| `uid` | String | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `labels` | HashMap<String, String> | Optional. A set of custom labels supplied by user. |
| `state_reason` | String | Output only. Human-readable description of why BackupPlan is in the current `state`. This field is only meant for human readability and should not be used programmatically as this field is not guaranteed to be consistent. |
| `backup_channel` | String | Output only. The fully qualified name of the BackupChannel to be used to create a backup. This field is set only if the cluster being backed up is in a different project. `projects/*/locations/*/backupChannels/*` |
| `rpo_risk_reason` | String | Output only. Human-readable description of why the BackupPlan is in the current rpo_risk_level and action items if any. |
| `update_time` | String | Output only. The timestamp when this BackupPlan resource was last updated. |
| `etag` | String | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a backup plan from overwriting each other. It is strongly suggested that systems make use of the 'etag' in the read-modify-write cycle to perform BackupPlan updates in order to avoid race conditions: An `etag` is returned in the response to `GetBackupPlan`, and systems are expected to put that etag in the request to `UpdateBackupPlan` or `DeleteBackupPlan` to ensure that their change will be applied to the same version of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_plan
backup_plan = provider.gkebackup_api.Backup_plan {
    parent = "value"  # Required. The location within which to create the BackupPlan. Format: `projects/*/locations/*`
}

# Access backup_plan outputs
backup_plan_id = backup_plan.id
backup_plan_name = backup_plan.name
backup_plan_protected_pod_count = backup_plan.protected_pod_count
backup_plan_description = backup_plan.description
backup_plan_rpo_risk_level = backup_plan.rpo_risk_level
backup_plan_cluster = backup_plan.cluster
backup_plan_state = backup_plan.state
backup_plan_deactivated = backup_plan.deactivated
backup_plan_backup_schedule = backup_plan.backup_schedule
backup_plan_last_successful_backup_time = backup_plan.last_successful_backup_time
backup_plan_retention_policy = backup_plan.retention_policy
backup_plan_backup_config = backup_plan.backup_config
backup_plan_create_time = backup_plan.create_time
backup_plan_uid = backup_plan.uid
backup_plan_labels = backup_plan.labels
backup_plan_state_reason = backup_plan.state_reason
backup_plan_backup_channel = backup_plan.backup_channel
backup_plan_rpo_risk_reason = backup_plan.rpo_risk_reason
backup_plan_update_time = backup_plan.update_time
backup_plan_etag = backup_plan.etag
```

---


### Restore_channel

Creates a new RestoreChannel in a given location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The fully qualified name of the RestoreChannel. `projects/*/locations/*/restoreChannels/*` |
| `uid` | String |  | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `destination_project_id` | String |  | Output only. The project_id where backups will be restored. Example Project ID: "my-project-id". This will be an OUTPUT_ONLY field to return the project_id of the destination project. |
| `etag` | String |  | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a RestoreChannel from overwriting each other. It is strongly suggested that systems make use of the 'etag' in the read-modify-write cycle to perform RestoreChannel updates in order to avoid race conditions: An `etag` is returned in the response to `GetRestoreChannel`, and systems are expected to put that etag in the request to `UpdateRestoreChannel` or `DeleteRestoreChannel` to ensure that their change will be applied to the same version of the resource. |
| `labels` | HashMap<String, String> |  | Optional. A set of custom labels supplied by user. |
| `create_time` | String |  | Output only. The timestamp when this RestoreChannel was created. |
| `destination_project` | String |  | Required. Immutable. The project into which the backups will be restored. The format is `projects/{projectId}` or `projects/{projectNumber}`. |
| `description` | String |  | Optional. User specified descriptive string for this RestoreChannel. |
| `update_time` | String |  | Output only. The timestamp when this RestoreChannel was last updated. |
| `parent` | String | ✅ | Required. The location within which to create the RestoreChannel. Format: `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The fully qualified name of the RestoreChannel. `projects/*/locations/*/restoreChannels/*` |
| `uid` | String | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `destination_project_id` | String | Output only. The project_id where backups will be restored. Example Project ID: "my-project-id". This will be an OUTPUT_ONLY field to return the project_id of the destination project. |
| `etag` | String | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a RestoreChannel from overwriting each other. It is strongly suggested that systems make use of the 'etag' in the read-modify-write cycle to perform RestoreChannel updates in order to avoid race conditions: An `etag` is returned in the response to `GetRestoreChannel`, and systems are expected to put that etag in the request to `UpdateRestoreChannel` or `DeleteRestoreChannel` to ensure that their change will be applied to the same version of the resource. |
| `labels` | HashMap<String, String> | Optional. A set of custom labels supplied by user. |
| `create_time` | String | Output only. The timestamp when this RestoreChannel was created. |
| `destination_project` | String | Required. Immutable. The project into which the backups will be restored. The format is `projects/{projectId}` or `projects/{projectNumber}`. |
| `description` | String | Optional. User specified descriptive string for this RestoreChannel. |
| `update_time` | String | Output only. The timestamp when this RestoreChannel was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create restore_channel
restore_channel = provider.gkebackup_api.Restore_channel {
    parent = "value"  # Required. The location within which to create the RestoreChannel. Format: `projects/*/locations/*`
}

# Access restore_channel outputs
restore_channel_id = restore_channel.id
restore_channel_name = restore_channel.name
restore_channel_uid = restore_channel.uid
restore_channel_destination_project_id = restore_channel.destination_project_id
restore_channel_etag = restore_channel.etag
restore_channel_labels = restore_channel.labels
restore_channel_create_time = restore_channel.create_time
restore_channel_destination_project = restore_channel.destination_project
restore_channel_description = restore_channel.description
restore_channel_update_time = restore_channel.update_time
```

---


### Restore_plan_binding

Retrieve the details of a single RestorePlanBinding.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. Server generated global unique identifier of [UUID4](https://en.wikipedia.org/wiki/Universally_unique_identifier) |
| `update_time` | String | Output only. The timestamp when this binding was created. |
| `restore_plan` | String | Output only. The fully qualified name of the RestorePlan bound to this RestoreChannel. `projects/*/locations/*/restorePlans/{restore_plan}` |
| `backup_plan` | String | Output only. The fully qualified name of the BackupPlan bound to the specified RestorePlan. `projects/*/locations/*/backukpPlans/{backup_plan}` |
| `etag` | String | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a RestorePlanBinding from overwriting each other. It is strongly suggested that systems make use of the 'etag' in the read-modify-write cycle to perform RestorePlanBinding updates in order to avoid race conditions: An `etag` is returned in the response to `GetRestorePlanBinding`, and systems are expected to put that etag in the request to `UpdateRestorePlanBinding` or `DeleteRestorePlanBinding` to ensure that their change will be applied to the same version of the resource. |
| `create_time` | String | Output only. The timestamp when this binding was created. |
| `name` | String | Identifier. The fully qualified name of the RestorePlanBinding. `projects/*/locations/*/restoreChannels/*/restorePlanBindings/*` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access restore_plan_binding outputs
restore_plan_binding_id = restore_plan_binding.id
restore_plan_binding_uid = restore_plan_binding.uid
restore_plan_binding_update_time = restore_plan_binding.update_time
restore_plan_binding_restore_plan = restore_plan_binding.restore_plan
restore_plan_binding_backup_plan = restore_plan_binding.backup_plan
restore_plan_binding_etag = restore_plan_binding.etag
restore_plan_binding_create_time = restore_plan_binding.create_time
restore_plan_binding_name = restore_plan_binding.name
```

---


### Backup

Creates a Backup for the given BackupPlan.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `manual` | bool |  | Output only. This flag indicates whether this Backup resource was created manually by a user or via a schedule in the BackupPlan. A value of True means that the Backup was created manually. |
| `contains_volume_data` | bool |  | Output only. Whether or not the Backup contains volume data. Controlled by the parent BackupPlan's include_volume_data value. |
| `etag` | String |  | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a backup from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform backup updates in order to avoid race conditions: An `etag` is returned in the response to `GetBackup`, and systems are expected to put that etag in the request to `UpdateBackup` or `DeleteBackup` to ensure that their change will be applied to the same version of the resource. |
| `permissive_mode` | bool |  | Output only. If false, Backup will fail when Backup for GKE detects Kubernetes configuration that is non-standard or requires additional setup to restore. Inherited from the parent BackupPlan's permissive_mode value. |
| `state` | String |  | Output only. Current state of the Backup |
| `description` | String |  | Optional. User specified descriptive string for this Backup. |
| `volume_count` | i64 |  | Output only. The total number of volume backups contained in the Backup. |
| `selected_applications` | String |  | Output only. If set, the list of ProtectedApplications whose resources were included in the Backup. |
| `selected_namespace_labels` | String |  | Output only. If set, the list of labels whose constituent namespaces were included in the Backup. |
| `all_namespaces` | bool |  | Output only. If True, all namespaces were included in the Backup. |
| `delete_lock_expire_time` | String |  | Output only. The time at which an existing delete lock will expire for this backup (calculated from create_time + delete_lock_days). |
| `satisfies_pzi` | bool |  | Output only. [Output Only] Reserved for future use. |
| `uid` | String |  | Output only. Server generated global unique identifier of [UUID4](https://en.wikipedia.org/wiki/Universally_unique_identifier) |
| `cluster_metadata` | String |  | Output only. Information about the GKE cluster from which this Backup was created. |
| `contains_secrets` | bool |  | Output only. Whether or not the Backup contains Kubernetes Secrets. Controlled by the parent BackupPlan's include_secrets value. |
| `resource_count` | i64 |  | Output only. The total number of Kubernetes resources included in the Backup. |
| `retain_days` | i64 |  | Optional. The age (in days) after which this Backup will be automatically deleted. Must be an integer value >= 0: - If 0, no automatic deletion will occur for this Backup. - If not 0, this must be >= delete_lock_days and <= 365. Once a Backup is created, this value may only be increased. Defaults to the parent BackupPlan's backup_retain_days value. |
| `satisfies_pzs` | bool |  | Output only. [Output Only] Reserved for future use. |
| `retain_expire_time` | String |  | Output only. The time at which this Backup will be automatically deleted (calculated from create_time + retain_days). |
| `troubleshooting_info` | String |  | Output only. Information about the troubleshooting steps which will provide debugging information to the end users. |
| `complete_time` | String |  | Output only. Completion time of the Backup |
| `selected_namespaces` | String |  | Output only. If set, the list of namespaces that were included in the Backup. |
| `labels` | HashMap<String, String> |  | Optional. A set of custom labels supplied by user. |
| `size_bytes` | String |  | Output only. The total size of the Backup in bytes = config backup size + sum(volume backup sizes) |
| `create_time` | String |  | Output only. The timestamp when this Backup resource was created. |
| `delete_lock_days` | i64 |  | Optional. Minimum age for this Backup (in days). If this field is set to a non-zero value, the Backup will be "locked" against deletion (either manual or automatic deletion) for the number of days provided (measured from the creation time of the Backup). MUST be an integer value between 0-90 (inclusive). Defaults to parent BackupPlan's backup_delete_lock_days setting and may only be increased (either at creation time or in a subsequent update). |
| `pod_count` | i64 |  | Output only. The total number of Kubernetes Pods contained in the Backup. |
| `config_backup_size_bytes` | String |  | Output only. The size of the config backup in bytes. |
| `state_reason` | String |  | Output only. Human-readable description of why the backup is in the current `state`. This field is only meant for human readability and should not be used programmatically as this field is not guaranteed to be consistent. |
| `encryption_key` | String |  | Output only. The customer managed encryption key that was used to encrypt the Backup's artifacts. Inherited from the parent BackupPlan's encryption_key value. |
| `name` | String |  | Output only. Identifier. The fully qualified name of the Backup. `projects/*/locations/*/backupPlans/*/backups/*` |
| `update_time` | String |  | Output only. The timestamp when this Backup resource was last updated. |
| `parent` | String | ✅ | Required. The BackupPlan within which to create the Backup. Format: `projects/*/locations/*/backupPlans/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `manual` | bool | Output only. This flag indicates whether this Backup resource was created manually by a user or via a schedule in the BackupPlan. A value of True means that the Backup was created manually. |
| `contains_volume_data` | bool | Output only. Whether or not the Backup contains volume data. Controlled by the parent BackupPlan's include_volume_data value. |
| `etag` | String | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a backup from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform backup updates in order to avoid race conditions: An `etag` is returned in the response to `GetBackup`, and systems are expected to put that etag in the request to `UpdateBackup` or `DeleteBackup` to ensure that their change will be applied to the same version of the resource. |
| `permissive_mode` | bool | Output only. If false, Backup will fail when Backup for GKE detects Kubernetes configuration that is non-standard or requires additional setup to restore. Inherited from the parent BackupPlan's permissive_mode value. |
| `state` | String | Output only. Current state of the Backup |
| `description` | String | Optional. User specified descriptive string for this Backup. |
| `volume_count` | i64 | Output only. The total number of volume backups contained in the Backup. |
| `selected_applications` | String | Output only. If set, the list of ProtectedApplications whose resources were included in the Backup. |
| `selected_namespace_labels` | String | Output only. If set, the list of labels whose constituent namespaces were included in the Backup. |
| `all_namespaces` | bool | Output only. If True, all namespaces were included in the Backup. |
| `delete_lock_expire_time` | String | Output only. The time at which an existing delete lock will expire for this backup (calculated from create_time + delete_lock_days). |
| `satisfies_pzi` | bool | Output only. [Output Only] Reserved for future use. |
| `uid` | String | Output only. Server generated global unique identifier of [UUID4](https://en.wikipedia.org/wiki/Universally_unique_identifier) |
| `cluster_metadata` | String | Output only. Information about the GKE cluster from which this Backup was created. |
| `contains_secrets` | bool | Output only. Whether or not the Backup contains Kubernetes Secrets. Controlled by the parent BackupPlan's include_secrets value. |
| `resource_count` | i64 | Output only. The total number of Kubernetes resources included in the Backup. |
| `retain_days` | i64 | Optional. The age (in days) after which this Backup will be automatically deleted. Must be an integer value >= 0: - If 0, no automatic deletion will occur for this Backup. - If not 0, this must be >= delete_lock_days and <= 365. Once a Backup is created, this value may only be increased. Defaults to the parent BackupPlan's backup_retain_days value. |
| `satisfies_pzs` | bool | Output only. [Output Only] Reserved for future use. |
| `retain_expire_time` | String | Output only. The time at which this Backup will be automatically deleted (calculated from create_time + retain_days). |
| `troubleshooting_info` | String | Output only. Information about the troubleshooting steps which will provide debugging information to the end users. |
| `complete_time` | String | Output only. Completion time of the Backup |
| `selected_namespaces` | String | Output only. If set, the list of namespaces that were included in the Backup. |
| `labels` | HashMap<String, String> | Optional. A set of custom labels supplied by user. |
| `size_bytes` | String | Output only. The total size of the Backup in bytes = config backup size + sum(volume backup sizes) |
| `create_time` | String | Output only. The timestamp when this Backup resource was created. |
| `delete_lock_days` | i64 | Optional. Minimum age for this Backup (in days). If this field is set to a non-zero value, the Backup will be "locked" against deletion (either manual or automatic deletion) for the number of days provided (measured from the creation time of the Backup). MUST be an integer value between 0-90 (inclusive). Defaults to parent BackupPlan's backup_delete_lock_days setting and may only be increased (either at creation time or in a subsequent update). |
| `pod_count` | i64 | Output only. The total number of Kubernetes Pods contained in the Backup. |
| `config_backup_size_bytes` | String | Output only. The size of the config backup in bytes. |
| `state_reason` | String | Output only. Human-readable description of why the backup is in the current `state`. This field is only meant for human readability and should not be used programmatically as this field is not guaranteed to be consistent. |
| `encryption_key` | String | Output only. The customer managed encryption key that was used to encrypt the Backup's artifacts. Inherited from the parent BackupPlan's encryption_key value. |
| `name` | String | Output only. Identifier. The fully qualified name of the Backup. `projects/*/locations/*/backupPlans/*/backups/*` |
| `update_time` | String | Output only. The timestamp when this Backup resource was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.gkebackup_api.Backup {
    parent = "value"  # Required. The BackupPlan within which to create the Backup. Format: `projects/*/locations/*/backupPlans/*`
}

# Access backup outputs
backup_id = backup.id
backup_manual = backup.manual
backup_contains_volume_data = backup.contains_volume_data
backup_etag = backup.etag
backup_permissive_mode = backup.permissive_mode
backup_state = backup.state
backup_description = backup.description
backup_volume_count = backup.volume_count
backup_selected_applications = backup.selected_applications
backup_selected_namespace_labels = backup.selected_namespace_labels
backup_all_namespaces = backup.all_namespaces
backup_delete_lock_expire_time = backup.delete_lock_expire_time
backup_satisfies_pzi = backup.satisfies_pzi
backup_uid = backup.uid
backup_cluster_metadata = backup.cluster_metadata
backup_contains_secrets = backup.contains_secrets
backup_resource_count = backup.resource_count
backup_retain_days = backup.retain_days
backup_satisfies_pzs = backup.satisfies_pzs
backup_retain_expire_time = backup.retain_expire_time
backup_troubleshooting_info = backup.troubleshooting_info
backup_complete_time = backup.complete_time
backup_selected_namespaces = backup.selected_namespaces
backup_labels = backup.labels
backup_size_bytes = backup.size_bytes
backup_create_time = backup.create_time
backup_delete_lock_days = backup.delete_lock_days
backup_pod_count = backup.pod_count
backup_config_backup_size_bytes = backup.config_backup_size_bytes
backup_state_reason = backup.state_reason
backup_encryption_key = backup.encryption_key
backup_name = backup.name
backup_update_time = backup.update_time
```

---


### Volume_backup

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a volume backup from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform volume backup updates in order to avoid race conditions. |
| `uid` | String | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `disk_size_bytes` | String | Output only. The minimum size of the disk to which this VolumeBackup can be restored. |
| `complete_time` | String | Output only. The timestamp when the associated underlying volume backup operation completed. |
| `format` | String | Output only. The format used for the volume backup. |
| `satisfies_pzs` | bool | Output only. [Output Only] Reserved for future use. |
| `source_pvc` | String | Output only. A reference to the source Kubernetes PVC from which this VolumeBackup was created. |
| `update_time` | String | Output only. The timestamp when this VolumeBackup resource was last updated. |
| `state_message` | String | Output only. A human readable message explaining why the VolumeBackup is in its current state. This field is only meant for human consumption and should not be used programmatically as this field is not guaranteed to be consistent. |
| `state` | String | Output only. The current state of this VolumeBackup. |
| `satisfies_pzi` | bool | Output only. [Output Only] Reserved for future use. |
| `create_time` | String | Output only. The timestamp when this VolumeBackup resource was created. |
| `name` | String | Output only. The full name of the VolumeBackup resource. Format: `projects/*/locations/*/backupPlans/*/backups/*/volumeBackups/*`. |
| `storage_bytes` | String | Output only. The aggregate size of the underlying artifacts associated with this VolumeBackup in the backup storage. This may change over time when multiple backups of the same volume share the same backup storage location. In particular, this is likely to increase in size when the immediately preceding backup of the same volume is deleted. |
| `volume_backup_handle` | String | Output only. A storage system-specific opaque handle to the underlying volume backup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create volume_backup
volume_backup = provider.gkebackup_api.Volume_backup {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access volume_backup outputs
volume_backup_id = volume_backup.id
volume_backup_etag = volume_backup.etag
volume_backup_uid = volume_backup.uid
volume_backup_disk_size_bytes = volume_backup.disk_size_bytes
volume_backup_complete_time = volume_backup.complete_time
volume_backup_format = volume_backup.format
volume_backup_satisfies_pzs = volume_backup.satisfies_pzs
volume_backup_source_pvc = volume_backup.source_pvc
volume_backup_update_time = volume_backup.update_time
volume_backup_state_message = volume_backup.state_message
volume_backup_state = volume_backup.state
volume_backup_satisfies_pzi = volume_backup.satisfies_pzi
volume_backup_create_time = volume_backup.create_time
volume_backup_name = volume_backup.name
volume_backup_storage_bytes = volume_backup.storage_bytes
volume_backup_volume_backup_handle = volume_backup.volume_backup_handle
```

---


### Restore_plan

Creates a new RestorePlan in a given location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a restore from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform restore updates in order to avoid race conditions: An `etag` is returned in the response to `GetRestorePlan`, and systems are expected to put that etag in the request to `UpdateRestorePlan` or `DeleteRestorePlan` to ensure that their change will be applied to the same version of the resource. |
| `backup_plan` | String |  | Required. Immutable. A reference to the BackupPlan from which Backups may be used as the source for Restores created via this RestorePlan. Format: `projects/*/locations/*/backupPlans/*`. |
| `restore_channel` | String |  | Output only. The fully qualified name of the RestoreChannel to be used to create a RestorePlan. This field is set only if the `backup_plan` is in a different project than the RestorePlan. Format: `projects/*/locations/*/restoreChannels/*` |
| `name` | String |  | Output only. Identifier. The full name of the RestorePlan resource. Format: `projects/*/locations/*/restorePlans/*`. |
| `uid` | String |  | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `create_time` | String |  | Output only. The timestamp when this RestorePlan resource was created. |
| `labels` | HashMap<String, String> |  | Optional. A set of custom labels supplied by user. |
| `description` | String |  | Optional. User specified descriptive string for this RestorePlan. |
| `cluster` | String |  | Required. Immutable. The target cluster into which Restores created via this RestorePlan will restore data. NOTE: the cluster's region must be the same as the RestorePlan. Valid formats: - `projects/*/locations/*/clusters/*` - `projects/*/zones/*/clusters/*` |
| `restore_config` | String |  | Required. Configuration of Restores created via this RestorePlan. |
| `state_reason` | String |  | Output only. Human-readable description of why RestorePlan is in the current `state`. This field is only meant for human readability and should not be used programmatically as this field is not guaranteed to be consistent. |
| `update_time` | String |  | Output only. The timestamp when this RestorePlan resource was last updated. |
| `state` | String |  | Output only. State of the RestorePlan. This State field reflects the various stages a RestorePlan can be in during the Create operation. |
| `parent` | String | ✅ | Required. The location within which to create the RestorePlan. Format: `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a restore from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform restore updates in order to avoid race conditions: An `etag` is returned in the response to `GetRestorePlan`, and systems are expected to put that etag in the request to `UpdateRestorePlan` or `DeleteRestorePlan` to ensure that their change will be applied to the same version of the resource. |
| `backup_plan` | String | Required. Immutable. A reference to the BackupPlan from which Backups may be used as the source for Restores created via this RestorePlan. Format: `projects/*/locations/*/backupPlans/*`. |
| `restore_channel` | String | Output only. The fully qualified name of the RestoreChannel to be used to create a RestorePlan. This field is set only if the `backup_plan` is in a different project than the RestorePlan. Format: `projects/*/locations/*/restoreChannels/*` |
| `name` | String | Output only. Identifier. The full name of the RestorePlan resource. Format: `projects/*/locations/*/restorePlans/*`. |
| `uid` | String | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `create_time` | String | Output only. The timestamp when this RestorePlan resource was created. |
| `labels` | HashMap<String, String> | Optional. A set of custom labels supplied by user. |
| `description` | String | Optional. User specified descriptive string for this RestorePlan. |
| `cluster` | String | Required. Immutable. The target cluster into which Restores created via this RestorePlan will restore data. NOTE: the cluster's region must be the same as the RestorePlan. Valid formats: - `projects/*/locations/*/clusters/*` - `projects/*/zones/*/clusters/*` |
| `restore_config` | String | Required. Configuration of Restores created via this RestorePlan. |
| `state_reason` | String | Output only. Human-readable description of why RestorePlan is in the current `state`. This field is only meant for human readability and should not be used programmatically as this field is not guaranteed to be consistent. |
| `update_time` | String | Output only. The timestamp when this RestorePlan resource was last updated. |
| `state` | String | Output only. State of the RestorePlan. This State field reflects the various stages a RestorePlan can be in during the Create operation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create restore_plan
restore_plan = provider.gkebackup_api.Restore_plan {
    parent = "value"  # Required. The location within which to create the RestorePlan. Format: `projects/*/locations/*`
}

# Access restore_plan outputs
restore_plan_id = restore_plan.id
restore_plan_etag = restore_plan.etag
restore_plan_backup_plan = restore_plan.backup_plan
restore_plan_restore_channel = restore_plan.restore_channel
restore_plan_name = restore_plan.name
restore_plan_uid = restore_plan.uid
restore_plan_create_time = restore_plan.create_time
restore_plan_labels = restore_plan.labels
restore_plan_description = restore_plan.description
restore_plan_cluster = restore_plan.cluster
restore_plan_restore_config = restore_plan.restore_config
restore_plan_state_reason = restore_plan.state_reason
restore_plan_update_time = restore_plan.update_time
restore_plan_state = restore_plan.state
```

---


### Restore

Creates a new Restore for the given RestorePlan.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | Optional. Immutable. Filters resources for `Restore`. If not specified, the scope of the restore will remain the same as defined in the `RestorePlan`. If this is specified and no resources are matched by the `inclusion_filters` or everything is excluded by the `exclusion_filters`, nothing will be restored. This filter can only be specified if the value of namespaced_resource_restore_mode is set to `MERGE_SKIP_ON_CONFLICT`, `MERGE_REPLACE_VOLUME_ON_CONFLICT` or `MERGE_REPLACE_ON_CONFLICT`. |
| `create_time` | String |  | Output only. The timestamp when this Restore resource was created. |
| `resources_excluded_count` | i64 |  | Output only. Number of resources excluded during the restore execution. |
| `cluster` | String |  | Output only. The target cluster into which this Restore will restore data. Valid formats: - `projects/*/locations/*/clusters/*` - `projects/*/zones/*/clusters/*` Inherited from parent RestorePlan's cluster value. |
| `update_time` | String |  | Output only. The timestamp when this Restore resource was last updated. |
| `description` | String |  | Optional. User specified descriptive string for this Restore. |
| `state` | String |  | Output only. The current state of the Restore. |
| `backup` | String |  | Required. Immutable. A reference to the Backup used as the source from which this Restore will restore. Note that this Backup must be a sub-resource of the RestorePlan's backup_plan. Format: `projects/*/locations/*/backupPlans/*/backups/*`. |
| `volumes_restored_count` | i64 |  | Output only. Number of volumes restored during the restore execution. |
| `uid` | String |  | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `state_reason` | String |  | Output only. Human-readable description of why the Restore is in its current state. This field is only meant for human readability and should not be used programmatically as this field is not guaranteed to be consistent. |
| `resources_failed_count` | i64 |  | Output only. Number of resources that failed to be restored during the restore execution. |
| `restore_config` | String |  | Output only. Configuration of the Restore. Inherited from parent RestorePlan's restore_config. |
| `labels` | HashMap<String, String> |  | A set of custom labels supplied by user. |
| `resources_restored_count` | i64 |  | Output only. Number of resources restored during the restore execution. |
| `troubleshooting_info` | String |  | Output only. Information about the troubleshooting steps which will provide debugging information to the end users. |
| `etag` | String |  | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a restore from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform restore updates in order to avoid race conditions: An `etag` is returned in the response to `GetRestore`, and systems are expected to put that etag in the request to `UpdateRestore` or `DeleteRestore` to ensure that their change will be applied to the same version of the resource. |
| `name` | String |  | Output only. Identifier. The full name of the Restore resource. Format: `projects/*/locations/*/restorePlans/*/restores/*` |
| `volume_data_restore_policy_overrides` | Vec<String> |  | Optional. Immutable. Overrides the volume data restore policies selected in the Restore Config for override-scoped resources. |
| `complete_time` | String |  | Output only. Timestamp of when the restore operation completed. |
| `parent` | String | ✅ | Required. The RestorePlan within which to create the Restore. Format: `projects/*/locations/*/restorePlans/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `filter` | String | Optional. Immutable. Filters resources for `Restore`. If not specified, the scope of the restore will remain the same as defined in the `RestorePlan`. If this is specified and no resources are matched by the `inclusion_filters` or everything is excluded by the `exclusion_filters`, nothing will be restored. This filter can only be specified if the value of namespaced_resource_restore_mode is set to `MERGE_SKIP_ON_CONFLICT`, `MERGE_REPLACE_VOLUME_ON_CONFLICT` or `MERGE_REPLACE_ON_CONFLICT`. |
| `create_time` | String | Output only. The timestamp when this Restore resource was created. |
| `resources_excluded_count` | i64 | Output only. Number of resources excluded during the restore execution. |
| `cluster` | String | Output only. The target cluster into which this Restore will restore data. Valid formats: - `projects/*/locations/*/clusters/*` - `projects/*/zones/*/clusters/*` Inherited from parent RestorePlan's cluster value. |
| `update_time` | String | Output only. The timestamp when this Restore resource was last updated. |
| `description` | String | Optional. User specified descriptive string for this Restore. |
| `state` | String | Output only. The current state of the Restore. |
| `backup` | String | Required. Immutable. A reference to the Backup used as the source from which this Restore will restore. Note that this Backup must be a sub-resource of the RestorePlan's backup_plan. Format: `projects/*/locations/*/backupPlans/*/backups/*`. |
| `volumes_restored_count` | i64 | Output only. Number of volumes restored during the restore execution. |
| `uid` | String | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `state_reason` | String | Output only. Human-readable description of why the Restore is in its current state. This field is only meant for human readability and should not be used programmatically as this field is not guaranteed to be consistent. |
| `resources_failed_count` | i64 | Output only. Number of resources that failed to be restored during the restore execution. |
| `restore_config` | String | Output only. Configuration of the Restore. Inherited from parent RestorePlan's restore_config. |
| `labels` | HashMap<String, String> | A set of custom labels supplied by user. |
| `resources_restored_count` | i64 | Output only. Number of resources restored during the restore execution. |
| `troubleshooting_info` | String | Output only. Information about the troubleshooting steps which will provide debugging information to the end users. |
| `etag` | String | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a restore from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform restore updates in order to avoid race conditions: An `etag` is returned in the response to `GetRestore`, and systems are expected to put that etag in the request to `UpdateRestore` or `DeleteRestore` to ensure that their change will be applied to the same version of the resource. |
| `name` | String | Output only. Identifier. The full name of the Restore resource. Format: `projects/*/locations/*/restorePlans/*/restores/*` |
| `volume_data_restore_policy_overrides` | Vec<String> | Optional. Immutable. Overrides the volume data restore policies selected in the Restore Config for override-scoped resources. |
| `complete_time` | String | Output only. Timestamp of when the restore operation completed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create restore
restore = provider.gkebackup_api.Restore {
    parent = "value"  # Required. The RestorePlan within which to create the Restore. Format: `projects/*/locations/*/restorePlans/*`
}

# Access restore outputs
restore_id = restore.id
restore_filter = restore.filter
restore_create_time = restore.create_time
restore_resources_excluded_count = restore.resources_excluded_count
restore_cluster = restore.cluster
restore_update_time = restore.update_time
restore_description = restore.description
restore_state = restore.state
restore_backup = restore.backup
restore_volumes_restored_count = restore.volumes_restored_count
restore_uid = restore.uid
restore_state_reason = restore.state_reason
restore_resources_failed_count = restore.resources_failed_count
restore_restore_config = restore.restore_config
restore_labels = restore.labels
restore_resources_restored_count = restore.resources_restored_count
restore_troubleshooting_info = restore.troubleshooting_info
restore_etag = restore.etag
restore_name = restore.name
restore_volume_data_restore_policy_overrides = restore.volume_data_restore_policy_overrides
restore_complete_time = restore.complete_time
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_display_name = location.display_name
location_location_id = location.location_id
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
```

---


### Volume_restore

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Full name of the VolumeRestore resource. Format: `projects/*/locations/*/restorePlans/*/restores/*/volumeRestores/*` |
| `volume_handle` | String | Output only. A storage system-specific opaque handler to the underlying volume created for the target PVC from the volume backup. |
| `target_pvc` | String | Output only. The reference to the target Kubernetes PVC to be restored. |
| `state` | String | Output only. The current state of this VolumeRestore. |
| `uid` | String | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `update_time` | String | Output only. The timestamp when this VolumeRestore resource was last updated. |
| `volume_type` | String | Output only. The type of volume provisioned |
| `volume_backup` | String | Output only. The full name of the VolumeBackup from which the volume will be restored. Format: `projects/*/locations/*/backupPlans/*/backups/*/volumeBackups/*`. |
| `state_message` | String | Output only. A human readable message explaining why the VolumeRestore is in its current state. |
| `etag` | String | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a volume restore from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform volume restore updates in order to avoid race conditions. |
| `complete_time` | String | Output only. The timestamp when the associated underlying volume restoration completed. |
| `create_time` | String | Output only. The timestamp when this VolumeRestore resource was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create volume_restore
volume_restore = provider.gkebackup_api.Volume_restore {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access volume_restore outputs
volume_restore_id = volume_restore.id
volume_restore_name = volume_restore.name
volume_restore_volume_handle = volume_restore.volume_handle
volume_restore_target_pvc = volume_restore.target_pvc
volume_restore_state = volume_restore.state
volume_restore_uid = volume_restore.uid
volume_restore_update_time = volume_restore.update_time
volume_restore_volume_type = volume_restore.volume_type
volume_restore_volume_backup = volume_restore.volume_backup
volume_restore_state_message = volume_restore.state_message
volume_restore_etag = volume_restore.etag
volume_restore_complete_time = volume_restore.complete_time
volume_restore_create_time = volume_restore.create_time
```

---


### Backup_channel

Creates a new BackupChannel in a given location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a BackupChannel from overwriting each other. It is strongly suggested that systems make use of the 'etag' in the read-modify-write cycle to perform BackupChannel updates in order to avoid race conditions: An `etag` is returned in the response to `GetBackupChannel`, and systems are expected to put that etag in the request to `UpdateBackupChannel` or `DeleteBackupChannel` to ensure that their change will be applied to the same version of the resource. |
| `destination_project_id` | String |  | Output only. The project_id where Backups are allowed to be stored. Example Project ID: "my-project-id". This will be an OUTPUT_ONLY field to return the project_id of the destination project. |
| `update_time` | String |  | Output only. The timestamp when this BackupChannel resource was last updated. |
| `create_time` | String |  | Output only. The timestamp when this BackupChannel resource was created. |
| `destination_project` | String |  | Required. Immutable. The project where Backups are allowed to be stored. The format is `projects/{projectId}` or `projects/{projectNumber}`. |
| `uid` | String |  | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `description` | String |  | Optional. User specified descriptive string for this BackupChannel. |
| `name` | String |  | Identifier. The fully qualified name of the BackupChannel. `projects/*/locations/*/backupChannels/*` |
| `labels` | HashMap<String, String> |  | Optional. A set of custom labels supplied by user. |
| `parent` | String | ✅ | Required. The location within which to create the BackupChannel. Format: `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a BackupChannel from overwriting each other. It is strongly suggested that systems make use of the 'etag' in the read-modify-write cycle to perform BackupChannel updates in order to avoid race conditions: An `etag` is returned in the response to `GetBackupChannel`, and systems are expected to put that etag in the request to `UpdateBackupChannel` or `DeleteBackupChannel` to ensure that their change will be applied to the same version of the resource. |
| `destination_project_id` | String | Output only. The project_id where Backups are allowed to be stored. Example Project ID: "my-project-id". This will be an OUTPUT_ONLY field to return the project_id of the destination project. |
| `update_time` | String | Output only. The timestamp when this BackupChannel resource was last updated. |
| `create_time` | String | Output only. The timestamp when this BackupChannel resource was created. |
| `destination_project` | String | Required. Immutable. The project where Backups are allowed to be stored. The format is `projects/{projectId}` or `projects/{projectNumber}`. |
| `uid` | String | Output only. Server generated global unique identifier of [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format. |
| `description` | String | Optional. User specified descriptive string for this BackupChannel. |
| `name` | String | Identifier. The fully qualified name of the BackupChannel. `projects/*/locations/*/backupChannels/*` |
| `labels` | HashMap<String, String> | Optional. A set of custom labels supplied by user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup_channel
backup_channel = provider.gkebackup_api.Backup_channel {
    parent = "value"  # Required. The location within which to create the BackupChannel. Format: `projects/*/locations/*`
}

# Access backup_channel outputs
backup_channel_id = backup_channel.id
backup_channel_etag = backup_channel.etag
backup_channel_destination_project_id = backup_channel.destination_project_id
backup_channel_update_time = backup_channel.update_time
backup_channel_create_time = backup_channel.create_time
backup_channel_destination_project = backup_channel.destination_project
backup_channel_uid = backup_channel.uid
backup_channel_description = backup_channel.description
backup_channel_name = backup_channel.name
backup_channel_labels = backup_channel.labels
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple operation resources
operation_0 = provider.gkebackup_api.Operation {
    name = "value-0"
}
operation_1 = provider.gkebackup_api.Operation {
    name = "value-1"
}
operation_2 = provider.gkebackup_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.gkebackup_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Gkebackup_api Documentation](https://cloud.google.com/gkebackup_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
