# Homegraph_api Service



**Resources**: 2

---

## Overview

The homegraph_api service provides access to 2 resource types:

- [Device](#device) [C]
- [Agent_user](#agent_user) [D]

---

## Resources


### Device

Gets the current states in Home Graph for the given set of the third-party user's devices. The third-party user's identity is passed in via the `agent_user_id` (see QueryRequest). This request must be authorized using service account credentials from your Actions console project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inputs` | Vec<String> |  | Required. Inputs containing third-party device IDs for which to get the device states. |
| `agent_user_id` | String |  | Required. Third-party user ID. |
| `request_id` | String |  | Request ID used for debugging. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device
device = provider.homegraph_api.Device {
}

```

---


### Agent_user



**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
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

# Create multiple device resources
device_0 = provider.homegraph_api.Device {
}
device_1 = provider.homegraph_api.Device {
}
device_2 = provider.homegraph_api.Device {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    device = provider.homegraph_api.Device {
    }
```

---

## Related Documentation

- [GCP Homegraph_api Documentation](https://cloud.google.com/homegraph_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
