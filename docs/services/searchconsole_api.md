# Searchconsole_api Service



**Resources**: 5

---

## Overview

The searchconsole_api service provides access to 5 resource types:

- [Sitemap](#sitemap) [RUD]
- [Searchanalytic](#searchanalytic) [C]
- [Index](#index) [C]
- [Site](#site) [RUD]
- [Mobile_friendly_test](#mobile_friendly_test) [C]

---

## Resources


### Sitemap

Retrieves information about a specific sitemap.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `feedpath` | String | ✅ | The URL of the actual sitemap. For example: `http://www.example.com/sitemap.xml`. |
| `site_url` | String | ✅ | The site's URL, including protocol. For example: `http://www.example.com/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_pending` | bool | If true, the sitemap has not been processed. |
| `contents` | Vec<String> | The various content types in the sitemap. |
| `last_downloaded` | String | Date & time in which this sitemap was last downloaded. Date format is in RFC 3339 format (yyyy-mm-dd). |
| `is_sitemaps_index` | bool | If true, the sitemap is a collection of sitemaps. |
| `last_submitted` | String | Date & time in which this sitemap was submitted. Date format is in RFC 3339 format (yyyy-mm-dd). |
| `errors` | String | Number of errors in the sitemap. These are issues with the sitemap itself that need to be fixed before it can be processed correctly. |
| `type` | String | The type of the sitemap. For example: `rssFeed`. |
| `path` | String | The url of the sitemap. |
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
sitemap_is_pending = sitemap.is_pending
sitemap_contents = sitemap.contents
sitemap_last_downloaded = sitemap.last_downloaded
sitemap_is_sitemaps_index = sitemap.is_sitemaps_index
sitemap_last_submitted = sitemap.last_submitted
sitemap_errors = sitemap.errors
sitemap_type = sitemap.type
sitemap_path = sitemap.path
sitemap_warnings = sitemap.warnings
```

---


### Searchanalytic

Query your data with filters and parameters that you define. Returns zero or more rows grouped by the row keys that you define. You must define a date range of one or more days. When date is one of the group by values, any days without data are omitted from the result list. If you need to know which days have data, issue a broad date range query grouped by date for any metric, and see which day rows are returned.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_state` | String |  | The data state to be fetched, can be full or all, the latter including full and partial data. |
| `start_date` | String |  |  [Required] Start date of the requested date range, in YYYY-MM-DD format, in PST time (UTC - 8:00). Must be less than or equal to the end date. This value is included in the range. |
| `dimension_filter_groups` | Vec<String> |  | [Optional] Zero or more filters to apply to the dimension grouping values; for example, 'query contains \"buy\"' to see only data where the query string contains the substring \"buy\" (not case-sensitive). You can filter by a dimension without grouping by it. |
| `type` | String |  | Optional. [Optional; Default is \"web\"] Type of report: search type, or either Discover or Gnews. |
| `aggregation_type` | String |  | [Optional; Default is \"auto\"] How data is aggregated. If aggregated by property, all data for the same property is aggregated; if aggregated by page, all data is aggregated by canonical URI. If you filter or group by page, choose AUTO; otherwise you can aggregate either by property or by page, depending on how you want your data calculated; see the help documentation to learn how data is calculated differently by site versus by page. **Note:** If you group or filter by page, you cannot aggregate by property. If you specify any value other than AUTO, the aggregation type in the result will match the requested type, or if you request an invalid type, you will get an error. The API will never change your aggregation type if the requested type is invalid. |
| `dimensions` | Vec<String> |  | [Optional] Zero or more dimensions to group results by. Dimensions are the group-by values in the Search Analytics page. Dimensions are combined to create a unique row key for each row. Results are grouped in the order that you supply these dimensions. |
| `end_date` | String |  | [Required] End date of the requested date range, in YYYY-MM-DD format, in PST (UTC - 8:00). Must be greater than or equal to the start date. This value is included in the range. |
| `row_limit` | i64 |  | [Optional; Default is 1000] The maximum number of rows to return. Must be a number from 1 to 25,000 (inclusive). |
| `start_row` | i64 |  | [Optional; Default is 0] Zero-based index of the first row in the response. Must be a non-negative number. |
| `search_type` | String |  | [Optional; Default is \"web\"] The search type to filter for. |
| `site_url` | String | ✅ | The site's URL, including protocol. For example: `http://www.example.com/`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create searchanalytic
searchanalytic = provider.searchconsole_api.Searchanalytic {
    site_url = "value"  # The site's URL, including protocol. For example: `http://www.example.com/`.
}

```

---


### Index

Index inspection.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language_code` | String |  | Optional. An [IETF BCP-47](https://en.wikipedia.org/wiki/IETF_language_tag) language code representing the requested language for translated issue messages, e.g. "en-US", "or "de-CH". Default value is "en-US". |
| `site_url` | String |  | Required. The URL of the property as defined in Search Console. **Examples:** `http://www.example.com/` for a URL-prefix property, or `sc-domain:example.com` for a Domain property. |
| `inspection_url` | String |  | Required. URL to inspect. Must be under the property specified in "site_url". |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create index
index = provider.searchconsole_api.Index {
}

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
| `permission_level` | String | The user's permission level for the site. |
| `site_url` | String | The URL of the site. |


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
site_permission_level = site.permission_level
site_site_url = site.site_url
```

---


### Mobile_friendly_test

Runs Mobile-Friendly Test for a given URL.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | String |  | URL for inspection. |
| `request_screenshot` | bool |  | Whether or not screenshot is requested. Default is false. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mobile_friendly_test
mobile_friendly_test = provider.searchconsole_api.Mobile_friendly_test {
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

# Create multiple sitemap resources
sitemap_0 = provider.searchconsole_api.Sitemap {
    feedpath = "value-0"
    site_url = "value-0"
}
sitemap_1 = provider.searchconsole_api.Sitemap {
    feedpath = "value-1"
    site_url = "value-1"
}
sitemap_2 = provider.searchconsole_api.Sitemap {
    feedpath = "value-2"
    site_url = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    sitemap = provider.searchconsole_api.Sitemap {
        feedpath = "production-value"
        site_url = "production-value"
    }
```

---

## Related Documentation

- [GCP Searchconsole_api Documentation](https://cloud.google.com/searchconsole_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
