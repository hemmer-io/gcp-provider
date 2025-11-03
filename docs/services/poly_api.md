# Poly_api Service



**Resources**: 2

---

## Overview

The poly_api service provides access to 2 resource types:

- [Asset](#asset) [R]
- [Likedasset](#likedasset) [R]

---

## Resources


### Asset

Returns detailed information about an asset given its name. PRIVATE assets are returned only if the currently authenticated user (via OAuth token) is the author of the asset.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `remix_info` | String | The remix info for the asset. |
| `thumbnail` | String | The thumbnail image for the asset. |
| `update_time` | String | The time when the asset was last modified. For published assets, whose contents are immutable, the update time changes only when metadata properties, such as visibility, are updated. |
| `visibility` | String | The visibility of the asset and who can access it. |
| `create_time` | String | For published assets, the time when the asset was published. For unpublished assets, the time when the asset was created. |
| `is_curated` | bool | Whether this asset has been curated by the Poly team. |
| `author_name` | String | The author's publicly visible name. Use this name when giving credit to the author. For more information, see [Licensing](/poly/discover/licensing). |
| `display_name` | String | The human-readable name, set by the asset's author. |
| `license` | String | The license under which the author has made the asset available for use, if any. |
| `metadata` | String | Application-defined opaque metadata for this asset. This field is only returned when querying for the signed-in user's own assets, not for public assets. This string is limited to 1K chars. It is up to the creator of the asset to define the format for this string (for example, JSON). |
| `name` | String | The unique identifier for the asset in the form: `assets/{ASSET_ID}`. |
| `presentation_params` | String | Hints for displaying the asset. Note that these parameters are not immutable; the author of an asset may change them post-publication. |
| `formats` | Vec<String> | A list of Formats where each format describes one representation of the asset. |
| `description` | String | The human-readable description, set by the asset's author. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access asset outputs
asset_id = asset.id
asset_remix_info = asset.remix_info
asset_thumbnail = asset.thumbnail
asset_update_time = asset.update_time
asset_visibility = asset.visibility
asset_create_time = asset.create_time
asset_is_curated = asset.is_curated
asset_author_name = asset.author_name
asset_display_name = asset.display_name
asset_license = asset.license
asset_metadata = asset.metadata
asset_name = asset.name
asset_presentation_params = asset.presentation_params
asset_formats = asset.formats
asset_description = asset.description
```

---


### Likedasset

Lists assets that the user has liked. Only the value 'me', representing the currently-authenticated user, is supported. May include assets with an access level of UNLISTED.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assets` | Vec<String> | A list of assets that match the criteria specified in the request. |
| `next_page_token` | String | The continuation token for retrieving the next page. If empty, indicates that there are no more pages. To get the next page, submit the same request specifying this value as the page_token. |
| `total_size` | i64 | The total number of assets in the list, without pagination. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access likedasset outputs
likedasset_id = likedasset.id
likedasset_assets = likedasset.assets
likedasset_next_page_token = likedasset.next_page_token
likedasset_total_size = likedasset.total_size
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple asset resources
asset_0 = provider.poly_api.Asset {
}
asset_1 = provider.poly_api.Asset {
}
asset_2 = provider.poly_api.Asset {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    asset = provider.poly_api.Asset {
    }
```

---

## Related Documentation

- [GCP Poly_api Documentation](https://cloud.google.com/poly_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
