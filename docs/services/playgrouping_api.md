# Playgrouping_api Service



**Resources**: 2

---

## Overview

The playgrouping_api service provides access to 2 resource types:

- [Tag](#tag) [C]
- [Token](#token) [C]

---

## Resources


### Tag

Create or update tags for the user and app that are represented by the given token.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | Tags to be inserted or updated. |
| `app_package` | String | ✅ | Required. App whose tags are being manipulated. Format: apps/{package_name} |
| `token` | String | ✅ | Required. Token for which the tags are being inserted or updated. Format: tokens/{token} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag
tag = provider.playgrouping_api.Tag {
    app_package = "value"  # Required. App whose tags are being manipulated. Format: apps/{package_name}
    token = "value"  # Required. Token for which the tags are being inserted or updated. Format: tokens/{token}
}

```

---


### Token

Verify an API token by asserting the app and persona it belongs to. The verification is a protection against client-side attacks and will fail if the contents of the token don't match the provided values. A token must be verified before it can be used to manipulate user tags.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `persona` | String |  | Required. Persona represented by the token. Format: personas/{persona} |
| `app_package` | String | ✅ | Required. App the token belongs to. Format: apps/{package_name} |
| `token` | String | ✅ | Required. The token to be verified. Format: tokens/{token} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create token
token = provider.playgrouping_api.Token {
    app_package = "value"  # Required. App the token belongs to. Format: apps/{package_name}
    token = "value"  # Required. The token to be verified. Format: tokens/{token}
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

# Create multiple tag resources
tag_0 = provider.playgrouping_api.Tag {
    app_package = "value-0"
    token = "value-0"
}
tag_1 = provider.playgrouping_api.Tag {
    app_package = "value-1"
    token = "value-1"
}
tag_2 = provider.playgrouping_api.Tag {
    app_package = "value-2"
    token = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    tag = provider.playgrouping_api.Tag {
        app_package = "production-value"
        token = "production-value"
    }
```

---

## Related Documentation

- [GCP Playgrouping_api Documentation](https://cloud.google.com/playgrouping_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
