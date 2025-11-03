# Airquality_api Service



**Resources**: 4

---

## Overview

The airquality_api service provides access to 4 resource types:

- [Heatmap_tile](#heatmap_tile) [R]
- [Current_condition](#current_condition) [C]
- [History](#history) [C]
- [Forecast](#forecast) [C]

---

## Resources


### Heatmap_tile

Returns a bytes array containing the data of the tile PNG image.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |


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
heatmap_tile_data = heatmap_tile.data
heatmap_tile_content_type = heatmap_tile.content_type
```

---


### Current_condition

The Current Conditions endpoint provides hourly air quality information in more than 100 countries, up to a 500 x 500 meters resolution. Includes over 70 local indexes and global air quality index and categories.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location` | String |  | Required. The longitude and latitude from which the API looks for air quality current conditions data. |
| `uaqi_color_palette` | String |  | Optional. Determines the color palette used for data provided by the 'Universal Air Quality Index' (UAQI). This color palette is relevant just for UAQI, other AQIs have a predetermined color palette that can't be controlled. |
| `language_code` | String |  | Optional. Allows the client to choose the language for the response. If data cannot be provided for that language the API uses the closest match. Allowed values rely on the IETF standard. Default value is en. |
| `custom_local_aqis` | Vec<String> |  | Optional. Expresses a 'country/region to AQI' relationship. Pairs a country/region with a desired AQI so that air quality data that is required for that country/region will be displayed according to the chosen AQI. This parameter can be used to specify a non-default AQI for a given country, for example, to get the US EPA index for Canada rather than the default index for Canada. |
| `extra_computations` | Vec<String> |  | Optional. Additional features that can be optionally enabled. Specifying extra computations will result in the relevant elements and fields to be returned in the response. |
| `universal_aqi` | bool |  | Optional. If set to true, the Universal AQI will be included in the 'indexes' field of the response. Default value is true. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create current_condition
current_condition = provider.airquality_api.Current_condition {
}

```

---


### History

Returns air quality history for a specific location for a given time range.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uaqi_color_palette` | String |  | Optional. Determines the color palette used for data provided by the 'Universal Air Quality Index' (UAQI). This color palette is relevant just for UAQI, other AQIs have a predetermined color palette that can't be controlled. |
| `custom_local_aqis` | Vec<String> |  | Optional. Expresses a 'country/region to AQI' relationship. Pairs a country/region with a desired AQI so that air quality data that is required for that country/region will be displayed according to the chosen AQI. This parameter can be used to specify a non-default AQI for a given country, for example, to get the US EPA index for Canada rather than the default index for Canada. |
| `page_token` | String |  | Optional. A page token received from a previous history call. It is used to retrieve the subsequent page. Note that when providing a value for this parameter all other parameters provided must match the call that provided the page token (the previous call). |
| `period` | String |  | Indicates the start and end period for which to get the historical data. The timestamp is rounded to the previous exact hour. |
| `language_code` | String |  | Optional. Allows the client to choose the language for the response. If data cannot be provided for that language the API uses the closest match. Allowed values rely on the IETF standard. Default value is en. |
| `universal_aqi` | bool |  | Optional. If set to true, the Universal AQI will be included in the 'indexes' field of the response. Default value is true. |
| `hours` | i64 |  | Number from 1 to 720 that indicates the hours range for the request. For example: A value of 48 will yield data from the last 48 hours. |
| `date_time` | String |  | A timestamp for which to return historical data. The timestamp is rounded to the previous exact hour. Note: this will return hourly data for the requested timestamp only (i.e. a single hourly info element). For example, a request sent where the dateTime parameter is set to 2023-01-03T11:05:49Z will be rounded down to 2023-01-03T11:00:00Z. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z". |
| `location` | String |  | Required. The latitude and longitude for which the API looks for air quality history data. |
| `extra_computations` | Vec<String> |  | Optional. Additional features that can be optionally enabled. Specifying extra computations will result in the relevant elements and fields to be returned in the response. |
| `page_size` | i64 |  | Optional. The maximum number of hourly info records to return per page. The default is 72 and the max value is 168 (7 days of data). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create history
history = provider.airquality_api.History {
}

```

---


### Forecast

Returns air quality forecast for a specific location for a given time range.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uaqi_color_palette` | String |  | Optional. Determines the color palette used for data provided by the 'Universal Air Quality Index' (UAQI). This color palette is relevant just for UAQI, other AQIs have a predetermined color palette that can't be controlled. |
| `location` | String |  | Required. The latitude and longitude for which the API looks for air quality data. |
| `custom_local_aqis` | Vec<String> |  | Optional. Expresses a 'country/region to AQI' relationship. Pairs a country/region with a desired AQI so that air quality data that is required for that country/region will be displayed according to the chosen AQI. This parameter can be used to specify a non-default AQI for a given country, for example, to get the US EPA index for Canada rather than the default index for Canada. |
| `extra_computations` | Vec<String> |  | Optional. Additional features that can be optionally enabled. Specifying extra computations will result in the relevant elements and fields to be returned in the response. |
| `date_time` | String |  | A timestamp for which to return the data for a specific point in time. The timestamp is rounded to the previous exact hour. Note: this will return hourly data for the requested timestamp only (i.e. a single hourly info element). For example, a request sent where the date_time parameter is set to 2023-01-03T11:05:49Z will be rounded down to 2023-01-03T11:00:00Z. |
| `language_code` | String |  | Optional. Allows the client to choose the language for the response. If data cannot be provided for that language the API uses the closest match. Allowed values rely on the IETF standard (default = 'en'). |
| `page_size` | i64 |  | Optional. The maximum number of hourly info records to return per page (default = 24). |
| `universal_aqi` | bool |  | Optional. If set to true, the Universal AQI will be included in the 'indexes' field of the response (default = true). |
| `period` | String |  | Indicates the start and end period for which to get the forecast data. The timestamp is rounded to the previous exact hour. |
| `page_token` | String |  | Optional. A page token received from a previous forecast call. It is used to retrieve the subsequent page. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create forecast
forecast = provider.airquality_api.Forecast {
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

# Create multiple heatmap_tile resources
heatmap_tile_0 = provider.airquality_api.Heatmap_tile {
}
heatmap_tile_1 = provider.airquality_api.Heatmap_tile {
}
heatmap_tile_2 = provider.airquality_api.Heatmap_tile {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    heatmap_tile = provider.airquality_api.Heatmap_tile {
    }
```

---

## Related Documentation

- [GCP Airquality_api Documentation](https://cloud.google.com/airquality_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
