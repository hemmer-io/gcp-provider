# Trafficdirector_api Service



**Resources**: 2

---

## Overview

The trafficdirector_api service provides access to 2 resource types:

- [Discovery](#discovery) [C]
- [Discovery](#discovery) [C]

---

## Resources


### Discovery



**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `node_matchers` | Vec<String> |  | Management server can use these match criteria to identify clients. The match follows OR semantics. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create discovery
discovery = provider.trafficdirector_api.Discovery {
}

```

---


### Discovery



**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `node_matchers` | Vec<String> |  | Management server can use these match criteria to identify clients. The match follows OR semantics. |
| `exclude_resource_contents` | bool |  | If true, the server will not include the resource contents in the response (i.e., the generic_xds_configs.xds_config field will not be populated). [#not-implemented-hide:] |
| `node` | String |  | The node making the csds request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create discovery
discovery = provider.trafficdirector_api.Discovery {
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

# Create multiple discovery resources
discovery_0 = provider.trafficdirector_api.Discovery {
}
discovery_1 = provider.trafficdirector_api.Discovery {
}
discovery_2 = provider.trafficdirector_api.Discovery {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    discovery = provider.trafficdirector_api.Discovery {
    }
```

---

## Related Documentation

- [GCP Trafficdirector_api Documentation](https://cloud.google.com/trafficdirector_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
