# Mybusinessbusinessinformation_api Service



**Resources**: 5

---

## Overview

The mybusinessbusinessinformation_api service provides access to 5 resource types:

- [Google_location](#google_location) [C]
- [Categorie](#categorie) [R]
- [Chain](#chain) [R]
- [Attribute](#attribute) [R]
- [Location](#location) [CRUD]

---

## Resources


### Google_location

Search all of the possible locations that are a match to the specified request.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location` | String |  | Location to search for. If provided, will find locations which match the provided location details, which must include a value for the title. |
| `page_size` | i64 |  | The number of matches to return. The default value is 3, with a maximum of 10. Note that latency may increase if more are requested. There is no pagination. |
| `query` | String |  | Text query to search for. The search results from a query string will be less accurate than if providing an exact location, but can provide more inexact matches. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create google_location
google_location = provider.mybusinessbusinessinformation_api.Google_location {
}

```

---


### Categorie

Returns a list of business categories. Search will match the category name but not the category ID. Search only matches the front of a category name (that is, 'food' may return 'Food Court' but not 'Fast Food Restaurant').

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `categories` | Vec<String> | The matching categories based on the requested parameters. |
| `next_page_token` | String | If the number of categories exceeded the requested page size, this field will be populated with a token to fetch the next page of categories on a subsequent call to `ListCategories`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access categorie outputs
categorie_id = categorie.id
categorie_categories = categorie.categories
categorie_next_page_token = categorie.next_page_token
```

---


### Chain

Gets the specified chain. Returns `NOT_FOUND` if the chain does not exist.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `websites` | Vec<String> | Websites of the chain. |
| `chain_names` | Vec<String> | Names of the chain. |
| `name` | String | Required. The chain's resource name, in the format `chains/{chain_id}`. |
| `location_count` | i64 | Number of locations that are part of this chain. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access chain outputs
chain_id = chain.id
chain_websites = chain.websites
chain_chain_names = chain.chain_names
chain_name = chain.name
chain_location_count = chain.location_count
```

---


### Attribute

Returns the list of attributes that would be available for a location with the given primary category and country.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If the number of attributes exceeded the requested page size, this field will be populated with a token to fetch the next page of attributes on a subsequent call to `attributes.list`. If there are no more attributes, this field will not be present in the response. |
| `attribute_metadata` | Vec<String> | A collection of attribute metadata for the available attributes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access attribute outputs
attribute_id = attribute.id
attribute_next_page_token = attribute.next_page_token
attribute_attribute_metadata = attribute.attribute_metadata
```

---


### Location

Creates a new Location that will be owned by the logged in user.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `profile` | String |  | Optional. Describes your business in your own voice and shares with users the unique story of your business and offerings. This field is required for all categories except lodging categories (e.g. hotels, motels, inns). |
| `store_code` | String |  | Optional. External identifier for this location, which must be unique within a given account. This is a means of associating the location with your own records. |
| `website_uri` | String |  | Optional. A URL for this business. If possible, use a URL that represents this individual business location instead of a generic website/URL that represents all locations, or the brand. |
| `special_hours` | String |  | Optional. Special hours for the business. This typically includes holiday hours, and other times outside of regular operating hours. These override regular business hours. This field cannot be set without regular hours. |
| `metadata` | String |  | Output only. Additional non-user-editable information. |
| `open_info` | String |  | Optional. A flag that indicates whether the location is currently open for business. |
| `more_hours` | Vec<String> |  | Optional. More hours for a business's different departments or specific customers. |
| `ad_words_location_extensions` | String |  | Optional. Additional information that is surfaced in AdWords. |
| `service_area` | String |  | Optional. Service area businesses provide their service at the customer's location. If this business is a service area business, this field describes the area(s) serviced by the business. |
| `labels` | Vec<String> |  | Optional. A collection of free-form strings to allow you to tag your business. These labels are NOT user facing; only you can see them. Must be between 1-255 characters per label. |
| `categories` | String |  | Optional. The different categories that describe the business. |
| `regular_hours` | String |  | Optional. Operating hours for the business. |
| `relationship_data` | String |  | Optional. All locations and chain related to this one. |
| `latlng` | String |  | Optional. User-provided latitude and longitude. When creating a location, this field is ignored if the provided address geocodes successfully. This field is only returned on get requests if the user-provided `latlng` value was accepted during create, or the `latlng` value was updated through the Google Business Profile website. This field can only be updated by approved clients. |
| `service_items` | Vec<String> |  | Optional. List of services supported by merchants. A service can be haircut, install water heater, etc. Duplicated service items will be removed automatically. |
| `title` | String |  | Required. Location name should reflect your business's real-world name, as used consistently on your storefront, website, and stationery, and as known to customers. Any additional information, when relevant, can be included in other fields of the resource (for example, `Address`, `Categories`). Don't add unnecessary information to your name (for example, prefer "Google" over "Google Inc. - Mountain View Corporate Headquarters"). Don't include marketing taglines, store codes, special characters, hours or closed/open status, phone numbers, website URLs, service/product information, location/address or directions, or containment information (for example, "Chase ATM in Duane Reade"). |
| `language_code` | String |  | Immutable. The language of the location. Set during creation and not updateable. |
| `phone_numbers` | String |  | Optional. The different phone numbers that customers can use to get in touch with the business. |
| `name` | String |  | Google identifier for this location in the form: `locations/{location_id}`. |
| `storefront_address` | String |  | Optional. A precise, accurate address to describe your business location. PO boxes or mailboxes located at remote locations are not acceptable. At this time, you can specify a maximum of five `address_lines` values in the address. This field should only be set for businesses that have a storefront. This field should not be set for locations of type `CUSTOMER_LOCATION_ONLY` but if set, any value provided will be discarded. |
| `parent` | String | ✅ | Required. The name of the account in which to create this location. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profile` | String | Optional. Describes your business in your own voice and shares with users the unique story of your business and offerings. This field is required for all categories except lodging categories (e.g. hotels, motels, inns). |
| `store_code` | String | Optional. External identifier for this location, which must be unique within a given account. This is a means of associating the location with your own records. |
| `website_uri` | String | Optional. A URL for this business. If possible, use a URL that represents this individual business location instead of a generic website/URL that represents all locations, or the brand. |
| `special_hours` | String | Optional. Special hours for the business. This typically includes holiday hours, and other times outside of regular operating hours. These override regular business hours. This field cannot be set without regular hours. |
| `metadata` | String | Output only. Additional non-user-editable information. |
| `open_info` | String | Optional. A flag that indicates whether the location is currently open for business. |
| `more_hours` | Vec<String> | Optional. More hours for a business's different departments or specific customers. |
| `ad_words_location_extensions` | String | Optional. Additional information that is surfaced in AdWords. |
| `service_area` | String | Optional. Service area businesses provide their service at the customer's location. If this business is a service area business, this field describes the area(s) serviced by the business. |
| `labels` | Vec<String> | Optional. A collection of free-form strings to allow you to tag your business. These labels are NOT user facing; only you can see them. Must be between 1-255 characters per label. |
| `categories` | String | Optional. The different categories that describe the business. |
| `regular_hours` | String | Optional. Operating hours for the business. |
| `relationship_data` | String | Optional. All locations and chain related to this one. |
| `latlng` | String | Optional. User-provided latitude and longitude. When creating a location, this field is ignored if the provided address geocodes successfully. This field is only returned on get requests if the user-provided `latlng` value was accepted during create, or the `latlng` value was updated through the Google Business Profile website. This field can only be updated by approved clients. |
| `service_items` | Vec<String> | Optional. List of services supported by merchants. A service can be haircut, install water heater, etc. Duplicated service items will be removed automatically. |
| `title` | String | Required. Location name should reflect your business's real-world name, as used consistently on your storefront, website, and stationery, and as known to customers. Any additional information, when relevant, can be included in other fields of the resource (for example, `Address`, `Categories`). Don't add unnecessary information to your name (for example, prefer "Google" over "Google Inc. - Mountain View Corporate Headquarters"). Don't include marketing taglines, store codes, special characters, hours or closed/open status, phone numbers, website URLs, service/product information, location/address or directions, or containment information (for example, "Chase ATM in Duane Reade"). |
| `language_code` | String | Immutable. The language of the location. Set during creation and not updateable. |
| `phone_numbers` | String | Optional. The different phone numbers that customers can use to get in touch with the business. |
| `name` | String | Google identifier for this location in the form: `locations/{location_id}`. |
| `storefront_address` | String | Optional. A precise, accurate address to describe your business location. PO boxes or mailboxes located at remote locations are not acceptable. At this time, you can specify a maximum of five `address_lines` values in the address. This field should only be set for businesses that have a storefront. This field should not be set for locations of type `CUSTOMER_LOCATION_ONLY` but if set, any value provided will be discarded. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.mybusinessbusinessinformation_api.Location {
    parent = "value"  # Required. The name of the account in which to create this location.
}

# Access location outputs
location_id = location.id
location_profile = location.profile
location_store_code = location.store_code
location_website_uri = location.website_uri
location_special_hours = location.special_hours
location_metadata = location.metadata
location_open_info = location.open_info
location_more_hours = location.more_hours
location_ad_words_location_extensions = location.ad_words_location_extensions
location_service_area = location.service_area
location_labels = location.labels
location_categories = location.categories
location_regular_hours = location.regular_hours
location_relationship_data = location.relationship_data
location_latlng = location.latlng
location_service_items = location.service_items
location_title = location.title
location_language_code = location.language_code
location_phone_numbers = location.phone_numbers
location_name = location.name
location_storefront_address = location.storefront_address
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple google_location resources
google_location_0 = provider.mybusinessbusinessinformation_api.Google_location {
}
google_location_1 = provider.mybusinessbusinessinformation_api.Google_location {
}
google_location_2 = provider.mybusinessbusinessinformation_api.Google_location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    google_location = provider.mybusinessbusinessinformation_api.Google_location {
    }
```

---

## Related Documentation

- [GCP Mybusinessbusinessinformation_api Documentation](https://cloud.google.com/mybusinessbusinessinformation_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
