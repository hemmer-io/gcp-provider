# Fcmdata_api Service



**Resources**: 1

---

## Overview

The fcmdata_api service provides access to 1 resource type:

- [Delivery_data](#delivery_data) [R]

---

## Resources


### Delivery_data

List aggregate delivery data for the given Android application.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `android_delivery_data` | Vec<String> | The delivery data for the provided app. There will be one entry per combination of app, date, and analytics label. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access delivery_data outputs
delivery_data_id = delivery_data.id
delivery_data_next_page_token = delivery_data.next_page_token
delivery_data_android_delivery_data = delivery_data.android_delivery_data
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple delivery_data resources
delivery_data_0 = provider.fcmdata_api.Delivery_data {
}
delivery_data_1 = provider.fcmdata_api.Delivery_data {
}
delivery_data_2 = provider.fcmdata_api.Delivery_data {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    delivery_data = provider.fcmdata_api.Delivery_data {
    }
```

---

## Related Documentation

- [GCP Fcmdata_api Documentation](https://cloud.google.com/fcmdata_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
