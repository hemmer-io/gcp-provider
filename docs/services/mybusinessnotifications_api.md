# Mybusinessnotifications_api Service



**Resources**: 1

---

## Overview

The mybusinessnotifications_api service provides access to 1 resource type:

- [Account](#account) [RU]

---

## Resources


### Account

Returns the pubsub notification settings for the account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_types` | Vec<String> |  | The types of notifications that will be sent to the Pub/Sub topic. To stop receiving notifications entirely, use NotificationSettings.UpdateNotificationSetting with an empty notification_types or set the pubsub_topic to an empty string. |
| `pubsub_topic` | String |  | Optional. The Google Pub/Sub topic that will receive notifications when locations managed by this account are updated. If unset, no notifications will be posted. The account mybusiness-api-pubsub@system.gserviceaccount.com must have at least Publish permissions on the Pub/Sub topic. |
| `name` | String |  | Required. The resource name this setting is for. This is of the form `accounts/{account_id}/notificationSetting`. |
| `name` | String | ✅ | Required. The resource name this setting is for. This is of the form `accounts/{account_id}/notificationSetting`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_types` | Vec<String> | The types of notifications that will be sent to the Pub/Sub topic. To stop receiving notifications entirely, use NotificationSettings.UpdateNotificationSetting with an empty notification_types or set the pubsub_topic to an empty string. |
| `pubsub_topic` | String | Optional. The Google Pub/Sub topic that will receive notifications when locations managed by this account are updated. If unset, no notifications will be posted. The account mybusiness-api-pubsub@system.gserviceaccount.com must have at least Publish permissions on the Pub/Sub topic. |
| `name` | String | Required. The resource name this setting is for. This is of the form `accounts/{account_id}/notificationSetting`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_notification_types = account.notification_types
account_pubsub_topic = account.pubsub_topic
account_name = account.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple account resources
account_0 = provider.mybusinessnotifications_api.Account {
    name = "value-0"
}
account_1 = provider.mybusinessnotifications_api.Account {
    name = "value-1"
}
account_2 = provider.mybusinessnotifications_api.Account {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account = provider.mybusinessnotifications_api.Account {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Mybusinessnotifications_api Documentation](https://cloud.google.com/mybusinessnotifications_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
