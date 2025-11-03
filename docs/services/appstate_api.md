# Appstate_api Service



**Resources**: 1

---

## Overview

The appstate_api service provides access to 1 resource type:

- [State](#state) [CRUD]

---

## Resources


### State

Clears (sets to empty) the data for the passed key if and only if the passed version matches the currently stored version. This method results in a conflict error on version mismatch.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_key` | i64 | ✅ | The key for the data to be retrieved. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The requested data. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string appstate#getResponse. |
| `state_key` | i64 | The key for the data. |
| `current_state_version` | String | The current app state version. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create state
state = provider.appstate_api.State {
    state_key = "value"  # The key for the data to be retrieved.
}

# Access state outputs
state_id = state.id
state_data = state.data
state_kind = state.kind
state_state_key = state.state_key
state_current_state_version = state.current_state_version
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple state resources
state_0 = provider.appstate_api.State {
    state_key = "value-0"
}
state_1 = provider.appstate_api.State {
    state_key = "value-1"
}
state_2 = provider.appstate_api.State {
    state_key = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    state = provider.appstate_api.State {
        state_key = "production-value"
    }
```

---

## Related Documentation

- [GCP Appstate_api Documentation](https://cloud.google.com/appstate_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
