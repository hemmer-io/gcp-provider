# Kmsinventory_api Service



**Resources**: 2

---

## Overview

The kmsinventory_api service provides access to 2 resource types:

- [Protected_resource](#protected_resource) [R]
- [Crypto_key](#crypto_key) [R]

---

## Resources


### Protected_resource

Returns metadata about the resources protected by the given Cloud KMS CryptoKey in the given Cloud organization.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `protected_resources` | Vec<String> | Protected resources for this page. |
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


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
protected_resource_protected_resources = protected_resource.protected_resources
protected_resource_next_page_token = protected_resource.next_page_token
```

---


### Crypto_key

Returns cryptographic keys managed by Cloud KMS in a given Cloud project. Note that this data is sourced from snapshots, meaning it may not completely reflect the actual state of key metadata at call time.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `crypto_keys` | Vec<String> | The list of CryptoKeys. |
| `next_page_token` | String | The page token returned from the previous response if the next page is desired. |


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
crypto_key_crypto_keys = crypto_key.crypto_keys
crypto_key_next_page_token = crypto_key.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple protected_resource resources
protected_resource_0 = provider.kmsinventory_api.Protected_resource {
}
protected_resource_1 = provider.kmsinventory_api.Protected_resource {
}
protected_resource_2 = provider.kmsinventory_api.Protected_resource {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    protected_resource = provider.kmsinventory_api.Protected_resource {
    }
```

---

## Related Documentation

- [GCP Kmsinventory_api Documentation](https://cloud.google.com/kmsinventory_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
