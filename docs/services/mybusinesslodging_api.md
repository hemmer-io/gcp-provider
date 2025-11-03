# Mybusinesslodging_api Service



**Resources**: 2

---

## Overview

The mybusinesslodging_api service provides access to 2 resource types:

- [Location](#location) [RU]
- [Lodging](#lodging) [R]

---

## Resources


### Location

Returns the Lodging of a specific location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `health_and_safety` | String |  | Health and safety measures implemented by the hotel during COVID-19. |
| `pools` | String |  | Swimming pool or recreational water facilities available at the hotel. |
| `transportation` | String |  | Vehicles or vehicular services facilitated or owned by the property. |
| `sustainability` | String |  | Sustainability practices implemented at the hotel. |
| `common_living_area` | String |  | Features of the shared living areas available in this Lodging. |
| `business` | String |  | Features of the property of specific interest to the business traveler. |
| `all_units` | String |  | Output only. All units on the property have at least these attributes. |
| `accessibility` | String |  | Physical adaptations made to the property in consideration of varying levels of human physical ability. |
| `connectivity` | String |  | The ways in which the property provides guests with the ability to access the internet. |
| `services` | String |  | Conveniences or help provided by the property to facilitate an easier, more comfortable stay. |
| `some_units` | String |  | Output only. Some units on the property have as much as these attributes. |
| `name` | String |  | Required. Google identifier for this location in the form: `locations/{location_id}/lodging` |
| `activities` | String |  | Amenities and features related to leisure and play. |
| `pets` | String |  | Policies regarding guest-owned animals. |
| `wellness` | String |  | Guest facilities at the property to promote or maintain health, beauty, and fitness. |
| `parking` | String |  | Parking options at the property. |
| `metadata` | String |  | Required. Metadata for the lodging. |
| `food_and_drink` | String |  | Meals, snacks, and beverages available at the property. |
| `housekeeping` | String |  | Conveniences provided in guest units to facilitate an easier, more comfortable stay. |
| `guest_units` | Vec<String> |  | Individual GuestUnitTypes that are available in this Lodging. |
| `property` | String |  | General factual information about the property's physical structure and important dates. |
| `policies` | String |  | Property rules that impact guests. |
| `families` | String |  | Services and amenities for families and young guests. |
| `name` | String | ✅ | Required. Google identifier for this location in the form: `locations/{location_id}/lodging` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `health_and_safety` | String | Health and safety measures implemented by the hotel during COVID-19. |
| `pools` | String | Swimming pool or recreational water facilities available at the hotel. |
| `transportation` | String | Vehicles or vehicular services facilitated or owned by the property. |
| `sustainability` | String | Sustainability practices implemented at the hotel. |
| `common_living_area` | String | Features of the shared living areas available in this Lodging. |
| `business` | String | Features of the property of specific interest to the business traveler. |
| `all_units` | String | Output only. All units on the property have at least these attributes. |
| `accessibility` | String | Physical adaptations made to the property in consideration of varying levels of human physical ability. |
| `connectivity` | String | The ways in which the property provides guests with the ability to access the internet. |
| `services` | String | Conveniences or help provided by the property to facilitate an easier, more comfortable stay. |
| `some_units` | String | Output only. Some units on the property have as much as these attributes. |
| `name` | String | Required. Google identifier for this location in the form: `locations/{location_id}/lodging` |
| `activities` | String | Amenities and features related to leisure and play. |
| `pets` | String | Policies regarding guest-owned animals. |
| `wellness` | String | Guest facilities at the property to promote or maintain health, beauty, and fitness. |
| `parking` | String | Parking options at the property. |
| `metadata` | String | Required. Metadata for the lodging. |
| `food_and_drink` | String | Meals, snacks, and beverages available at the property. |
| `housekeeping` | String | Conveniences provided in guest units to facilitate an easier, more comfortable stay. |
| `guest_units` | Vec<String> | Individual GuestUnitTypes that are available in this Lodging. |
| `property` | String | General factual information about the property's physical structure and important dates. |
| `policies` | String | Property rules that impact guests. |
| `families` | String | Services and amenities for families and young guests. |


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
location_health_and_safety = location.health_and_safety
location_pools = location.pools
location_transportation = location.transportation
location_sustainability = location.sustainability
location_common_living_area = location.common_living_area
location_business = location.business
location_all_units = location.all_units
location_accessibility = location.accessibility
location_connectivity = location.connectivity
location_services = location.services
location_some_units = location.some_units
location_name = location.name
location_activities = location.activities
location_pets = location.pets
location_wellness = location.wellness
location_parking = location.parking
location_metadata = location.metadata
location_food_and_drink = location.food_and_drink
location_housekeeping = location.housekeeping
location_guest_units = location.guest_units
location_property = location.property
location_policies = location.policies
location_families = location.families
```

---


### Lodging

Returns the Google updated Lodging of a specific location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `diff_mask` | String | Required. The fields in the Lodging that have been updated by Google. Repeated field items are not individually specified. |
| `lodging` | String | Required. The Google updated Lodging. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access lodging outputs
lodging_id = lodging.id
lodging_diff_mask = lodging.diff_mask
lodging_lodging = lodging.lodging
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple location resources
location_0 = provider.mybusinesslodging_api.Location {
    name = "value-0"
}
location_1 = provider.mybusinesslodging_api.Location {
    name = "value-1"
}
location_2 = provider.mybusinesslodging_api.Location {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.mybusinesslodging_api.Location {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Mybusinesslodging_api Documentation](https://cloud.google.com/mybusinesslodging_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
