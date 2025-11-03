# Indexing_api Service



**Resources**: 1

---

## Overview

The indexing_api service provides access to 1 resource type:

- [Url_notification](#url_notification) [CR]

---

## Resources


### Url_notification

Notifies that a URL has been updated or deleted.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | The URL life cycle event that Google is being notified about. |
| `notify_time` | String |  | Creation timestamp for this notification. Users should _not_ specify it, the field is ignored at the request time. |
| `url` | String |  | The object of this notification. The URL must be owned by the publisher of this notification and, in case of `URL_UPDATED` notifications, it _must_ be crawlable by Google. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `latest_update` | String | Latest notification received with type `URL_UPDATED`. |
| `latest_remove` | String | Latest notification received with type `URL_REMOVED`. |
| `url` | String | URL to which this metadata refers. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create url_notification
url_notification = provider.indexing_api.Url_notification {
}

# Access url_notification outputs
url_notification_id = url_notification.id
url_notification_latest_update = url_notification.latest_update
url_notification_latest_remove = url_notification.latest_remove
url_notification_url = url_notification.url
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple url_notification resources
url_notification_0 = provider.indexing_api.Url_notification {
}
url_notification_1 = provider.indexing_api.Url_notification {
}
url_notification_2 = provider.indexing_api.Url_notification {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    url_notification = provider.indexing_api.Url_notification {
    }
```

---

## Related Documentation

- [GCP Indexing_api Documentation](https://cloud.google.com/indexing_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
