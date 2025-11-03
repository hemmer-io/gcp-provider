# Solar_api Service



**Resources**: 3

---

## Overview

The solar_api service provides access to 3 resource types:

- [Building_insight](#building_insight) [R]
- [Data_layer](#data_layer) [R]
- [Geo_tiff](#geo_tiff) [R]

---

## Resources


### Building_insight

Locates the building whose centroid is closest to a query point. Returns an error with code `NOT_FOUND` if there are no buildings within approximately 50m of the query point.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `imagery_date` | String | Date that the underlying imagery was acquired. This is approximate. |
| `center` | String | A point near the center of the building. |
| `name` | String | The resource name for the building, of the format `buildings/{place_id}`. |
| `administrative_area` | String | Administrative area 1 (e.g., in the US, the state) that contains this building. For example, in the US, the abbreviation might be "MA" or "CA." |
| `region_code` | String | Region code for the country (or region) this building is in. |
| `postal_code` | String | Postal code (e.g., US zip code) this building is contained by. |
| `imagery_processed_date` | String | When processing was completed on this imagery. |
| `imagery_quality` | String | The quality of the imagery used to compute the data for this building. |
| `bounding_box` | String | The bounding box of the building. |
| `statistical_area` | String | Statistical area (e.g., US census tract) this building is in. |
| `solar_potential` | String | Solar potential of the building. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access building_insight outputs
building_insight_id = building_insight.id
building_insight_imagery_date = building_insight.imagery_date
building_insight_center = building_insight.center
building_insight_name = building_insight.name
building_insight_administrative_area = building_insight.administrative_area
building_insight_region_code = building_insight.region_code
building_insight_postal_code = building_insight.postal_code
building_insight_imagery_processed_date = building_insight.imagery_processed_date
building_insight_imagery_quality = building_insight.imagery_quality
building_insight_bounding_box = building_insight.bounding_box
building_insight_statistical_area = building_insight.statistical_area
building_insight_solar_potential = building_insight.solar_potential
```

---


### Data_layer

Gets solar information for a region surrounding a location. Returns an error with code `NOT_FOUND` if the location is outside the coverage area.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `imagery_processed_date` | String | When processing was completed on this imagery. |
| `rgb_url` | String | The URL for an image of RGB data (aerial photo) of the region. |
| `hourly_shade_urls` | Vec<String> | Twelve URLs for hourly shade, corresponding to January...December, in order. Each GeoTIFF will contain 24 bands, corresponding to the 24 hours of the day. Each pixel is a 32 bit integer, corresponding to the (up to) 31 days of that month; a 1 bit means that the corresponding location is able to see the sun at that day, of that hour, of that month. Invalid locations are stored as -9999 (since this is negative, it has bit 31 set, and no valid value could have bit 31 set as that would correspond to the 32nd day of the month). An example may be useful. If you want to know whether a point (at pixel location (x, y)) saw sun at 4pm on the 22nd of June you would: 1. fetch the sixth URL in this list (corresponding to June). 1. look up the 17th channel (corresponding to 4pm). 1. read the 32-bit value at (x, y). 1. read bit 21 of the value (corresponding to the 22nd of the month). 1. if that bit is a 1, then that spot saw the sun at 4pm 22 June. More formally: Given `month` (1-12), `day` (1...month max; February has 28 days) and `hour` (0-23), the shade/sun for that month/day/hour at a position `(x, y)` is the bit ``` (hourly_shade[month - 1])(x, y)[hour] & (1 << (day - 1)) ``` where `(x, y)` is spatial indexing, `[month - 1]` refers to fetching the `month - 1`st URL (indexing from zero), `[hour]` is indexing into the channels, and a final non-zero result means "sunny". There are no leap days, and DST doesn't exist (all days are 24 hours long; noon is always "standard time" noon). |
| `imagery_quality` | String | The quality of the result's imagery. |
| `annual_flux_url` | String | The URL for the annual flux map (annual sunlight on roofs) of the region. Values are kWh/kW/year. This is *unmasked flux*: flux is computed for every location, not just building rooftops. Invalid locations are stored as -9999: locations outside our coverage area will be invalid, and a few locations inside the coverage area, where we were unable to calculate flux, will also be invalid. |
| `dsm_url` | String | The URL for an image of the DSM (Digital Surface Model) of the region. Values are in meters above EGM96 geoid (i.e., sea level). Invalid locations (where we don't have data) are stored as -9999. |
| `monthly_flux_url` | String | The URL for the monthly flux map (sunlight on roofs, broken down by month) of the region. Values are kWh/kW/year. The GeoTIFF pointed to by this URL will contain twelve bands, corresponding to January...December, in order. |
| `mask_url` | String | The URL for the building mask image: one bit per pixel saying whether that pixel is considered to be part of a rooftop or not. |
| `imagery_date` | String | When the source imagery (from which all the other data are derived) in this region was taken. It is necessarily somewhat approximate, as the images may have been taken over more than one day. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access data_layer outputs
data_layer_id = data_layer.id
data_layer_imagery_processed_date = data_layer.imagery_processed_date
data_layer_rgb_url = data_layer.rgb_url
data_layer_hourly_shade_urls = data_layer.hourly_shade_urls
data_layer_imagery_quality = data_layer.imagery_quality
data_layer_annual_flux_url = data_layer.annual_flux_url
data_layer_dsm_url = data_layer.dsm_url
data_layer_monthly_flux_url = data_layer.monthly_flux_url
data_layer_mask_url = data_layer.mask_url
data_layer_imagery_date = data_layer.imagery_date
```

---


### Geo_tiff

Returns an image by its ID.

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

# Access geo_tiff outputs
geo_tiff_id = geo_tiff.id
geo_tiff_extensions = geo_tiff.extensions
geo_tiff_data = geo_tiff.data
geo_tiff_content_type = geo_tiff.content_type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple building_insight resources
building_insight_0 = provider.solar_api.Building_insight {
}
building_insight_1 = provider.solar_api.Building_insight {
}
building_insight_2 = provider.solar_api.Building_insight {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    building_insight = provider.solar_api.Building_insight {
    }
```

---

## Related Documentation

- [GCP Solar_api Documentation](https://cloud.google.com/solar_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
