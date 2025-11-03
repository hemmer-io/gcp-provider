# Alertcenter_api Service



**Resources**: 3

---

## Overview

The alertcenter_api service provides access to 3 resource types:

- [Feedback](#feedback) [CR]
- [Alertcenter](#alertcenter) [RU]
- [Alert](#alert) [CRD]

---

## Resources


### Feedback

Creates new feedback for an alert. Attempting to create a feedback for a non-existent alert returns `NOT_FOUND` error. Attempting to create a feedback for an alert that is marked for deletion returns `FAILED_PRECONDITION' error.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `feedback_id` | String |  | Output only. The unique identifier for the feedback. |
| `email` | String |  | Output only. The email of the user that provided the feedback. |
| `customer_id` | String |  | Output only. The unique identifier of the Google Workspace account of the customer. |
| `create_time` | String |  | Output only. The time this feedback was created. |
| `alert_id` | String |  | Output only. The alert identifier. |
| `type` | String |  | Required. The type of the feedback. |
| `alert_id` | String | ✅ | Required. The identifier of the alert this feedback belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `feedback` | Vec<String> | The list of alert feedback. Feedback entries for each alert are ordered by creation time descending. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feedback
feedback = provider.alertcenter_api.Feedback {
    alert_id = "value"  # Required. The identifier of the alert this feedback belongs to.
}

# Access feedback outputs
feedback_id = feedback.id
feedback_feedback = feedback.feedback
```

---


### Alertcenter

Returns customer-level settings.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notifications` | Vec<String> |  | The list of notifications. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notifications` | Vec<String> | The list of notifications. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access alertcenter outputs
alertcenter_id = alertcenter.id
alertcenter_notifications = alertcenter.notifications
```

---


### Alert

Restores, or "undeletes", an alert that was marked for deletion within the past 30 days. Attempting to undelete an alert which was marked for deletion over 30 days ago (which has been removed from the Alert Center database) or a nonexistent alert returns a `NOT_FOUND` error. Attempting to undelete an alert which has not been marked for deletion has no effect.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer_id` | String |  | Optional. The unique identifier of the Google Workspace account of the customer the alert is associated with. The `customer_id` must have the initial "C" stripped (for example, `046psxkn`). Inferred from the caller identity if not provided. [Find your customer ID](https://support.google.com/cloudidentity/answer/10070793). |
| `alert_id` | String | ✅ | Required. The identifier of the alert to undelete. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Required. The type of the alert. This is output only after alert is created. For a list of available alert types see [Google Workspace Alert types](https://developers.google.com/workspace/admin/alertcenter/reference/alert-types). |
| `etag` | String | Optional. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of an alert from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform alert updates in order to avoid race conditions: An `etag` is returned in the response which contains alerts, and systems are expected to put that etag in the request to update alert to ensure that their change will be applied to the same version of the alert. If no `etag` is provided in the call to update alert, then the existing alert is overwritten blindly. |
| `end_time` | String | Optional. The time the event that caused this alert ceased being active. If provided, the end time must not be earlier than the start time. If not provided, it indicates an ongoing alert. |
| `security_investigation_tool_link` | String | Output only. An optional [Security Investigation Tool](https://support.google.com/a/answer/7575955) query for this alert. |
| `source` | String | Required. A unique identifier for the system that reported the alert. This is output only after alert is created. Supported sources are any of the following: * Google Operations * Mobile device management * Gmail phishing * Data Loss Prevention * Domain wide takeout * State sponsored attack * Google identity * Apps outage |
| `update_time` | String | Output only. The time this alert was last updated. |
| `alert_id` | String | Output only. The unique identifier for the alert. |
| `customer_id` | String | Output only. The unique identifier of the Google Workspace account of the customer. |
| `deleted` | bool | Output only. `True` if this alert is marked for deletion. |
| `create_time` | String | Output only. The time this alert was created. |
| `data` | HashMap<String, String> | Optional. The data associated with this alert, for example google.apps.alertcenter.type.DeviceCompromised. |
| `metadata` | String | Output only. The metadata associated with this alert. |
| `start_time` | String | Required. The time the event that caused this alert was started or detected. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create alert
alert = provider.alertcenter_api.Alert {
    alert_id = "value"  # Required. The identifier of the alert to undelete.
}

# Access alert outputs
alert_id = alert.id
alert_type = alert.type
alert_etag = alert.etag
alert_end_time = alert.end_time
alert_security_investigation_tool_link = alert.security_investigation_tool_link
alert_source = alert.source
alert_update_time = alert.update_time
alert_alert_id = alert.alert_id
alert_customer_id = alert.customer_id
alert_deleted = alert.deleted
alert_create_time = alert.create_time
alert_data = alert.data
alert_metadata = alert.metadata
alert_start_time = alert.start_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple feedback resources
feedback_0 = provider.alertcenter_api.Feedback {
    alert_id = "value-0"
}
feedback_1 = provider.alertcenter_api.Feedback {
    alert_id = "value-1"
}
feedback_2 = provider.alertcenter_api.Feedback {
    alert_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    feedback = provider.alertcenter_api.Feedback {
        alert_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Alertcenter_api Documentation](https://cloud.google.com/alertcenter_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
