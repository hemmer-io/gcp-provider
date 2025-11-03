# Webmasters_api Service



**Resources**: 3

---

## Overview

The webmasters_api service provides access to 3 resource types:

- [Searchanalytic](#searchanalytic) [C]
- [Sitemap](#sitemap) [RUD]
- [Site](#site) [RUD]

---

## Resources


### Searchanalytic

Query your data with filters and parameters that you define. Returns zero or more rows grouped by the row keys that you define. You must define a date range of one or more days.

When date is one of the group by values, any days without data are omitted from the result list. If you need to know which days have data, issue a broad date range query grouped by date for any metric, and see which day rows are returned.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_state` | String |  | [Optional] If "all" (case-insensitive), data will include fresh data. If "final" (case-insensitive) or if this parameter is omitted, the returned data will include only finalized data. |
| `start_row` | i64 |  | [Optional; Default is 0] Zero-based index of the first row in the response. Must be a non-negative number. |
| `dimension_filter_groups` | Vec<String> |  | [Optional] Zero or more filters to apply to the dimension grouping values; for example, 'query contains "buy"' to see only data where the query string contains the substring "buy" (not case-sensitive). You can filter by a dimension without grouping by it. |
| `row_limit` | i64 |  | [Optional; Default is 1000] The maximum number of rows to return. Must be a number from 1 to 5,000 (inclusive). |
| `search_type` | String |  | [Optional; Default is "web"] The search type to filter for. |
| `dimensions` | Vec<String> |  | [Optional] Zero or more dimensions to group results by. Dimensions are the group-by values in the Search Analytics page. Dimensions are combined to create a unique row key for each row. Results are grouped in the order that you supply these dimensions. |
| `start_date` | String |  | [Required] Start date of the requested date range, in YYYY-MM-DD format, in PST time (UTC - 8:00). Must be less than or equal to the end date. This value is included in the range. |
| `aggregation_type` | String |  | [Optional; Default is "auto"] How data is aggregated. If aggregated by property, all data for the same property is aggregated; if aggregated by page, all data is aggregated by canonical URI. If you filter or group by page, choose AUTO; otherwise you can aggregate either by property or by page, depending on how you want your data calculated; see  the help documentation to learn how data is calculated differently by site versus by page.

Note: If you group or filter by page, you cannot aggregate by property.

If you specify any value other than AUTO, the aggregation type in the result will match the requested type, or if you request an invalid type, you will get an error. The API will never change your aggregation type if the requested type is invalid. |
| `end_date` | String |  | [Required] End date of the requested date range, in YYYY-MM-DD format, in PST (UTC - 8:00). Must be greater than or equal to the start date. This value is included in the range. |
| `site_url` | String | ✅ | The site's URL, including protocol. For example: http://www.example.com/ |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create searchanalytic
searchanalytic = provider.webmasters_api.Searchanalytic {
    site_url = "value"  # The site's URL, including protocol. For example: http://www.example.com/
}

```

---


### Sitemap

Retrieves information about a specific sitemap.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `feedpath` | String | ✅ | The URL of the sitemap to add. For example: http://www.example.com/sitemap.xml |
| `site_url` | String | ✅ | The site's URL, including protocol. For example: http://www.example.com/ |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `path` | String | The url of the sitemap. |
| `contents` | Vec<String> | The various content types in the sitemap. |
| `is_sitemaps_index` | bool | If true, the sitemap is a collection of sitemaps. |
| `errors` | String | Number of errors in the sitemap. These are issues with the sitemap itself that need to be fixed before it can be processed correctly. |
| `is_pending` | bool | If true, the sitemap has not been processed. |
| `last_downloaded` | String | Date & time in which this sitemap was last downloaded. Date format is in RFC 3339 format (yyyy-mm-dd). |
| `last_submitted` | String | Date & time in which this sitemap was submitted. Date format is in RFC 3339 format (yyyy-mm-dd). |
| `type` | String | The type of the sitemap. For example: rssFeed. |
| `warnings` | String | Number of warnings for the sitemap. These are generally non-critical issues with URLs in the sitemaps. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access sitemap outputs
sitemap_id = sitemap.id
sitemap_path = sitemap.path
sitemap_contents = sitemap.contents
sitemap_is_sitemaps_index = sitemap.is_sitemaps_index
sitemap_errors = sitemap.errors
sitemap_is_pending = sitemap.is_pending
sitemap_last_downloaded = sitemap.last_downloaded
sitemap_last_submitted = sitemap.last_submitted
sitemap_type = sitemap.type
sitemap_warnings = sitemap.warnings
```

---


### Site

Retrieves information about specific site.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `site_url` | String | ✅ | The URL of the site to add. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `site_url` | String | The URL of the site. |
| `permission_level` | String | The user's permission level for the site. |


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
site_site_url = site.site_url
site_permission_level = site.permission_level
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple searchanalytic resources
searchanalytic_0 = provider.webmasters_api.Searchanalytic {
    site_url = "value-0"
}
searchanalytic_1 = provider.webmasters_api.Searchanalytic {
    site_url = "value-1"
}
searchanalytic_2 = provider.webmasters_api.Searchanalytic {
    site_url = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    searchanalytic = provider.webmasters_api.Searchanalytic {
        site_url = "production-value"
    }
```

---

## Related Documentation

- [GCP Webmasters_api Documentation](https://cloud.google.com/webmasters_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
