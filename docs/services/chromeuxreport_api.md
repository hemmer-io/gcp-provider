# Chromeuxreport_api Service



**Resources**: 1

---

## Overview

The chromeuxreport_api service provides access to 1 resource type:

- [Record](#record) [C]

---

## Resources


### Record

Queries the Chrome User Experience Report for a timeseries `history record` for a given site. Returns a `history record` that contains one or more `metric timeseries` corresponding to performance data about the requested site.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | String |  | The url pattern "url" refers to a url pattern that is any arbitrary url. Examples: "https://example.com/", "https://cloud.google.com/why-google-cloud/" |
| `origin` | String |  | The url pattern "origin" refers to a url pattern that is the origin of a website. Examples: "https://example.com", "https://cloud.google.com" |
| `metrics` | Vec<String> |  | The metrics that should be included in the response. If none are specified then any metrics found will be returned. Allowed values: ["first_contentful_paint", "first_input_delay", "largest_contentful_paint", "cumulative_layout_shift", "experimental_time_to_first_byte", "experimental_interaction_to_next_paint"] |
| `collection_period_count` | i64 |  | The number of collection periods to return. If not specified, the default is 25. If present, must be in the range [1, 40]. |
| `form_factor` | String |  | The form factor is a query dimension that specifies the device class that the record's data should belong to. Note: If no form factor is specified, then a special record with aggregated data over all form factors will be returned. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create record
record = provider.chromeuxreport_api.Record {
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

# Create multiple record resources
record_0 = provider.chromeuxreport_api.Record {
}
record_1 = provider.chromeuxreport_api.Record {
}
record_2 = provider.chromeuxreport_api.Record {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    record = provider.chromeuxreport_api.Record {
    }
```

---

## Related Documentation

- [GCP Chromeuxreport_api Documentation](https://cloud.google.com/chromeuxreport_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
