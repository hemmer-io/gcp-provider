# Pollen_api Service



**Resources**: 2

---

## Overview

The pollen_api service provides access to 2 resource types:

- [Forecast](#forecast) [R]
- [Heatmap_tile](#heatmap_tile) [R]

---

## Resources


### Forecast

Returns up to 5 days of daily pollen information in more than 65 countries, up to 1km resolution.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `daily_info` | Vec<String> | Required. This object contains the daily forecast information for each day requested. |
| `region_code` | String | The ISO_3166-1 alpha-2 code of the country/region corresponding to the location provided in the request. This field might be omitted from the response if the location provided in the request resides in a disputed territory. |
| `next_page_token` | String | Optional. The token to retrieve the next page. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access forecast outputs
forecast_id = forecast.id
forecast_daily_info = forecast.daily_info
forecast_region_code = forecast.region_code
forecast_next_page_token = forecast.next_page_token
```

---


### Heatmap_tile

Returns a byte array containing the data of the tile PNG image.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `data` | String | The HTTP request/response body as raw binary. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access heatmap_tile outputs
heatmap_tile_id = heatmap_tile.id
heatmap_tile_extensions = heatmap_tile.extensions
heatmap_tile_content_type = heatmap_tile.content_type
heatmap_tile_data = heatmap_tile.data
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple forecast resources
forecast_0 = provider.pollen_api.Forecast {
}
forecast_1 = provider.pollen_api.Forecast {
}
forecast_2 = provider.pollen_api.Forecast {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    forecast = provider.pollen_api.Forecast {
    }
```

---

## Related Documentation

- [GCP Pollen_api Documentation](https://cloud.google.com/pollen_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
