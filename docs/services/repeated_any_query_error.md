# Repeated_any_query_error Service



**Resources**: 1

---

## Overview

The repeated_any_query_error service provides access to 1 resource type:

- [Entrie](#entrie) [C]

---

## Resources


### Entrie

Creates one or more log entries in a log. You must supply a list of `LogEntry` objects, named `entries`. Each `LogEntry` object must contain a payload object and a `LogEntryMetadata` object that describes the entry. You must fill in all the fields of the entry, metadata, and payload. You can also supply a map, `commonLabels`, that supplies default (key, value) data for the `entries[].metadata.labels` maps, saving you the trouble of creating identical copies for each entry.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | Log entries to insert. |
| `common_labels` | HashMap<String, String> |  | Metadata labels that apply to all entries in this request. If one of the log entries contains a (key, value) with the same key that is in `commonLabels`, then the entry's (key, value) overrides the one in `commonLabels`. |
| `projects_id` | String | ✅ | Part of `logName`. The name of the log resource into which to insert the log entries. |
| `logs_id` | String | ✅ | Part of `logName`. See documentation of `projectsId`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entrie
entrie = provider.repeated_any_query_error.Entrie {
    projects_id = "value"  # Part of `logName`. The name of the log resource into which to insert the log entries.
    logs_id = "value"  # Part of `logName`. See documentation of `projectsId`.
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

# Create multiple entrie resources
entrie_0 = provider.repeated_any_query_error.Entrie {
    projects_id = "value-0"
    logs_id = "value-0"
}
entrie_1 = provider.repeated_any_query_error.Entrie {
    projects_id = "value-1"
    logs_id = "value-1"
}
entrie_2 = provider.repeated_any_query_error.Entrie {
    projects_id = "value-2"
    logs_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    entrie = provider.repeated_any_query_error.Entrie {
        projects_id = "production-value"
        logs_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Repeated_any_query_error Documentation](https://cloud.google.com/repeated_any_query_error/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
