# Kgsearch_api Service



**Resources**: 1

---

## Overview

The kgsearch_api service provides access to 1 resource type:

- [Entitie](#entitie) [R]

---

## Resources


### Entitie

Searches Knowledge Graph for entities that match the constraints. A list of matched entities will be returned in response, which will be in JSON-LD format and compatible with http://schema.org

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `@context` | String | The local context applicable for the response. See more details at http://www.w3.org/TR/json-ld/#context-definitions. |
| `@type` | String | The schema type of top-level JSON-LD object, e.g. ItemList. |
| `item_list_element` | Vec<String> | The item list of search results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access entitie outputs
entitie_id = entitie.id
entitie_@context = entitie.@context
entitie_@type = entitie.@type
entitie_item_list_element = entitie.item_list_element
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple entitie resources
entitie_0 = provider.kgsearch_api.Entitie {
}
entitie_1 = provider.kgsearch_api.Entitie {
}
entitie_2 = provider.kgsearch_api.Entitie {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    entitie = provider.kgsearch_api.Entitie {
    }
```

---

## Related Documentation

- [GCP Kgsearch_api Documentation](https://cloud.google.com/kgsearch_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
