# Groupsmigration_api Service



**Resources**: 1

---

## Overview

The groupsmigration_api service provides access to 1 resource type:

- [Archive](#archive) [C]

---

## Resources


### Archive

Inserts a new mail into the archive of the Google group.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_id` | String | ✅ | The group ID |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create archive
archive = provider.groupsmigration_api.Archive {
    group_id = "value"  # The group ID
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

# Create multiple archive resources
archive_0 = provider.groupsmigration_api.Archive {
    group_id = "value-0"
}
archive_1 = provider.groupsmigration_api.Archive {
    group_id = "value-1"
}
archive_2 = provider.groupsmigration_api.Archive {
    group_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    archive = provider.groupsmigration_api.Archive {
        group_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Groupsmigration_api Documentation](https://cloud.google.com/groupsmigration_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
