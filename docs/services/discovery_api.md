# Discovery_api Service



**Resources**: 1

---

## Overview

The discovery_api service provides access to 1 resource type:

- [Api](#api) [R]

---

## Resources


### Api

Retrieve the list of APIs supported at this endpoint.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The kind for this response. |
| `discovery_version` | String | Indicate the version of the Discovery API used to generate this doc. |
| `items` | Vec<String> | The individual directory entries. One entry per api/version pair. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access api outputs
api_id = api.id
api_kind = api.kind
api_discovery_version = api.discovery_version
api_items = api.items
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple api resources
api_0 = provider.discovery_api.Api {
}
api_1 = provider.discovery_api.Api {
}
api_2 = provider.discovery_api.Api {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    api = provider.discovery_api.Api {
    }
```

---

## Related Documentation

- [GCP Discovery_api Documentation](https://cloud.google.com/discovery_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
