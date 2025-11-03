# Customsearch_api Service



**Resources**: 2

---

## Overview

The customsearch_api service provides access to 2 resource types:

- [Cse](#cse) [R]
- [Siterestrict](#siterestrict) [R]

---

## Resources


### Cse

Returns metadata about the search performed, metadata about the engine used for the search, and the search results.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `search_information` | String | Metadata about a search operation. |
| `context` | HashMap<String, String> | Metadata and refinements associated with the given search engine, including: * The name of the search engine that was used for the query. * A set of [facet objects](https://developers.google.com/custom-search/docs/refinements#create) (refinements) you can use for refining a search. |
| `queries` | String | Query metadata for the previous, current, and next pages of results. |
| `promotions` | Vec<String> | The set of [promotions](https://developers.google.com/custom-search/docs/promotions). Present only if the custom search engine's configuration files define any promotions for the given query. |
| `kind` | String | Unique identifier for the type of current object. For this API, it is customsearch#search. |
| `spelling` | String | Spell correction information for a query. |
| `items` | Vec<String> | The current set of custom search results. |
| `url` | String | OpenSearch template and URL. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access cse outputs
cse_id = cse.id
cse_search_information = cse.search_information
cse_context = cse.context
cse_queries = cse.queries
cse_promotions = cse.promotions
cse_kind = cse.kind
cse_spelling = cse.spelling
cse_items = cse.items
cse_url = cse.url
```

---


### Siterestrict

Returns metadata about the search performed, metadata about the engine used for the search, and the search results. Uses a small set of url patterns.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `search_information` | String | Metadata about a search operation. |
| `context` | HashMap<String, String> | Metadata and refinements associated with the given search engine, including: * The name of the search engine that was used for the query. * A set of [facet objects](https://developers.google.com/custom-search/docs/refinements#create) (refinements) you can use for refining a search. |
| `queries` | String | Query metadata for the previous, current, and next pages of results. |
| `promotions` | Vec<String> | The set of [promotions](https://developers.google.com/custom-search/docs/promotions). Present only if the custom search engine's configuration files define any promotions for the given query. |
| `kind` | String | Unique identifier for the type of current object. For this API, it is customsearch#search. |
| `spelling` | String | Spell correction information for a query. |
| `items` | Vec<String> | The current set of custom search results. |
| `url` | String | OpenSearch template and URL. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access siterestrict outputs
siterestrict_id = siterestrict.id
siterestrict_search_information = siterestrict.search_information
siterestrict_context = siterestrict.context
siterestrict_queries = siterestrict.queries
siterestrict_promotions = siterestrict.promotions
siterestrict_kind = siterestrict.kind
siterestrict_spelling = siterestrict.spelling
siterestrict_items = siterestrict.items
siterestrict_url = siterestrict.url
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple cse resources
cse_0 = provider.customsearch_api.Cse {
}
cse_1 = provider.customsearch_api.Cse {
}
cse_2 = provider.customsearch_api.Cse {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    cse = provider.customsearch_api.Cse {
    }
```

---

## Related Documentation

- [GCP Customsearch_api Documentation](https://cloud.google.com/customsearch_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
