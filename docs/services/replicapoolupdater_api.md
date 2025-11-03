# Replicapoolupdater_api Service



**Resources**: 2

---

## Overview

The replicapoolupdater_api service provides access to 2 resource types:

- [Rolling_update](#rolling_update) [CR]
- [Zone_operation](#zone_operation) [R]

---

## Resources


### Rolling_update

Inserts and starts a new update.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `old_instance_template` | String |  | Fully-qualified URL of the instance template encountered while starting the update. |
| `creation_timestamp` | String |  | [Output Only] Creation timestamp in RFC3339 text format. |
| `policy` | String |  | Parameters of the update process. |
| `status_message` | String |  | [Output Only] An optional textual description of the current status of the update. |
| `self_link` | String |  | [Output Only] The fully qualified URL for the resource. |
| `status` | String |  | [Output Only] Status of the update. Possible values are:  
- "ROLLING_FORWARD": The update is going forward. 
- "ROLLING_BACK": The update is being rolled back. 
- "PAUSED": The update is temporarily paused (inactive). 
- "ROLLED_OUT": The update is finished, all instances have been updated successfully. 
- "ROLLED_BACK": The update is finished, all instances have been reverted to the previous template. 
- "CANCELLED": The update is paused and no longer can be resumed, undefined how many instances are running in which template. |
| `id` | String |  | [Output Only] Unique identifier for the resource; defined by the server. |
| `instance_template` | String |  | Fully-qualified URL of an instance template to apply. |
| `progress` | i64 |  | [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess at when the update will be complete. This number should be monotonically increasing as the update progresses. |
| `action_type` | String |  | Specifies the action to take for each instance within the instance group. This can be RECREATE which will recreate each instance and is only available for managed instance groups. It can also be REBOOT which performs a soft reboot for each instance and is only available for regular (non-managed) instance groups. |
| `error` | String |  | [Output Only] Errors that occurred during the rolling update. |
| `instance_group_manager` | String |  | Fully-qualified URL of an instance group manager being updated. Exactly one of instanceGroupManager and instanceGroup must be set. |
| `instance_group` | String |  | Fully-qualified URL of an instance group being updated. Exactly one of instanceGroupManager and instanceGroup must be set. |
| `description` | String |  | An optional textual description of the resource; provided by the client when the resource is created. |
| `kind` | String |  | [Output Only] Type of the resource. |
| `user` | String |  | [Output Only] User who requested the update, for example: user@example.com. |
| `zone` | String | ✅ | The name of the zone in which the update's target resides. |
| `project` | String | ✅ | The Google Developers Console project name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `old_instance_template` | String | Fully-qualified URL of the instance template encountered while starting the update. |
| `creation_timestamp` | String | [Output Only] Creation timestamp in RFC3339 text format. |
| `policy` | String | Parameters of the update process. |
| `status_message` | String | [Output Only] An optional textual description of the current status of the update. |
| `self_link` | String | [Output Only] The fully qualified URL for the resource. |
| `status` | String | [Output Only] Status of the update. Possible values are:  
- "ROLLING_FORWARD": The update is going forward. 
- "ROLLING_BACK": The update is being rolled back. 
- "PAUSED": The update is temporarily paused (inactive). 
- "ROLLED_OUT": The update is finished, all instances have been updated successfully. 
- "ROLLED_BACK": The update is finished, all instances have been reverted to the previous template. 
- "CANCELLED": The update is paused and no longer can be resumed, undefined how many instances are running in which template. |
| `id` | String | [Output Only] Unique identifier for the resource; defined by the server. |
| `instance_template` | String | Fully-qualified URL of an instance template to apply. |
| `progress` | i64 | [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess at when the update will be complete. This number should be monotonically increasing as the update progresses. |
| `action_type` | String | Specifies the action to take for each instance within the instance group. This can be RECREATE which will recreate each instance and is only available for managed instance groups. It can also be REBOOT which performs a soft reboot for each instance and is only available for regular (non-managed) instance groups. |
| `error` | String | [Output Only] Errors that occurred during the rolling update. |
| `instance_group_manager` | String | Fully-qualified URL of an instance group manager being updated. Exactly one of instanceGroupManager and instanceGroup must be set. |
| `instance_group` | String | Fully-qualified URL of an instance group being updated. Exactly one of instanceGroupManager and instanceGroup must be set. |
| `description` | String | An optional textual description of the resource; provided by the client when the resource is created. |
| `kind` | String | [Output Only] Type of the resource. |
| `user` | String | [Output Only] User who requested the update, for example: user@example.com. |


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
    zone = "value"  # The name of the zone in which the update's target resides.
    project = "value"  # The Google Developers Console project name.
}

# Access rolling_update outputs
rolling_update_id = rolling_update.id
rolling_update_old_instance_template = rolling_update.old_instance_template
rolling_update_creation_timestamp = rolling_update.creation_timestamp
rolling_update_policy = rolling_update.policy
rolling_update_status_message = rolling_update.status_message
rolling_update_self_link = rolling_update.self_link
rolling_update_status = rolling_update.status
rolling_update_id = rolling_update.id
rolling_update_instance_template = rolling_update.instance_template
rolling_update_progress = rolling_update.progress
rolling_update_action_type = rolling_update.action_type
rolling_update_error = rolling_update.error
rolling_update_instance_group_manager = rolling_update.instance_group_manager
rolling_update_instance_group = rolling_update.instance_group
rolling_update_description = rolling_update.description
rolling_update_kind = rolling_update.kind
rolling_update_user = rolling_update.user
```

---


### Zone_operation

Retrieves the specified zone-specific operation resource.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_link` | String | [Output Only] URL of the resource the operation is mutating. |
| `status_message` | String | [Output Only] An optional textual description of the current status of the operation. |
| `region` | String | [Output Only] URL of the region where the operation resides. |
| `status` | String | [Output Only] Status of the operation. Can be one of the following: "PENDING", "RUNNING", or "DONE". |
| `creation_timestamp` | String | [Output Only] Creation timestamp in RFC3339 text format. |
| `warnings` | Vec<String> |  |
| `zone` | String | [Output Only] URL of the zone where the operation resides. |
| `name` | String | [Output Only] Name of the resource. |
| `operation_type` | String |  |
| `insert_time` | String | [Output Only] The time that this operation was requested. This is in RFC 3339 format. |
| `client_operation_id` | String |  |
| `user` | String |  |
| `start_time` | String | [Output Only] The time that this operation was started by the server. This is in RFC 3339 format. |
| `kind` | String | [Output Only] Type of the resource. Always replicapoolupdater#operation for Operation resources. |
| `end_time` | String |  |
| `http_error_message` | String |  |
| `progress` | i64 |  |
| `http_error_status_code` | i64 |  |
| `self_link` | String | [Output Only] The fully qualified URL for the resource. |
| `target_id` | String | [Output Only] Unique target id which identifies a particular incarnation of the target. |
| `error` | String | [Output Only] If errors occurred during processing of this operation, this field will be populated. |
| `id` | String | [Output Only] Unique identifier for the resource; defined by the server. |


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
zone_operation_target_link = zone_operation.target_link
zone_operation_status_message = zone_operation.status_message
zone_operation_region = zone_operation.region
zone_operation_status = zone_operation.status
zone_operation_creation_timestamp = zone_operation.creation_timestamp
zone_operation_warnings = zone_operation.warnings
zone_operation_zone = zone_operation.zone
zone_operation_name = zone_operation.name
zone_operation_operation_type = zone_operation.operation_type
zone_operation_insert_time = zone_operation.insert_time
zone_operation_client_operation_id = zone_operation.client_operation_id
zone_operation_user = zone_operation.user
zone_operation_start_time = zone_operation.start_time
zone_operation_kind = zone_operation.kind
zone_operation_end_time = zone_operation.end_time
zone_operation_http_error_message = zone_operation.http_error_message
zone_operation_progress = zone_operation.progress
zone_operation_http_error_status_code = zone_operation.http_error_status_code
zone_operation_self_link = zone_operation.self_link
zone_operation_target_id = zone_operation.target_id
zone_operation_error = zone_operation.error
zone_operation_id = zone_operation.id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple rolling_update resources
rolling_update_0 = provider.replicapoolupdater_api.Rolling_update {
    zone = "value-0"
    project = "value-0"
}
rolling_update_1 = provider.replicapoolupdater_api.Rolling_update {
    zone = "value-1"
    project = "value-1"
}
rolling_update_2 = provider.replicapoolupdater_api.Rolling_update {
    zone = "value-2"
    project = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    rolling_update = provider.replicapoolupdater_api.Rolling_update {
        zone = "production-value"
        project = "production-value"
    }
```

---

## Related Documentation

- [GCP Replicapoolupdater_api Documentation](https://cloud.google.com/replicapoolupdater_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
