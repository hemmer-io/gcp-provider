# Cloudlocationfinder_api Service



**Resources**: 4

---

## Overview

The cloudlocationfinder_api service provides access to 4 resource types:

- [Cloud_location](#cloud_location) [R]
- [Location](#location) [R]
- [Location](#location) [R]
- [Cloud_location](#cloud_location) [R]

---

## Resources


### Cloud_location

Retrieves a resource containing information about a cloud location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_provider` | String | Optional. The provider of the cloud location. Values can be Google Cloud or third-party providers, including AWS, Azure, or Oracle Cloud Infrastructure. |
| `carbon_free_energy_percentage` | f64 | Optional. The carbon free energy percentage of the cloud location. This represents the average percentage of time customers' application will be running on carbon-free energy. See https://cloud.google.com/sustainability/region-carbon for more details. There is a difference between default value 0 and unset value. 0 means the carbon free energy percentage is 0%, while unset value means the carbon footprint data is not available. |
| `display_name` | String | Optional. The human-readable name of the cloud location. Example: us-east-2, us-east1. |
| `name` | String | Identifier. Name of the cloud location. Unique name of the cloud location including project and location using the form: `projects/{project_id}/locations/{location}/cloudLocations/{cloud_location}` |
| `territory_code` | String | Optional. The two-letter ISO 3166-1 alpha-2 code of the cloud location. Examples: US, JP, KR. |
| `containing_cloud_location` | String | Output only. The containing cloud location in the strict nesting hierarchy. For example, the containing cloud location of a zone is a region. |
| `cloud_location_type` | String | Optional. The type of the cloud location. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access cloud_location outputs
cloud_location_id = cloud_location.id
cloud_location_cloud_provider = cloud_location.cloud_provider
cloud_location_carbon_free_energy_percentage = cloud_location.carbon_free_energy_percentage
cloud_location_display_name = cloud_location.display_name
cloud_location_name = cloud_location.name
cloud_location_territory_code = cloud_location.territory_code
cloud_location_containing_cloud_location = cloud_location.containing_cloud_location
cloud_location_cloud_location_type = cloud_location.cloud_location_type
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


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
location_location_id = location.location_id
location_display_name = location.display_name
location_name = location.name
location_metadata = location.metadata
location_labels = location.labels
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |


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
location_labels = location.labels
location_name = location.name
location_location_id = location.location_id
location_display_name = location.display_name
location_metadata = location.metadata
```

---


### Cloud_location

Retrieves a resource containing information about a cloud location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `containing_cloud_location` | String | Output only. The containing cloud location in the strict nesting hierarchy. For example, the containing cloud location of a zone is a region. |
| `name` | String | Identifier. Name of the cloud location. Unique name of the cloud location including project and location using the form: `projects/{project_id}/locations/{location}/cloudLocations/{cloud_location}` |
| `display_name` | String | Optional. The human-readable name of the cloud location. Example: us-east-2, us-east1. |
| `carbon_free_energy_percentage` | f64 | Optional. The carbon free energy percentage of the cloud location. This represents the average percentage of time customers' application will be running on carbon-free energy. See https://cloud.google.com/sustainability/region-carbon for more details. There is a difference between default value 0 and unset value. 0 means the carbon free energy percentage is 0%, while unset value means the carbon footprint data is not available. |
| `cloud_location_type` | String | Optional. The type of the cloud location. |
| `territory_code` | String | Optional. The two-letter ISO 3166-1 alpha-2 code of the cloud location. Examples: US, JP, KR. |
| `cloud_provider` | String | Optional. The provider of the cloud location. Values can be Google Cloud or third-party providers, including AWS, Azure, or Oracle Cloud Infrastructure. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access cloud_location outputs
cloud_location_id = cloud_location.id
cloud_location_containing_cloud_location = cloud_location.containing_cloud_location
cloud_location_name = cloud_location.name
cloud_location_display_name = cloud_location.display_name
cloud_location_carbon_free_energy_percentage = cloud_location.carbon_free_energy_percentage
cloud_location_cloud_location_type = cloud_location.cloud_location_type
cloud_location_territory_code = cloud_location.territory_code
cloud_location_cloud_provider = cloud_location.cloud_provider
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple cloud_location resources
cloud_location_0 = provider.cloudlocationfinder_api.Cloud_location {
}
cloud_location_1 = provider.cloudlocationfinder_api.Cloud_location {
}
cloud_location_2 = provider.cloudlocationfinder_api.Cloud_location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    cloud_location = provider.cloudlocationfinder_api.Cloud_location {
    }
```

---

## Related Documentation

- [GCP Cloudlocationfinder_api Documentation](https://cloud.google.com/cloudlocationfinder_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
