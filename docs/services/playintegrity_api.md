# Playintegrity_api Service



**Resources**: 2

---

## Overview

The playintegrity_api service provides access to 2 resource types:

- [Device_recall](#device_recall) [C]
- [Playintegrity](#playintegrity) [C]

---

## Resources


### Device_recall

Writes recall bits for the device where Play Integrity API token is obtained. The endpoint is available to select Play partners in an early access program (EAP).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `new_values` | String |  | Required. The new values for the device recall bits to be written. |
| `integrity_token` | String |  | Required. Integrity token obtained from calling Play Integrity API. |
| `package_name` | String | ✅ | Required. Package name of the app the attached integrity token belongs to. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device_recall
device_recall = provider.playintegrity_api.Device_recall {
    package_name = "value"  # Required. Package name of the app the attached integrity token belongs to.
}

```

---


### Playintegrity

Decodes the PC integrity token and returns the PC token payload.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `integrity_token` | String |  | Encoded integrity token. |
| `package_name` | String | ✅ | Package name of the app the attached integrity token belongs to. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create playintegrity
playintegrity = provider.playintegrity_api.Playintegrity {
    package_name = "value"  # Package name of the app the attached integrity token belongs to.
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

# Create multiple device_recall resources
device_recall_0 = provider.playintegrity_api.Device_recall {
    package_name = "value-0"
}
device_recall_1 = provider.playintegrity_api.Device_recall {
    package_name = "value-1"
}
device_recall_2 = provider.playintegrity_api.Device_recall {
    package_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    device_recall = provider.playintegrity_api.Device_recall {
        package_name = "production-value"
    }
```

---

## Related Documentation

- [GCP Playintegrity_api Documentation](https://cloud.google.com/playintegrity_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
