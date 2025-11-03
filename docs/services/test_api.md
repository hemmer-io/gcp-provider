# Test_api Service



**Resources**: 2

---

## Overview

The test_api service provides access to 2 resource types:

- [Bucket](#bucket) [R]
- [Oauth2](#oauth2) [R]

---

## Resources


### Bucket

d

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String |  |
| `cors` | Vec<String> |  |
| `kind` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access bucket outputs
bucket_id = bucket.id
bucket_id = bucket.id
bucket_cors = bucket.cors
bucket_kind = bucket.kind
```

---


### Oauth2



**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String |  |
| `cors` | Vec<String> |  |
| `kind` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access oauth2 outputs
oauth2_id = oauth2.id
oauth2_id = oauth2.id
oauth2_cors = oauth2.cors
oauth2_kind = oauth2.kind
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple bucket resources
bucket_0 = provider.test_api.Bucket {
}
bucket_1 = provider.test_api.Bucket {
}
bucket_2 = provider.test_api.Bucket {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    bucket = provider.test_api.Bucket {
    }
```

---

## Related Documentation

- [GCP Test_api Documentation](https://cloud.google.com/test_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
