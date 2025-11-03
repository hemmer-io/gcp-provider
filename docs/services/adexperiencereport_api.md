# Adexperiencereport_api Service



**Resources**: 2

---

## Overview

The adexperiencereport_api service provides access to 2 resource types:

- [Site](#site) [R]
- [Violating_site](#violating_site) [R]

---

## Resources


### Site

Gets a site's Ad Experience Report summary.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `desktop_summary` | String | The site's Ad Experience Report summary on desktop. |
| `mobile_summary` | String | The site's Ad Experience Report summary on mobile. |
| `reviewed_site` | String | The name of the reviewed site, e.g. `google.com`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access site outputs
site_id = site.id
site_desktop_summary = site.desktop_summary
site_mobile_summary = site.mobile_summary
site_reviewed_site = site.reviewed_site
```

---


### Violating_site

Lists sites that are failing in the Ad Experience Report on at least one platform.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `violating_sites` | Vec<String> | The list of violating sites. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access violating_site outputs
violating_site_id = violating_site.id
violating_site_violating_sites = violating_site.violating_sites
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple site resources
site_0 = provider.adexperiencereport_api.Site {
}
site_1 = provider.adexperiencereport_api.Site {
}
site_2 = provider.adexperiencereport_api.Site {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    site = provider.adexperiencereport_api.Site {
    }
```

---

## Related Documentation

- [GCP Adexperiencereport_api Documentation](https://cloud.google.com/adexperiencereport_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
