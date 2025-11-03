# Playcustomapp_api Service



**Resources**: 1

---

## Overview

The playcustomapp_api service provides access to 1 resource type:

- [Custom_app](#custom_app) [C]

---

## Resources


### Custom_app

Creates a new custom app.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organizations` | Vec<String> |  | Organizations to which the custom app should be made available. If the request contains any organizations, then the app will be restricted to only these organizations. To support the organization linked to the developer account, the organization ID should be provided explicitly together with other organizations. If no organizations are provided, then the app is only available to the organization linked to the developer account. |
| `package_name` | String |  | Output only. Package name of the created Android app. Only present in the API response. |
| `title` | String |  | Title for the Android app. |
| `language_code` | String |  | Default listing language in BCP 47 format. |
| `account` | String | ✅ | Developer account ID. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_app
custom_app = provider.playcustomapp_api.Custom_app {
    account = "value"  # Developer account ID.
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

# Create multiple custom_app resources
custom_app_0 = provider.playcustomapp_api.Custom_app {
    account = "value-0"
}
custom_app_1 = provider.playcustomapp_api.Custom_app {
    account = "value-1"
}
custom_app_2 = provider.playcustomapp_api.Custom_app {
    account = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    custom_app = provider.playcustomapp_api.Custom_app {
        account = "production-value"
    }
```

---

## Related Documentation

- [GCP Playcustomapp_api Documentation](https://cloud.google.com/playcustomapp_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
