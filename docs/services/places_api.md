# Places_api Service



**Resources**: 2

---

## Overview

The places_api service provides access to 2 resource types:

- [Place](#place) [CR]
- [Photo](#photo) [R]

---

## Resources


### Place

Returns predictions for the given input.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language_code` | String |  | Optional. The language in which to return results. Defaults to en-US. The results may be in mixed languages if the language used in `input` is different from `language_code` or if the returned Place does not have a translation from the local language to `language_code`. |
| `included_primary_types` | Vec<String> |  | Optional. Included primary Place type (for example, "restaurant" or "gas_station") in Place Types (https://developers.google.com/maps/documentation/places/web-service/place-types), or only `(regions)`, or only `(cities)`. A Place is only returned if its primary type is included in this list. Up to 5 values can be specified. If no types are specified, all Place types are returned. |
| `include_pure_service_area_businesses` | bool |  | Optional. Include pure service area businesses if the field is set to true. Pure service area business is a business that visits or delivers to customers directly but does not serve customers at their business address. For example, businesses like cleaning services or plumbers. Those businesses do not have a physical address or location on Google Maps. Places will not return fields including `location`, `plus_code`, and other location related fields for these businesses. |
| `include_query_predictions` | bool |  | Optional. If true, the response will include both Place and query predictions. Otherwise the response will only return Place predictions. |
| `input` | String |  | Required. The text string on which to search. |
| `included_region_codes` | Vec<String> |  | Optional. Only include results in the specified regions, specified as up to 15 CLDR two-character region codes. An empty set will not restrict the results. If both `location_restriction` and `included_region_codes` are set, the results will be located in the area of intersection. |
| `input_offset` | i64 |  | Optional. A zero-based Unicode character offset of `input` indicating the cursor position in `input`. The cursor position may influence what predictions are returned. If empty, defaults to the length of `input`. |
| `location_bias` | String |  | Optional. Bias results to a specified location. At most one of `location_bias` or `location_restriction` should be set. If neither are set, the results will be biased by IP address, meaning the IP address will be mapped to an imprecise location and used as a biasing signal. |
| `origin` | String |  | Optional. The origin point from which to calculate geodesic distance to the destination (returned as `distance_meters`). If this value is omitted, geodesic distance will not be returned. |
| `session_token` | String |  | Optional. A string which identifies an Autocomplete session for billing purposes. Must be a URL and filename safe base64 string with at most 36 ASCII characters in length. Otherwise an INVALID_ARGUMENT error is returned. The session begins when the user starts typing a query, and concludes when they select a place and a call to Place Details or Address Validation is made. Each session can have multiple queries, followed by one Place Details or Address Validation request. The credentials used for each request within a session must belong to the same Google Cloud Console project. Once a session has concluded, the token is no longer valid; your app must generate a fresh token for each session. If the `session_token` parameter is omitted, or if you reuse a session token, the session is charged as if no session token was provided (each request is billed separately). We recommend the following guidelines: * Use session tokens for all Place Autocomplete calls. * Generate a fresh token for each session. Using a version 4 UUID is recommended. * Ensure that the credentials used for all Place Autocomplete, Place Details, and Address Validation requests within a session belong to the same Cloud Console project. * Be sure to pass a unique session token for each new session. Using the same token for more than one session will result in each request being billed individually. |
| `location_restriction` | String |  | Optional. Restrict results to a specified location. At most one of `location_bias` or `location_restriction` should be set. If neither are set, the results will be biased by IP address, meaning the IP address will be mapped to an imprecise location and used as a biasing signal. |
| `region_code` | String |  | Optional. The region code, specified as a CLDR two-character region code. This affects address formatting, result ranking, and may influence what results are returned. This does not restrict results to the specified region. To restrict results to a region, use `region_code_restriction`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `consumer_alert` | String | The consumer alert message for the place when we detect suspicious review activity on a business or a business violates our policies. |
| `types` | Vec<String> | A set of type tags for this result. For example, "political" and "locality". For the complete list of possible values, see Table A and Table B at https://developers.google.com/maps/documentation/places/web-service/place-types |
| `display_name` | String | The localized name of the place, suitable as a short human-readable description. For example, "Google Sydney", "Starbucks", "Pyrmont", etc. |
| `good_for_watching_sports` | bool | Place is suitable for watching sports. |
| `primary_type_display_name` | String | The display name of the primary type, localized to the request language if applicable. For the complete list of possible values, see Table A and Table B at https://developers.google.com/maps/documentation/places/web-service/place-types. The primary type may be missing if the place's primary type is not a supported type. |
| `id` | String | The unique identifier of a place. |
| `google_maps_links` | String | Links to trigger different Google Maps actions. |
| `outdoor_seating` | bool | Place provides outdoor seating. |
| `review_summary` | String | AI-generated summary of the place using user reviews. |
| `serves_wine` | bool | Specifies if the place serves wine. |
| `takeout` | bool | Specifies if the business supports takeout. |
| `name` | String | This Place's resource name, in `places/{place_id}` format. Can be used to look up the Place. |
| `restroom` | bool | Place has restroom. |
| `serves_vegetarian_food` | bool | Specifies if the place serves vegetarian food. |
| `plus_code` | String | Plus code of the place location lat/long. |
| `moved_place_id` | String | If this Place is permanently closed and has moved to a new Place, this field contains the new Place's place ID. If this Place moved multiple times, this field will represent the first moved Place. This field will not be populated if this Place has not moved. |
| `serves_brunch` | bool | Specifies if the place serves brunch. |
| `adr_format_address` | String | The place's address in adr microformat: http://microformats.org/wiki/adr. |
| `website_uri` | String | The authoritative website for this place, e.g. a business' homepage. Note that for places that are part of a chain (e.g. an IKEA store), this will usually be the website for the individual store, not the overall chain. |
| `accessibility_options` | String | Information about the accessibility options a place offers. |
| `moved_place` | String | If this Place is permanently closed and has moved to a new Place, this field contains the new Place's resource name, in `places/{place_id}` format. If this Place moved multiple times, this field will represent the first moved place. This field will not be populated if this Place has not moved. |
| `icon_background_color` | String | Background color for icon_mask in hex format, e.g. #909CE1. |
| `postal_address` | String | The address in postal address format. |
| `ev_charge_amenity_summary` | String | The summary of amenities near the EV charging station. |
| `fuel_options` | String | The most recent information about fuel options in a gas station. This information is updated regularly. |
| `address_descriptor` | String | The address descriptor of the place. Address descriptors include additional information that help describe a location using landmarks and areas. See address descriptor regional coverage in https://developers.google.com/maps/documentation/geocoding/address-descriptors/coverage. |
| `pure_service_area_business` | bool | Indicates whether the place is a pure service area business. Pure service area business is a business that visits or delivers to customers directly but does not serve customers at their business address. For example, businesses like cleaning services or plumbers. Those businesses may not have a physical address or location on Google Maps. |
| `editorial_summary` | String | Contains a summary of the place. A summary is comprised of a textual overview, and also includes the language code for these if applicable. Summary text must be presented as-is and can not be modified or altered. |
| `serves_dessert` | bool | Place serves dessert. |
| `regular_secondary_opening_hours` | Vec<String> | Contains an array of entries for information about regular secondary hours of a business. Secondary hours are different from a business's main hours. For example, a restaurant can specify drive through hours or delivery hours as its secondary hours. This field populates the type subfield, which draws from a predefined list of opening hours types (such as DRIVE_THROUGH, PICKUP, or TAKEOUT) based on the types of the place. |
| `google_maps_uri` | String | A URL providing more information about this place. |
| `national_phone_number` | String | A human-readable phone number for the place, in national format. |
| `time_zone` | String | IANA Time Zone Database time zone. For example "America/New_York". |
| `regular_opening_hours` | String | The regular hours of operation. Note that if a place is always open (24 hours), the `close` field will not be set. Clients can rely on always open (24 hours) being represented as an [`open`](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Period) period containing [`day`](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Point) with value `0`, [`hour`](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Point) with value `0`, and [`minute`](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Point) with value `0`. |
| `serves_lunch` | bool | Specifies if the place serves lunch. |
| `good_for_groups` | bool | Place accommodates groups. |
| `good_for_children` | bool | Place is good for children. |
| `location` | String | The position of this place. |
| `sub_destinations` | Vec<String> | A list of sub-destinations related to the place. |
| `payment_options` | String | Payment options the place accepts. If a payment option data is not available, the payment option field will be unset. |
| `price_range` | String | The price range associated with a Place. |
| `viewport` | String | A viewport suitable for displaying the place on an average-sized map. This viewport should not be used as the physical boundary or the service area of the business. |
| `current_opening_hours` | String | The hours of operation for the next seven days (including today). The time period starts at midnight on the date of the request and ends at 11:59 pm six days later. This field includes the special_days subfield of all hours, set for dates that have exceptional hours. |
| `generative_summary` | String | AI-generated summary of the place. |
| `reservable` | bool | Specifies if the place supports reservations. |
| `serves_dinner` | bool | Specifies if the place serves dinner. |
| `short_formatted_address` | String | A short, human-readable address for this place. |
| `current_secondary_opening_hours` | Vec<String> | Contains an array of entries for the next seven days including information about secondary hours of a business. Secondary hours are different from a business's main hours. For example, a restaurant can specify drive through hours or delivery hours as its secondary hours. This field populates the type subfield, which draws from a predefined list of opening hours types (such as DRIVE_THROUGH, PICKUP, or TAKEOUT) based on the types of the place. This field includes the special_days subfield of all hours, set for dates that have exceptional hours. |
| `icon_mask_base_uri` | String | A truncated URL to an icon mask. User can access different icon type by appending type suffix to the end (eg, ".svg" or ".png"). |
| `live_music` | bool | Place provides live music. |
| `rating` | f64 | A rating between 1.0 and 5.0, based on user reviews of this place. |
| `serves_breakfast` | bool | Specifies if the place serves breakfast. |
| `international_phone_number` | String | A human-readable phone number for the place, in international format. |
| `allows_dogs` | bool | Place allows dogs. |
| `formatted_address` | String | A full, human-readable address for this place. |
| `neighborhood_summary` | String | A summary of points of interest near the place. |
| `delivery` | bool | Specifies if the business supports delivery. |
| `curbside_pickup` | bool | Specifies if the business supports curbside pickup. |
| `primary_type` | String | The primary type of the given result. This type must be one of the Places API supported types. For example, "restaurant", "cafe", "airport", etc. A place can only have a single primary type. For the complete list of possible values, see Table A and Table B at https://developers.google.com/maps/documentation/places/web-service/place-types. The primary type may be missing if the place's primary type is not a supported type. When a primary type is present, it is always one of the types in the `types` field. |
| `photos` | Vec<String> | Information (including references) about photos of this place. A maximum of 10 photos can be returned. |
| `address_components` | Vec<String> | Repeated components for each locality level. Note the following facts about the address_components[] array: - The array of address components may contain more components than the formatted_address. - The array does not necessarily include all the political entities that contain an address, apart from those included in the formatted_address. To retrieve all the political entities that contain a specific address, you should use reverse geocoding, passing the latitude/longitude of the address as a parameter to the request. - The format of the response is not guaranteed to remain the same between requests. In particular, the number of address_components varies based on the address requested and can change over time for the same address. A component can change position in the array. The type of the component can change. A particular component may be missing in a later response. |
| `user_rating_count` | i64 | The total number of reviews (with or without text) for this place. |
| `business_status` | String | The business status for the place. |
| `containing_places` | Vec<String> | List of places in which the current place is located. |
| `serves_coffee` | bool | Place serves coffee. |
| `dine_in` | bool | Specifies if the business supports indoor or outdoor seating options. |
| `menu_for_children` | bool | Place has a children's menu. |
| `serves_cocktails` | bool | Place serves cocktails. |
| `attributions` | Vec<String> | A set of data provider that must be shown with this result. |
| `serves_beer` | bool | Specifies if the place serves beer. |
| `utc_offset_minutes` | i64 | Number of minutes this place's timezone is currently offset from UTC. This is expressed in minutes to support timezones that are offset by fractions of an hour, e.g. X hours and 15 minutes. |
| `parking_options` | String | Options of parking provided by the place. |
| `price_level` | String | Price level of the place. |
| `reviews` | Vec<String> | List of reviews about this place, sorted by relevance. A maximum of 5 reviews can be returned. |
| `ev_charge_options` | String | Information of ev charging options. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create place
place = provider.places_api.Place {
}

# Access place outputs
place_id = place.id
place_consumer_alert = place.consumer_alert
place_types = place.types
place_display_name = place.display_name
place_good_for_watching_sports = place.good_for_watching_sports
place_primary_type_display_name = place.primary_type_display_name
place_id = place.id
place_google_maps_links = place.google_maps_links
place_outdoor_seating = place.outdoor_seating
place_review_summary = place.review_summary
place_serves_wine = place.serves_wine
place_takeout = place.takeout
place_name = place.name
place_restroom = place.restroom
place_serves_vegetarian_food = place.serves_vegetarian_food
place_plus_code = place.plus_code
place_moved_place_id = place.moved_place_id
place_serves_brunch = place.serves_brunch
place_adr_format_address = place.adr_format_address
place_website_uri = place.website_uri
place_accessibility_options = place.accessibility_options
place_moved_place = place.moved_place
place_icon_background_color = place.icon_background_color
place_postal_address = place.postal_address
place_ev_charge_amenity_summary = place.ev_charge_amenity_summary
place_fuel_options = place.fuel_options
place_address_descriptor = place.address_descriptor
place_pure_service_area_business = place.pure_service_area_business
place_editorial_summary = place.editorial_summary
place_serves_dessert = place.serves_dessert
place_regular_secondary_opening_hours = place.regular_secondary_opening_hours
place_google_maps_uri = place.google_maps_uri
place_national_phone_number = place.national_phone_number
place_time_zone = place.time_zone
place_regular_opening_hours = place.regular_opening_hours
place_serves_lunch = place.serves_lunch
place_good_for_groups = place.good_for_groups
place_good_for_children = place.good_for_children
place_location = place.location
place_sub_destinations = place.sub_destinations
place_payment_options = place.payment_options
place_price_range = place.price_range
place_viewport = place.viewport
place_current_opening_hours = place.current_opening_hours
place_generative_summary = place.generative_summary
place_reservable = place.reservable
place_serves_dinner = place.serves_dinner
place_short_formatted_address = place.short_formatted_address
place_current_secondary_opening_hours = place.current_secondary_opening_hours
place_icon_mask_base_uri = place.icon_mask_base_uri
place_live_music = place.live_music
place_rating = place.rating
place_serves_breakfast = place.serves_breakfast
place_international_phone_number = place.international_phone_number
place_allows_dogs = place.allows_dogs
place_formatted_address = place.formatted_address
place_neighborhood_summary = place.neighborhood_summary
place_delivery = place.delivery
place_curbside_pickup = place.curbside_pickup
place_primary_type = place.primary_type
place_photos = place.photos
place_address_components = place.address_components
place_user_rating_count = place.user_rating_count
place_business_status = place.business_status
place_containing_places = place.containing_places
place_serves_coffee = place.serves_coffee
place_dine_in = place.dine_in
place_menu_for_children = place.menu_for_children
place_serves_cocktails = place.serves_cocktails
place_attributions = place.attributions
place_serves_beer = place.serves_beer
place_utc_offset_minutes = place.utc_offset_minutes
place_parking_options = place.parking_options
place_price_level = place.price_level
place_reviews = place.reviews
place_ev_charge_options = place.ev_charge_options
```

---


### Photo

Get a photo media with a photo reference string.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `photo_uri` | String | A short-lived uri that can be used to render the photo. |
| `name` | String | The resource name of a photo media in the format: `places/{place_id}/photos/{photo_reference}/media`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access photo outputs
photo_id = photo.id
photo_photo_uri = photo.photo_uri
photo_name = photo.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple place resources
place_0 = provider.places_api.Place {
}
place_1 = provider.places_api.Place {
}
place_2 = provider.places_api.Place {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    place = provider.places_api.Place {
    }
```

---

## Related Documentation

- [GCP Places_api Documentation](https://cloud.google.com/places_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
