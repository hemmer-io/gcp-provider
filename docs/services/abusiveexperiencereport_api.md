# Abusiveexperiencereport_api Service



**Resources**: 2

---

## Overview

The abusiveexperiencereport_api service provides access to 2 resource types:

- [Violating_site](#violating_site) [R]
- [Site](#site) [R]

---

## Resources


### Violating_site

Lists sites that are failing in the Abusive Experience Report.

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


### Site

Gets a site's Abusive Experience Report summary.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `abusive_status` | String | The site's Abusive Experience Report status. |
| `enforcement_time` | String | The time at which [enforcement](https://support.google.com/webtools/answer/7538608) against the site began or will begin. Not set when the filter_status is OFF. |
| `filter_status` | String | The site's [enforcement status](https://support.google.com/webtools/answer/7538608). |
| `reviewed_site` | String | The name of the reviewed site, e.g. `google.com`. |
| `report_url` | String | A link to the full Abusive Experience Report for the site. Not set in ViolatingSitesResponse. Note that you must complete the [Search Console verification process](https://support.google.com/webmasters/answer/9008080) for the site before you can access the full report. |
| `last_change_time` | String | The time at which the site's status last changed. |
| `under_review` | bool | Whether the site is currently under review. |


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
site_abusive_status = site.abusive_status
site_enforcement_time = site.enforcement_time
site_filter_status = site.filter_status
site_reviewed_site = site.reviewed_site
site_report_url = site.report_url
site_last_change_time = site.last_change_time
site_under_review = site.under_review
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple violating_site resources
violating_site_0 = provider.abusiveexperiencereport_api.Violating_site {
}
violating_site_1 = provider.abusiveexperiencereport_api.Violating_site {
}
violating_site_2 = provider.abusiveexperiencereport_api.Violating_site {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    violating_site = provider.abusiveexperiencereport_api.Violating_site {
    }
```

---

## Related Documentation

- [GCP Abusiveexperiencereport_api Documentation](https://cloud.google.com/abusiveexperiencereport_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
