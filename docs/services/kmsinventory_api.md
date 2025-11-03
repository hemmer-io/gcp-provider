# Kmsinventory_api Service



**Resources**: 2

---

## Overview

The kmsinventory_api service provides access to 2 resource types:

- [Crypto_key](#crypto_key) [R]
- [Protected_resource](#protected_resource) [R]

---

## Resources


### Crypto_key

Returns aggregate information about the resources protected by the given Cloud KMS CryptoKey. Only resources within the same Cloud organization as the key will be returned. The project that holds the key must be part of an organization in order for this call to succeed.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_types` | HashMap<String, String> | The number of resources protected by the key grouped by resource type. |
| `locations` | HashMap<String, String> | The number of resources protected by the key grouped by region. |
| `cloud_products` | HashMap<String, String> | The number of resources protected by the key grouped by Cloud product. |
| `name` | String | The full name of the ProtectedResourcesSummary resource. Example: projects/test-project/locations/us/keyRings/test-keyring/cryptoKeys/test-key/protectedResourcesSummary |
| `resource_count` | String | The total number of protected resources in the same Cloud organization as the key. |
| `project_count` | i64 | The number of distinct Cloud projects in the same Cloud organization as the key that have resources protected by the key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access crypto_key outputs
crypto_key_id = crypto_key.id
crypto_key_resource_types = crypto_key.resource_types
crypto_key_locations = crypto_key.locations
crypto_key_cloud_products = crypto_key.cloud_products
crypto_key_name = crypto_key.name
crypto_key_resource_count = crypto_key.resource_count
crypto_key_project_count = crypto_key.project_count
```

---


### Protected_resource

Returns metadata about the resources protected by the given Cloud KMS CryptoKey in the given Cloud organization.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `protected_resources` | Vec<String> | Protected resources for this page. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access protected_resource outputs
protected_resource_id = protected_resource.id
protected_resource_next_page_token = protected_resource.next_page_token
protected_resource_protected_resources = protected_resource.protected_resources
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple crypto_key resources
crypto_key_0 = provider.kmsinventory_api.Crypto_key {
}
crypto_key_1 = provider.kmsinventory_api.Crypto_key {
}
crypto_key_2 = provider.kmsinventory_api.Crypto_key {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    crypto_key = provider.kmsinventory_api.Crypto_key {
    }
```

---

## Related Documentation

- [GCP Kmsinventory_api Documentation](https://cloud.google.com/kmsinventory_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
