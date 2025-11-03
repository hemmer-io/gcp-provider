# Advisorynotifications_api Service



**Resources**: 2

---

## Overview

The advisorynotifications_api service provides access to 2 resource types:

- [Location](#location) [RU]
- [Notification](#notification) [R]

---

## Resources


### Location

Get notification settings.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_settings` | HashMap<String, String> |  | Required. Map of each notification type and its settings to get/set all settings at once. The server will validate the value for each notification type. |
| `name` | String |  | Identifier. The resource name of the settings to retrieve. Format: organizations/{organization}/locations/{location}/settings or projects/{projects}/locations/{location}/settings. |
| `etag` | String |  | Required. Fingerprint for optimistic concurrency returned in Get requests. Must be provided for Update requests. If the value provided does not match the value known to the server, ABORTED will be thrown, and the client should retry the read-modify-write cycle. |
| `name` | String | ✅ | Identifier. The resource name of the settings to retrieve. Format: organizations/{organization}/locations/{location}/settings or projects/{projects}/locations/{location}/settings. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_settings` | HashMap<String, String> | Required. Map of each notification type and its settings to get/set all settings at once. The server will validate the value for each notification type. |
| `name` | String | Identifier. The resource name of the settings to retrieve. Format: organizations/{organization}/locations/{location}/settings or projects/{projects}/locations/{location}/settings. |
| `etag` | String | Required. Fingerprint for optimistic concurrency returned in Get requests. Must be provided for Update requests. If the value provided does not match the value known to the server, ABORTED will be thrown, and the client should retry the read-modify-write cycle. |


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
location_notification_settings = location.notification_settings
location_name = location.name
location_etag = location.etag
```

---


### Notification

Gets a notification.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the notification. Format: organizations/{organization}/locations/{location}/notifications/{notification} or projects/{project}/locations/{location}/notifications/{notification}. |
| `create_time` | String | Output only. Time the notification was created. |
| `notification_type` | String | Type of notification |
| `messages` | Vec<String> | A list of messages in the notification. |
| `subject` | String | The subject line of the notification. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access notification outputs
notification_id = notification.id
notification_name = notification.name
notification_create_time = notification.create_time
notification_notification_type = notification.notification_type
notification_messages = notification.messages
notification_subject = notification.subject
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple location resources
location_0 = provider.advisorynotifications_api.Location {
    name = "value-0"
}
location_1 = provider.advisorynotifications_api.Location {
    name = "value-1"
}
location_2 = provider.advisorynotifications_api.Location {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.advisorynotifications_api.Location {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Advisorynotifications_api Documentation](https://cloud.google.com/advisorynotifications_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
