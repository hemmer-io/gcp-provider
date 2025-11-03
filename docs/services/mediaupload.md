# Mediaupload Service



**Resources**: 1

---

## Overview

The mediaupload service provides access to 1 resource type:

- [Caption](#caption) [C]

---

## Resources


### Caption

Inserts a new resource into this collection.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The ID that YouTube uses to uniquely identify the caption track. |
| `etag` | String |  | Etag of this resource. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#caption". |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create caption
caption = provider.mediaupload.Caption {
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

# Create multiple caption resources
caption_0 = provider.mediaupload.Caption {
}
caption_1 = provider.mediaupload.Caption {
}
caption_2 = provider.mediaupload.Caption {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    caption = provider.mediaupload.Caption {
    }
```

---

## Related Documentation

- [GCP Mediaupload Documentation](https://cloud.google.com/mediaupload/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
