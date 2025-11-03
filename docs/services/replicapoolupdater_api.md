# Replicapoolupdater_api Service



**Resources**: 2

---

## Overview

The replicapoolupdater_api service provides access to 2 resource types:

- [Zone_operation](#zone_operation) [R]
- [Rolling_update](#rolling_update) [CR]

---

## Resources


### Zone_operation

Retrieves the specified zone-specific operation resource.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `region` | String | [Output Only] URL of the region where the operation resides. |
| `insert_time` | String | [Output Only] The time that this operation was requested. This is in RFC 3339 format. |
| `name` | String | [Output Only] Name of the resource. |
| `warnings` | Vec<String> |  |
| `status_message` | String | [Output Only] An optional textual description of the current status of the operation. |
| `id` | String | [Output Only] Unique identifier for the resource; defined by the server. |
| `end_time` | String |  |
| `start_time` | String | [Output Only] The time that this operation was started by the server. This is in RFC 3339 format. |
| `target_id` | String | [Output Only] Unique target id which identifies a particular incarnation of the target. |
| `self_link` | String | [Output Only] The fully qualified URL for the resource. |
| `status` | String | [Output Only] Status of the operation. Can be one of the following: "PENDING", "RUNNING", or "DONE". |
| `creation_timestamp` | String | [Output Only] Creation timestamp in RFC3339 text format. |
| `http_error_status_code` | i64 |  |
| `http_error_message` | String |  |
| `user` | String |  |
| `client_operation_id` | String |  |
| `progress` | i64 |  |
| `zone` | String | [Output Only] URL of the zone where the operation resides. |
| `kind` | String | [Output Only] Type of the resource. Always replicapoolupdater#operation for Operation resources. |
| `operation_type` | String |  |
| `error` | String | [Output Only] If errors occurred during processing of this operation, this field will be populated. |
| `target_link` | String | [Output Only] URL of the resource the operation is mutating. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access zone_operation outputs
zone_operation_id = zone_operation.id
zone_operation_region = zone_operation.region
zone_operation_insert_time = zone_operation.insert_time
zone_operation_name = zone_operation.name
zone_operation_warnings = zone_operation.warnings
zone_operation_status_message = zone_operation.status_message
zone_operation_id = zone_operation.id
zone_operation_end_time = zone_operation.end_time
zone_operation_start_time = zone_operation.start_time
zone_operation_target_id = zone_operation.target_id
zone_operation_self_link = zone_operation.self_link
zone_operation_status = zone_operation.status
zone_operation_creation_timestamp = zone_operation.creation_timestamp
zone_operation_http_error_status_code = zone_operation.http_error_status_code
zone_operation_http_error_message = zone_operation.http_error_message
zone_operation_user = zone_operation.user
zone_operation_client_operation_id = zone_operation.client_operation_id
zone_operation_progress = zone_operation.progress
zone_operation_zone = zone_operation.zone
zone_operation_kind = zone_operation.kind
zone_operation_operation_type = zone_operation.operation_type
zone_operation_error = zone_operation.error
zone_operation_target_link = zone_operation.target_link
```

---


### Rolling_update

Inserts and starts a new update.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action_type` | String |  | Specifies the action to take for each instance within the instance group. This can be RECREATE which will recreate each instance and is only available for managed instance groups. It can also be REBOOT which performs a soft reboot for each instance and is only available for regular (non-managed) instance groups. |
| `id` | String |  | [Output Only] Unique identifier for the resource; defined by the server. |
| `progress` | i64 |  | [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess at when the update will be complete. This number should be monotonically increasing as the update progresses. |
| `self_link` | String |  | [Output Only] The fully qualified URL for the resource. |
| `old_instance_template` | String |  | Fully-qualified URL of the instance template encountered while starting the update. |
| `status` | String |  | [Output Only] Status of the update. Possible values are:  
- "ROLLING_FORWARD": The update is going forward. 
- "ROLLING_BACK": The update is being rolled back. 
- "PAUSED": The update is temporarily paused (inactive). 
- "ROLLED_OUT": The update is finished, all instances have been updated successfully. 
- "ROLLED_BACK": The update is finished, all instances have been reverted to the previous template. 
- "CANCELLED": The update is paused and no longer can be resumed, undefined how many instances are running in which template. |
| `instance_group` | String |  | Fully-qualified URL of an instance group being updated. Exactly one of instanceGroupManager and instanceGroup must be set. |
| `status_message` | String |  | [Output Only] An optional textual description of the current status of the update. |
| `description` | String |  | An optional textual description of the resource; provided by the client when the resource is created. |
| `policy` | String |  | Parameters of the update process. |
| `error` | String |  | [Output Only] Errors that occurred during the rolling update. |
| `instance_template` | String |  | Fully-qualified URL of an instance template to apply. |
| `user` | String |  | [Output Only] User who requested the update, for example: user@example.com. |
| `creation_timestamp` | String |  | [Output Only] Creation timestamp in RFC3339 text format. |
| `kind` | String |  | [Output Only] Type of the resource. |
| `instance_group_manager` | String |  | Fully-qualified URL of an instance group manager being updated. Exactly one of instanceGroupManager and instanceGroup must be set. |
| `project` | String | ✅ | The Google Developers Console project name. |
| `zone` | String | ✅ | The name of the zone in which the update's target resides. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action_type` | String | Specifies the action to take for each instance within the instance group. This can be RECREATE which will recreate each instance and is only available for managed instance groups. It can also be REBOOT which performs a soft reboot for each instance and is only available for regular (non-managed) instance groups. |
| `id` | String | [Output Only] Unique identifier for the resource; defined by the server. |
| `progress` | i64 | [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess at when the update will be complete. This number should be monotonically increasing as the update progresses. |
| `self_link` | String | [Output Only] The fully qualified URL for the resource. |
| `old_instance_template` | String | Fully-qualified URL of the instance template encountered while starting the update. |
| `status` | String | [Output Only] Status of the update. Possible values are:  
- "ROLLING_FORWARD": The update is going forward. 
- "ROLLING_BACK": The update is being rolled back. 
- "PAUSED": The update is temporarily paused (inactive). 
- "ROLLED_OUT": The update is finished, all instances have been updated successfully. 
- "ROLLED_BACK": The update is finished, all instances have been reverted to the previous template. 
- "CANCELLED": The update is paused and no longer can be resumed, undefined how many instances are running in which template. |
| `instance_group` | String | Fully-qualified URL of an instance group being updated. Exactly one of instanceGroupManager and instanceGroup must be set. |
| `status_message` | String | [Output Only] An optional textual description of the current status of the update. |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |
| `policy` | String | Parameters of the update process. |
| `error` | String | [Output Only] Errors that occurred during the rolling update. |
| `instance_template` | String | Fully-qualified URL of an instance template to apply. |
| `user` | String | [Output Only] User who requested the update, for example: user@example.com. |
| `creation_timestamp` | String | [Output Only] Creation timestamp in RFC3339 text format. |
| `kind` | String | [Output Only] Type of the resource. |
| `instance_group_manager` | String | Fully-qualified URL of an instance group manager being updated. Exactly one of instanceGroupManager and instanceGroup must be set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rolling_update
rolling_update = provider.replicapoolupdater_api.Rolling_update {
    project = "value"  # The Google Developers Console project name.
    zone = "value"  # The name of the zone in which the update's target resides.
}

# Access rolling_update outputs
rolling_update_id = rolling_update.id
rolling_update_action_type = rolling_update.action_type
rolling_update_id = rolling_update.id
rolling_update_progress = rolling_update.progress
rolling_update_self_link = rolling_update.self_link
rolling_update_old_instance_template = rolling_update.old_instance_template
rolling_update_status = rolling_update.status
rolling_update_instance_group = rolling_update.instance_group
rolling_update_status_message = rolling_update.status_message
rolling_update_description = rolling_update.description
rolling_update_policy = rolling_update.policy
rolling_update_error = rolling_update.error
rolling_update_instance_template = rolling_update.instance_template
rolling_update_user = rolling_update.user
rolling_update_creation_timestamp = rolling_update.creation_timestamp
rolling_update_kind = rolling_update.kind
rolling_update_instance_group_manager = rolling_update.instance_group_manager
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple zone_operation resources
zone_operation_0 = provider.replicapoolupdater_api.Zone_operation {
}
zone_operation_1 = provider.replicapoolupdater_api.Zone_operation {
}
zone_operation_2 = provider.replicapoolupdater_api.Zone_operation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    zone_operation = provider.replicapoolupdater_api.Zone_operation {
    }
```

---

## Related Documentation

- [GCP Replicapoolupdater_api Documentation](https://cloud.google.com/replicapoolupdater_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
