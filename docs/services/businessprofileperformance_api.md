# Businessprofileperformance_api Service



**Resources**: 2

---

## Overview

The businessprofileperformance_api service provides access to 2 resource types:

- [Monthly](#monthly) [R]
- [Location](#location) [R]

---

## Resources


### Monthly

Returns the search keywords used to find a business in search or maps. Each search keyword is accompanied by impressions which are aggregated on a monthly basis. Example request: `GET https://businessprofileperformance.googleapis.com/v1/locations/12345/searchkeywords/impressions/monthly?monthly_range.start_month.year=2022&monthly_range.start_month.month=1&monthly_range.end_month.year=2022&monthly_range.end_month.month=3`

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `search_keywords_counts` | Vec<String> | Search terms which have been used to find a business. |
| `next_page_token` | String | A token indicating the last paginated result returned. This can be used by succeeding requests to get the next "page" of keywords. It will only be present when there are more results to be returned. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access monthly outputs
monthly_id = monthly.id
monthly_search_keywords_counts = monthly.search_keywords_counts
monthly_next_page_token = monthly.next_page_token
```

---


### Location

 Returns the values for each date from a given time range and optionally the sub entity type, where applicable, that are associated with the specific daily metrics. Example request: `GET https://businessprofileperformance.googleapis.com/v1/locations/12345:fetchMultiDailyMetricsTimeSeries?dailyMetrics=WEBSITE_CLICKS&dailyMetrics=CALL_CLICKS&daily_range.start_date.year=2022&daily_range.start_date.month=1&daily_range.start_date.day=1&daily_range.end_date.year=2022&daily_range.end_date.month=3&daily_range.end_date.day=31`

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `multi_daily_metric_time_series` | Vec<String> | DailyMetrics and their corresponding time series. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_multi_daily_metric_time_series = location.multi_daily_metric_time_series
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple monthly resources
monthly_0 = provider.businessprofileperformance_api.Monthly {
}
monthly_1 = provider.businessprofileperformance_api.Monthly {
}
monthly_2 = provider.businessprofileperformance_api.Monthly {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    monthly = provider.businessprofileperformance_api.Monthly {
    }
```

---

## Related Documentation

- [GCP Businessprofileperformance_api Documentation](https://cloud.google.com/businessprofileperformance_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
