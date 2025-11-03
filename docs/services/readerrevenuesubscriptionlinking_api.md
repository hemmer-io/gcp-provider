# Readerrevenuesubscriptionlinking_api Service



**Resources**: 1

---

## Overview

The readerrevenuesubscriptionlinking_api service provides access to 1 resource type:

- [Reader](#reader) [RUD]

---

## Resources


### Reader

Gets a reader of a publication. Returns NOT_FOUND if the reader does not exist.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entitlements` | Vec<String> |  | All of the entitlements for a publication reader. |
| `name` | String |  | Output only. The resource name of the singleton. |
| `name` | String | ✅ | Output only. The resource name of the singleton. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ppid` | String | Output only. The publisher provided id of the reader. |
| `create_time` | String | Output only. Time the publication reader was created and associated with a Google user. |
| `originating_publication_id` | String | Output only. The SwG publication id that the reader's subscription linking was originating from. |
| `publication_id` | String | Output only. The SwG publication id that the reader has linked their subscription to. |
| `name` | String | Output only. The resource name of the reader. The last part of ppid in the resource name is the publisher provided id. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access reader outputs
reader_id = reader.id
reader_ppid = reader.ppid
reader_create_time = reader.create_time
reader_originating_publication_id = reader.originating_publication_id
reader_publication_id = reader.publication_id
reader_name = reader.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple reader resources
reader_0 = provider.readerrevenuesubscriptionlinking_api.Reader {
    name = "value-0"
}
reader_1 = provider.readerrevenuesubscriptionlinking_api.Reader {
    name = "value-1"
}
reader_2 = provider.readerrevenuesubscriptionlinking_api.Reader {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    reader = provider.readerrevenuesubscriptionlinking_api.Reader {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Readerrevenuesubscriptionlinking_api Documentation](https://cloud.google.com/readerrevenuesubscriptionlinking_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
