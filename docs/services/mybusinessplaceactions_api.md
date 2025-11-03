# Mybusinessplaceactions_api Service



**Resources**: 2

---

## Overview

The mybusinessplaceactions_api service provides access to 2 resource types:

- [Place_action_link](#place_action_link) [CRUD]
- [Place_action_type_metadata](#place_action_type_metadata) [R]

---

## Resources


### Place_action_link

Creates a place action link associated with the specified location, and returns it. The request is considered duplicate if the `parent`, `place_action_link.uri` and `place_action_link.place_action_type` are the same as a previous request.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uri` | String |  | Required. The link uri. The same uri can be reused for different action types across different locations. However, only one place action link is allowed for each unique combination of (uri, place action type, location). |
| `name` | String |  | Optional. The resource name, in the format `locations/{location_id}/placeActionLinks/{place_action_link_id}`. The name field will only be considered in UpdatePlaceActionLink and DeletePlaceActionLink requests for updating and deleting links respectively. However, it will be ignored in CreatePlaceActionLink request, where `place_action_link_id` will be assigned by the server on successful creation of a new link and returned as part of the response. |
| `update_time` | String |  | Output only. The time when the place action link was last modified. |
| `provider_type` | String |  | Output only. Specifies the provider type. |
| `create_time` | String |  | Output only. The time when the place action link was created. |
| `is_preferred` | bool |  | Optional. Whether this link is preferred by the merchant. Only one link can be marked as preferred per place action type at a location. If a future request marks a different link as preferred for the same place action type, then the current preferred link (if any exists) will lose its preference. |
| `place_action_type` | String |  | Required. The type of place action that can be performed using this link. |
| `is_editable` | bool |  | Output only. Indicates whether this link can be edited by the client. |
| `parent` | String | ✅ | Required. The resource name of the location where to create this place action link. `locations/{location_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uri` | String | Required. The link uri. The same uri can be reused for different action types across different locations. However, only one place action link is allowed for each unique combination of (uri, place action type, location). |
| `name` | String | Optional. The resource name, in the format `locations/{location_id}/placeActionLinks/{place_action_link_id}`. The name field will only be considered in UpdatePlaceActionLink and DeletePlaceActionLink requests for updating and deleting links respectively. However, it will be ignored in CreatePlaceActionLink request, where `place_action_link_id` will be assigned by the server on successful creation of a new link and returned as part of the response. |
| `update_time` | String | Output only. The time when the place action link was last modified. |
| `provider_type` | String | Output only. Specifies the provider type. |
| `create_time` | String | Output only. The time when the place action link was created. |
| `is_preferred` | bool | Optional. Whether this link is preferred by the merchant. Only one link can be marked as preferred per place action type at a location. If a future request marks a different link as preferred for the same place action type, then the current preferred link (if any exists) will lose its preference. |
| `place_action_type` | String | Required. The type of place action that can be performed using this link. |
| `is_editable` | bool | Output only. Indicates whether this link can be edited by the client. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create place_action_link
place_action_link = provider.mybusinessplaceactions_api.Place_action_link {
    parent = "value"  # Required. The resource name of the location where to create this place action link. `locations/{location_id}`.
}

# Access place_action_link outputs
place_action_link_id = place_action_link.id
place_action_link_uri = place_action_link.uri
place_action_link_name = place_action_link.name
place_action_link_update_time = place_action_link.update_time
place_action_link_provider_type = place_action_link.provider_type
place_action_link_create_time = place_action_link.create_time
place_action_link_is_preferred = place_action_link.is_preferred
place_action_link_place_action_type = place_action_link.place_action_type
place_action_link_is_editable = place_action_link.is_editable
```

---


### Place_action_type_metadata

Returns the list of available place action types for a location or country.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If the number of action types exceeded the requested page size, this field will be populated with a token to fetch the next page on a subsequent call to `placeActionTypeMetadata.list`. If there are no more results, this field will not be present in the response. |
| `place_action_type_metadata` | Vec<String> | A collection of metadata for the available place action types. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access place_action_type_metadata outputs
place_action_type_metadata_id = place_action_type_metadata.id
place_action_type_metadata_next_page_token = place_action_type_metadata.next_page_token
place_action_type_metadata_place_action_type_metadata = place_action_type_metadata.place_action_type_metadata
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple place_action_link resources
place_action_link_0 = provider.mybusinessplaceactions_api.Place_action_link {
    parent = "value-0"
}
place_action_link_1 = provider.mybusinessplaceactions_api.Place_action_link {
    parent = "value-1"
}
place_action_link_2 = provider.mybusinessplaceactions_api.Place_action_link {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    place_action_link = provider.mybusinessplaceactions_api.Place_action_link {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Mybusinessplaceactions_api Documentation](https://cloud.google.com/mybusinessplaceactions_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
