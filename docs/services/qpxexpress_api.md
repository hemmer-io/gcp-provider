# Qpxexpress_api Service



**Resources**: 1

---

## Overview

The qpxexpress_api service provides access to 1 resource type:

- [Trip](#trip) [C]

---

## Resources


### Trip

Returns a list of flights.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request` | String |  | A QPX Express search request. Required values are at least one adult or senior passenger, an origin, a destination, and a date. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trip
trip = provider.qpxexpress_api.Trip {
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

# Create multiple trip resources
trip_0 = provider.qpxexpress_api.Trip {
}
trip_1 = provider.qpxexpress_api.Trip {
}
trip_2 = provider.qpxexpress_api.Trip {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    trip = provider.qpxexpress_api.Trip {
    }
```

---

## Related Documentation

- [GCP Qpxexpress_api Documentation](https://cloud.google.com/qpxexpress_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
