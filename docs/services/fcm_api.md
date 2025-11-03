# Fcm_api Service



**Resources**: 1

---

## Overview

The fcm_api service provides access to 1 resource type:

- [Message](#message) [C]

---

## Resources


### Message

Send a message to specified target (a registration token, topic or condition).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `validate_only` | bool |  | Flag for testing the request without actually delivering the message. |
| `message` | String |  | Required. Message to send. |
| `parent` | String | ✅ | Required. It contains the Firebase project id (i.e. the unique identifier for your Firebase project), in the format of `projects/{project_id}`. The numeric project number with no padding is also supported in the format of `projects/{project_number}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create message
message = provider.fcm_api.Message {
    parent = "value"  # Required. It contains the Firebase project id (i.e. the unique identifier for your Firebase project), in the format of `projects/{project_id}`. The numeric project number with no padding is also supported in the format of `projects/{project_number}`.
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple message resources
message_0 = provider.fcm_api.Message {
    parent = "value-0"
}
message_1 = provider.fcm_api.Message {
    parent = "value-1"
}
message_2 = provider.fcm_api.Message {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    message = provider.fcm_api.Message {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Fcm_api Documentation](https://cloud.google.com/fcm_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
