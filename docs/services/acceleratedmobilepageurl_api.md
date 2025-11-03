# Acceleratedmobilepageurl_api Service



**Resources**: 1

---

## Overview

The acceleratedmobilepageurl_api service provides access to 1 resource type:

- [Amp_url](#amp_url) [C]

---

## Resources


### Amp_url

Returns AMP URL(s) and equivalent [AMP Cache URL(s)](/amp/cache/overview#amp-cache-url-format).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lookup_strategy` | String |  | The lookup_strategy being requested. |
| `urls` | Vec<String> |  | List of URLs to look up for the paired AMP URLs. The URLs are case-sensitive. Up to 50 URLs per lookup (see [Usage Limits](/amp/cache/reference/limits)). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create amp_url
amp_url = provider.acceleratedmobilepageurl_api.Amp_url {
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

# Create multiple amp_url resources
amp_url_0 = provider.acceleratedmobilepageurl_api.Amp_url {
}
amp_url_1 = provider.acceleratedmobilepageurl_api.Amp_url {
}
amp_url_2 = provider.acceleratedmobilepageurl_api.Amp_url {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    amp_url = provider.acceleratedmobilepageurl_api.Amp_url {
    }
```

---

## Related Documentation

- [GCP Acceleratedmobilepageurl_api Documentation](https://cloud.google.com/acceleratedmobilepageurl_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
