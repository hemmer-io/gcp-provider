# Vectortile_api Service



**Resources**: 2

---

## Overview

The vectortile_api service provides access to 2 resource types:

- [Terraintile](#terraintile) [R]
- [Featuretile](#featuretile) [R]

---

## Resources


### Terraintile

Gets a terrain tile by its tile resource name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `second_derivative` | String | Terrain elevation data encoded as a SecondDerivativeElevationGrid. cs/symbol:SecondDerivativeElevationGrid. See go/byte-encoded-terrain for more details. |
| `first_derivative` | String | Terrain elevation data encoded as a FirstDerivativeElevationGrid. cs/symbol:FirstDerivativeElevationGrid. |
| `coordinates` | String | The global tile coordinates that uniquely identify this tile. |
| `name` | String | Resource name of the tile. The tile resource name is prefixed by its collection ID `terrain/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `terrain/@1,2,3z`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access terraintile outputs
terraintile_id = terraintile.id
terraintile_second_derivative = terraintile.second_derivative
terraintile_first_derivative = terraintile.first_derivative
terraintile_coordinates = terraintile.coordinates
terraintile_name = terraintile.name
```

---


### Featuretile

Gets a feature tile by its tile resource name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `coordinates` | String | The global tile coordinates that uniquely identify this tile. |
| `features` | Vec<String> | Features present on this map tile. |
| `version_id` | String | An opaque value, usually less than 30 characters, that contains version info about this tile and the data that was used to generate it. The client should store this value in its tile cache and pass it back to the API in the client_tile_version_id field of subsequent tile requests in order to enable the API to detect when the new tile would be the same as the one the client already has in its cache. Also see STATUS_OK_DATA_UNCHANGED. |
| `providers` | Vec<String> | Data providers for the data contained in this tile. |
| `status` | String | Tile response status code to support tile caching. |
| `name` | String | Resource name of the tile. The tile resource name is prefixed by its collection ID `tiles/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `tiles/@1,2,3z`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access featuretile outputs
featuretile_id = featuretile.id
featuretile_coordinates = featuretile.coordinates
featuretile_features = featuretile.features
featuretile_version_id = featuretile.version_id
featuretile_providers = featuretile.providers
featuretile_status = featuretile.status
featuretile_name = featuretile.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple terraintile resources
terraintile_0 = provider.vectortile_api.Terraintile {
}
terraintile_1 = provider.vectortile_api.Terraintile {
}
terraintile_2 = provider.vectortile_api.Terraintile {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    terraintile = provider.vectortile_api.Terraintile {
    }
```

---

## Related Documentation

- [GCP Vectortile_api Documentation](https://cloud.google.com/vectortile_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
