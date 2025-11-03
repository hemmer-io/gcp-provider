# Webfonts_api Service



**Resources**: 1

---

## Overview

The webfonts_api service provides access to 1 resource type:

- [Webfont](#webfont) [R]

---

## Resources


### Webfont

Retrieves the list of fonts currently served by the Google Fonts Developer API.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | This kind represents a list of webfont objects in the webfonts service. |
| `items` | Vec<String> | The list of fonts currently served by the Google Fonts API. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access webfont outputs
webfont_id = webfont.id
webfont_kind = webfont.kind
webfont_items = webfont.items
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple webfont resources
webfont_0 = provider.webfonts_api.Webfont {
}
webfont_1 = provider.webfonts_api.Webfont {
}
webfont_2 = provider.webfonts_api.Webfont {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    webfont = provider.webfonts_api.Webfont {
    }
```

---

## Related Documentation

- [GCP Webfonts_api Documentation](https://cloud.google.com/webfonts_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
