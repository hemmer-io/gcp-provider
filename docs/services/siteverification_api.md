# Siteverification_api Service



**Resources**: 1

---

## Overview

The siteverification_api service provides access to 1 resource type:

- [Web_resource](#web_resource) [CRUD]

---

## Resources


### Web_resource

Attempt verification of a website or domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `owners` | Vec<String> |  | The email addresses of all verified owners. |
| `site` | String |  | The address and type of a site that is verified or will be verified. |
| `id` | String |  | The string used to identify this site. This value should be used in the "id" portion of the REST URL for the Get, Update, and Delete operations. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `owners` | Vec<String> | The email addresses of all verified owners. |
| `site` | String | The address and type of a site that is verified or will be verified. |
| `id` | String | The string used to identify this site. This value should be used in the "id" portion of the REST URL for the Get, Update, and Delete operations. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create web_resource
web_resource = provider.siteverification_api.Web_resource {
}

# Access web_resource outputs
web_resource_id = web_resource.id
web_resource_owners = web_resource.owners
web_resource_site = web_resource.site
web_resource_id = web_resource.id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple web_resource resources
web_resource_0 = provider.siteverification_api.Web_resource {
}
web_resource_1 = provider.siteverification_api.Web_resource {
}
web_resource_2 = provider.siteverification_api.Web_resource {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    web_resource = provider.siteverification_api.Web_resource {
    }
```

---

## Related Documentation

- [GCP Siteverification_api Documentation](https://cloud.google.com/siteverification_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
