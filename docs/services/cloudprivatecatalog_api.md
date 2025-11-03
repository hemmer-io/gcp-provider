# Cloudprivatecatalog_api Service



**Resources**: 3

---

## Overview

The cloudprivatecatalog_api service provides access to 3 resource types:

- [Catalog](#catalog) [R]
- [Version](#version) [R]
- [Product](#product) [R]

---

## Resources


### Catalog

Search Catalog resources that consumers have access to, within the
scope of the consumer cloud resource hierarchy context.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A pagination token returned from a previous call to SearchCatalogs that
indicates from where listing should continue.
This field is optional. |
| `catalogs` | Vec<String> | The `Catalog`s computed from the resource context. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access catalog outputs
catalog_id = catalog.id
catalog_next_page_token = catalog.next_page_token
catalog_catalogs = catalog.catalogs
```

---


### Version

Search Version resources that consumers have access to, within the
scope of the consumer cloud resource hierarchy context.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `versions` | Vec<String> | The `Version` resources computed from the resource context. |
| `next_page_token` | String | A pagination token returned from a previous call to SearchVersions that
indicates from where the listing should continue.
This field is optional. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access version outputs
version_id = version.id
version_versions = version.versions
version_next_page_token = version.next_page_token
```

---


### Product

Search Product resources that consumers have access to, within the
scope of the consumer cloud resource hierarchy context.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A pagination token returned from a previous call to SearchProducts that
indicates from where listing should continue.
This field is optional. |
| `products` | Vec<String> | The `Product` resources computed from the resource context. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access product outputs
product_id = product.id
product_next_page_token = product.next_page_token
product_products = product.products
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple catalog resources
catalog_0 = provider.cloudprivatecatalog_api.Catalog {
}
catalog_1 = provider.cloudprivatecatalog_api.Catalog {
}
catalog_2 = provider.cloudprivatecatalog_api.Catalog {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    catalog = provider.cloudprivatecatalog_api.Catalog {
    }
```

---

## Related Documentation

- [GCP Cloudprivatecatalog_api Documentation](https://cloud.google.com/cloudprivatecatalog_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
