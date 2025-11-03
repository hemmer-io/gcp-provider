# Playablelocations_api Service



**Resources**: 1

---

## Overview

The playablelocations_api service provides access to 1 resource type:

- [Playablelocation](#playablelocation) [C]

---

## Resources


### Playablelocation

Returns a set of playable locations that lie within a specified area, that satisfy optional filter criteria. Note: Identical `SamplePlayableLocations` requests can return different results as the state of the world changes over time.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `criteria` | Vec<String> |  | Required. Specifies one or more (up to 5) criteria for filtering the returned playable locations. |
| `area_filter` | String |  | Required. Specifies the area to search within for playable locations. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create playablelocation
playablelocation = provider.playablelocations_api.Playablelocation {
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

# Create multiple playablelocation resources
playablelocation_0 = provider.playablelocations_api.Playablelocation {
}
playablelocation_1 = provider.playablelocations_api.Playablelocation {
}
playablelocation_2 = provider.playablelocations_api.Playablelocation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    playablelocation = provider.playablelocations_api.Playablelocation {
    }
```

---

## Related Documentation

- [GCP Playablelocations_api Documentation](https://cloud.google.com/playablelocations_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
